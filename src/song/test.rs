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
            sequence!(
                self, 1.,
                A B _ B
            )
            .collect::<Vec<f64>>()
            .repeat(4),
        )
    }

    // fn track2(&self) -> Option<Vec<f64>> {
    //     Some(
    //         silence()
    //             .take(self.beats(2.))
    //             .chain(
    //                 self.sound_signal("assets/beep.wav")
    //                     .take(self.beats(1.))
    //                     .chain(silence().take(self.beats(3.)))
    //                     .collect::<Vec<f64>>()
    //                     .repeat(4),
    //             )
    //             .collect(),
    //     )
    // }
}
