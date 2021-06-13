//! Dawremi has some support of traditional western music theory
//! This is a demonstration of how it works

use dawremi::prelude::*;

fn main() {
    let config = SongConfig {
        name: "Music theory".to_string(),
        bpm: 120.,
        ..Default::default()
    };
    let mut song = Song::new(vec![plucked_track.into()], config);
    song.play().expect("Unable to play song");
}

fn guitar(song: &Song, frequency: Frequency, length: usize, burst: InitialBurstType) -> Vec<Frame> {
    Plucked(burst).generate(
        length,
        frequency,
        song.sample_rate(),
        Plucked::default_adsr(song.sample_rate()),
    )
}

fn plucked_track(song: &Song) -> Vec<Frame> {
    let notes = {
        use Note::*;
        [
            (C4 >> Chord::Major).beats(1.),
            (F4 >> Chord::Minor).beats(1.),
            (F4 >> Chord::DominantSeventh).beats(1.),
            Silence.beats(2.),
            (C4 >> Chord::Major).beats(1.),
            (F4 >> Chord::Minor).beats(1.),
            (F4 >> Chord::DominantSeventh).beats(1.),
        ]
    };

    notes
        .generate(
            song,
            // Function to use to generate the audio
            &mut |note, length| guitar(song, note, length, InitialBurstType::Triangle(2, 3)),
        )
        .effect(&Volume {
            mult: Automation::Const(0.3),
        })
}
