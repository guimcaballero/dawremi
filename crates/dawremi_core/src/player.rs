use crate::sound_files::io::save_file;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::sync::mpsc;

use crate::song::Song;

pub fn start_song(song: Box<dyn Song>) -> Result<(), anyhow::Error> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .expect("failed to find a default output device");
    let config = device.default_output_config()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), song)?,
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), song)?,
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), song)?,
    }

    Ok(())
}

fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    mut song: Box<dyn Song>,
) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let sample_rate = config.sample_rate.0;
    song.set_sample_rate(sample_rate as f64);
    let mut song_audio = song.generate();

    // Save to a file
    save_file(
        song_audio.clone().collect(),
        &format!("output/{}.wav", song.name()),
        sample_rate,
    );

    // A channel for indicating when playback has completed.
    let (complete_tx, complete_rx) = mpsc::sync_channel(1);

    // Create and run the stream.
    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);
    let channels = config.channels as usize;
    let stream = device.build_output_stream(
        config,
        move |output: &mut [T], _: &cpal::OutputCallbackInfo| {
            for frame in output.chunks_mut(channels) {
                let sample = match &song_audio.next() {
                    None => {
                        complete_tx.try_send(()).ok();
                        0.0
                    }
                    Some(sample) => *sample as f32,
                };
                let value: T = cpal::Sample::from::<f32>(&sample);
                for sample in frame.iter_mut() {
                    *sample = value;
                }
            }
        },
        err_fn,
    )?;
    println!("Playing song: {}", song.name());
    stream.play()?;

    // Wait for playback to complete.
    complete_rx.recv().unwrap();
    stream.pause()?;

    Ok(())
}
