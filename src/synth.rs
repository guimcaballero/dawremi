use crate::helpers::*;
use dasp::signal::{self, Signal};

#[derive(Clone, Copy)]
pub struct SynthParams {
    pub attack: f64,
    pub decay: f64,
    pub release: f64,

    pub attack_amplitude: f64,
    pub sustain_amplitude: f64,
}

impl Default for SynthParams {
    fn default() -> Self {
        Self {
            attack: 0.01,
            decay: 0.01,
            release: 0.02,

            attack_amplitude: 1.,
            sustain_amplitude: 0.8,
        }
    }
}

pub struct Synth {
    pub params: SynthParams,
    pub note: Note,
    pub sample_rate: f64,
}
impl Synth {
    pub fn new(params: SynthParams, note: Note, sample_rate: f64) -> Self {
        Self {
            params,
            note,
            sample_rate,
        }
    }
}

impl Synth {
    /// Returns the number of samples that should be taken to pass x seconds
    fn seconds(&self, x: f64) -> usize {
        (self.sample_rate * x) as usize
    }

    fn attack(&self) -> usize {
        self.seconds(self.params.attack)
    }
    fn decay(&self) -> usize {
        self.seconds(self.params.decay)
    }
    fn release(&self) -> usize {
        self.seconds(self.params.release)
    }

    pub fn take(&self, samples: usize) -> Vec<f64> {
        let freq: Frequency = self.note.into();
        let vec: Vec<f64> = signal::rate(self.sample_rate)
            .const_hz(freq.0)
            .sine()
            .take(samples)
            .collect();

        // Make a vec with the volumes and multiply them
        let attack = self.attack();
        let decay = self.decay();
        let release = self.release();
        let attack_sustain_diff = self.params.attack_amplitude - self.params.sustain_amplitude;
        let volume_without_release: Vec<f64> = (0..samples)
            .map(|i| {
                let volume = if i < attack {
                    ((attack - i) as f64 / attack as f64) * self.params.attack_amplitude
                } else if i < attack + decay {
                    self.params.attack_amplitude
                        - ((decay - (i - attack)) as f64 / decay as f64) * attack_sustain_diff
                } else {
                    self.params.sustain_amplitude
                };

                let release_multiplier = if i > samples - release {
                    (samples - i) as f64 / release as f64
                } else {
                    1.
                };

                volume * release_multiplier
            })
            .collect();

        assert_eq!(vec.len(), volume_without_release.len());

        vec.iter()
            .zip(volume_without_release)
            .map(|(val, vol)| val * vol)
            .collect()
    }
}
