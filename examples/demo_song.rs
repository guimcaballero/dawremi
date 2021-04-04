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
    let mut song = Song::new(vec![plucked_track.into(), other_track.into()], config);
    song.play().expect("Unable to play song");
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
        Box::new(Plucked::new(burst, frequency.into(), song.sample_rate())),
        song.sample_rate(),
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
