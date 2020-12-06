#![feature(box_syntax)]
#![feature(arbitrary_enum_discriminant)]

mod helpers;
mod song;
use song::*;
mod player;
use player::start;
mod notes;
mod synth;

fn main() -> Result<(), anyhow::Error> {
    let song = select_song();
    start(song)
}
