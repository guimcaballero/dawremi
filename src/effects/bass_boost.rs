use super::*;

// From: https://www.musicdsp.org/en/latest/Filters/235-bass-booster.html

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
