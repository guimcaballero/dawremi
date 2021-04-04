/*!
Dawremi (pronounced do-re-mi) is a code-based Digital Audio Workstation.

This crates objective is to be able to write songs easily and expressively
while coding in Rust.

To use Dawremi, declare a struct using the `song!()` macro, and implement the `Song` trait for it:
```
#[macro_use]
extern crate dawremi;
use dawremi::prelude::*;

fn main() {
    let config = SongConfig {
        name: "Demo song".to_string(),
        bpm: 120.,
        duration: Duration::Beats(16.),
        ..Default::default()
    };
    let mut song = Song::new(
        vec_into![|song: &Song| noise::noise(3333, song.duration().unwrap()).into_frames()],
        config
    );
    // Uncomment the following line to play the song
    // song.play().expect("Unable to play song");
}
```

Documentation is still a bit barebones still, so you'll have to dig a bit through the code and the
examples to get an idea of how to use the crate.
*/

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

pub use dawremi_core::*;

// The following makes it so `cargo test --all` also tests the Readme code
// From https://github.com/rust-lang/cargo/issues/383#issuecomment-720873790

#[cfg(doctest)]
mod test_readme {
    macro_rules! external_doc_test {
        ($x:expr) => {
            #[doc = $x]
            extern "C" {}
        };
    }

    external_doc_test!(include_str!("../README.md"));
}
