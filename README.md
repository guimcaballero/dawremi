# Dawremi

This is a toy code-based Digital Audio Workstation.

It's in a very early stage and documentation is basically non-existant.

## How to run

Make sure you have Nightly Rust installed. Then you can clone this repository and run:

```bash
cargo run
```

This will let you choose a song, which will then play and be saved to the `output` directory.

You can also use `cargo run --features metronome` to run with the metronome active.

## Making songs

Create a new Rust file in the `src/song/` directory. You have to create a new struct using the `song!()` macro, and then implement the `Song` trait:

```rust
use super::*;
use crate::synth::*;

song!(Test,);

impl Song for Test {
    fn name(&self) -> &'static str {
        "test"
    }

    fn bpm(&self) -> usize {
        120
    }
    fn duration(&self) -> usize {
        self.beats(16.)
    }

    #[allow(unreachable_code)]
    fn track1(&self) -> Option<Vec<f64>> {
        return None;
        Some(sequence!(
                self,
                len: 1.,
                fun: |note| self.hihat(note),

                G4 G4 D4 D4 E4 E4 (D4 * 2.)
        ))
    }
}

impl Test {
    fn harmonica(&self, note: Note) -> Synth {
        Synth::new(
            box Harmonica::new(note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }
}
```
