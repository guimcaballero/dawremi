use cpal;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

use crate::song::{Audio, Song};

pub fn start<S>(song: S) -> Result<(), anyhow::Error>
where
    S: Song,
{
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32, S>(&device, &config.into(), song)?,
        cpal::SampleFormat::I16 => run::<i16, S>(&device, &config.into(), song)?,
        cpal::SampleFormat::U16 => run::<u16, S>(&device, &config.into(), song)?,
    }

    Ok(())
}

fn run<T, S>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    song: S,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
    S: Song,
{
    let mut synth = song.play(config.sample_rate.0 as f64, config.sample_rate.0 as usize);

    // A channel for indicating when playback has completed.
    let (complete_tx, complete_rx) = mpsc::sync_channel(1);

    // Create and run the stream.
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels, &complete_tx, &mut synth)
        },
        err_fn,
    )?;
    stream.play()?;

    // Wait for playback to complete.
    complete_rx.recv().unwrap();
    stream.pause()?;

    Ok(())
}

fn write_data<T>(
    output: &mut [T],
    channels: usize,
    complete_tx: &mpsc::SyncSender<()>,
    signal: &mut Audio,
) where
    T: cpal::Sample,
{
    for frame in output.chunks_mut(channels) {
        let sample = match signal.next() {
            None => {
                complete_tx.try_send(()).ok();
                0.0
            }
            Some(sample) => sample,
        };
        let value: T = cpal::Sample::from::<f32>(&sample);
        for sample in frame.iter_mut() {
            *sample = value;
        }
    }
}
