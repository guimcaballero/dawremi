use super::*;

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

    fn track1(&self) -> Option<Vec<f64>> {
        Some(
            self.hz(Note::A)
                .sine()
                .take(self.beats(1.))
                .chain(self.hz(Note::As).sine().take(self.beats(1.)))
                .chain(self.hz(Note::B).sine().take(self.beats(1.)))
                .chain(self.hz(Note::C).sine().take(self.beats(1.)))
                .chain(self.hz(Note::Cs).sine().take(self.beats(1.)))
                .chain(self.hz(Note::D).sine().take(self.beats(1.)))
                .chain(self.hz(Note::Ds).sine().take(self.beats(1.)))
                .chain(self.hz(Note::E).sine().take(self.beats(1.)))
                .chain(self.hz(Note::F).sine().take(self.beats(1.)))
                .chain(self.hz(Note::Fs).sine().take(self.beats(1.)))
                .chain(self.hz(Note::G).sine().take(self.beats(1.)))
                .chain(self.hz(Note::Gs).sine().take(self.beats(1.)))
                .collect::<Vec<f64>>()
                .repeat(4),
        )
    }

    fn track2(&self) -> Option<Vec<f64>> {
        return None;
        Some(
            silence()
                .take(self.beats(2.))
                .chain(
                    self.sound_signal("assets/beep.wav")
                        .take(self.beats(1.))
                        .chain(silence().take(self.beats(3.)))
                        .collect::<Vec<f64>>()
                        .repeat(4),
                )
                .collect(),
        )
    }
}
