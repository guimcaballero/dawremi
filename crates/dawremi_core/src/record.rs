//! Record uses cpal under the hood to record audio

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::io::stdin;
use std::sync::{Arc, Mutex};

use crate::frame::Frame;
use crate::sound_files::io::save_file;

pub fn record_and_save() {
    let audio = record_input().expect("Input could not be recorded");

    let time = {
        use std::time::{SystemTime, UNIX_EPOCH};

        let start = SystemTime::now();
        let since_the_epoch = start
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        since_the_epoch.as_millis()
    };

    // The WAV file we're recording to.
    let path = format!("assets/recorded/recorded_{}.wav", time);

    // NOTE We're grabbing the config again, we should probably try to reuse it
    // Specially when we allow selecting different devices

    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    let stream_config: cpal::StreamConfig = config.into();
    let sample_rate = stream_config.sample_rate.0;
    let bits_per_sample = 24;

    println!("File saved: {}", &path);
    save_file(audio, &path, sample_rate, bits_per_sample);
}

pub fn record_input() -> Result<Vec<Frame>, anyhow::Error> {
    // Use the default host for working with audio devices.
    let host = cpal::default_host();

    // Setup the default input device and stream with the default input config.
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");
    println!("Default input device: {}", device.name()?);
    let config = device
        .default_input_config()
        .expect("Failed to get default input config");

    loop {
        let config = config.clone();
        println!("Press q to start recording");
        loop {
            let mut s = String::new();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if s.trim() == "q" {
                break;
            }
        }
        println!("Begin recording...");

        let vec: Vec<Frame> = vec![];
        let vec = Arc::new(Mutex::new(vec));
        let vec2 = Arc::clone(&vec);

        let err_fn = move |err| {
            println!("an error occurred on stream: {}", err);
        };

        let channels = config.channels();

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32>(data, &vec, channels),
                err_fn,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16>(data, &vec, channels),
                err_fn,
            )?,
            cpal::SampleFormat::U16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<u16>(data, &vec, channels),
                err_fn,
            )?,
        };

        stream.play()?;

        // Let recording go until we finish it
        println!("Press q to finish recording");
        loop {
            let mut s = String::new();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if s.trim() == "q" {
                break;
            }
        }

        drop(stream);
        println!("Recording complete!");
        println!("Keep recording? y/n");
        loop {
            let mut s = String::new();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");
            if s.trim() == "y" {
                return Ok(Arc::try_unwrap(vec2).unwrap().into_inner().unwrap());
            }
            if s.trim() == "n" {
                break;
            }
        }
    }
}

fn write_input_data<T>(input: &[T], vec: &Arc<Mutex<Vec<Frame>>>, channels: u16)
where
    T: cpal::Sample + std::fmt::Debug,
{
    if let Ok(mut vec) = vec.try_lock() {
        for frame in input.chunks(channels.into()) {
            match frame {
                [left, right] => vec.push(Frame::new(left.to_f32() as f64, right.to_f32() as f64)),
                [a] => vec.push(Frame::mono(a.to_f32() as f64)),
                _ => {}
            }
        }
    }
}
