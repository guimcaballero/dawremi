#![feature(box_syntax)]
#![feature(arbitrary_enum_discriminant)]

use std::io::stdin;

mod helpers;
mod song;
use song::*;
mod player;
use player::start;
mod notes;
mod record;
mod synth;

fn main() -> Result<(), anyhow::Error> {
    // Recording mode
    println!("Press R to go into recording mode");
    println!("Press anything else to go into playing mode");
    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    if s.trim() == "R" {
        return record::main();
    }

    // Playing mode
    let song = select_song();
    start(song)
}
