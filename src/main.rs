#![feature(box_syntax)]
#![feature(arbitrary_enum_discriminant)]
#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::style,
    clippy::perf,
    clippy::complexity
)]
#![allow(clippy::wildcard_imports, clippy::enum_glob_use)]
#![warn(clippy::wrong_pub_self_convention, clippy::unseparated_literal_suffix)]

use std::io::stdin;

#[macro_use]
extern crate dawremi_core;
use dawremi_core::record;

mod loopers;
mod songs;

fn main() {
    // Recording mode
    println!("Press R to go into recording mode");
    println!("Press L to go into looper mode");
    println!("Press S to save a song as a file");
    println!("Press anything else to go into playing mode");

    let mut s = String::new();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");

    println!();

    match s.trim() {
        "R" | "r" => record::main().expect("Recording failed"),
        "L" | "l" => {
            let mut l = loopers::select_looper();
            l.play().expect("Unable to play song")
        }
        "S" | "s" => {
            // Playing mode
            let mut song = songs::select_song();
            song.set_sample_rate(44_100.);
            song.save_to_file()
        }
        _ => {
            // Playing mode
            let mut song = songs::select_song();
            song.play().expect("Unable to play song")
        }
    }
}
