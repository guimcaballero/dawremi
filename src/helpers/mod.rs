//! Various helper functions and macros

pub mod delay_line;
mod extensions;
#[macro_use]
mod macros;
pub mod pitch_detection;
pub mod resampling;

pub use extensions::*;
pub use macros::*;

use crate::frame::*;

// The following need a better place to live in, but I don't really want to have a whole file for each of them

pub fn silence() -> Vec<Frame> {
    vec![Frame::default()]
}

/// Joins all of the tracks into a single one
pub fn join_tracks(tracks: Vec<Vec<Frame>>) -> Vec<Frame> {
    // Get the max length of the tracks
    let len = &tracks
        .iter()
        .map(|track| track.len())
        .max()
        .expect("There should be at least one track to join");

    (0..*len)
        .map(|i| {
            let mut val = Frame::default();
            for track in &tracks {
                if let Some(value) = track.get(i) {
                    val += value;
                }
            }
            val
        })
        .collect()
}

/// Concatenates tracks by trimming and overlapping the audios
pub fn overlap_add(vec: Vec<Vec<Frame>>, overlap: usize) -> Vec<Frame> {
    let mut result = vec![Frame::default(); overlap + 1];
    for part in vec {
        result = result.overlap(part, overlap);
    }

    result
}

/// Concatenates tracks by trimming and overlapping the audios
pub fn concat_by_trim_overlap(vec: Vec<Vec<Frame>>, overlap: usize) -> Vec<Frame> {
    let mut result = vec![Frame::default(); overlap + 1];
    for part in vec {
        result = result.overlap(part.trim(), overlap);
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn join_tracks_test() {
        let tracks = vec![
            vec![1., 1., 0., 0.5, 0.3].into_frames(),
            vec![0., 1., 0., 0.5, 0.5].into_frames(),
        ];

        assert_eq!(
            vec![1.0, 2., 0., 1.0, 0.8].into_frames(),
            join_tracks(tracks)
        )
    }
}
