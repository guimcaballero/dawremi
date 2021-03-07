use crate::frame::Frame;
use crate::helpers::*;

// TODO Currently this is the only place we use Dasp. We could probably replace it with something else
use dasp::interpolate::linear::Linear;
use dasp::{signal, Signal};

/// Passing factor = 2 will make the audio length be twice as long
pub fn stretch(frames: Vec<f64>, factor: f64) -> Vec<f64> {
    let len = (frames.len() as f64 * factor) as usize;

    let mut source = signal::from_iter(frames);
    let a = source.next();
    let b = source.next();
    let interp = Linear::new(a, b);

    source.scale_hz(interp, 1. / factor).take(len).collect()
}

/// Passing factor = 2 will make the audio length be twice as long
pub fn stretch_frames(frames: Vec<Frame>, factor: f64) -> Vec<Frame> {
    // Get each side
    let (left, right) = frames.split_sides();

    // Resample them
    let left = stretch(left, factor);
    let right = stretch(right, factor);

    // Join them together
    join_left_and_right_channels(left, right)
}

pub fn resample(frames: Vec<f64>, old_sample_rate: f64, new_sample_rate: f64) -> Vec<f64> {
    stretch(frames, new_sample_rate / old_sample_rate)
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_resample_vector() {
        let smth = vec![0., 1., 1., 0.5, 0.];
        let result = resample(smth, 1., 2.);

        assert_eq!(vec![0., 0.5, 1., 1., 1., 0.75, 0.5, 0.25, 0., 0.], result);
    }

    #[test]
    fn can_stretch_vector() {
        let smth = vec![0., 1., 1., 0.5, 0.];
        let result = stretch(smth, 2.);

        assert_eq!(vec![0., 0.5, 1., 1., 1., 0.75, 0.5, 0.25, 0., 0.], result);
    }

    #[test]
    fn can_stretch_vector_with_1() {
        let smth = vec![0., 1., 1., 0.5, 0.];
        let result = stretch(smth.clone(), 1.);

        assert_eq!(smth, result);
    }
}
