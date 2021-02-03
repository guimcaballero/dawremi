#![allow(dead_code)]

use dawremi_core::prelude::*;
use dawremi_core::sound_files::enums::princess_girlfriend::*;

song!(Test,);

impl Looper for Test {
    fn name(&self) -> &'static str {
        "test looper"
    }

    fn bpm(&self) -> usize {
        180
    }
    fn tracks(&mut self) -> Vec<Vec<f64>> {
        vec![self.drum(), self.smth()]
    }
}

impl Test {
    fn drum(&mut self) -> Vec<f64> {
        sequence!(
            self,
            len: 1.,
            signal: self.sound(Snares::DeathStarSnare.into()),

            x __
        )
        .effect(&Volume { mult: 1.5 })
    }

    fn smth(&mut self) -> Vec<f64> {
        sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.kick(note),

            __ B4 __ B4
        )
        .effect(&Volume { mult: 0.5 })
    }

    // Instruments

    fn bell(&self, note: Note) -> Synth {
        Synth::new(
            box Bell::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
    fn kick(&self, note: Note) -> Synth {
        Synth::new(
            box DrumKick::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
    fn snare(&self, note: Note) -> Synth {
        Synth::new(
            box DrumSnare::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
    fn hihat(&self, note: Note) -> Synth {
        Synth::new(
            box DrumHiHat::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
}
