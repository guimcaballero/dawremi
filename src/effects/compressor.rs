use super::*;

pub struct Compressor {
    /// Amplitude above which the samples will be compressed
    /// Should be between 0 and 1
    pub threshold: Automation<f64>,
    pub ratio: Automation<f64>,
    pub gain: Automation<f64>,
}
impl Effect for Compressor {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let (left, right) = input.split_sides();

        join_left_and_right_channels(run(self, left), run(self, right))
    }
}

fn run(comp: &Compressor, input: Vec<f64>) -> Vec<f64> {
    input
        .iter()
        .enumerate()
        .map(|(idx, val)| {
            let threshold = comp.threshold.value(idx).abs();
            let gain = comp.gain.value(idx).abs();
            let ratio = comp.ratio.value(idx).abs();

            let absval = val.abs();

            let res = if absval > threshold {
                let extra = absval - threshold;
                val.signum() * (threshold + (extra / ratio))
            } else {
                *val
            };
            res * gain
        })
        .collect()
}
