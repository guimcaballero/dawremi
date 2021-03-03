#![allow(dead_code)]

use dawremi_core::prelude::*;

song!(Test,);

impl Song for Test {
    fn name(&self) -> &'static str {
        "test"
    }
    fn bpm(&self) -> usize {
        180
    }
    fn duration(&self) -> usize {
        self.seconds(20.)
    }
    fn tracks(&mut self) -> Vec<Vec<Frame>> {
        vec![self.stereo()]
    }
}

impl Test {
    fn stereo(&mut self) -> Vec<Frame> {
        let left = sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Triangle(2, 3)),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        );
        let right = sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.bell(note),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        );

        join_left_and_right_channels(left.to_mono(), right.to_mono())
    }

    fn afrodite(&mut self) -> Vec<Frame> {
        let sound1 = sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Triangle(2, 3)),

            D3 D3 D3 (D3 * 0.5) D4 _
            [D3 G4] G6 G7 B6 G7 G7 G6 _
            D7 (G6 * 0.5) G7 D7 D6 D4

            [L5 A5 D5]
        );

        let chord = {
            use GuitarFretboard::*;
            vec![A4, D3, G4, B2, E4]
        };
        let notes = vec![chord.clone(), chord.clone(), chord.clone(), chord];
        let sound = notes.generate(
            &|note, length| {
                self.plucked(note, InitialBurstType::Triangle(2, 3))
                    .take_samples(length)
            },
            self.beats(1.),
        );

        sound1
            .chain(sound)
            .effect(&Convolution::new(self.sound(Reverb::ParkingGarage.into())))
    }

    fn test_new_sequence(&mut self) -> Vec<Frame> {
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

    fn test_multiple_notes(&mut self) -> Vec<Frame> {
        sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Triangle(2, 3)),

            (L5 * 2.) ([L5 L6] * 1.) _ L8 L8 _ L1 L1 _ L4 L4
        )
    }

    fn microtonal(&self) -> Vec<Frame> {
        let notes1 = {
            let n = |a, b| vec![Tet72::new(a, b)];
            vec![n(2, 0), n(2, 1), n(2, 2), n(2, 3), n(2, 19)]
        };

        notes1.generate(
            &|note, length| {
                self.plucked(note, InitialBurstType::Triangle(2, 3))
                    .take_samples(length)
            },
            self.beats(1.),
        )
    }

    fn plucked_track(&mut self) -> Vec<Frame> {
        sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Triangle(2, 3)),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        )
        .chain(sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::DoubleTriangle),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
        .chain(sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Hill),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
        .chain(sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Sine),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
        .chain(sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::Random),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
    }

    fn track2(&self) -> Vec<Frame> {
        pattern!(
            self,
            note: Note,
            repetitions: 4,

            beat: 1.,
            // note: GuitarFretboard,
            fun: |note| self.bell(note),
            pat: (__ __ __ __  __ C5 __ __),

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
    fn harmonica(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            box Harmonica::new(frequency.into(), self.get_sample_rate()),
            self.get_sample_rate(),
        )
    }
    fn plucked(&self, frequency: impl Into<Frequency>, burst: InitialBurstType) -> Synth {
        Synth::new(
            box Plucked::new(burst, frequency.into(), self.get_sample_rate()),
            self.get_sample_rate(),
        )
    }
    fn bell(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            box Bell::new(frequency.into(), self.get_sample_rate()),
            self.get_sample_rate(),
        )
    }
    fn kick(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            box DrumKick::new(frequency.into(), self.get_sample_rate()),
            self.get_sample_rate(),
        )
    }
    fn snare(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            box DrumSnare::new(frequency.into(), self.get_sample_rate()),
            self.get_sample_rate(),
        )
    }
    fn hihat(&self, frequency: impl Into<Frequency>) -> Synth {
        Synth::new(
            box DrumHiHat::new(frequency.into(), self.get_sample_rate()),
            self.get_sample_rate(),
        )
    }
}
