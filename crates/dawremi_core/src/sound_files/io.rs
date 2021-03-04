use crate::frame::*;
use crate::helpers::resampling::resample_frames;
use hound::WavIntoSamples;
use hound::WavReader;
use hound::WavWriter;
use std::fs::*;
use std::io::BufReader;

pub fn open_file(path: &str, sample_rate: u32) -> Vec<Frame> {
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
                return transform_samples_to_frames(
                    processed_file.into_samples::<i32>(),
                    processed_spec.channels,
                    processed_spec.bits_per_sample,
                );
            };
        }

        // Otherwise we resample it, save it as a new file, and return it
        resample_and_save(reader, &processed_filename, sample_rate)
    } else {
        transform_samples_to_frames(
            reader.into_samples::<i32>(),
            spec.channels,
            spec.bits_per_sample,
        )
    }
}

fn transform_samples_to_frames(
    samples: WavIntoSamples<BufReader<File>, i32>,
    num_channels: u16,
    bits_per_sample: u16,
) -> Vec<Frame> {
    samples
        .map(Result::unwrap)
        .map(|val| i_to_f(val, bits_per_sample))
        .collect::<Vec<f64>>()
        .windows(num_channels.into())
        .map(|sample| match sample {
            [left, right] => Frame::new(*left, *right),
            [a, ..] => Frame::mono(*a),
            [] => panic!("Sample has 0 channels"),
        })
        .collect::<Vec<Frame>>()
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

fn resample_and_save(
    reader: WavReader<BufReader<File>>,
    processed_filename: &str,
    sample_rate: u32,
) -> Vec<Frame> {
    let spec = reader.spec();
    let orig = transform_samples_to_frames(
        reader.into_samples::<i32>(),
        spec.channels,
        spec.bits_per_sample,
    );

    let vec = resample_frames(orig, spec.sample_rate as f64, sample_rate as f64);

    save_file(vec.clone(), &processed_filename, sample_rate, 24);

    vec
}

pub fn save_file(audio: Vec<Frame>, path: &str, sample_rate: u32, bits_per_sample: u16) {
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate,
        bits_per_sample,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create(path, spec)
        .unwrap_or_else(|_| panic!("File could not be saved at {}", path));

    for val in audio {
        let val = val.clamp(-1., 1.);

        let left = f_to_i(val.left, bits_per_sample);
        writer
            .write_sample(left)
            .expect("Frame's left value could not be written");
        let right = f_to_i(val.right, bits_per_sample);
        writer
            .write_sample(right)
            .expect("Frame's left value could not be written");
    }
}

fn i_to_f(val: i32, bits_per_sample: u16) -> f64 {
    val as f64 / (1_usize << (bits_per_sample - 1)) as f64
}

fn f_to_i(val: f64, bits_per_sample: u16) -> i32 {
    (val * (1_usize << (bits_per_sample - 1)) as f64) as i32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f_to_i_for_0() {
        assert_eq!(0, f_to_i(0., 16));
        assert_eq!(0, f_to_i(0., 24));
        assert_eq!(0, f_to_i(0., 32));

        assert_eq!(-(1 << 14), f_to_i(-0.5_f64, 16));
        assert_eq!(-(1 << 22), f_to_i(-0.5_f64, 24));
        assert_eq!(-(1 << 30), f_to_i(-0.5_f64, 32));

        assert_eq!(1 << 15, f_to_i(1.0_f64, 16));
        assert_eq!(1 << 23, f_to_i(1.0_f64, 24));
        assert_eq!(i32::MAX, f_to_i(1.0_f64, 32));

        assert_eq!(-(1 << 15), f_to_i(-1.0_f64, 16));
        assert_eq!(-(1 << 23), f_to_i(-1.0_f64, 24));
        assert_eq!(i32::MIN, f_to_i(-1.0_f64, 32));
    }

    #[test]
    fn i_to_f_for_0() {
        assert_eq!(0., i_to_f(0, 16));
        assert_eq!(0., i_to_f(0, 24));
        assert_eq!(0., i_to_f(0, 32));

        assert_eq!(-0.5_f64, i_to_f(-(1 << 14), 16));
        assert_eq!(-0.5_f64, i_to_f(-(1 << 22), 24));
        assert_eq!(-0.5_f64, i_to_f(-(1 << 30), 32));

        assert_eq!(1.0_f64, i_to_f(1 << 15, 16));
        assert_eq!(1.0_f64, i_to_f(1 << 23, 24));
        assert!((i_to_f(i32::MAX, 32) - 1.0_f64).abs() < 0.00000001);

        assert_eq!(-1.0_f64, i_to_f(-(1 << 15), 16));
        assert_eq!(-1.0_f64, i_to_f(-(1 << 23), 24));
        assert_eq!(-1.0_f64, i_to_f(i32::MIN, 32));
    }
}
