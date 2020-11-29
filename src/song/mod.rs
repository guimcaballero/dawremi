use dasp::{
    signal::{self, ConstHz},
    Signal,
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

macro_rules! silence {
    () => {
        signal::equilibrium()
    };
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
            // We add the metronome after the volume
            .add_amp(signal::from_iter(self.metronome()))
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

    fn metronome(&self) -> Vec<f64> {
        if cfg!(feature = "metronome") {
            // TODO Change sound
            signal::noise(420)
                .take(self.beats(0.2))
                .chain(silence!().take(self.beats(0.8)))
                .cycle()
                .take(self.duration())
                .collect()
        } else {
            silence!().take(self.duration()).collect()
        }
    }

    fn volume(&self) -> Vec<f64> {
        signal::gen(|| 0.5).take(self.duration()).collect()
    }

    fn duration(&self) -> usize;
    fn bpm(&self) -> usize {
        120
    }

    /// Returns a ConstHz with this song's sample rate
    fn hz(&self, freq: f64) -> ConstHz {
        signal::rate(self.get_sample_rate()).const_hz(freq)
    }

    fn sound(&self, path: &str) -> Vec<f64> {
        let reader = hound::WavReader::open(path).unwrap();
        reader
            .into_samples::<i16>()
            .filter_map(Result::ok)
            .map(|x| x as f64)
            .collect()
    }
    fn sound_signal(&self, path: &str) -> signal::FromIterator<std::vec::IntoIter<f64>> {
        signal::from_iter(self.sound(path))
    }

    /// Returns the number of samples that should be taken to pass x seconds
    fn seconds(&self, x: f64) -> usize {
        (self.get_sample_rate() * x) as usize
    }
    /// Returns the number of samples that should be taken to pass x beats
    fn beats(&self, x: f64) -> usize {
        let bps = self.bpm() as f64 / 60.;
        self.seconds(x / bps)
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

trait RepeatExtension {
    fn repeat(self, times: usize) -> Vec<f64>;
}
impl RepeatExtension for Vec<f64> {
    fn repeat(self, times: usize) -> Vec<f64> {
        self.iter()
            .cloned()
            .cycle()
            .take(self.len() * times)
            .collect()
    }
}

pub mod test;
