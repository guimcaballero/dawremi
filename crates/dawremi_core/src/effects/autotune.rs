use super::*;
use crate::notes::*;

use pvoc::{Bin, PhaseVocoder};

pub struct Autotune {
    pub sample_rate: f64,

    // TODO Change Note for Frequency
    pub notes: Vec<Option<Note>>,

    // TODO Change for Automation
    pub beat_length: usize,
}

impl Effect for Autotune {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let chunks = input.chunks(self.beat_length);
        let notes = pad_notes(self.notes.clone(), chunks.len());

        chunks
            .zip(notes)
            .map(|(chunk, note)| {
                // TODO Currently there's no pitch detection, it just scales it assuming it's a C4
                // TODO Implement pitch detection to check the actual pitch

                let freq: Frequency = if let Some(note) = note {
                    note.into()
                } else {
                    return chunk.to_vec();
                };

                let c4f: Frequency = Note::C4.into();
                let shift = freq.0 / c4f.0;

                let output_left = {
                    let mut pvoc = PhaseVocoder::new(1, self.sample_rate, 256, 4);
                    let mut output = vec![0.0; chunk.len()];

                    pvoc.process(
                        &[&chunk.left()],
                        &mut [&mut output],
                        |channels: usize,
                         bins: usize,
                         input: &[Vec<Bin>],
                         output: &mut [Vec<Bin>]| {
                            for i in 0..channels {
                                for j in 0..bins / 2 {
                                    let index = ((j as f64) * shift) as usize;
                                    if index < bins / 2 {
                                        output[i][index].freq = input[i][j].freq * shift;
                                        output[i][index].amp += input[i][j].amp;
                                    }
                                }
                            }
                        },
                    );

                    output
                };

                let output_right = {
                    let mut pvoc = PhaseVocoder::new(1, self.sample_rate, 256, 4);
                    let mut output = vec![0.0; chunk.len()];

                    pvoc.process(
                        &[&chunk.right()],
                        &mut [&mut output],
                        |channels: usize,
                         bins: usize,
                         input: &[Vec<Bin>],
                         output: &mut [Vec<Bin>]| {
                            for i in 0..channels {
                                for j in 0..bins / 2 {
                                    let index = ((j as f64) * shift) as usize;
                                    if index < bins / 2 {
                                        output[i][index].freq = input[i][j].freq * shift;
                                        output[i][index].amp += input[i][j].amp;
                                    }
                                }
                            }
                        },
                    );

                    output
                };

                join_left_and_right_channels(output_left, output_right)
            })
            .flatten()
            .collect::<Vec<Frame>>()
    }
}

fn pad_notes(mut notes: Vec<Option<Note>>, len: usize) -> Vec<Option<Note>> {
    while notes.len() < len {
        notes.push(None);
    }
    notes
}
