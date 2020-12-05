use super::*;
use crate::synth::*;

song!(Test,);

impl Song for Test {
    fn name(&self) -> &'static str {
        "test"
    }

    fn bpm(&self) -> usize {
        120
    }
    fn duration(&self) -> usize {
        self.beats(8. * 6.)
    }

    fn track1(&self) -> Option<Vec<f64>> {
        let params = SynthParams::default();
        Some(sequence!(@lyrics
            self,
            len: 0.5,
            fun: |note| Synth::new(params, note, self.get_sample_rate()),

            [twin-kle  twin-kle  lit-tle star],
            (G _ G _ D _ D _ E _ E _ (D * 2.) _ _),

            [how  I    won-der  how  you  are],
            (C _ C _ B _ B _ A _ A _ (G * 2.) _ _),

            (D _ D _ C _ C _ B _ B _ (A * 2.) _ _),
            (D _ D _ C _ C _ B _ B _ (A * 2.) _ _),
            (G _ G _ D _ D _ E _ E _ (D * 2.) _ _),
            (C _ C _ B _ B _ A _ A _ (G * 2.) _ _),
        ))
    }

    // fn track2(&self) -> Option<Vec<f64>> {
    //     let sign = CustomSignal {
    //         sample: 0,
    //         sample_rate: self.get_sample_rate(),
    //     };
    //     Some(sequence!(@lyrics
    //         self,
    //         len: 0.5, signal: sign,

    //         [twin-kle  twin-kle  lit-tle star],
    //         (x _  x _  x _  x _  x _ x _ x x _ _),

    //         [how  I    won-der  how  you  are],
    //         (x _  x _  x _ x _  x _  x _  x x _ _),

    //         (x _ x _ x _ x _ x _ x _ x x _ _),
    //         (x _ x _ x _ x _ x _ x _ x x _ _),
    //         (x _ x _ x _ x _ x _ x _ x x _ _),
    //         (x _ x _ x _ x _ x _ x _ x x _ _),
    //     ))
    // }
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
