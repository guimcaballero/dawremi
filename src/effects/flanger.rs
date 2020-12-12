use super::*;

// From: https://blog.demofox.org/2015/03/16/diy-synth-flange-effect/

pub struct Flanger {
    /// LFO frequency
    pub freq: f64,
    /// How many samples back the flanger goes
    pub sample_amplitude: usize,
}
impl Effect for Flanger {
    fn run(&self, input: Vec<f64>) -> Vec<f64> {
        input
            .iter()
            .enumerate()
            .map(|(i, val)| {
                let offset =
                    self.sample_amplitude as f64 * 0.5 * ((self.freq * i as f64).sin() - 1.);

                if let Some(value) = input.get(i.saturating_sub(offset as usize)) {
                    val * 0.7 + value * 0.3
                } else {
                    *val
                }
            })
            .collect()
    }
}
