use super::*;
use crate::synth::*;

// Twinkle Twinkle
//
// Demo of using the sequence! macro to declare notes for a song
// Displays it's use for both arbitrary functions and signals
//
// Uses a harmonica synth and a custom signal that generates a Sine wave

song!(TwinkleTwinkle,);

impl Song for TwinkleTwinkle {
    fn name(&self) -> &'static str {
        "Twinkle Twinkle"
    }

    fn bpm(&self) -> usize {
        120
    }
    fn duration(&self) -> usize {
        self.beats(8. * 6.)
    }

    fn track1(&self) -> Option<Vec<f64>> {
        Some(sequence!(@lyrics
                self,
                len: 0.5,
                fun: |note| self.plucked(note),

                [twin-kle  twin-kle  lit-tle star],
                (G4 __ G4 __ D4 __ D4 __ E4 __ E4 __ (D4 * 2.) __ __),
                [how  I    won-der  how  you  are],
                (C4 __ C4 __ B4 __ B4 __ A4 __ A4 __ (G4 * 2.) __ __),
                (D4 __ D4 __ C4 __ C4 __ B4 __ B4 __ (A4 * 2.) __ __),
                (D4 __ D4 __ C4 __ C4 __ B4 __ B4 __ (A4 * 2.) __ __),
                (G4 __ G4 __ D4 __ D4 __ E4 __ E4 __ (D4 * 2.) __ __),
                (C4 __ C4 __ B4 __ B4 __ A4 __ A4 __ (G4 * 2.) __ __),
        ))
    }

    #[allow(unreachable_code)]
    fn track2(&self) -> Option<Vec<f64>> {
        return None;
        let sign = CustomSignal {
            sample: 0,
            sample_rate: self.get_sample_rate(),
        };
        Some(sequence!(@lyrics
                self,
                len: 0.5, signal: sign,

                [twin-kle  twin-kle  lit-tle star],
                (x _ x _ x _ x _ x _ x _ x x _ _),
                [how  I    won-der  how  you  are],
                (x _ x _ x _ x _ x _ x _ x x _ _),
                (x _ x _ x _ x _ x _ x _ x x _ _),
                (x _ x _ x _ x _ x _ x _ x x _ _),
                (x _ x _ x _ x _ x _ x _ x x _ _),
                (x _ x _ x _ x _ x _ x _ x x _ _),
        ))
    }
}

impl TwinkleTwinkle {
    fn synth(&self, note: Note) -> Synth {
        Synth::new(
            box Harmonica::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
    fn plucked(&self, note: Note) -> Synth {
        Synth::new(
            box Plucked::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
}

#[derive(Default, Copy, Clone)]
struct CustomSignal {
    pub sample_rate: f64,
    pub sample: usize,
}

const PI_4: f64 = core::f64::consts::PI * 2.0;
impl Signal for CustomSignal {
    type Frame = f64;

    #[inline]
    fn next(&mut self) -> Self::Frame {
        let freq = 220.;

        let phase = self.sample as f64 * (freq / self.sample_rate);

        self.sample += 1;
        (PI_4 * phase).sin()
    }
}
