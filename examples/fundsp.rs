//! This example demonstrates how to use Dawremi with FunDSP

use dawremi::prelude::*;

fn main() {
    let config = SongConfig {
        name: "FunDSP".to_string(),
        bpm: 120.,
        ..Default::default()
    };
    let mut song = Song::new(vec![main_track.into()], config);
    song.play().expect("Unable to play song");
}

fn main_track(song: &Song) -> Vec<Frame> {
    use dawremi::fundsp::prelude::hacker::*;

    let mut c = lfo(|t| {
        xerp11(50.0, 5000.0, snoise(0, t)) * lerp11(0.5, 1.0, ewave_hz(sigmoid(1.0), 8.0, t))
    }) >> triangle();
    c.reset(Some(song.sample_rate() as f64));

    c.take_stereo(song.seconds(10.))
}
