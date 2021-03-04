use crate::frame::Frame;
use crate::helpers::*;

// TODO Currently this is the only place we use Dasp. We could probably replace it with something else
use dasp::{interpolate::sinc::Sinc, ring_buffer};
use dasp::{signal, Signal};

pub fn resample(frames: Vec<f64>, old_sample_rate: f64, new_sample_rate: f64) -> Vec<f64> {
    // Convert the signal's sample rate using `Sinc` interpolation.

    let signal = signal::from_interleaved_samples_iter(frames);
    let ring_buffer = ring_buffer::Fixed::from([[0.0f64]; 100]);
    let sinc = Sinc::new(ring_buffer);
    let new_signal = signal.from_hz_to_hz(sinc, old_sample_rate, new_sample_rate);
    new_signal
        .until_exhausted()
        .map(|frame| frame[0])
        .collect::<Vec<f64>>()
}

pub fn resample_frames(
    frames: Vec<Frame>,
    old_sample_rate: f64,
    new_sample_rate: f64,
) -> Vec<Frame> {
    // Get each side
    let (left, right) = frames.split_sides();

    // Resample them
    let left = resample(left, old_sample_rate, new_sample_rate);
    let right = resample(right, old_sample_rate, new_sample_rate);

    // Join them together
    join_left_and_right_channels(left, right)
}
