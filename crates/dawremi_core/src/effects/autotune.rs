use super::*;
use crate::notes::*;

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

                // TODO Get actual pitch for each side
                // TODO Find closest note
                // TODO Calculate the shift needed
                // TODO Pitch shift each section

                let freq: Frequency = if let Some(note) = note {
                    note.into()
                } else {
                    return chunk.to_vec();
                };

                let c4f: Frequency = Note::C4.into();
                let shift = freq.0 / c4f.0;

                chunk.to_vec().effect(&PitchShift {
                    sample_rate: self.sample_rate,
                    shift: Frame::mono(shift),
                })
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
