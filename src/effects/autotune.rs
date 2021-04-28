use super::*;
use crate::helpers::pitch_detection::detect;
use crate::notes::*;

pub enum PitchCorrectionMode {
    /// Will snap to closest note
    Snap,
    /// Will shift each chunk to the provided frequency
    /// A None will behave like Snap
    Frequencies(Vec<Option<Frequency>>),
}

pub struct PitchCorrection {
    pub sample_rate: u32,
    pub beat_length: usize,
    pub mode: PitchCorrectionMode,
}

impl Effect for PitchCorrection {
    fn run(&self, input: Vec<Frame>) -> Vec<Frame> {
        // We chunk the input up
        let chunks = input.chunks(self.beat_length).collect::<Vec<&[Frame]>>();

        // We get the list of frequencies to change to
        // None means that it will snap to closest
        let frequencies = match &self.mode {
            PitchCorrectionMode::Snap => vec![None; chunks.len()],
            PitchCorrectionMode::Frequencies(frequencies) => {
                pad_notes(frequencies.clone(), chunks.len())
            }
        };

        let parts = chunks
            // We iterate through the chunks in windows, this way we can later overlap and add the audios
            // and get something that sounds continuous
            .windows(2)
            .zip(frequencies)
            .map(|(two_chunks, freq)| {
                // Join the two chunks into a big one
                let chunk = two_chunks.concat();

                // Get by how much we have to shift each channel
                let shift = Frame::new(
                    {
                        // Get actual pitch
                        let current_pitch = detect(chunk.left(), self.sample_rate).unwrap_or(440.);
                        // Find pitch to change to
                        // If a frequency was provided, change to that
                        // Else, snap to closest note
                        let change_to: Frequency = if let Some(change_to) = freq {
                            change_to
                        } else {
                            Note::closest_to_frequency(current_pitch).into()
                        };

                        // Return the shifting needed to bring current_pitch to change_to
                        change_to / current_pitch
                    },
                    {
                        let current_pitch = detect(chunk.right(), self.sample_rate).unwrap_or(440.);
                        let change_to: Frequency = if let Some(change_to) = freq {
                            change_to
                        } else {
                            Note::closest_to_frequency(current_pitch).into()
                        };
                        change_to / current_pitch
                    },
                );

                // Apply pitch shift to the chunk
                chunk.to_vec().effect(&PitchShift {
                    sample_rate: self.sample_rate,
                    shift,
                })
            })
            .collect::<Vec<Vec<Frame>>>();

        // We then overlap with the length of one beat
        overlap_add(parts, self.beat_length)
    }
}

fn pad_notes<T>(mut notes: Vec<Option<T>>, len: usize) -> Vec<Option<T>> {
    while notes.len() < len {
        notes.push(None);
    }
    notes
}
