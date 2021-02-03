#![feature(box_syntax)]
#![feature(arbitrary_enum_discriminant)]

use std::io::stdin;

#[macro_use]
extern crate dawremi_core;
use dawremi_core::record;

mod songs;
use songs::*;
mod loopers;
use loopers::*;

fn main() -> Result<(), anyhow::Error> {
    // Recording mode
    println!("Press R to go into recording mode");
    println!("Press L to go into looper mode");
    println!("Press anything else to go into playing mode");

    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    match s.trim() {
        "R" => record::main(),
        "L" | "l" => {
            let mut l = select_looper();
            l.play()
        }
        _ => {
            // Playing mode
            let mut song = select_song();
            song.play()
        }
    }
}
