/// This example demonstrates how to create a simple effect
/// We'll be making a copy of the builtin `Volume` effect

#[macro_use]
extern crate dawremi;
use dawremi::prelude::*;

fn main() {
    let mut song = CustomEffectSong::default();
    song.play().expect("Unable to play song");
}

song!(CustomEffectSong,);

impl Song for CustomEffectSong {
    fn name(&self) -> &'static str {
        "Custom effect"
    }

    fn bpm(&self) -> usize {
        120
    }

    fn duration(&self) -> Option<usize> {
        Some(self.seconds(10.))
    }

    fn tracks(&mut self) -> Vec<Vec<Frame>> {
        vec![self.noise()]
    }
}

impl CustomEffectSong {
    fn noise(&self) -> Vec<Frame> {
        // We'll be making a noise track, and we'll apply our effect twice
        // First time will be with a constant value as multiplier
        // Second time will be with a sine wave of constant frequency

        noise::noise(3333, self.duration().unwrap())
            .into_frames()
            .effect(&MyCustomEffect {
                mult: Automation::Const(0.5),
            })
            .effect(&MyCustomEffect {
                mult: Automation::Vec(waves::sine(
                    self.duration().unwrap(),
                    // This second parameter is also an automation, so we could modify it
                    // with an `Automation::Vec`
                    Automation::Const(self.frequency(0.5)),
                )),
            })
    }
}

/// My amazing custom effect
pub struct MyCustomEffect {
    pub mult: Automation<f64>,
}
impl Effect for MyCustomEffect {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        // The use of `Automation` allows us to code effects independently
        // of whether they'll be used with a constant value or with a vector

        input
            .iter()
            .enumerate()
            .map(|(idx, val)| val * self.mult.value(idx))
            .collect()
    }
}
