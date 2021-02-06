#![allow(dead_code)]

use dawremi_core::prelude::*;

song!(GuitarTests,);

impl Song for GuitarTests {
    fn name(&self) -> &'static str {
        "Guitar tests"
    }
    fn bpm(&self) -> usize {
        240
    }
    fn duration(&self) -> usize {
        self.beats(8. * 6.)
    }
    fn tracks(&mut self) -> Vec<Vec<f64>> {
        vec![self.in_between_the_castle_ruins_i_found_a_book()]
    }
}

impl GuitarTests {
    fn in_between_the_castle_ruins_i_found_a_book(&self) -> Vec<f64> {
        let guitar = {
            let s1 = sequence!(
                self,
                len: 1., note: GuitarFretboard,
                fun: |note| guitar(self, note),

                L12 L12 L12 L12 (D15 * 2.0)
                L12 L12 (D16 * 2.0)
                L12 L12 (D14 * 2.0)

                L12 L12 L12 L12 (D15 * 2.0)
                (L12 * 2.0) (D16 * 2.0)
                (L12 * 2.0) (D14 * 2.0)
            );

            let s2 = sequence!(
                self,
                len: 2., note: GuitarFretboard,
                fun: |note| guitar(self, note),

                L0 D12 L0
                D12 D15 A16 A12
                D12 D15 A16 A12
                D12 D15 A16 A12
            );

            s1.chain(s2)
        };
        let bass = {
            let s1 = sequence!(
                self,
                len: 2., note: GuitarFretboard,
                fun: |note| bass(self, note),

                _
            );
            // TODO

            s1
        };

        join_tracks(vec![guitar, bass])
    }
}

fn guitar(song: &dyn Song, note: Note) -> SynthGroup {
    let guitar1 = Synth::new(
        box Plucked::new(
            InitialBurstType::DoubleTriangle,
            note,
            song.get_sample_rate(),
        ),
        note,
        song.get_sample_rate(),
    );

    let guitar2 = Synth::new(
        box Plucked::new(InitialBurstType::Random, note, song.get_sample_rate()),
        note,
        song.get_sample_rate(),
    );

    SynthGroup(vec![guitar1, guitar2])
}

fn bass(song: &dyn Song, note: Note) -> Synth {
    Synth::new(
        box Plucked::new(InitialBurstType::Sine, note, song.get_sample_rate()),
        note,
        song.get_sample_rate(),
    )
}
