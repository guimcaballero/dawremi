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

pub fn silence() -> Vec<Frame> {
    vec![Frame::default()]
}

/// Joins all of the tracks into a single one by mixing them equally
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
            let mut count = 0;
            for track in &tracks {
                if let Some(value) = track.get(i) {
                    val += value;
                    count += 1;
                }
            }
            val / count as f64
        })
        .collect()
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
            vec![0.5, 1., 0., 0.5, 0.4].into_frames(),
            join_tracks(tracks)
        )
    }
}
