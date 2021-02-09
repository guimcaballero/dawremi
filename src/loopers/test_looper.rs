#![allow(dead_code)]

use dawremi_core::prelude::*;

song!(Test,);

impl Looper for Test {
    fn name(&self) -> &'static str {
        "test looper"
    }

    fn bpm(&self) -> usize {
        180
    }
    fn tracks(&mut self) -> Vec<Vec<f64>> {
        vec![
            // self.kick(),
            self.snare(),
            // self.hihat(),
        ]
    }
}

impl Test {
    fn kick(&mut self) -> Vec<f64> {
        sequence!(
            self,
            len: 1.,
            signal: kick(self),

            x _ x _
        )
    }

    fn snare(&mut self) -> Vec<f64> {
        sequence!(
            self,
            len: 1., note: Note,
            fun: |note| snare(self, note),

            _ C1 _ _
        )
    }

    fn hihat(&mut self) -> Vec<f64> {
        sequence!(
            self,
            len: 1., note: Note,
            fun: |note| hihat(self, note),

            _ C4 _ _
        )
    }
}

fn kick(song: &dyn HasSampleRate) -> Synth {
    Synth::new(
        box DrumKick::new(Note::C1.into(), song.get_sample_rate()),
        song.get_sample_rate(),
    )
}
fn snare(song: &dyn HasSampleRate, frequency: impl Into<Frequency>) -> Synth {
    Synth::new(
        box DrumSnare::new(frequency.into(), song.get_sample_rate()),
        song.get_sample_rate(),
    )
}
fn hihat(song: &dyn HasSampleRate, frequency: impl Into<Frequency>) -> Synth {
    Synth::new(
        box DrumHiHat::new(frequency.into(), song.get_sample_rate()),
        song.get_sample_rate(),
    )
}
