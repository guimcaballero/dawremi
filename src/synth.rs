use crate::helpers::*;
use rand::prelude::*;

const PI_4: f64 = core::f64::consts::PI * 2.0;

pub struct Synth {
    pub instrument: Box<dyn SynthInstrument>,
    pub note: Note,
    pub sample_rate: f64,
}
impl Synth {
    pub fn new(instrument: Box<dyn SynthInstrument>, note: Note, sample_rate: f64) -> Self {
        Self {
            instrument,
            note,
            sample_rate,
        }
    }

    /// Returns the number of samples that should be taken to pass x seconds
    fn seconds(&self, x: f64) -> usize {
        (self.sample_rate * x) as usize
    }

    pub fn take_samples(&mut self, samples: usize) -> Vec<f64> {
        let params = self.instrument.get_params();
        let vec: Vec<f64> = self.instrument.take_samples(samples);

        // Make a vec with the volumes and multiply them
        let attack = self.seconds(params.attack);
        let decay = self.seconds(params.decay);
        let release = self.seconds(params.release);

        let attack_sustain_diff = params.attack_amplitude - params.sustain_amplitude;
        let volume_without_release: Vec<f64> = (0..samples)
            .map(|i| {
                let volume = if i < attack {
                    (i as f64 / attack as f64) * params.attack_amplitude
                } else if i < attack + decay {
                    params.attack_amplitude
                        - ((i - attack) as f64 / decay as f64) * attack_sustain_diff
                } else {
                    params.sustain_amplitude
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

#[derive(Clone, Copy)]
pub struct SynthParams {
    attack: f64,
    decay: f64,
    release: f64,

    attack_amplitude: f64,
    sustain_amplitude: f64,
}

pub trait SynthInstrument: HasSample {
    fn get_params(&self) -> SynthParams;
    fn note(&mut self) -> f64;

    fn time(&self) -> f64 {
        PI_4 * self.sample() / self.sample_rate()
    }
    fn take_samples(&mut self, samples: usize) -> Vec<f64> {
        (0..samples).map(|_| self.note()).collect()
    }
}
pub trait HasSample {
    fn sample(&self) -> f64;
    fn sample_rate(&self) -> f64;
}

macro_rules! instrument {
    ($name:ident, $( $id:ident : $type:ty ),*) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            note: Note,
            sample_rate: f64,
            sample: usize,
            $( $id: $type )*
        }

        impl $name {
            pub fn new(note: Note, sample_rate: f64) -> Self {
                Self {
                    note,
                    sample_rate,
                    sample: 0,
                }
            }
        }

        impl HasSample for $name {
            fn sample(&self) -> f64 {
                self.sample as f64
            }
            fn sample_rate(&self) -> f64 {
                self.sample_rate
            }
        }
    };
}

instrument!(Harmonica,);
impl SynthInstrument for Harmonica {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: 0.1,
            decay: 0.1,
            release: 0.2,

            attack_amplitude: 1.0,
            sustain_amplitude: 0.8,
        }
    }

    fn note(&mut self) -> f64 {
        let freq: Frequency = self.note.into();
        self.sample += 1;
        let a_lfo = 0.005;
        let f_lfo = 7.0;

        let square_1 =
            if (freq.0 * self.time() + a_lfo * freq.0 * (f_lfo * self.time()).sin()).sin() > 0. {
                1.
            } else {
                -1.
            };
        let square_2 = if (freq.0 * 1.5 * self.time()).sin() > 0. {
            1.
        } else {
            -1.
        };
        let square_3 = if (freq.0 * 2.0 * self.time()).sin() > 0. {
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
