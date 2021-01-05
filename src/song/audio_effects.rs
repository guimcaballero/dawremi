#![allow(dead_code)]

use super::*;
use crate::effects::*;
use crate::sound_files::Reverb;
use crate::synth::*;

song!(AudioEffectsDemo,);

impl Song for AudioEffectsDemo {
    fn name(&self) -> &'static str {
        "test"
    }
    fn bpm(&self) -> usize {
        180
    }
    fn duration(&self) -> usize {
        self.seconds(18.)
    }
    fn tracks(&mut self) -> Vec<Vec<f64>> {
        vec![
            // self.bass_boost(),
            // self.mt_reverb(),
            self.conv_reverb(),
        ]
    }
}

impl AudioEffectsDemo {
    fn bass_boost(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .take_samples(self.seconds(7.))
            .chain(
                &mut self
                    .sound("assets/audio.wav")
                    .effect(&BassBoost {
                        selectivity: 140.,
                        gain: 1.,
                        bass_ratio: 0.6,
                        input_ratio: 0.4,
                    })
                    .take_samples(self.seconds(7.)),
            )
    }

    fn conv_reverb(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .effect(&Convolution::new(
                self.sound(Reverb::LargeLongEchoHall.into()),
            ))
            .chain(&mut self.sound("assets/audio.wav"))
    }

    fn mt_reverb(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .effect(&MultitapReverb::new(self.get_sample_rate()))
            .take_samples(self.seconds(3.))
            .chain(
                &mut self
                    .sound("assets/audio.wav")
                    .take_samples(self.seconds(7.)),
            )
    }
}
