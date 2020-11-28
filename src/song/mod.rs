use dasp::{
    signal::{self, ConstHz},
    Sample, Signal,
};

/// Adds the expressions to a vector if they're not None
macro_rules! some_vec {
    ($($x:expr),* $(,)?) => (
        {
            let mut temp = Vec::new();
            $(
                if let Some(val) = $x {
                    temp.push(val);
                }
            )*
            temp
        }
    );
}

pub trait Song: HasSampleRate {
    fn play(&self) -> Audio {
        let mut tracks = some_vec![
            self.track1(),
            self.track2(),
            self.track3(),
            self.track4(),
            self.track5(),
            self.track6(),
        ];
        let number_of_tracks = tracks.len();

        // Join all of the tracks into one
        let track = signal::from_iter(
            tracks
                .pop()
                .expect("There should be at least one working Track"),
        )
        // Add the track or an empty Signal
        .add_amp(signal::from_iter(tracks.pop().unwrap_or_else(|| {
            signal::equilibrium().take(self.duration()).collect()
        })))
        .add_amp(signal::from_iter(tracks.pop().unwrap_or_else(|| {
            signal::equilibrium().take(self.duration()).collect()
        })))
        .add_amp(signal::from_iter(tracks.pop().unwrap_or_else(|| {
            signal::equilibrium().take(self.duration()).collect()
        })))
        .add_amp(signal::from_iter(tracks.pop().unwrap_or_else(|| {
            signal::equilibrium().take(self.duration()).collect()
        })))
        .add_amp(signal::from_iter(
            tracks
                .pop()
                .unwrap_or_else(|| signal::equilibrium().take(self.duration()).collect()),
        ));

        let synth = track
            .map(move |s| s / (number_of_tracks as f64))
            .mul_amp(signal::from_iter(self.volume()))
            .take(self.duration());

        Box::new(synth)
    }

    fn track1(&self) -> Option<Vec<f64>> {
        None
    }
    fn track2(&self) -> Option<Vec<f64>> {
        None
    }
    fn track3(&self) -> Option<Vec<f64>> {
        None
    }
    fn track4(&self) -> Option<Vec<f64>> {
        None
    }
    fn track5(&self) -> Option<Vec<f64>> {
        None
    }
    fn track6(&self) -> Option<Vec<f64>> {
        None
    }

    fn volume(&self) -> Vec<f64> {
        signal::gen(|| 0.5).take(self.duration()).collect()
    }

    fn duration(&self) -> usize;

    /// Returns a ConstHz with this song's sample rate
    fn hz(&self, freq: f64) -> ConstHz {
        signal::rate(self.get_sample_rate()).const_hz(freq)
    }
    /// Returns the number of samples that should be taken to pass x seconds
    fn seconds(&self, x: f64) -> usize {
        (self.get_sample_rate() * x) as usize
    }
}

pub trait HasSampleRate {
    /// Sets sample rate for the Song
    fn set_sample_rate(&mut self, sample_rate: f64);
    /// Should only be called after setting sample_rate
    fn get_sample_rate(&self) -> f64;
}

pub type Audio = Box<dyn Iterator<Item = f64> + Send>;

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
                self.sample_rate.expect("Sample rate should have been set before playing the song")
            }
        }
    };
}

pub mod test;
