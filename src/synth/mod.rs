use crate::notes::*;
use core::f64::consts::TAU;

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
        let samples_release_diff = samples.checked_sub(release).unwrap_or_default();

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

                let release_multiplier = if i > samples_release_diff {
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
        TAU * self.sample() / self.sample_rate()
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
            #[allow(dead_code)]
            pub fn new(note: Note, sample_rate: f64, $( $id: $type, )*) -> Self {
                Self {
                    note,
                    sample_rate,
                    sample: 0,
                    $($id,)*
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

mod harmonica;
pub use harmonica::Harmonica;
mod bell;
pub use bell::Bell;
