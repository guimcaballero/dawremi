mod song;
use song::Test;
mod player;
use player::start;

fn main() -> Result<(), anyhow::Error> {
    let song = Test::default();
    start(song)
}
