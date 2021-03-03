use crate::frame::*;
use crate::notes::*;
use crate::sound_files::io::open_file;
use dasp::signal::{self, ConstHz};
use std::collections::HashMap;

pub trait HasSampleRate {
    /// Sets sample rate for the Song
    fn set_sample_rate(&mut self, sample_rate: f64);
    /// Should only be called after setting sample_rate
    fn get_sample_rate(&self) -> f64;

    // Helper methods

    /// Returns a ConstHz with this song's sample rate
    fn hz(&self, freq: Frequency) -> ConstHz {
        signal::rate(self.get_sample_rate()).const_hz(freq.0)
    }
    /// Returns the number of samples that should be taken to pass x seconds
    fn seconds(&self, x: f64) -> usize {
        (self.get_sample_rate() * x) as usize
    }
}

pub trait HasSoundHashMap: HasSampleRate {
    /// Should only be called after setting sample_rate
    fn get_sound_hashmap(&mut self) -> &mut HashMap<String, Vec<Frame>>;

    fn sound(&mut self, path: &str) -> Vec<Frame> {
        let sample_rate = self.get_sample_rate();

        let hashmap = self.get_sound_hashmap();

        // If the audio is in the hashmap, return that
        // Else, load it and insert it in the hashmap
        if let Some(vec) = hashmap.get(path) {
            vec.to_vec()
        } else {
            let vec = open_file(path, sample_rate as u32);
            hashmap.insert(path.to_string(), vec.clone());

            vec
        }
    }
}

#[macro_export]
macro_rules! song {
    ($name:ident, $( $id:ident : $type:ty ),*) => {
        #[derive(Default)]
        pub struct $name {
            sample_rate: Option<f64>,
            sound_hashmap: HashMap<String, Vec<Frame>>,
            $( $id: $type )*
        }

        impl self::HasSampleRate for $name {
            fn set_sample_rate(&mut self, sample_rate: f64) {
                self.sample_rate = Some(sample_rate);
            }
            fn get_sample_rate(&self) -> f64 {
                self.sample_rate.expect("Sample rate should have been set before playing the song")
            }
        }

        impl self::HasSoundHashMap for $name {
            fn get_sound_hashmap(&mut self) -> &mut HashMap<String, Vec<Frame>>{
                &mut self.sound_hashmap
            }
        }
    };
}
