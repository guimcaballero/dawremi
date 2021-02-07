use super::*;
use core::f64::consts::PI;

// From https://github.com/tesselode/kira/blob/main/kira/src/mixer/effect/filter.rs
// Also https://github.com/wrl/baseplug/blob/trunk/examples/svf/svf_simper.rs

pub struct Filter {
    pub mode: FilterMode,

    pub sample_rate: f64,

    /// The cutoff frequency of the filter (in hertz).
    pub cutoff: f64,

    /// The resonance of the filter.
    ///
    /// The resonance is a feedback effect that produces
    /// a distinctive "ringing" sound.
    pub resonance: f64,
}

pub enum FilterMode {
    /// Removes frequencies above the cutoff frequency.
    LowPass,
    /// Removes frequencies above and below the cutoff frequency.
    BandPass,
    /// Removes frequencies below the cutoff frequency.
    HighPass,
    /// Removes frequencies around the cutoff frequency.
    Notch,
}

impl Effect for Filter {
    fn run(&self, input: Vec<f64>) -> Vec<f64> {
        let mut ic1eq: f64 = 0.;
        let mut ic2eq: f64 = 0.;

        let g = (PI * (self.cutoff / self.sample_rate)).tan();
        let k = 2.0 - (1.9 * self.resonance.min(1.0).max(0.0));

        let a1 = 1.0 / (1.0 + (g * (g + k)));
        let a2 = g * a1;
        let a3 = g * a2;

        input
            .iter()
            .map(|val| {
                let v3 = val - ic2eq;
                let v1 = (ic1eq * (a1)) + (v3 * (a2));
                let v2 = ic2eq + (ic1eq * (a2)) + (v3 * (a3));
                ic1eq = (v1 * 2.0) - ic1eq;
                ic2eq = (v2 * 2.0) - ic2eq;
                match self.mode {
                    FilterMode::LowPass => v2,
                    FilterMode::BandPass => v1,
                    FilterMode::HighPass => val - v1 * k - v2,
                    FilterMode::Notch => val - v1 * k,
                }
            })
            .collect()
    }
}
