use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, SampleFormat, StreamConfig};
use std::collections::VecDeque;
use std::sync::mpsc;

pub(crate) struct PlayerConfig {
    pub sample_rate: u32,
    pub sample_format: SampleFormat,
    pub device: Device,
    pub stream_config: StreamConfig,
}

pub(crate) fn get_player_config() -> PlayerConfig {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device
        .default_output_config()
        .expect("failed to get default output config");

    let sample_format = config.sample_format();

    let stream_config: cpal::StreamConfig = config.into();
    let sample_rate = stream_config.sample_rate.0;

    PlayerConfig {
        sample_rate,
        sample_format,
        device,
        stream_config,
    }
}

pub(crate) struct Player {
    pub audio: VecDeque<f64>,
    pub cycle: bool,
}

pub(crate) fn run_player(
    player: Player,
    PlayerConfig {
        sample_rate,
        sample_format,
        device,
        stream_config,
    }: PlayerConfig,
) -> Result<()> {
    match sample_format {
        cpal::SampleFormat::F32 => run::<f32>(&device, &stream_config, player),
        cpal::SampleFormat::I16 => run::<i16>(&device, &stream_config, player),
        cpal::SampleFormat::U16 => run::<u16>(&device, &stream_config, player),
    }
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut player: Player,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    // A channel for indicating when playback has completed.
    let (complete_tx, complete_rx) = mpsc::sync_channel(1);

    // Create and run the stream.
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in output.chunks_mut(channels) {
                let sample = match player.audio.pop_front() {
                    None => {
                        complete_tx.try_send(()).ok();
                        0.0
                    }
                    Some(sample) => {
                        if player.cycle {
                            player.audio.push_back(sample);
                        }
                        sample as f32
                    }
                };
                let value: T = cpal::Sample::from::<f32>(&sample);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        |err| eprintln!("an error occurred on stream: {}", err),
    )?;
    println!("Starting to play");
    stream.play()?;

    // Wait for playback to complete.
    complete_rx.recv().unwrap();
    stream.pause()?;

    Ok(())
}
