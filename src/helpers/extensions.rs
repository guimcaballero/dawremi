use dasp::signal::Signal;

pub trait TakeSamplesExtension {
    fn take_samples(self, samples: usize) -> Vec<f64>;
}
impl<T: Signal<Frame = f64>> TakeSamplesExtension for T {
    fn take_samples(self, samples: usize) -> Vec<f64> {
        self.take(samples).collect()
    }
}

pub trait RepeatExtension {
    fn repeat(self, times: usize) -> Vec<f64>;
    fn collect(self) -> Self;
    fn take_samples(self, samples: usize) -> Vec<f64>;
    fn chain(self, new: &mut Vec<f64>) -> Vec<f64>;
}

impl RepeatExtension for Vec<f64> {
    fn collect(self) -> Self {
        self
    }

    fn take_samples(self, samples: usize) -> Vec<f64> {
        self.iter().cloned().take(samples).collect()
    }

    fn repeat(self, times: usize) -> Vec<f64> {
        self.iter()
            .cloned()
            .cycle()
            .take(self.len() * times)
            .collect()
    }

    fn chain(mut self, new: &mut Vec<f64>) -> Vec<f64> {
        self.append(new);
        self
    }
}
