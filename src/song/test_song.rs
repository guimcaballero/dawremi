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
        self.beats(16.)
    }

    #[allow(unreachable_code)]
    fn track1(&self) -> Option<Vec<f64>> {
        Some(sequence!(
                self,
                len: 1.,
                fun: |note| self.bell(note),

                G4 G4 D4 D4 E4 E4 (D4 * 2.)
        ))
    }

    fn track2(&self) -> Option<Vec<f64>> {
        return None;
        let tracks = pattern!(
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
        );
        Some(tracks)
    }
}

impl Test {
    #[allow(dead_code)]
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
