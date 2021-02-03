use crate::helpers::*;
use crate::player::*;
use crate::traits::*;
use anyhow::Result;
use num::integer::lcm;

pub trait Looper: HasSampleRate + HasSoundHashMap {
    fn generate(&mut self) -> Vec<f64> {
        let mut tracks: Vec<(usize, Vec<f64>)> = self
            .tracks()
            .drain(..)
            .map(|track| (track.len(), track))
            .collect();

        let lcm = tracks.iter().fold(1, |acc, track| lcm(acc, track.0));

        let tracks = tracks
            .drain(..)
            .map(|track| track.1.repeat(lcm / track.0))
            .collect();

        join_tracks(tracks)
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

    fn tracks(&mut self) -> Vec<Vec<f64>>;
    fn bpm(&self) -> usize;
    fn name(&self) -> &'static str;
}
