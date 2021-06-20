//! This example demonstrates using ARPL to generate random notes
//! The output of this example will always be the same unless you change the used pattern

use dawremi::prelude::*;

fn main() {
    let config = SongConfig {
        name: "ARPL".to_string(),
        bpm: 120.,
        ..Default::default()
    };
    let mut song = Song::new(vec![plucked_track.into()], config);
    song.play().expect("Unable to play song");
}

fn plucked_track(song: &Song) -> Vec<Frame> {
    // Make a pattern
    let pat = Pattern::new("A&OH3+45kdn+jAJ!K247[N,J|hy]");
    // Change this pattern to generate different notes
    // A good way to generate interesting sounds is to smash the keyboard
    // other options are to take Aphex Twin song titles
    // or you can make a gay person laugh and take their keysmash

    // Make it generate notes out of the Major Pentatonic scale
    let pat = pat.scale(Note::C4, Scale::MajorPentatonic);

    // Take 20 notes and make them last 1 beat
    let notes = pat.take(20).map(|note| note.beats(1.)).collect::<Vec<_>>();

    notes
        .generate(
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
        )
        .effect(&Volume {
            mult: Automation::Const(0.3),
        })
}
