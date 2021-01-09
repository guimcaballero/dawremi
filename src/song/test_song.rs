#![allow(dead_code)]

use super::*;
use crate::effects::*;
use crate::sound_files::Reverb;
use crate::synth::*;

song!(Test,);

impl Song for Test {
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
        vec![self.plucked_track()]
    }
}

impl Test {
    fn test_new_sequence(&mut self) -> Vec<f64> {
        let notes1 = {
            use Note::*;
            note_list![[A4, C4], A5, A6, _, A6]
        };

        let sound1 = notes1.generate(
            &|note, length| {
                self.plucked(note, InitialBurstType::Triangle(2, 3))
                    .take_samples(length)
            },
            self.beats(1.),
        );
        let sound2 = notes1.map_notes(Note::up_an_octave).generate(
            &|note, length| {
                self.plucked(note, InitialBurstType::Triangle(2, 3))
                    .take_samples(length)
            },
            self.beats(1.),
        );

        let bass = {
            use GuitarFretboard::*;
            note_list![L5, L5, _, L8, L8, _, L1, L1, _, L4, L4,]
        }
        .into_notes()
        .generate(
            &|note, length| {
                self.plucked(note, InitialBurstType::Sine)
                    .take_samples(length)
            },
            self.beats(1.),
        );

        join_tracks(vec![sound1, sound2, bass])
            .effect(&Convolution::new(
                self.sound(Reverb::LargeLongEchoHall.into()),
            ))
            .effect(&Volume { mult: 0.5 })
    }

    fn effect_bundle(&mut self) -> EffectBundle {
        EffectBundle(vec![
            box Convolution::new(self.sound(Reverb::LargeLongEchoHall.into())),
            box Volume { mult: 0.5 },
        ])
    }

    fn plucked_track(&mut self) -> Vec<f64> {
        sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Triangle(2, 3)),

            (L5 * 2.) ([L5 L6] * 1.) _ L8 L8 _ L1 L1 _ L4 L4
        )
        .chain(&mut sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::DoubleTriangle),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
        .chain(&mut sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Hill),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
        .chain(&mut sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Sine),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
        .chain(&mut sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Random),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
        .effect(&self.effect_bundle())
    }

    fn track2(&self) -> Vec<f64> {
        pattern!(
            self,
            note: Note,
            repetitions: 4,

            beat: 1.,
            note: GuitarFretboard,
            fun: |note| self.bell(note),
            pat: (__ __ __ __  __ L5 __ __),

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
    fn plucked(&self, note: Note, burst: InitialBurstType) -> Synth {
        Synth::new(
            box Plucked::new(burst, note, self.get_sample_rate()),
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
