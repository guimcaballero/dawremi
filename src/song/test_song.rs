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
        self.beats(4.)
    }

    fn track1(&self) -> Option<Vec<f64>> {
        Some(sequence!(@lyrics
                       self,
                       len: 0.5,
                       fun: |note| self.synth(note),

            (C4 _ C4 _ (C4 * 2.) _ _),
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
