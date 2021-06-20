//! Dawremi has some support of traditional western music theory
//! This is a demonstration of how it works

use dawremi::prelude::*;

fn main() {
    let config = SongConfig {
        name: "Drum patterns".to_string(),
        bpm: 160.,
        ..Default::default()
    };
    let mut song = Song::new(vec![drums.into()], config);
    song.play().expect("Unable to play song");
}

fn drums(song: &Song) -> Vec<Frame> {
    use PatternStep::*;

    // There's a bunch of ways to make patterns
    // You can use an array of PatternStep
    // You can use a binary number (or any number actually)
    // You can use an array of booleans

    // Usually you'd use only one to keep consistency
    // Here we use all as a demo

    [
        [X, O, X, O, X, O, X, O].sound("assets/examples/kick.wav"),
        0b00100010usize.sound("assets/examples/snare.wav"),
        // Same as:
        // 0x22usize.sound("assets/examples/snare.wav"),
        // 34usize.sound("assets/examples/snare.wav"),
        [true; 8].sound("assets/examples/hihat.wav"),
    ]
    .generate(song, 2)
}
