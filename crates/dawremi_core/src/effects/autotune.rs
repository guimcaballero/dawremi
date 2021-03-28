use super::*;
use crate::helpers::pitch_detection::detect;
use crate::notes::*;

pub enum AutotuneConfig {
    /// Will snap to closest note
    Snap,
    /// Will shift each chunk to the provided frequency
    /// A None will behave like Snap
    Frequencies(Vec<Option<Frequency>>),
}

pub struct Autotune {
    pub sample_rate: f64,
    // TODO This can be changed for a list of numbers, to split in different sizes
    pub beat_length: usize,
    pub config: AutotuneConfig,
}

impl Effect for Autotune {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        let chunks = input.chunks(self.beat_length);

        let frequencies = match &self.config {
            AutotuneConfig::Snap => vec![None; chunks.len()],
            AutotuneConfig::Frequencies(frequencies) => {
                pad_notes(frequencies.clone(), chunks.len())
            }
        };

        chunks
            .zip(frequencies)
            .map(|(chunk, freq)| {
                let shift = Frame::new(
                    {
                        // Get actual pitch
                        let current_pitch = detect(chunk.left(), self.sample_rate);
                        // Find pitch to change to
                        // If a frequency was provided, change to that
                        // Else, snap to closest note
                        let change_to: Frequency = if let Some(change_to) = freq {
                            change_to
                        } else {
                            Note::closest_to_frequency(current_pitch).into()
                        };

                        // Return the shifting needed to bring current_pitch to change_to
                        change_to.0 / current_pitch
                    },
                    {
                        let current_pitch = detect(chunk.right(), self.sample_rate);
                        let change_to: Frequency = if let Some(change_to) = freq {
                            change_to
                        } else {
                            Note::closest_to_frequency(current_pitch).into()
                        };
                        change_to.0 / current_pitch
                    },
                );

                chunk.to_vec().effect(&PitchShift {
                    sample_rate: self.sample_rate,
                    shift,
                })
            })
            .flatten()
            .collect::<Vec<Frame>>()
    }
}

fn pad_notes<T>(mut notes: Vec<Option<T>>, len: usize) -> Vec<Option<T>> {
    while notes.len() < len {
        notes.push(None);
    }
    notes
}
