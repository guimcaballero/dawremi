use crate::helpers::*;
use dasp::{
    signal::{self, ConstHz},
    Sample, Signal,
};
use std::collections::HashMap;

pub trait Song: HasSampleRate + HasSoundHashMap {
    fn play(&mut self) -> Audio {
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
        .add_amp(signal::from_iter(
            tracks
                .pop()
                .unwrap_or_else(|| silence().take(self.duration()).collect()),
        ))
        .add_amp(signal::from_iter(
            tracks
                .pop()
                .unwrap_or_else(|| silence().take(self.duration()).collect()),
        ))
        .add_amp(signal::from_iter(
            tracks
                .pop()
                .unwrap_or_else(|| silence().take(self.duration()).collect()),
        ))
        .add_amp(signal::from_iter(
            tracks
                .pop()
                .unwrap_or_else(|| silence().take(self.duration()).collect()),
        ))
        .add_amp(signal::from_iter(
            tracks
                .pop()
                .unwrap_or_else(|| silence().take(self.duration()).collect()),
        ));

        let synth = track
            .map(move |s| s / (number_of_tracks as f64))
            .mul_amp(signal::from_iter(self.volume()))
            // Add some delay in the front if we enable metronome
            .delay(if cfg!(feature = "metronome") {
                self.beats(3.)
            } else {
                0
            })
            // We add the metronome after the volume
            .add_amp(signal::from_iter(self.metronome()))
            .take(self.duration());

        Box::new(synth)
    }

    fn metronome(&mut self) -> Vec<f64> {
        if cfg!(feature = "metronome") {
            self.sound_signal("assets/metronome.wav")
                .take(self.beats(0.2))
                .chain(silence().take(self.beats(0.8)))
                .cycle()
                .take(self.duration())
                .collect()
        } else {
            silence().take(self.duration()).collect()
        }
    }

    // Tracks

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

    // Helper methods to use on tracks

    /// Returns a ConstHz with this song's sample rate
    fn hz(&self, freq: impl Into<Frequency>) -> ConstHz {
        let freq = freq.into();
        signal::rate(self.get_sample_rate()).const_hz(freq.0)
    }

    fn sound(&mut self, path: &str) -> Vec<f64> {
        let hashmap = self.get_sound_hashmap();
        if let Some(vec) = hashmap.get(path) {
            vec.to_vec()
        } else {
            let reader = hound::WavReader::open(path).unwrap();

            let vec = reader
                .into_samples::<i16>()
                .filter_map(Result::ok)
                .map(i16::to_sample::<f64>)
                .collect::<Vec<f64>>();

            hashmap.insert(path.to_string(), vec.clone());
            vec
        }
    }
    fn sound_signal(&mut self, path: &str) -> signal::FromIterator<std::vec::IntoIter<f64>> {
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

    // Methods to overload for song customization

    /// General volume for all tracks
    fn volume(&self) -> Vec<f64> {
        signal::gen(|| 0.5).take(self.duration()).collect()
    }
    fn name(&self) -> &'static str;
    fn duration(&self) -> usize;
    fn bpm(&self) -> usize;
}

pub trait HasSampleRate {
    /// Sets sample rate for the Song
    fn set_sample_rate(&mut self, sample_rate: f64);
    /// Should only be called after setting sample_rate
    fn get_sample_rate(&self) -> f64;
}

pub trait HasSoundHashMap {
    /// Should only be called after setting sample_rate
    fn get_sound_hashmap(&mut self) -> &mut HashMap<String, Vec<f64>>;
}

macro_rules! song {
    ($name:ident, $( $id:ident : $type:ty ),*) => {
        #[derive(Default)]
        pub struct $name {
            sample_rate: Option<f64>,
            sound_hashmap: HashMap<String, Vec<f64>>,
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

        impl HasSoundHashMap for $name {
            fn get_sound_hashmap(&mut self) -> &mut HashMap<String, Vec<f64>>{
                &mut self.sound_hashmap
            }
        }
    };
}

pub type Audio = Box<dyn Iterator<Item = f64> + Send>;

// Songs

pub mod test_song;
