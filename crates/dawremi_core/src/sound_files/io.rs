use dasp::{signal, Signal};
use hound::WavReader;
use hound::WavWriter;
use std::fs::*;
use std::io::BufReader;

pub fn open_file(path: &str, sample_rate: u32) -> Vec<f64> {
    let reader =
        hound::WavReader::open(path).unwrap_or_else(|_| panic!("File {} should exist", path));
    let spec = reader.spec();

    // Check if the file has the same sample rate as the song
    // If it doesn't we resample the file
    // If it does, we just return the file
    if spec.sample_rate != sample_rate {
        let processed_filename = format!(
            "assets/processed/{}-{}",
            sample_rate,
            path.replace("/", "_")
        );

        // If the processed file exists and hasn't been modified, just return it
        let modified = has_file_been_modified(path, &processed_filename);
        if metadata(&processed_filename).is_ok() && !modified {
            if let Ok(processed_file) = hound::WavReader::open(&processed_filename) {
                let processed_spec = processed_file.spec();
                return processed_file
                    .into_samples::<i32>()
                    // NOTE Eventually this will be removed when we implement stereo
                    .step_by(processed_spec.channels.into())
                    .map(Result::unwrap)
                    .map(|val| sample(val, processed_spec.bits_per_sample))
                    .collect::<Vec<f64>>();
            };
        }

        // Otherwise we resample it, save it as a new file, and return it
        resample_and_save(reader, &processed_filename, sample_rate)
    } else {
        reader
            .into_samples::<i32>()
            // NOTE Eventually this will be removed when we implement stereo
            .step_by(spec.channels.into())
            .map(Result::unwrap)
            .map(|val| sample(val, spec.bits_per_sample))
            .collect::<Vec<f64>>()
    }
}

/// Returns true if file at path1 has been changed after path2
fn has_file_been_modified(path1: &str, path2: &str) -> bool {
    // Okay this code is horrible, but my brain isn't working well now
    // I'll fix later to make it nicer

    if let Ok(metadata1) = metadata(path1) {
        if let Ok(metadata2) = metadata(path2) {
            if let Ok(time1) = metadata1.modified() {
                if let Ok(time2) = metadata2.modified() {
                    return time1 > time2;
                }
            }
        }
    }

    false
}

fn sample(val: i32, bits_per_sample: u16) -> f64 {
    val as f64 / 2f64.powi(bits_per_sample as i32)
}

fn resample_and_save(
    reader: WavReader<BufReader<File>>,
    processed_filename: &str,
    sample_rate: u32,
) -> Vec<f64> {
    let spec = reader.spec();
    let orig = reader
        .into_samples::<i32>()
        // NOTE Eventually this will be removed when we implement stereo
        .step_by(spec.channels.into())
        .map(Result::unwrap)
        .map(|val| sample(val, spec.bits_per_sample));

    // Convert the signal's sample rate using `Sinc` interpolation.
    use dasp::{interpolate::sinc::Sinc, ring_buffer};
    let signal = signal::from_interleaved_samples_iter(orig);
    let ring_buffer = ring_buffer::Fixed::from([[0.0f64]; 100]);
    let sinc = Sinc::new(ring_buffer);
    let new_signal = signal.from_hz_to_hz(sinc, spec.sample_rate as f64, sample_rate as f64);

    let vec = new_signal
        .until_exhausted()
        .map(|frame| frame[0])
        .collect::<Vec<f64>>();

    save_file(vec.clone(), &processed_filename, sample_rate);

    vec
}

pub fn save_file(audio: Vec<f64>, path: &str, sample_rate: u32) {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(path, spec)
        .unwrap_or_else(|_| panic!("File could not be saved at {}", path));

    for i in audio {
        let val = i as f32;
        let value: i16 = cpal::Sample::from::<f32>(&val);

        writer
            .write_sample(value)
            .expect("Sample could not be written");
    }
}
