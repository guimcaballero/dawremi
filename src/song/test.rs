use super::*;

song!(Test,);

impl Song for Test {
    fn play(&self) -> Audio {
        if self.sample_rate.is_none() {
            panic!("Should set sample rate first");
        }

        // Create two signals that we'll merge
        let synth1 = self.hz(440.).clone().sine().take(self.seconds(2.));
        let synth2 = self
            .hz(0.)
            .sine()
            .take(self.seconds(1.))
            .chain(self.hz(220.).sine().take(self.seconds(2.)));
        let signal1 = signal::from_iter(synth1);
        let signal2 = signal::from_iter(synth2);

        // Merge the signals into one
        let synth = signal1
            .zip_map(signal2, |a, b| (a + b) / 2.)
            .map(|s| s.to_sample::<f32>() * 0.2)
            .take(self.seconds(3.));

        Box::new(synth)
    }
}
