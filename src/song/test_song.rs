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
                fun: |note| self.bell(note),

                (G4 G4 D4 D4 E4 E4 (D4 * 2.)),
        ))
    }

    fn track2(&self) -> Option<Vec<f64>> {
        let tracks = pattern!(
            self,
            repetitions: 4,

            len: 1.,
            fun: |note| self.harmonica(note),
            pat: (C4 _  _  _),

            len: 1.,
            fun: |note| self.bell(note),
            pat: (C4 _  C4 _),
        );
        Some(tracks)
    }
}

impl Test {
    fn harmonica(&self, note: Note) -> Synth {
        Synth::new(
            box Harmonica::new(note, self.get_sample_rate()),
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
}
