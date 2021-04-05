//! `Synth` implements a simple ASDR envelope for `SynthInstruments`

use crate::frame::*;
use crate::helpers::*;
use crate::notes::*;
use rand::prelude::*;
use std::f64::consts::TAU;

pub struct Synth {
    pub instrument: Box<dyn SynthInstrument>,
    pub sample_rate: u32,
}
impl Synth {
    pub fn new(instrument: Box<dyn SynthInstrument>, sample_rate: u32) -> Self {
        Self {
            instrument,
            sample_rate,
        }
    }

    pub fn take_samples(&mut self, samples: usize) -> Vec<Frame> {
        let params = self.instrument.get_params();
        let vec: Vec<Frame> = self.instrument.take_samples(samples);

        // Make a vec with the volumes and multiply them
        let attack = params.attack;
        let decay = params.decay;
        let release = params.release;

        let attack_sustain_diff = params.attack_amplitude - params.sustain_amplitude;
        let samples_release_diff = samples.checked_sub(release).unwrap_or_default();

        let volume_without_release: Vec<Frame> = (0..samples)
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
            .collect::<Vec<f64>>()
            .into_frames();

        assert_eq!(vec.len(), volume_without_release.len());

        vec.multiply(&volume_without_release)
    }
}

#[derive(Clone, Copy)]
pub struct SynthParams {
    pub attack: usize,
    pub decay: usize,
    pub release: usize,

    pub attack_amplitude: f64,
    pub sustain_amplitude: f64,
}

pub struct SynthGroup(pub Vec<Synth>);
impl SynthGroup {
    pub fn take_samples(&mut self, samples: usize) -> Vec<Frame> {
        join_tracks(
            self.0
                .iter_mut()
                .map(|synth| synth.take_samples(samples))
                .collect(),
        )
    }
}

// Okay, this is a bit weird, but I want to auto implement a couple functions, so I'm splitting the
// trait into 2, and one of them gets implemented in the macro. There's probably a better way to do this,
// but I can't really think of any right now

pub trait SynthInstrument: HasSample {
    fn get_params(&self) -> SynthParams;
    fn frame(&mut self) -> Frame;

    fn time(&self) -> f64 {
        self.sample() / self.sample_rate()
    }
    fn take_samples(&mut self, samples: usize) -> Vec<Frame> {
        (0..samples)
            .map(|_| {
                self.increase_sample();
                self.frame()
            })
            .collect()
    }
}
pub trait HasSample {
    fn sample(&self) -> f64;
    fn sample_rate(&self) -> f64;
    fn increase_sample(&mut self);

    fn seconds(&self, x: f64) -> usize {
        (self.sample_rate() * x) as usize
    }
}

#[macro_export]
macro_rules! simple_instrument {
    ($name:ident) => {
        #[derive(Clone, Copy)]
        pub struct $name {
            frequency: Frequency,
            sample_rate: u32,
            sample: usize,
        }

        impl $name {
            #[allow(dead_code)]
            pub fn new(frequency: Frequency, sample_rate: u32) -> Self {
                Self {
                    frequency,
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
                self.sample_rate as f64
            }
            fn increase_sample(&mut self) {
                self.sample += 1;
            }
        }
    };
}

#[macro_export]
macro_rules! instrument {
    ($name:ident $(, $id:ident : $type:ty )* $(,)?) => {
        #[derive(Clone)]
        pub struct $name {
            frequency: Frequency,
            sample_rate: u32,
            sample: usize,
            $( $id: $type, )*
        }

        impl HasSample for $name {
            fn sample(&self) -> f64 {
                self.sample as f64
            }
            fn sample_rate(&self) -> f64 {
                self.sample_rate as f64
            }
            fn increase_sample(&mut self) {
                self.sample += 1;
            }
        }
    };
}

mod harmonica;
pub use harmonica::Harmonica;
mod bell;
pub use bell::Bell;
mod drum_kick;
pub use drum_kick::DrumKick;
mod drum_snare;
pub use drum_snare::DrumSnare;
mod drum_hihat;
pub use drum_hihat::DrumHiHat;
mod plucked;
pub use plucked::{InitialBurstType, Plucked};
