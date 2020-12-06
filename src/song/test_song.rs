use super::*;
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
        self.beats(10.)
    }

    fn track1(&self) -> Option<Vec<f64>> {
        Some(sequence!(@lyrics
                self,
                len: 1.,
                fun: |note| self.synth(note),

                (G4 G4 D4 D4 E4 E4 (D4 * 2.)),
        ))
    }
}

impl Test {
    fn synth(&self, note: Note) -> Synth {
        Synth::new(
            box Bell::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
}
