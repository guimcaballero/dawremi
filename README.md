# Dawremi

Dawremi (pronounced do-re-mi) is a toy code-based Digital Audio Workstation.

It's in a very early stage and documentation is basically non-existant.

## How to run

Make sure you have Nightly Rust installed. Then you can clone this repository and run:

```bash
cargo run
```

This will let you choose a song, which will then play and be saved to the `output` directory.

You can also use `cargo run --features metronome` to run with the metronome active.

## Making songs

Create a new Rust file in the `src/song/` directory. You have to create a new struct using the `song!()` macro, and then implement the `Song` trait. The following is a demo song:

```rust
use super::*;
use crate::synth::*;
use crate::effects::*;

song!(Test,);

impl Song for Test {
    /// Display name for the song
    fn name(&self) -> &'static str {
        "test"
    }

    /// Song's beats per minute
    fn bpm(&self) -> usize {
        120
    }

    /// Song's duration (in samples)
    fn duration(&self) -> usize {
        /// We're using the beats helper function that converts 16 beats to the number of samples to take
        self.beats(16.)
    }
 
    /// list of tracks on this song. Each track is just a list of samples (Vec<f64>)
    /// All of the tracks will be mixed equally
    fn tracks(&mut self) -> Vec<Vec<f64>> {
        vec![
            self.plucked_track(),
            self.other_track(),
        ]
    }
}

impl Test {
    /// This is a track
    fn plucked_track(&self) -> Vec<f64> {
        /// We ue a macro to play a sequence of notes. Returns a Vec<f64>
        sequence!(
            self,
            // The lenght of one note in beats
            len: 1.,
            // The note representation we want to use. GuitarFretboard simulates a guitar tab,
            // using the first letter as the string, and the number as the finger position
            // L is the low e string
            note: GuitarFretboard,
            // The function we want to use to generate the sound
            fun: |note| self.plucked(note, InitialBurstType::Triangle(2, 3)),

            // List of notes. Underscores signal silences
            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        )
        // We can chain another sequence
        .chain(&mut sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::DoubleTriangle),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
    }

    /// This is a helper function
    fn plucked(&self, note: Note, burst: InitialBurstType) -> Synth {
        Synth::new(
            box Plucked::new(burst, note, self.get_sample_rate()),
            note,
            self.get_sample_rate(),
        )
    }

    fn other_track(&mut self) -> Vec<f64> {
        // We can also make a list of notes and manipulate it. Nones are silences
        let notes1: Vec<Option<Note>> = {
            use Note::*;
            // This is equivalent to vec![Some(A4), Some(A5), Some(A6), None, Some(A6)]
            option_vec![A4, A5, A6, _, A6]
        };

        // `generate` will transform the notes into sounds
        let sound1 = notes1.generate(
            // Function to use
            &|note, length| {
                self.plucked(note, InitialBurstType::Triangle(2, 3))
                    .take_samples(length)
            },
            // Length of each note
            self.beats(1.),
        );

        // Generate sounds one octave higher
        let sound2 = notes1
            // We can map over the notes
            .map_notes(Note::up_an_octave)
            .generate(
                &|note, length| {
                    self.plucked(note, InitialBurstType::Triangle(2, 3))
                        .take_samples(length)
                },
                self.beats(1.),
            );

        // We can use other types of notes too, converting it with `into_notes`
        let bass = {
            use GuitarFretboard::*;
            option_vec![L5, L5, _, L8, L8, _, L1, L1, _, L4, L4,]
        }
        .into_notes()
        .generate(
            &|note, length| {
                self.plucked(note, InitialBurstType::Sine)
                    .take_samples(length)
            },
            self.beats(1.),
        );

        // We then joing the subtracks into one
        join_tracks(vec![sound1, sound2, bass])
            // We can also add effects to the whole track, like Echo
            .effect(&Convolution::new(
                self.sound(Reverb::LargeLongEchoHall.into()),
            ))
            // Or volume
            .effect(&Volume { mult: 0.5 })
    }
}
```
