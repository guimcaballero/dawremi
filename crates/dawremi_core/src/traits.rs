//! Contains the `song!()` macro and some inner traits

use crate::frame::*;
use crate::sound_files::io::open_file;
use crate::sound_files::Sound;
use std::collections::HashMap;

pub trait HasSampleRate {
    /// Sets sample rate for the Song
    fn set_sample_rate(&mut self, sample_rate: f64);
    /// Should only be called after setting sample_rate
    fn get_sample_rate(&self) -> f64;

    // Helper methods

    /// Returns the number of samples that should be taken to pass x seconds
    fn seconds(&self, x: f64) -> usize {
        (self.get_sample_rate() * x) as usize
    }

    fn frequency(&self, x: f64) -> f64 {
        x / self.get_sample_rate()
    }
}

pub trait HasSoundHashMap: HasSampleRate {
    /// Should only be called after setting sample_rate
    fn get_sound_hashmap(&mut self) -> &mut HashMap<String, Vec<Frame>>;

    fn sound(&mut self, sound: Sound) -> Vec<Frame> {
        let sample_rate = self.get_sample_rate();

        let hashmap = self.get_sound_hashmap();

        // If hashmap does not contain this audio, load it and insert it
        if !hashmap.contains_key(&sound.path) {
            let vec = open_file(&sound.path, sample_rate as u32);
            hashmap.insert(sound.path.clone(), vec);
        }

        // If the audio is in the hashmap, return that
        // Else, load it and insert it in the hashmap
        let vec = hashmap.get(&sound.path).unwrap();

        // Return only from begin to end
        if let Some(end) = sound.end {
            vec[sound.begin..end].to_vec()
        } else {
            vec[sound.begin..].to_vec()
        }
    }
}

#[macro_export]
macro_rules! song {
    ($name:ident $(, $id:ident : $type:ty )* $(,)?) => {
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

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name())
            }
        }
    };
}
