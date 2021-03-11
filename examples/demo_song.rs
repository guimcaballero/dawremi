#[macro_use]
extern crate dawremi;

use dawremi::prelude::*;

fn main() {
    let mut song = DemoSong::default();
    song.play().expect("Unable to play song");
}

song!(DemoSong,);

impl Song for DemoSong {
    /// Display name for the song
    fn name(&self) -> &'static str {
        "Demo song"
    }

    /// Song's beats per minute
    fn bpm(&self) -> usize {
        120
    }

    /// Song's duration (in samples)
    fn duration(&self) -> usize {
        // We're using the beats helper function that converts 16 beats to the number of samples to take
        self.beats(16.)
    }

    /// List of tracks on this song. Each track is just a list of samples (Vec<Frame>)
    /// All of the tracks will be mixed equally
    fn tracks(&mut self) -> Vec<Vec<Frame>> {
        vec![self.plucked_track(), self.other_track()]
    }
}

impl DemoSong {
    /// This is a track
    fn plucked_track(&self) -> Vec<Frame> {
        // We ue a macro to play a sequence of notes. Returns a Vec<Frame>
        sequence!(
            self,
            // The lenght of one note in beats
            len: 1.,
            // The note representation we want to use. GuitarFretboard simulates a guitar tab,
            // using the first letter as the string, and the number as the finger position
            // L is the low e string
            note: GuitarFretboard,
            // The function we want to use to generate the sound
            fun: |note| self.plucked(note, InitialBurstType::Triangle(2, 3)),

            // List of notes. Underscores signal silences
            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        )
        // We can chain another sequence
        .chain(sequence!(
            self,
            len: 1., note: GuitarFretboard,
            fun: |note| self.plucked(note, InitialBurstType::DoubleTriangle),

            L5 L5 _ L8 L8 _ L1 L1 _ L4 L4
        ))
    }

    /// This is a helper function
    fn plucked(&self, frequency: impl Into<Frequency>, burst: InitialBurstType) -> Synth {
        Synth::new(
            Box::new(Plucked::new(
                burst,
                frequency.into(),
                self.get_sample_rate(),
            )),
            self.get_sample_rate(),
        )
    }

    fn other_track(&mut self) -> Vec<Frame> {
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
            &|note, length| {
                self.plucked(note, InitialBurstType::Triangle(2, 3))
                    .take_samples(length)
            },
            // Length of each note
            self.beats(1.),
        );

        // Generate sounds one octave higher
        let sound2 = notes1
            // We can map over the notes
            .map_notes(Note::up_an_octave)
            .generate(
                &|note, length| {
                    self.plucked(note, InitialBurstType::Triangle(2, 3))
                        .take_samples(length)
                },
                self.beats(1.),
            );

        // We can use other types of notes too
        let bass = {
            use GuitarFretboard::*;
            note_list![L5, L5, _, L8, L8, _, L1, L1, _, L4, L4,]
        }
        .generate(
            &|note, length| {
                self.plucked(note, InitialBurstType::Sine)
                    .take_samples(length)
            },
            self.beats(1.),
        );

        // We then joing the subtracks into one
        join_tracks(vec![sound1, sound2, bass])
            // We can also add effects to the whole track, like Reverb (using convolution)
            .effect(&Convolution::new(
                self.sound(Reverb::LargeLongEchoHall.into()),
            ))
            // Or volume
            .effect(&Volume {
                mult: Automation::Const(0.5),
            })
    }
}
