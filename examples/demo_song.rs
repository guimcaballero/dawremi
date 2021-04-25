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
    song.play().expect("Unable to play song");
}

/// This is a helper function
fn guitar(song: &Song, frequency: Frequency, length: usize, burst: InitialBurstType) -> Vec<Frame> {
    Plucked(burst).generate(
        length,
        frequency,
        song.sample_rate(),
        Plucked::default_adsr(song.sample_rate()),
    )
}

// This is a track
fn plucked_track(song: &Song) -> Vec<Frame> {
    // We can make a list of notes and manipulate it.
    let notes1: Vec<Vec<Note>> = {
        use Note::*;
        // This is equivalent to vec![vec!(A4, C4), vec!(A5), vec!(A6), vec!(), vec!(A6)]
        // A4 and C4 will be played together, followed by A5, then A6, then a silence, then A6
        note_list![[A4, C4], A5, A6, _, A6]
    };

    // `generate` will transform the notes into sounds
    let sound1 = notes1.generate(
        // Function to use to generate the audio
        &mut |note, length| guitar(song, note, length, InitialBurstType::Triangle(2, 3)),
        // Length of each note
        Automation::Const(song.beats(1.)),
    );

    // Generate sounds one octave higher
    let sound2 = notes1
        // We can map over the notes
        .map_notes(Note::up_an_octave)
        .generate(
            &mut |note, length| guitar(song, note, length, InitialBurstType::Triangle(2, 3)),
            Automation::Const(song.beats(1.)),
        );

    // We can use other types of notes too
    let bass = {
        use GuitarFretboard::*;
        note_list![L5, L5, _, L8, L8, _, L1, L1, _, L4, L4,]
    }
    .generate(
        &mut |note, length| guitar(song, note, length, InitialBurstType::Sine),
        Automation::Const(song.beats(1.)),
    );

    // We then joing the subtracks into one
    let track = join_tracks(vec![sound1, sound2, bass])
        // We can also add effects to the whole track, like Reverb (using convolution)
        .effect(&Convolution::new(
            song.sound(Reverb::LargeLongEchoHall.into()),
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
