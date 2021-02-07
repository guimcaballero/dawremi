#![allow(dead_code)]

use dawremi_core::prelude::*;

song!(AudioEffectsDemo,);

impl Song for AudioEffectsDemo {
    fn name(&self) -> &'static str {
        "Audio effects"
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
            // self.conv_reverb(),
            // self.pitch_shift(),
            self.autotune(),
        ]
    }
}

impl AudioEffectsDemo {
    fn bass_boost(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .take_samples(self.seconds(7.))
            .chain(
                self.sound("assets/audio.wav")
                    .effect(&BassBoost {
                        selectivity: 140.,
                        gain: 1.,
                        bass_ratio: 0.6,
                        input_ratio: 0.4,
                    })
                    .take_samples(self.seconds(7.)),
            )
    }

    fn autotune(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .take_samples(self.seconds(4.))
            .chain(
                self.sound("assets/audio.wav")
                    .effect(&Autotune {
                        sample_rate: self.get_sample_rate(),
                        beat_length: self.seconds(0.5),
                        notes: {
                            use Note::*;
                            note_option![A3, C4, E4, F4, A3, C4, F4, E4]
                        },
                    })
                    .take_samples(self.seconds(4.)),
            )
    }

    fn pitch_shift(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .take_samples(self.seconds(3.))
            .chain(
                self.sound("assets/audio.wav")
                    .effect(&PitchShift {
                        sample_rate: self.get_sample_rate(),
                        shift: 1.3,
                    })
                    .take_samples(self.seconds(3.)),
            )
            .chain(
                self.sound("assets/audio.wav")
                    .effect(&PitchShift {
                        sample_rate: self.get_sample_rate(),
                        shift: 0.8,
                    })
                    .take_samples(self.seconds(3.)),
            )
    }

    fn conv_reverb(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .effect(&Convolution::new(
                self.sound(Reverb::LargeLongEchoHall.into()),
            ))
            .chain(self.sound("assets/audio.wav"))
    }

    fn mt_reverb(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .effect(&MultitapReverb::new(self.get_sample_rate()))
            .take_samples(self.seconds(3.))
            .chain(
                self.sound("assets/audio.wav")
                    .take_samples(self.seconds(7.)),
            )
    }
}
