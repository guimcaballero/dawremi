//! This example demonstrates how to create a simple effect
//! We'll be making a copy of the builtin `Volume` effect

use dawremi::prelude::*;

fn main() {
    let config = SongConfig {
        name: "Custom effects".to_string(),
        bpm: 120.,
        duration: Duration::Seconds(10.),
        ..Default::default()
    };
    let mut song = Song::new(vec![noise.into()], config);
    song.play().expect("Unable to play song");
}

fn noise(song: &Song) -> Vec<Frame> {
    // We'll be making a noise track, and we'll apply our effect twice
    // First time will be with a constant value as multiplier
    // Second time will be with a sine wave of constant frequency

    noise::noise(song.duration().unwrap(), 3333)
        .as_frames()
        .effect(&MyCustomEffect {
            mult: Automation::Const(0.5),
        })
        .effect(&MyCustomEffect {
            mult: Automation::Vec(waves::sine(
                song.duration().unwrap(),
                // This second parameter is also an automation, so we could modify it
                // with an `Automation::Vec`
                Automation::Const(1.),
                song.sample_rate(),
            )),
        })
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
