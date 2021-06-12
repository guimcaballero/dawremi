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
        &mut |note, length| guitar(song, note, length, InitialBurstType::Triangle(2, 3)),
    );

    // Generate sounds one octave higher
    let sound2 = notes1
        // We can map over the frequencies
        .map_frequencies(|frequency| *frequency * 2.)
        .generate(song, &mut |note, length| {
            guitar(song, note, length, InitialBurstType::Triangle(2, 3))
        });

    // We can use other types of notes too
    let bass = {
        use GuitarFretboard::*;
        // Ln represents the Low E string
        note_list![L5, L5, _, L8, L8, _, L1, L1, _, L4, L4, [L4, E4]]
            .into_iter()
            .map(|notes| notes.beats(1.))
            .collect::<Vec<Trigger>>()
    }
    .generate(song, &mut |note, length| {
        guitar(song, note, length, InitialBurstType::Sine)
    });

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
