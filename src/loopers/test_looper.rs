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
        vec![self.drum(), self.snare()]
    }
}

impl Test {
    fn drum(&mut self) -> Vec<f64> {
        sequence!(
            self,
            len: 1.,
            signal: self.sound(Claps::DeepFriedClap.into()),

            _ x _ x
        )
        .effect(&Volume { mult: 1.5 })
    }

    fn snare(&mut self) -> Vec<f64> {
        sequence!(
            self,
            len: 1.,
            signal: self.sound(Snares::DeathStarSnare.into()),

            _ _ x _
        )
        .effect(&Volume { mult: 1.5 })
    }
}
