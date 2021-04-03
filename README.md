<img src="/images/logo.png" width="320px">

# Dawremi

Dawremi (pronounced do-re-mi) is a code-based Digital Audio Workstation.

This project kinda works, but is generally in an early stage, and documentation and guides are non-existant. I'll try to improve this once the API stabilizes. For the time being, you can look through the examples or reading through the autogenerated docs, which should be somewhat understandable. But then again, I'm the one who wrote the code, if I didn't think it's understandable we would have a problem.

## Getting started

Then add this repo as a dependency in your `Cargo.toml`:

```toml
dawremi = { git = "https://github.com/guimcaballero/dawremi" }
```

Copy the `assets` folder into your project's root folder, so you can access preadded samples and other needed assets. You can get it easily with `svn` with:

```bash
svn checkout https://github.com/guimcaballero/dawremi/trunk/assets
```

Then, making a new song is easy! You have to create a new struct using the `song!()` macro, and then implement the `Song` trait for that struct. The following is a demo song, copied from `examples/demo_song.rs`:

```rust
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
    let mut song = Song::new(vec![plucked_track, other_track], config);
    // Uncomment the following line to play the song
    // song.play().expect("Unable to play song");
}

/// This is a track
fn plucked_track(song: &Song) -> Vec<Frame> {
    // We ue a macro to play a sequence of notes. Returns a Vec<Frame>
    sequence!(
        song,
        // The lenght of one note in beats
        len: 1.,
        // The note representation we want to use. GuitarFretboard simulates a guitar tab,
        // using the first letter as the string, and the number as the finger position
        // L is the low e string
        note: GuitarFretboard,
        // The function we want to use to generate the sound
        fun: |note| plucked(song, note, InitialBurstType::Triangle(2, 3)),

        // List of notes. Underscores signal silences
        L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
    )
    // We can chain another sequence
    .chain(sequence!(
        song,
        len: 1., note: GuitarFretboard,
        fun: |note| plucked(song, note, InitialBurstType::DoubleTriangle),

        L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
    ))
}

/// This is a helper function
fn plucked(song: &Song, frequency: impl Into<Frequency>, burst: InitialBurstType) -> Synth {
    Synth::new(
        Box::new(Plucked::new(
            burst,
            frequency.into(),
            song.sample_rate() as f64,
        )),
        song.sample_rate() as f64,
    )
}

fn other_track(song: &Song) -> Vec<Frame> {
    // We can also make a list of notes and manipulate it.
    let notes1: Vec<Vec<Note>> = {
        use Note::*;
        // This is equivalent to vec![vec!(A4, C4), vec!(A5), vec!(A6), vec!(), vec!(A6)]
        // A4 and C4 will be played together, followed by A5, then A6, then a silence, then A6
        note_list![[A4, C4], A5, A6, _, A6]
    };

    // `generate` will transform the notes into sounds
    let sound1 = notes1.generate(
        // Function to use to generate the audio
        &mut |note, length| {
            plucked(song, note, InitialBurstType::Triangle(2, 3)).take_samples(length)
        },
        // Length of each note
        song.beats(1.),
    );

    // Generate sounds one octave higher
    let sound2 = notes1
        // We can map over the notes
        .map_notes(Note::up_an_octave)
        .generate(
            &mut |note, length| {
                plucked(song, note, InitialBurstType::Triangle(2, 3)).take_samples(length)
            },
            song.beats(1.),
        );

    // We can use other types of notes too
    let bass = {
        use GuitarFretboard::*;
        note_list![L5, L5, _, L8, L8, _, L1, L1, _, L4, L4,]
    }
    .generate(
        &mut |note, length| plucked(song, note, InitialBurstType::Sine).take_samples(length),
        song.beats(1.),
    );

    // We then joing the subtracks into one
    join_tracks(vec![sound1, sound2, bass])
        // We can also add effects to the whole track, like Reverb (using convolution)
        .effect(&Convolution::new(
            song.sound(Reverb::LargeLongEchoHall.into()),
        ))
        // Or volume
        .effect(&Volume {
            mult: Automation::Const(0.5),
        })
}
```

## Importing assets

If you would like to use external sounds with Dawremi, but don't like to use paths as a string, you can add build script to autogenerate enums for files inside folders. Create a file called `build.rs`:

```no_run
fn main() {
    dawremi::prelude::generate_sound_enums(vec!["path/to/assets"]);
}
```

and add `dawremi` to your `build-dependencies` section in `Cargo.toml`. You can pass as many different folders as you want to `generate_sound_enums`.

This is implemented very naively currently, so there's a high probability of it breaking if you do anything complex (Contributions are welcome!). Specifically, if you have two folders named the same, it'll fail to compile.

For example, if you have a folder like the following:

```ignore
assets/internal/reverbs
├── block_inside.wav
├── bottle_hall.wav
├── cement_block2.wav
├── cement_blocks1.wav
├── chateau_de_logne.wav
├── conic_long_echo_hall.wav
├── deep_space.wav
└── vocal_duo.wav
```

The following enum will be generated:

```rust
pub enum Reverb {
    BlockInside,
    BottleHall,
    CementBlock2,
    CementBlocks1,
    ChateauDeLogne,
    ConicLongEchoHall,
    DeepSpace,
    VocalDuo,
}
```

Which will implement `Into<Sound>`, so you can do `self.sound(Rever::VocalDuo.into())` in a song.

Finally, call `include_sound_enums!()` from somewhere in you crate:

```ignore
#[macro_use]
extern crate dawremi;

dawremi::include_sound_enums!();

fn main() {
    // Something here
}
```

## Minimum Supported Rust Version

This crate requires a Rust version equal or superior to `1.51`. ~~const generics go brrrrrr~~

## Contributing

If you are using the library and find any bug/problem, please open an issue! I'm focusing most of my time on developing the library instead of using it, so there's probably a lot of stuff I haven't caught yet.

Contributions are welcome, but please do open an issue before starting to code anything, as I might not accept your PR for whatever reason, and I don't want anyone wasting work.

