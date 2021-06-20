//! Contains stuff dealing with audio files

pub mod enums;
pub mod io;
pub mod pattern;

#[derive(Eq, PartialEq, Ord, PartialOrd, Hash, Clone)]
pub struct Sound {
    /// Path to audio file
    pub path: String,
    /// From where the audio should start
    pub begin: usize,
    /// None represents until end of sound
    pub end: Option<usize>,
}

impl From<String> for Sound {
    fn from(path: String) -> Self {
        Self {
            path,
            begin: 0,
            end: None,
        }
    }
}
impl From<&str> for Sound {
    fn from(path: &str) -> Self {
        Self {
            path: path.to_string(),
            begin: 0,
            end: None,
        }
    }
}
