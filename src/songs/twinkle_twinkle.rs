#![allow(dead_code)]

use dawremi_core::prelude::*;

// Twinkle Twinkle
//
// Demo of using the sequence! macro to declare notes for a song
// Displays it's use for both arbitrary functions and signals

song!(TwinkleTwinkle,);

impl Song for TwinkleTwinkle {
    fn name(&self) -> &'static str {
        "Twinkle Twinkle"
    }
    fn bpm(&self) -> usize {
        240
    }
    fn duration(&self) -> usize {
        self.beats(8. * 6.)
    }
    fn tracks(&mut self) -> Vec<Vec<Frame>> {
        vec![self.main_track(), self.synth_track(), self.bass_track()]
    }
}

impl TwinkleTwinkle {
    fn main_track(&self) -> Vec<Frame> {
        sequence!(@lyrics
                self,
                len: 1.,
                fun: |note| self.plucked(note),

                [twin-kle  twin-kle  lit-tle star],
                (G4 __ G4 __ D4 __ D4 __ E4 __ E4 __ (D4 * 2.) __ __),
                [how  I    won-der  how  you  are],
                (C4 __ C4 __ B4 __ B4 __ A4 __ A4 __ (G4 * 2.) __ __),
                (D4 __ D4 __ C4 __ C4 __ B4 __ B4 __ (A4 * 2.) __ __),
                (D4 __ D4 __ C4 __ C4 __ B4 __ B4 __ (A4 * 2.) __ __),
                (G4 __ G4 __ D4 __ D4 __ E4 __ E4 __ (D4 * 2.) __ __),
                (C4 __ C4 __ B4 __ B4 __ A4 __ A4 __ (G4 * 2.) __ __),
        )
    }

    fn bass_track(&self) -> Vec<Frame> {
        sequence!(@lyrics
                  self,
                  len: 1., fun: |note| self.bass(note),

                  [twin-kle  twin-kle  lit-tle star],
                  (G4 __ G4 __ D4 __ D4 __ E4 __ E4 __ (D4 * 2.) __ __),
                  [how  I    won-der  how  you  are],
                  (C4 __ C4 __ B4 __ B4 __ A4 __ A4 __ (G4 * 2.) __ __),
                  (D4 __ D4 __ C4 __ C4 __ B4 __ B4 __ (A4 * 2.) __ __),
                  (D4 __ D4 __ C4 __ C4 __ B4 __ B4 __ (A4 * 2.) __ __),
                  (G4 __ G4 __ D4 __ D4 __ E4 __ E4 __ (D4 * 2.) __ __),
                  (C4 __ C4 __ B4 __ B4 __ A4 __ A4 __ (G4 * 2.) __ __),
        )
    }

    fn synth_track(&self) -> Vec<Frame> {
        sequence!(@lyrics
                self,
                len: 1., fun: |note| self.harmonica(note),

                [twin-kle  twin-kle  lit-tle star],
                (G4 __ G4 __ D4 __ D4 __ E4 __ E4 __ (D4 * 2.) __ __),
                [how  I    won-der  how  you  are],
                (C4 __ C4 __ B4 __ B4 __ A4 __ A4 __ (G4 * 2.) __ __),
                (D4 __ D4 __ C4 __ C4 __ B4 __ B4 __ (A4 * 2.) __ __),
                (D4 __ D4 __ C4 __ C4 __ B4 __ B4 __ (A4 * 2.) __ __),
                (G4 __ G4 __ D4 __ D4 __ E4 __ E4 __ (D4 * 2.) __ __),
                (C4 __ C4 __ B4 __ B4 __ A4 __ A4 __ (G4 * 2.) __ __),
        )
        .effect(&Volume { mult: 0.5 })
    }

    fn harmonica(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            box Harmonica::new(frequency.into(), self.get_sample_rate()),
            self.get_sample_rate(),
        )
    }
    fn plucked(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            box Plucked::new(
                InitialBurstType::Random,
                frequency.into(),
                self.get_sample_rate(),
            ),
            self.get_sample_rate(),
        )
    }
    fn bass(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            box Plucked::new(
                InitialBurstType::DoubleTriangle,
                frequency.into(),
                self.get_sample_rate(),
            ),
            self.get_sample_rate(),
        )
    }
}
