mod extensions;
mod macros;

pub mod delay_line;
pub mod interpolation;

pub use extensions::*;
pub use macros::*;

use crate::frame::*;

pub fn silence() -> Vec<Frame> {
    vec![Frame::default()]
}

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn join_tracks_test() {
        let tracks = vec![vec![1., 1., 0., 0.5, 0.3], vec![0., 1., 0., 0.5, 0.5]];

        assert_eq!(vec![0.5, 1., 0., 0.5, 0.4], join_tracks(tracks))
    }
}
