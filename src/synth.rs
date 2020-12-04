use crate::helpers::*;
use dasp::Signal;
use rand::prelude::*;

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
            attack: 0.1,
            decay: 0.1,
            release: 0.2,

            attack_amplitude: 1.0,
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

    pub fn take_samples(&self, samples: usize) -> Vec<f64> {
        let vec: Vec<f64> = SynthSignal::new(self.note, self.sample_rate).take_samples(samples);

        // Make a vec with the volumes and multiply them
        let attack = self.attack();
        let decay = self.decay();
        let release = self.release();
        let attack_sustain_diff = self.params.attack_amplitude - self.params.sustain_amplitude;
        let volume_without_release: Vec<f64> = (0..samples)
            .map(|i| {
                let volume = if i < attack {
                    (i as f64 / attack as f64) * self.params.attack_amplitude
                } else if i < attack + decay {
                    self.params.attack_amplitude
                        - ((i - attack) as f64 / decay as f64) * attack_sustain_diff
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

// TODO Change this to be a trait or an enum so we can make different instruments

#[derive(Copy, Clone)]
/// Iterator for the synth
struct SynthSignal {
    note: Note,
    sample_rate: f64,
    sample: usize,
}
impl SynthSignal {
    fn new(note: Note, sample_rate: f64) -> Self {
        Self {
            note,
            sample_rate,
            sample: 0,
        }
    }
}

const PI_4: f64 = core::f64::consts::PI * 2.0;
impl Signal for SynthSignal {
    type Frame = f64;

    #[inline]
    fn next(&mut self) -> Self::Frame {
        self.sample += 1;

        let freq: Frequency = self.note.into();
        let a_lfo = 0.005;
        let f_lfo = 7.0;

        let time = PI_4 * self.sample as f64 / self.sample_rate;

        // Start

        // This is kind of a harmonica

        let square_1 = if (freq.0 * time + a_lfo * freq.0 * (f_lfo * time).sin()).sin() > 0. {
            1.
        } else {
            -1.
        };
        let square_2 = if (freq.0 * 1.5 * time).sin() > 0. {
            1.
        } else {
            -1.
        };
        let square_3 = if (freq.0 * 2.0 * time).sin() > 0. {
            1.
        } else {
            -1.
        };

        0.02 * square_1
            + 0.5 * square_2
            + 0.25 * square_3
            + 0.01 * rand::thread_rng().gen_range(-1., 1.)
    }
}
