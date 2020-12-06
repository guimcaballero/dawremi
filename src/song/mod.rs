use crate::helpers::*;
use dasp::{
    signal::{self, ConstHz},
    Sample, Signal,
};
use std::collections::HashMap;
use std::io::stdin;

pub trait Song: HasSampleRate + HasSoundHashMap {
    fn play(&mut self) -> Audio {
        let synth = join_tracks![
            duration: self.duration(),

            self.track1(),
            self.track2(),
            self.track3(),
            self.track4(),
            self.track5(),
            self.track6(),
        ];

        let synth = synth
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
            silence().take_samples(self.duration())
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
    fn hz(&self, freq: Frequency) -> ConstHz {
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
        signal::gen(|| 0.5).take_samples(self.duration())
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

macro_rules! songs {
    ( $($num:pat =>  $mod:ident => $struct:ident,)* ) => {
        $(
            mod $mod;
            pub use $mod::$struct;
        )*

        pub fn select_song() -> Box<dyn Song> {
            println!("Select a song:");

            $(
                println!("[{}]: {}", stringify!($num), $struct::default().name());
            )*

            let mut s = String::new();
            stdin()
                .read_line(&mut s)
                .expect("Did not enter a correct string");

            match s.trim().parse().unwrap_or(u32::MAX) {
                $(
                    $num => box $struct::default(),
                )*
            }
        }
    };
}

songs!(
    1 => twinkle_twinkle => TwinkleTwinkle,
    _ => test_song => Test,
);

// Tests

#[cfg(test)]
mod test {
    use super::*;

    song!(EmptySong,);
    impl Song for EmptySong {
        fn name(&self) -> &'static str {
            "test"
        }

        fn bpm(&self) -> usize {
            120
        }
        fn duration(&self) -> usize {
            self.beats(12.)
        }
    }

    #[test]
    #[should_panic]
    /// Songs should have at least one working track to work
    fn cant_play_empty_song() {
        let mut song = EmptySong::default();
        let _ = song.play();
    }

    #[test]
    /// When loading a sound, it should get added to the hashmap so we don't load it twice
    fn sounds_get_added_to_hashmap() {
        let mut song = EmptySong::default();

        let path = "assets/beep.wav";
        let _ = song.sound(path);

        assert!(song.sound_hashmap.contains_key(path));
    }

    #[test]
    #[should_panic]
    fn opening_nonexistant_sound_panics() {
        let mut song = EmptySong::default();

        let path = "assets/filethatdoesntexist.aaaaaaaaaaaaaa";
        let _ = song.sound(path);
    }

    song!(SongWithTrack,);
    impl Song for SongWithTrack {
        fn name(&self) -> &'static str {
            "test"
        }

        fn bpm(&self) -> usize {
            120
        }
        fn duration(&self) -> usize {
            self.beats(12.)
        }

        fn track1(&self) -> Option<Vec<f64>> {
            // We have twinkle twinkle with lyrics to ensure that the lyrics macro works when changing things
            Some(sequence!(@lyrics
                self,
                len: 0.5, fun: |note| self.hz(note).sine(),

                [twin-kle  twin-kle  lit-tle star],
                (G _ G _ D _ D _ E _ E _ D D _ _),

                [how  I    won-der  how  you  are],
                (C _ C _ B _ B _ A _ A _ G G _ _),

                (D _ D _ C _ C _ B _ B _ A A _ _),
                (D _ D _ C _ C _ B _ B _ A A _ _),
                (G _ G _ D _ D _ E _ E _ D D _ _),
                (C _ C _ B _ B _ A _ A _ G G _ _),
            ))
        }
    }

    #[test]
    #[should_panic]
    fn playing_song_before_setting_sample_rate_panics() {
        let mut song = SongWithTrack::default();
        let _ = song.play();
    }

    #[test]
    fn can_play_song() {
        let mut song = SongWithTrack::default();
        song.set_sample_rate(44_000.0);
        let _ = song.play();
    }
}
