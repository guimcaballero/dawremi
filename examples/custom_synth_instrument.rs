//! This example demonstrates how to create a simple effect
//! We'll be making a copy of the builtin `Volume` effect

#[macro_use]
extern crate dawremi;

use dawremi::prelude::*;
use std::f64::consts::TAU;

fn main() {
    let config = SongConfig {
        name: "Custom Synth Instrument".to_string(),
        bpm: 120.,
        duration: Duration::GeneratedTrack,
        ..Default::default()
    };
    let mut song = Song::new(vec![track], config);
    song.play().expect("Unable to play song");
}

fn track(song: &Song) -> Vec<Frame> {
    sequence!(
        song,
        len: 1., note: Note,
        fun: |note| instrument(song, note),
        A1 A2 A3 A4 A5 A6
    )
}

fn instrument(song: &Song, frequency: impl Into<Frequency>) -> Synth {
    Synth::new(
        Box::new(Sine::new(frequency.into(), song.sample_rate() as f64)),
        song.sample_rate() as f64,
    )
}

simple_instrument!(Sine);
impl SynthInstrument for Sine {
    fn get_params(&self) -> SynthParams {
        SynthParams {
            attack: self.seconds(0.01),
            decay: self.seconds(0.15),
            release: 0,

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    fn frame(&mut self) -> Frame {
        // You have access to:
        // self.frequency, which is the Frequency from the notes
        // self.sample(), which is the current number
        // self.sample_rate(), which is the sample rate
        // self.time(), which is sample / sample_rate
        // self.seconds(x), which will return the number of samples needed to pass x seconds

        let result = (TAU * self.frequency.0 * self.time()).sin();
        Frame::mono(result)
    }
}
