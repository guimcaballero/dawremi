use super::*;

song!(Test,);

impl Song for Test {
    fn duration(&self) -> usize {
        self.seconds(3.)
    }

    fn track1(&self) -> Option<Vec<f64>> {
        Some(
            self.hz(440.)
                .clone()
                .sine()
                .take(self.seconds(2.))
                .collect(),
        )
    }

    fn track2(&self) -> Option<Vec<f64>> {
        Some(
            signal::equilibrium()
                .take(self.seconds(1.))
                .chain(self.hz(220.).sine().take(self.seconds(2.)))
                .collect(),
        )
    }
}
