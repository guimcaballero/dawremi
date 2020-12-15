use crate::helpers::*;
use crate::notes::*;
use crate::sound_files::Metronome;
use dasp::{
    signal::{self, ConstHz},
    Sample, Signal,
};
use std::collections::HashMap;
use std::io::stdin;

pub trait Song: HasSampleRate + HasSoundHashMap {
    fn play(&mut self) -> Audio {
        let synth = join_tracks(self.tracks());

        let synth = signal::from_iter(synth)
            .mul_amp(signal::from_iter(self.volume()))
            // Add some delay in the front if we enable metronome
            // This way we get like 3 beats of the metronome before we start
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

    fn tracks(&mut self) -> Vec<Vec<f64>>;

    fn metronome(&mut self) -> Vec<f64> {
        if cfg!(feature = "metronome") {
            self.sound_signal(Metronome.into())
                .take(self.beats(0.2))
                .chain(silence().take(self.beats(0.8)))
                .cycle()
                .take(self.duration())
                .collect()
        } else {
            silence().take_samples(self.duration())
        }
    }

    // Helper methods to use on tracks

    /// Returns a ConstHz with this song's sample rate
    fn hz(&self, freq: Frequency) -> ConstHz {
        signal::rate(self.get_sample_rate()).const_hz(freq.0)
    }

    fn sound(&mut self, path: &str) -> Vec<f64> {
        let sample_rate = self.get_sample_rate();

        let hashmap = self.get_sound_hashmap();
        if let Some(vec) = hashmap.get(path) {
            vec.to_vec()
        } else {
            let reader = hound::WavReader::open(path).unwrap();

            let spec = reader.spec();

            // Only process when sample rate is different
            // We do the abs thing cause we have them as floats
            let vec = if (spec.sample_rate as f64 - sample_rate).abs() > 0.01 {
                let orig = reader
                    .into_samples::<i16>()
                    // NOTE Eventually this will be removed when we implement stereo
                    .step_by(spec.channels.into())
                    .filter_map(Result::ok)
                    .map(i16::to_sample::<f64>);

                // Convert the signal's sample rate using `Sinc` interpolation.
                use dasp::{interpolate::sinc::Sinc, ring_buffer};
                let signal = signal::from_interleaved_samples_iter(orig);
                let ring_buffer = ring_buffer::Fixed::from([[0.0f64]; 100]);
                let sinc = Sinc::new(ring_buffer);
                let new_signal = signal.from_hz_to_hz(sinc, spec.sample_rate as f64, sample_rate);

                // TODO We probably should implement something to save this to a file
                // with the new sample rate, so we don't process it every time

                new_signal
                    .until_exhausted()
                    .map(|frame| frame[0])
                    .collect::<Vec<f64>>()
            } else {
                reader
                    .into_samples::<i16>()
                    // NOTE Eventually this will be removed when we implement stereo
                    .step_by(spec.channels.into())
                    .filter_map(Result::ok)
                    .map(i16::to_sample::<f64>)
                    .collect::<Vec<f64>>()
            };

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

        fn tracks(&mut self) -> Vec<Vec<f64>> {
            vec![]
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
        song.set_sample_rate(48_000.);

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

        fn tracks(&mut self) -> Vec<Vec<f64>> {
            vec![sequence!(@lyrics
                      self,
                      len: 0.5, fun: |note: Note| self.hz(note.into()).sine(),

                      [twin-kle  twin-kle  lit-tle star],
                      (G4 _ G4 _ D4 _ D4 _ E4 _ E4 _ (D4 * 2.) _ _),
            )]
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
        song.set_sample_rate(400.0);
        let _ = song.play();
    }
}
