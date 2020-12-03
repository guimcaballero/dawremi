mod helpers;
mod song;
use song::test_song::Test;
mod player;
use player::start;
mod synth;

fn main() -> Result<(), anyhow::Error> {
    let song = Test::default();
    start(song)
}
