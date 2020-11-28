use dasp::{
    signal::{self, ConstHz},
    Sample, Signal,
};

pub trait HasSampleRate {
    /// Sets sample rate for the Song
    fn set_sample_rate(&mut self, sample_rate: f64);
    /// Should only be called after setting sample_rate
    fn get_sample_rate(&self) -> f64;
}

pub trait Song: HasSampleRate {
    fn play(&self) -> Audio;

    /// Returns a ConstHz with this song's sample rate
    fn hz(&self, freq: f64) -> ConstHz {
        signal::rate(self.get_sample_rate()).const_hz(freq)
    }
    /// Returns the number of samples that should be taken to pass x seconds
    fn seconds(&self, x: f64) -> usize {
        (self.get_sample_rate() * x) as usize
    }
}

pub type Audio = Box<dyn Iterator<Item = f32> + Send>;

macro_rules! song {
    ($name:ident, $( $id:ident : $type:ty ),*) => {
        #[derive(Default)]
        pub struct $name {
            sample_rate: Option<f64>,
            $( $id: $type )*
        }

        impl HasSampleRate for $name {
            fn set_sample_rate(&mut self, sample_rate: f64) {
                self.sample_rate = Some(sample_rate);
            }
            fn get_sample_rate(&self) -> f64 {
                self.sample_rate.unwrap()
            }
        }
    };
}

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

fn saw(sample_rate: f64, duration: usize) -> Vec<f64> {
    let hz = signal::rate(sample_rate).const_hz(440.0);
    hz.clone().saw().take(duration).collect()
}
