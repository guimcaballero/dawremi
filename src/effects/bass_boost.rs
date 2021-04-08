use super::*;

// From: https://www.musicdsp.org/en/latest/Filters/235-bass-booster.html

pub struct BassBoost {
    /// Frequency response of the LP (higher value gives a steeper one) [70.0 to 140.0 sounds good]
    pub selectivity: Automation<f64>,
    /// How much of the filtered signal is mixed
    pub bass_ratio: Automation<f64>,
    /// How much of the original signal is mixed
    pub input_ratio: Automation<f64>,
    /// Adjusts the final volume to handle cut-offs (might be good to set dynamically)
    pub gain: Automation<f64>,
}
impl Effect for BassBoost {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let mut cap_left = 0.;
        let mut cap_right = 0.;

        input
            .iter()
            .enumerate()
            .map(|(idx, frame)| {
                let selectivity = self.selectivity.value(idx);
                let bass_ratio = self.bass_ratio.value(idx);
                let input_ratio = self.input_ratio.value(idx);
                let gain = self.gain.value(idx);

                frame.map_left_right(
                    |val: f64| {
                        let gain2 = 1.0 / (selectivity + 1.0);
                        cap_left = (val + cap_left * selectivity) * gain2;
                        ((val * input_ratio + cap_left * bass_ratio) * gain).clamp(-1., 1.)
                    },
                    |val: f64| {
                        let gain2 = 1.0 / (selectivity + 1.0);
                        cap_right = (val + cap_right * selectivity) * gain2;
                        ((val * input_ratio + cap_right * bass_ratio) * gain).clamp(-1., 1.)
                    },
                )
            })
            .collect()
    }
}
