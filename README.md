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
//! Song to demo Dawremi's features. Sounds horrible, listening is not recommended

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
    let mut song = Song::new(vec![plucked_track.into()], config);

    // Uncomment the line below to play the song through your speakers
    // song.play().expect("Unable to play song");
}

// This is a track
fn plucked_track(song: &Song) -> Vec<Frame> {
    let notes1 = {
        use Note::*;
        [
            [A4, C4].beats(1.),       // Two notes at once for one beat
            Silence.beats(2.),        // Silence for two beats
            C4.beats(1.),             // Single note for a beat
            [C4, C5, C6].seconds(3.), // Three notes for three seconds
        ]
    };

    // `generate` will transform the notes into sounds
    let sound1 = notes1.generate(
        song,
        // Function to use to generate the audio
        &mut |note, length| {
            Plucked(InitialBurstType::Triangle(2, 3)).generate(
                length,
                note.into(),
                song.sample_rate(),
            )
        },
        Plucked::default_adsr(song.sample_rate()),
    );

    // Generate sounds one octave higher
    let sound2 = notes1
        // We can map over the frequencies
        .map_frequencies(|frequency| *frequency * 2.)
        .generate(
            song,
            &mut |note, length| {
                Plucked(InitialBurstType::Triangle(2, 3)).generate(
                    length,
                    note.into(),
                    song.sample_rate(),
                )
            },
            Plucked::default_adsr(song.sample_rate()),
        );

    // We can use other types of notes too
    let bass = {
        use GuitarFretboard::*;
        // Ln represents the Low E string
        note_list![L5, L5, _, L8, L8, _, L1, L1, _, L4, L4, [L4, E4]]
            .into_iter()
            .map(|notes| notes.beats(1.))
            .collect::<Vec<Trigger>>()
    }
    .generate(
        song,
        &mut |note, length| {
            Plucked(InitialBurstType::Sine).generate(length, note.into(), song.sample_rate())
        },
        Plucked::default_adsr(song.sample_rate()),
    );

    // We then joing the subtracks into one
    let track = join_tracks(vec![sound1, sound2, bass])
        // We can also add effects to the whole track, like Reverb (using convolution)
        .effect(&Convolution::new(
            song.sound(Reverb::LargeLongEchoHall),
        ));

    let track_len = track.len();

    // Effects usually take automations
    track
        .effect(&Volume {
            // To an automation we can pass a constant value
            mult: Automation::Const(0.8),
        })
        .effect(&Volume {
            // Or we can pass a vector. In this case, we pass a sine wave
            mult: Automation::Vec(waves::sine(
                track_len,
                Automation::Const(1.),
                song.sample_rate(),
            )),
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
pub enum Reverbs {
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

Which will implement `Into<Sound>`, so you can do `self.sound(Reverbs::VocalDuo)` in a song.

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

