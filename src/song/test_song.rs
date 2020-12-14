#![allow(dead_code)]

use super::*;
use crate::effects::*;
use crate::synth::*;

song!(Test,);

impl Song for Test {
    fn name(&self) -> &'static str {
        "test"
    }
    fn bpm(&self) -> usize {
        120
    }
    fn duration(&self) -> usize {
        self.seconds(14.)
    }
    fn tracks(&mut self) -> Vec<Vec<f64>> {
        vec![
            // self.bass_boost(),
            // self.mt_reverb(),
            self.conv_reverb(),
        ]
    }
}

impl Test {
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

    fn mt_reverb(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .effect(&Convolution::new(
                self.sound("assets/reverbs/large_long_echo_hall.wav"),
            ))
            .take_samples(self.seconds(7.))
            .chain(
                &mut self
                    .sound("assets/audio.wav")
                    .take_samples(self.seconds(7.)),
            )
    }

    fn conv_reverb(&mut self) -> Vec<f64> {
        self.sound("assets/audio.wav")
            .effect(&MultitapReverb::new(self.get_sample_rate()))
            .take_samples(self.seconds(7.))
            .chain(
                &mut self
                    .sound("assets/audio.wav")
                    .take_samples(self.seconds(7.)),
            )
    }

    fn plucked_track(&self) -> Vec<f64> {
        sequence!(
            self,
            len: 1.,
            fun: |note| self.plucked(note),

            G2 G2 D2 D2 E4 E4 (D4 * 2.)
        )
    }

    fn track2(&self) -> Vec<f64> {
        pattern!(
            self,
            repetitions: 4,

            beat: 1.,
            fun: |note| self.bell(note),
            pat: (__ __ __ __  __ C4 __ __),

            beat: 1.,
            fun: |note| self.kick(note),
            pat: (C4 __ __ __  C4 __ __ __),

            beat: 1.,
            fun: |note| self.snare(note),
            pat: (__ __ C4 __  __ __ C4 __),

            beat: 1.,
            fun: |note| self.hihat(note),
            pat: (C4 C4 C4 C4  C4 C4 C4 C4),
        )
    }

    #[allow(dead_code)]
    fn harmonica(&self, note: Note) -> Synth {
        Synth::new(
            box Harmonica::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
    fn plucked(&self, note: Note) -> Synth {
        Synth::new(
            box Plucked::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
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
