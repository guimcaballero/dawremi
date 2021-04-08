//! Contains some default instruments to generate interesting sounds

use crate::{frame::*, helpers::*, notes::*, signals::asdr::*};

use core::f64::consts::TAU;
use rand::Rng;

pub trait Instrument {
    fn default_asdr(_sample_rate: u32) -> Asdr {
        Asdr::default()
    }
    fn generate(
        &self,
        length: usize,
        frequency: Frequency,
        sample_rate: u32,
        asdr: Asdr,
    ) -> Vec<Frame>;
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
