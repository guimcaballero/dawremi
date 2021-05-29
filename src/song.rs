use crate::{
    effects::{Automation, EffectExtension, Volume},
    frame::Frame,
    helpers::*,
    player::*,
    sound_files::{enums::Metronome, io::*, Sound},
    vst::*,
};

use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use vst::host::PluginLoader;

pub enum TrackGenerator {
    Fn(Box<dyn Fn(&Song) -> Vec<Frame>>),
    FnMut(Box<Mutex<dyn FnMut(&Song) -> Vec<Frame>>>),
    Song(Box<Mutex<Song>>),
}
impl TrackGenerator {
    fn call(&self, song: &Song) -> Vec<Frame> {
        match self {
            Self::Fn(f) => f(song),
            Self::FnMut(f) => {
                let mut f = f.lock().unwrap();
                f(song)
            }
            Self::Song(other) => {
                let mut other = other.lock().unwrap();
                other.generate(song.sample_rate.expect("Sample rate should have been set"));
                other.generated.clone().unwrap()
            }
        }
    }
}

impl<F: 'static + (FnMut(&Song) -> Vec<Frame>)> From<F> for TrackGenerator {
    fn from(f: F) -> Self {
        Self::FnMut(Box::new(Mutex::from(f)))
    }
}
impl From<Song> for TrackGenerator {
    fn from(song: Song) -> Self {
        Self::Song(Box::new(Mutex::from(song)))
    }
}

pub struct SongConfig {
    pub name: String,
    pub bpm: f64,
    pub volume: Automation<f64>,
    pub duration: Duration,

    pub metronome: bool,
    pub loop_on_play: bool,
}
impl Default for SongConfig {
    fn default() -> Self {
        Self {
            name: Default::default(),
            bpm: 100.,
            volume: Automation::Const(1.),
            duration: Duration::GeneratedTrack,

            metronome: false,
            loop_on_play: false,
        }
    }
}

/// Duration for the Song
///
/// Since we don't know the sample rate when creating the SongConfig, we can't tell
/// how many samples one second/beat is. By using the following enum, we can decide it
pub enum Duration {
    /// Makes the songs duration be the length of the generated track
    GeneratedTrack,
    Samples(usize),
    Seconds(f64),
    Beats(f64),
}
impl Default for Duration {
    fn default() -> Self {
        Self::GeneratedTrack
    }
}

pub struct Song {
    sample_rate: Option<u32>,
    /// Contains all of the loaded sounds used in a song
    /// Is wrapped in a Mutex so we can modify it with a non-mut reference
    sounds: Mutex<HashMap<String, Vec<Frame>>>,
    generated: Option<Vec<Frame>>,

    /// Shared VST host
    vst_host: Arc<Mutex<SimpleHost>>,
    vst_instances: Mutex<HashMap<PathBuf, Mutex<PluginLoader<SimpleHost>>>>,

    tracks: Vec<TrackGenerator>,
    config: SongConfig,
}

impl Song {
    pub fn new(tracks: Vec<TrackGenerator>, config: SongConfig) -> Self {
        Self {
            sample_rate: None,
            sounds: Default::default(),
            generated: None,
            vst_host: new_host(),
            vst_instances: Default::default(),

            tracks,
            config,
        }
    }

    // Non-mutable methods
    // These are the ones that can be called from a generator

    /// Returns duration in sample of the song
    /// Returns None if duration is Duration::GeneratedTrack
    pub fn duration(&self) -> Option<usize> {
        match self.config.duration {
            Duration::GeneratedTrack => None,
            Duration::Samples(a) => Some(a),
            Duration::Seconds(seconds) => Some(self.seconds(seconds)),
            Duration::Beats(beats) => Some(self.beats(beats)),
        }
    }

    pub fn name(&self) -> String {
        self.config.name.clone()
    }

    /// Returns the number of samples in x seconds
    pub fn seconds(&self, x: f64) -> usize {
        (self.sample_rate.unwrap() as f64 * x) as usize
    }

    /// Returns the number of samples in x microseconds
    pub fn milliseconds(&self, x: f64) -> usize {
        (self.sample_rate.unwrap() as f64 * x * 0.001) as usize
    }

    pub const fn bpm(&self) -> f64 {
        self.config.bpm
    }

    /// Returns the number of samples in x beats
    pub fn beats(&self, x: f64) -> usize {
        let bps = self.config.bpm / 60.;
        self.seconds(x / bps)
    }

    /// Loads a sound and saves it
    ///
    /// The file will only be opened the first time this method is called.
    /// Subsequent calls with the same sound will result in the vector being cloned from the hashmap
    pub fn sound(&self, sound: Sound) -> Vec<Frame> {
        assert!(self.sample_rate.is_some(), "Sample rate has not been set");

        let mut sounds = self.sounds.lock().unwrap();

        // If hashmap does not contain this audio, load it and insert it
        if !sounds.contains_key(&sound.path) {
            let vec = open_file(&sound.path, self.sample_rate.unwrap());
            sounds.insert(sound.path.clone(), vec);
        }

        // If the audio is in the hashmap, return that
        // Else, load it and insert it in the hashmap
        let vec = sounds.get(&sound.path).unwrap();

        // Return only from begin to end
        if let Some(end) = sound.end {
            vec[sound.begin..end].to_vec()
        } else {
            vec[sound.begin..].to_vec()
        }
    }

    /// Returns a new instance of the plugin
    ///
    /// It reuses the loader, but returns a new instance
    pub fn get_new_plugin_instance<P: AsRef<Path>>(&self, path: P) -> VstPlugin {
        let mut loaders = self.vst_instances.lock().unwrap();

        let path = path.as_ref().to_path_buf();

        if !loaders.contains_key(&path) {
            let loader = load_plugin(&path, &self.vst_host);
            loaders.insert(path.clone(), Mutex::from(loader));
        }

        let mut loader = loaders.get(&path).unwrap().lock().unwrap();
        VstPlugin::new(&mut loader, self.sample_rate())
    }

    /// Returns the set sample rate
    ///
    /// # Panics
    ///
    /// Panics if the sample rate was not set before calling this song
    /// This should not be an issue if you call this method from a TrackGenerator
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
            .expect("Sample rate was not set before calling this method")
    }

    /// Returns a reference to the generated audio
    /// Wrapped in an option, as it may not have been generated yet
    pub fn generated(&self) -> Option<&[Frame]> {
        self.generated.as_deref()
    }

    pub fn debug(&self, filename: &str, open_in_browser: bool) {
        if let Some(vec) = self.generated() {
            crate::debug::debug_frames_vec_web(vec, filename, open_in_browser)
        }
    }

    // Mutable methods

    /// Saves this song to a file
    pub fn save_to_file(&mut self, bits_per_sample: u16, sample_rate: u32) {
        self.generate(sample_rate);
        save_file(
            self.generated().unwrap(),
            &format!("output/{}.wav", self.config.name),
            self.sample_rate.unwrap(),
            bits_per_sample,
        );
    }

    /// Generate and start playing the song
    /// The sample rate will be chosen according to the output config from cpal
    pub fn play(&mut self) -> Result<()> {
        let config = get_player_config();

        self.generate(config.sample_rate);

        let player = Player {
            audio: self.generated().unwrap().into(),
            cycle: self.config.loop_on_play,
        };

        run_player(player, config)
    }

    /// Generates the song and saves the value
    pub fn generate(&mut self, sample_rate: u32) {
        // If it's already generated, and it's the same sample_rate, just exit
        if self.generated.is_some() {
            if let Some(prev_sample_rate) = self.sample_rate {
                if prev_sample_rate == sample_rate {
                    return;
                }
            }
        }

        self.generated = None;
        self.sample_rate = Some(sample_rate);

        let mut tracks = vec![];
        let generators = &self.tracks;
        for track in generators {
            tracks.push(track.call(self));
        }

        let tracks = join_tracks(tracks);

        let mut vec = tracks
            .effect(&Volume {
                mult: self.config.volume.clone(),
            })
            // Add some delay in the front if we enable metronome
            // This way we get like 3 beats of the metronome before we start
            .delay(if self.config.metronome {
                self.beats(3.)
            } else {
                0
            });

        if self.config.metronome {
            let metronome = self
                .sound(Metronome.into())
                .take_samples(self.beats(0.2))
                .chain(silence().take_samples(self.beats(0.8)))
                .cycle_until_samples(vec.len());
            vec = vec.add(&metronome);
        }

        // If a duration was specified, take that number of samples
        if let Some(duration) = self.duration() {
            vec = vec.take_samples(duration)
        }

        // Clamp song
        vec = vec.iter().map(|a| a.clamp(-1., 1.)).collect();

        self.generated = Some(vec);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_song() {
        let config = SongConfig {
            name: "Hello".to_string(),
            ..Default::default()
        };
        let song = Song::new(vec![], config);
        assert_eq!("Hello".to_string(), song.name());
    }

    fn test_generator(_song: &Song) -> Vec<Frame> {
        vec![Frame::default()]
    }

    #[test]
    fn can_create_track_generators() {
        let _track_gen: TrackGenerator = (|song: &Song| {
            // Can use some functions
            let beat = song.beats(1.);
            let _sound = song.sound("ehllo".into());

            vec![Frame::mono(beat as f64)]
        })
        .into();
        let _track_gen: TrackGenerator = test_generator.into();

        let vec = vec![Frame::default(); 100];
        let _track_gen: TrackGenerator = (move |_song: &Song| vec.clone()).into();

        assert!(true);
    }

    #[test]
    fn can_generate_song() {
        let track_gen: TrackGenerator = (|_song: &Song| vec![Frame::mono(1.); 100]).into();
        let mut song = Song::new(vec_into![track_gen, test_generator], SongConfig::default());

        song.generate(44_100);

        assert_eq!(100, song.generated().unwrap().len())
    }

    #[test]
    #[should_panic]
    /// Songs should have at least one working track to work
    fn cant_play_empty_song() {
        let mut song = Song::new(vec![], SongConfig::default());
        let _ = song.generate(44_100);
    }

    #[test]
    /// When loading a sound, it should get added to the hashmap so we don't load it twice
    fn sounds_get_added_to_hashmap() {
        let mut song = Song::new(vec![], SongConfig::default());
        // NOTE: In case you're reading this to learn how to use Song, sample_rate isn't a public field
        // so you can't edit it manually. When you call `generate`, you need to pass the value you want
        // It'll then be set when the generators are called, so the `sample_rate()` getter will work correctly
        song.sample_rate = Some(44_100);

        let path = "assets/beep.wav";
        let _ = song.sound(path.into());

        assert!(song.sounds.lock().unwrap().contains_key(path));
    }

    #[test]
    /// When loading a sound, it should get added to the hashmap so we don't load it twice
    fn can_get_sound_from_begin_to_end() {
        let mut song = Song::new(vec![], SongConfig::default());
        song.sample_rate = Some(44_100);

        let path = "assets/beep.wav";
        let audio = song.sound(Sound {
            path: path.to_string(),
            begin: 13,
            end: Some(23),
        });

        // Returned audio is the correct slice
        assert_eq!(10, audio.len());

        // Vec gets still added to hashmap
        assert!(song.sounds.lock().unwrap().contains_key(path));
        // Vec in hashmap has the full length of the audio
        assert_eq!(441000, song.sounds.lock().unwrap().get(path).unwrap().len());
        // Length of the beep.wav audio is 441000
    }

    #[test]
    #[should_panic]
    fn opening_nonexistant_sound_panics() {
        let mut song = Song::new(vec![], SongConfig::default());
        song.sample_rate = Some(44_100);

        let path = "assets/filethatdoesntexist.aaaaaaaaaaaaaa";
        let _ = song.sound(path.into());
    }

    #[test]
    fn duration_takes_just_the_correct_ammount_of_samples() {
        let config = SongConfig {
            duration: Duration::Samples(23),
            ..Default::default()
        };
        let mut song = Song::new(
            vec_into![|_song: &Song| vec![Frame::default(); 100]],
            config,
        );
        song.generate(44_100);

        let audio = song.generated().unwrap();
        assert_eq!(23, audio.len());
    }

    #[test]
    fn can_compose_songs() {
        let inner_song = Song::new(
            vec_into![|_song: &Song| vec![Frame::default(); 100]],
            SongConfig::default(),
        );

        let _song = Song::new(
            vec_into![|_song: &Song| vec![Frame::default(); 100], inner_song],
            SongConfig::default(),
        );
    }

    #[test]
    fn can_use_song_as_part_of_track() {
        let mut inner_song = Song::new(
            vec_into![|_song: &Song| vec![Frame::default(); 100]],
            SongConfig::default(),
        );

        let mut song = Song::new(
            vec_into![move |song: &Song| {
                let vec = vec![Frame::mono(1.); 100];

                inner_song.generate(song.sample_rate());
                vec.chain(inner_song.generated.clone().unwrap())
            }],
            SongConfig::default(),
        );
        song.generate(44_100);

        assert_eq!(200, song.generated().as_ref().unwrap().len());
    }

    #[cfg(target_os = "macos")]
    #[test]
    fn can_load_vst_plugin() {
        let mut song = Song::new(vec![], SongConfig::default());
        song.sample_rate = Some(44_100);

        let plugin =
            song.get_new_plugin_instance("assets/vsts/TestPlugin.vst/Contents/MacOS/TestPlugin");

        let audio = plugin.process_samples(vec![Frame::mono(0.); 1000]);

        assert_eq!(1000, audio.len());

        // TestPlugin returns a sine wave at a specific frequency
        // Here we check that we get a predefined set of values
        assert!(audio[0].left < 0.0000001);
        assert!(audio[0].right < 0.0000001);

        assert!(audio[1].left > 0.000001);
        assert!(audio[1].right > 0.000001);

        assert!(audio[999].left < -0.0201566071);
        assert!(audio[999].left > -0.0201566072);
        assert!(audio[999].right < -0.0201566071);
        assert!(audio[999].right > -0.0201566072);
    }
}
