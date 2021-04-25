//! This example demonstrates how to create a simple effect
//! We'll be making a copy of the builtin `Volume` effect

#[macro_use]
extern crate dawremi;

use dawremi::prelude::*;
use std::f64::consts::TAU;

fn main() {
    let config = SongConfig {
        name: "Custom Synth Instrument".to_string(),
        bpm: 120.,
        duration: Duration::GeneratedTrack,
        ..Default::default()
    };
    let mut song = Song::new(vec![track.into()], config);
    song.play().expect("Unable to play song");
}

fn track(song: &Song) -> Vec<Frame> {
    {
        use Note::*;
        note_list![[A4, C4], A5, A6, _, A6]
    }
    .generate(
        &mut |note, length| instrument(song, note.into(), length),
        Automation::Const(song.beats(1.)),
    )
}

fn instrument(song: &Song, frequency: Frequency, length: usize) -> Vec<Frame> {
    Sine.generate(
        length,
        frequency,
        song.sample_rate(),
        Sine::default_adsr(song.sample_rate()),
    )
}

// `Sine` will be our custom Instrument
pub struct Sine;
impl Instrument for Sine {
    /// The default adsr values for your instrument. You can skip implementing this method
    fn default_adsr(sample_rate: u32) -> Adsr {
        let sr = sample_rate as f64;
        Adsr {
            attack: (sr * 0.01) as usize,
            decay: (sr * 0.15) as usize,
            release: 0,

            attack_amplitude: 1.,
            sustain_amplitude: 0.,
        }
    }

    /// This method has to return a Vec<Frame> with the audio generated
    fn generate(
        &self,
        length: usize,
        frequency: Frequency,
        sample_rate: u32,
        adsr: Adsr,
    ) -> Vec<Frame> {
        let vec: Vec<Frame> = (0..length)
            .map(|sample| {
                let time = TAU * (sample as f64 / sample_rate as f64);

                let result = (frequency * time).sin();
                Frame::mono(result)
            })
            .collect();
        vec.multiply(&adsr.generate(length).into_frames())
    }
}
