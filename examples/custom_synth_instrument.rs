/// This example demonstrates how to create a simple effect
/// We'll be making a copy of the builtin `Volume` effect

#[macro_use]
extern crate dawremi;
use dawremi::prelude::*;

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

    fn duration(&self) -> usize {
        self.seconds(3.)
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
        let result = (self.frequency.0 * self.time()).sin();
        Frame::mono(result)
    }
}
