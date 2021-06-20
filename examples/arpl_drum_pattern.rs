//! You can also make sound patterns out of ARPL

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
    // How many steps we want
    const LEN: usize = 16;

    // ARPL pattern
    let pat = Pattern::new("A&OH3+45kdn+jAJ!K247[N,J|hy]");

    [
        // We can generate a SoundPattern directly
        pat.clone().sound("assets/examples/kick.wav").get::<LEN>(),
        // Or we can also give it a (u64 -> bool) function to determine when a step is on
        // `get` uses i % 2 == 0
        pat.sound("assets/examples/snare.wav")
            .with::<LEN>(&|i| (i + 1) % 3 == 0),
        // We can combine them with other patterns
        [true; LEN].sound("assets/examples/hihat.wav"),
    ]
    .generate(song, 1)
}
