use crate::frame::*;
use crate::helpers::*;
use crate::player::*;
use crate::traits::*;
use anyhow::Result;
use num::integer::lcm;

pub trait Looper: HasSampleRate + HasSoundHashMap {
    fn generate(&mut self) -> Vec<Frame> {
        let mut tracks: Vec<(usize, Vec<Frame>)> = self
            .tracks()
            .drain(..)
            .map(|track| (track.len(), track))
            .collect();

        let lcm = tracks.iter().fold(1, |acc, track| lcm(acc, track.0));

        let tracks = tracks
            .drain(..)
            .map(|track| track.1.repeat(lcm / track.0))
            .collect();

        let vec = join_tracks(tracks);

        if self.enable_normalization() {
            // Normalize
            let (max, min) = vec
                .iter()
                .cloned()
                .fold((-1. / 0., 1. / 0.), |(max, min), a| {
                    (f64::max(max, a.max()), f64::min(min, a.min()))
                });
            let max = f64::max(max.abs(), min.abs());
            vec.iter().map(|a| a / max).collect()
        } else {
            vec
        }
    }

    fn play(&mut self) -> Result<()> {
        let config = get_player_config();

        self.set_sample_rate(config.sample_rate as f64);

        let player = Player {
            audio: self.generate().into(),
            cycle: true,
        };

        run_player(player, config)
    }

    /// Returns the number of samples that should be taken to pass x beats
    fn beats(&self, x: f64) -> usize {
        let bps = self.bpm() as f64 / 60.;
        self.seconds(x / bps)
    }

    fn tracks(&mut self) -> Vec<Vec<Frame>>;

    // Settings

    /// Display name for the Looper
    fn name(&self) -> &'static str;

    /// Beats per minute for the Looper
    fn bpm(&self) -> usize;

    /// Toggles normalization of frames
    fn enable_normalization(&self) -> bool {
        true
    }
}
