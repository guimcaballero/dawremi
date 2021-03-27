//! Use the `Song` trait to make your songs!

use crate::effects::{Automation, EffectExtension, Volume};
use crate::frame::Frame;
use crate::helpers::*;
use crate::player::*;
use crate::sound_files::enums::Metronome;
use crate::sound_files::io::save_file;
use crate::traits::*;
use anyhow::Result;

pub trait Song: HasSampleRate + HasSoundHashMap {
    /// Saves the song to the output folder
    fn save_to_file(&mut self, bits_per_sample: u16, sample_rate: usize) {
        self.set_sample_rate(sample_rate as f64);
        save_file(
            self.generate(),
            &format!("output/{}.wav", self.name()),
            self.get_sample_rate() as u32,
            bits_per_sample,
        );
    }

    /// Generate and start playing the song
    /// The sample rate will be chosen according to the output config from cpal
    fn play(&mut self) -> Result<()> {
        let config = get_player_config();

        self.set_sample_rate(config.sample_rate as f64);

        let player = Player {
            audio: self.generate().into(),
            cycle: false,
        };

        run_player(player, config)
    }

    /// Generates the song and returns the vector of frames
    fn generate(&mut self) -> Vec<Frame> {
        let tracks = join_tracks(self.tracks());

        let mut vec = tracks
            .effect(&Volume {
                mult: self.volume(),
            })
            // Add some delay in the front if we enable metronome
            // This way we get like 3 beats of the metronome before we start
            .delay(if self.enable_metronome() {
                self.beats(3.)
            } else {
                0
            });

        if self.enable_metronome() {
            let metronome = self
                .sound(Metronome.into())
                .take_samples(self.beats(0.2))
                .chain(silence().take_samples(self.beats(0.8)))
                .cycle_until_samples(vec.len());
            vec = vec.add(metronome);
        }

        // If a duration was specified, take that number of samples
        if let Some(duration) = self.duration() {
            vec = vec.take_samples(duration)
        }

        if self.enable_normalization() {
            // Normalize
            let (max, min) = vec
                .iter()
                .cloned()
                .fold((-1. / 0., 1. / 0.), |(max, min), a| {
                    (f64::max(max, a.max()), f64::min(min, a.min()))
                });
            let max = f64::max(max.abs(), min.abs());
            vec = vec.iter().map(|a| a / max).collect();
        }

        vec
    }

    // Helper methods to use on tracks

    /// Returns the number of samples that should be taken to pass x beats
    fn beats(&self, x: f64) -> usize {
        let bps = self.bpm() as f64 / 60.;
        self.seconds(x / bps)
    }

    /// General volume for all tracks
    fn volume(&self) -> Automation<f64> {
        Automation::Const(1.)
    }

    /// Contains the list of tracks that should be mixed into a song
    fn tracks(&mut self) -> Vec<Vec<Frame>>;

    // Settings

    /// Display name for the song
    fn name(&self) -> &'static str;

    /// Duration in samples the song should have
    /// If None, the whole song will be used
    fn duration(&self) -> Option<usize> {
        None
    }

    /// Beats per minute for the song
    fn bpm(&self) -> usize;

    /// Toggles normalization of frames
    fn enable_normalization(&self) -> bool {
        true
    }

    /// Toggles metronome
    fn enable_metronome(&self) -> bool {
        false
    }
}

// Tests

#[cfg(test)]
mod test {
    use super::*;
    use crate::notes::*;
    use crate::sound_files::Sound;
    use crate::{sequence, song};
    use std::collections::HashMap;

    song!(EmptySong,);
    impl Song for EmptySong {
        fn name(&self) -> &'static str {
            "test"
        }

        fn bpm(&self) -> usize {
            120
        }
        fn tracks(&mut self) -> Vec<Vec<Frame>> {
            vec![]
        }
    }

    #[test]
    #[should_panic]
    /// Songs should have at least one working track to work
    fn cant_play_empty_song() {
        let mut song = EmptySong::default();
        let _ = song.generate();
    }

    #[test]
    /// When loading a sound, it should get added to the hashmap so we don't load it twice
    fn sounds_get_added_to_hashmap() {
        let mut song = EmptySong::default();
        song.set_sample_rate(44_100.);

        let path = "../../assets/beep.wav";
        let _ = song.sound(path.into());

        assert!(song.sound_hashmap.contains_key(path));
    }

    #[test]
    /// When loading a sound, it should get added to the hashmap so we don't load it twice
    fn can_get_sound_from_begin_to_end() {
        let mut song = EmptySong::default();
        song.set_sample_rate(44_100.);

        let path = "../../assets/beep.wav";
        let audio = song.sound(Sound {
            path: path.to_string(),
            begin: 13,
            end: Some(23),
        });

        // Returned audio is the correct slice
        assert_eq!(10, audio.len());

        // Vec gets still added to hashmap
        assert!(song.sound_hashmap.contains_key(path));
        let hashmap_vec = song.sound_hashmap.get(path).unwrap();
        // Vec in hashmap has the full length of the audio
        assert_eq!(441000, hashmap_vec.len());
        // Length of the beep.wav audio is 441000
    }

    #[test]
    #[should_panic]
    fn opening_nonexistant_sound_panics() {
        let mut song = EmptySong::default();

        let path = "assets/filethatdoesntexist.aaaaaaaaaaaaaa";
        let _ = song.sound(path.into());
    }

    song!(SongWithTrack,);
    impl Song for SongWithTrack {
        fn name(&self) -> &'static str {
            "test"
        }

        fn bpm(&self) -> usize {
            120
        }

        fn duration(&self) -> Option<usize> {
            Some(23)
        }

        fn tracks(&mut self) -> Vec<Vec<Frame>> {
            vec![sequence!(@lyrics
                      self,
                      len: 0.5, fun: |_note: Note| vec![Frame::mono(0.5); 100],

                      [twin-kle  twin-kle  lit-tle star],
                      (G4 _ G4 _ D4 _ D4 _ E4 _ E4 _ (D4 * 2.) _ _),
            )]
        }
    }

    #[test]
    #[should_panic]
    fn playing_song_before_setting_sample_rate_panics() {
        let mut song = SongWithTrack::default();
        let _ = song.generate();
    }

    #[test]
    fn can_play_song() {
        let mut song = SongWithTrack::default();
        song.set_sample_rate(400.0);
        let _ = song.generate();
    }

    #[test]
    fn duration_takes_just_the_correct_ammount_of_samples() {
        let mut song = SongWithTrack::default();
        song.set_sample_rate(400.0);
        let audio = song.generate();
        assert_eq!(23, audio.len());
    }
}
