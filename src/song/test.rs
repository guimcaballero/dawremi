use super::*;

song!(Test,);

impl Song for Test {
    fn bpm(&self) -> usize {
        120
    }
    fn duration(&self) -> usize {
        self.seconds(8.)
    }

    fn track1(&self) -> Option<Vec<f64>> {
        let smth: Vec<f64> = self
            .hz(440.)
            .sine()
            .take(self.beats(1.))
            .chain(silence!().take(self.beats(3.)))
            .collect();
        Some(smth.repeat(4))
    }

    // fn track2(&self) -> Option<Vec<f64>> {
    //     Some(
    //         signal::equilibrium()
    //             .take(self.seconds(1.))
    //             .chain(self.hz(220.).sine().take(self.seconds(2.)))
    //             .collect(),
    //     )
    // }
}
