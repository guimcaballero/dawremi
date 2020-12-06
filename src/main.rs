#![feature(box_syntax)]

mod helpers;
mod song;
use song::*;
mod player;
use player::start;
mod synth;

fn main() -> Result<(), anyhow::Error> {
    let song = select_song();
    start(song)
}
