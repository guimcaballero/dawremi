pub trait EffectExtension {
    fn effect(self, effect: &impl Effect) -> Vec<f64>;
}
impl EffectExtension for Vec<f64> {
    fn effect(self, effect: &impl Effect) -> Vec<f64> {
        effect.run(self)
    }
}

pub trait Effect {
    fn run(&self, input: Vec<f64>) -> Vec<f64>;
}

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

pub struct BassBoost {
    /// frequency response of the LP (higher value gives a steeper one) [70.0 to 140.0 sounds good]
    pub selectivity: f64,
    /// how much of the filtered signal is mixed to the original
    pub ratio: f64,
    /// adjusts the final volume to handle cut-offs (might be good to set dynamically)
    pub gain: f64,
}
impl Effect for BassBoost {
    fn run(&self, input: Vec<f64>) -> Vec<f64> {
        let mut cap = 0.;

        input
            .iter()
            .map(|val| {
                let gain1 = 1.0 / (self.selectivity + 1.0);
                cap = (val + cap * self.selectivity) * gain1;
                ((val + cap * self.ratio) * self.gain).clamp(-1., 1.)
            })
            .collect()
    }
}
