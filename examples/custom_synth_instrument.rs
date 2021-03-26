/// This example demonstrates how to create a simple effect
/// We'll be making a copy of the builtin `Volume` effect

#[macro_use]
extern crate dawremi;
use dawremi::prelude::*;
use std::f64::consts::TAU;

fn main() {
    let mut song = MySong::default();
    song.play().expect("Unable to play song");
}

song!(MySong);

impl Song for MySong {
    fn name(&self) -> &'static str {
        "Custom Synth Instrument"
    }

    fn bpm(&self) -> usize {
        120
    }

    fn tracks(&mut self) -> Vec<Vec<Frame>> {
        vec![self.track()]
    }
}

impl MySong {
    fn track(&self) -> Vec<Frame> {
        sequence!(
            self,
            len: 1., note: Note,
            fun: |note| self.instrument(note),
            A1 A2 A3 A4 A5 A6
        )
    }

    fn instrument(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            Box::new(Sine::new(frequency.into(), self.get_sample_rate())),
            self.get_sample_rate(),
        )
    }
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
