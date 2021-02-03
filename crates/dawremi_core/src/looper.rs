use crate::helpers::*;
use num::integer::lcm;

pub struct Looper {
    tracks: Vec<LooperTrack>,
}
impl Looper {
    fn new(tracks: Vec<LooperTrack>) -> Self {
        Self { tracks }
    }

    fn generate(mut self) -> Vec<f64> {
        if self.tracks.is_empty() {
            return Vec::new();
        }

        let lcm = self.tracks.iter().fold(1, |acc, track| lcm(acc, track.len));

        let tracks = self
            .tracks
            .drain(..)
            .map(|track| track.track.repeat(lcm / track.len))
            .collect();

        join_tracks(tracks)
    }
}

pub struct LooperTrack {
    track: Vec<f64>,
    len: usize,
}
impl LooperTrack {
    fn new(track: Vec<f64>) -> Self {
        Self {
            len: track.len(),
            track,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn generate() {
        let looper = Looper::new(vec![
            LooperTrack::new(vec![0., 1., 0.]),
            LooperTrack::new(vec![0., 1., 1., 1.]),
        ]);

        let expected = vec![0.0, 1.0, 0.5, 0.5, 0.5, 0.5, 0.5, 1.0, 0.0, 0.5, 1.0, 0.5];
        assert_eq!(expected, looper.generate());
    }
}
