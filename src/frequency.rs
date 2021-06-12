use crate::frame::*;
use crate::helpers::*;
use crate::notes::*;
use crate::song::Song;

pub type Frequency = f64;

pub enum Length {
    Samples(usize),
    Beats(f64),
    Seconds(f64),
}
impl Length {
    /// Returns the length in samples
    fn get(&self, song: &Song) -> usize {
        match *self {
            Length::Samples(length) => length,
            Length::Beats(length) => song.beats(length),
            Length::Seconds(length) => song.seconds(length),
        }
    }
}

/// Keeps a list of frequencies and the length, so it can be computed into a chunk of audio
/// Naming inspired by the Elektron Digitone
pub struct Trigger {
    freqs: Vec<Frequency>,
    length: Length,
}

impl Trigger {
    fn is_empty(&self) -> bool {
        self.freqs.is_empty()
    }

    /// Returns the length in samples
    fn length(&self, song: &Song) -> usize {
        self.length.get(song)
    }
}

impl From<Frequency> for Trigger {
    fn from(freq: Frequency) -> Self {
        Self {
            freqs: vec![freq],
            length: Length::Beats(1.),
        }
    }
}

pub trait IntoTrigger {
    fn beats(self, length: f64) -> Trigger;
    fn seconds(self, length: f64) -> Trigger;
    fn samples(self, length: usize) -> Trigger;
}
impl IntoTrigger for Vec<Frequency> {
    fn beats(self, length: f64) -> Trigger {
        Trigger {
            freqs: self,
            length: Length::Beats(length),
        }
    }

    fn seconds(self, length: f64) -> Trigger {
        Trigger {
            freqs: self,
            length: Length::Seconds(length),
        }
    }

    fn samples(self, length: usize) -> Trigger {
        Trigger {
            freqs: self,
            length: Length::Samples(length),
        }
    }
}
impl<const N: usize> IntoTrigger for [Frequency; N] {
    fn beats(self, length: f64) -> Trigger {
        Trigger {
            freqs: self.into(),
            length: Length::Beats(length),
        }
    }

    fn seconds(self, length: f64) -> Trigger {
        Trigger {
            freqs: self.into(),
            length: Length::Seconds(length),
        }
    }

    fn samples(self, length: usize) -> Trigger {
        Trigger {
            freqs: self.into(),
            length: Length::Samples(length),
        }
    }
}
impl IntoTrigger for Frequency {
    fn beats(self, length: f64) -> Trigger {
        Trigger {
            freqs: vec![self],
            length: Length::Beats(length),
        }
    }

    fn seconds(self, length: f64) -> Trigger {
        Trigger {
            freqs: vec![self],
            length: Length::Seconds(length),
        }
    }

    fn samples(self, length: usize) -> Trigger {
        Trigger {
            freqs: vec![self],
            length: Length::Samples(length),
        }
    }
}
impl<N: Into<Note>> IntoTrigger for N {
    fn beats(self, length: f64) -> Trigger {
        let n: Note = self.into();
        let n: Frequency = n.into();
        n.beats(length)
    }

    fn seconds(self, length: f64) -> Trigger {
        let n: Note = self.into();
        let n: Frequency = n.into();
        n.seconds(length)
    }

    fn samples(self, length: usize) -> Trigger {
        let n: Note = self.into();
        let n: Frequency = n.into();
        n.samples(length)
    }
}
impl<NOTE: Into<Note> + Clone, const N: usize> IntoTrigger for [NOTE; N] {
    fn beats(self, length: f64) -> Trigger {
        let mut array = [0.; N];
        for idx in 0..N {
            let n: Note = self[idx].clone().into();
            let n: Frequency = n.into();
            array[idx] = n;
        }
        array.beats(length)
    }

    fn seconds(self, length: f64) -> Trigger {
        let mut array = [0.; N];
        for idx in 0..N {
            let n: Note = self[idx].clone().into();
            let n: Frequency = n.into();
            array[idx] = n;
        }
        array.seconds(length)
    }

    fn samples(self, length: usize) -> Trigger {
        let mut array = [0.; N];
        for idx in 0..N {
            let n: Note = self[idx].clone().into();
            let n: Frequency = n.into();
            array[idx] = n;
        }
        array.samples(length)
    }
}
impl<NOTE: Into<Note>> IntoTrigger for Vec<NOTE> {
    fn beats(self, length: f64) -> Trigger {
        let mut array = Vec::<Frequency>::with_capacity(self.len());
        for note in self {
            let n: Note = note.into();
            let n: Frequency = n.into();
            array.push(n)
        }
        array.beats(length)
    }

    fn seconds(self, length: f64) -> Trigger {
        let mut array = Vec::<Frequency>::with_capacity(self.len());
        for note in self {
            let n: Note = note.into();
            let n: Frequency = n.into();
            array.push(n)
        }
        array.seconds(length)
    }

    fn samples(self, length: usize) -> Trigger {
        let mut array = Vec::<Frequency>::with_capacity(self.len());
        for note in self {
            let n: Note = note.into();
            let n: Frequency = n.into();
            array.push(n)
        }
        array.samples(length)
    }
}
impl<const M: u8> IntoTrigger for n_tet::NTet<M> {
    fn beats(self, length: f64) -> Trigger {
        let freq: Frequency = self.into();
        freq.beats(length)
    }

    fn seconds(self, length: f64) -> Trigger {
        let freq: Frequency = self.into();
        freq.seconds(length)
    }

    fn samples(self, length: usize) -> Trigger {
        let freq: Frequency = self.into();
        freq.samples(length)
    }
}
impl<const N: usize, const M: u8> IntoTrigger for [n_tet::NTet<M>; N] {
    fn beats(self, length: f64) -> Trigger {
        let mut array = [0.; N];
        for idx in 0..N {
            let n: Frequency = self[idx].clone().into();
            array[idx] = n;
        }
        array.beats(length)
    }

    fn seconds(self, length: f64) -> Trigger {
        let mut array = [0.; N];
        for idx in 0..N {
            let n: Frequency = self[idx].clone().into();
            array[idx] = n;
        }
        array.seconds(length)
    }

    fn samples(self, length: usize) -> Trigger {
        let mut array = [0.; N];
        for idx in 0..N {
            let n: Frequency = self[idx].clone().into();
            array[idx] = n;
        }
        array.samples(length)
    }
}
impl<const M: u8> IntoTrigger for Vec<n_tet::NTet<M>> {
    fn beats(self, length: f64) -> Trigger {
        let mut array = Vec::<Frequency>::with_capacity(self.len());
        for note in self {
            let n: Frequency = note.into();
            array.push(n)
        }
        array.beats(length)
    }

    fn seconds(self, length: f64) -> Trigger {
        let mut array = Vec::<Frequency>::with_capacity(self.len());
        for note in self {
            let n: Frequency = note.into();
            array.push(n)
        }
        array.seconds(length)
    }

    fn samples(self, length: usize) -> Trigger {
        let mut array = Vec::<Frequency>::with_capacity(self.len());
        for note in self {
            let n: Frequency = note.into();
            array.push(n)
        }
        array.samples(length)
    }
}

/// Makes a FrequencyLength with no frequencies
pub struct Silence;
impl IntoTrigger for Silence {
    fn beats(self, length: f64) -> Trigger {
        Trigger {
            freqs: vec![],
            length: Length::Beats(length),
        }
    }

    fn seconds(self, length: f64) -> Trigger {
        Trigger {
            freqs: vec![],
            length: Length::Seconds(length),
        }
    }

    fn samples(self, length: usize) -> Trigger {
        Trigger {
            freqs: vec![],
            length: Length::Samples(length),
        }
    }
}

pub trait TriggerListExtension<'a> {
    fn generate(
        &self,
        song: &Song,
        fun: &'a mut dyn FnMut(Frequency, usize) -> Vec<Frame>,
    ) -> Vec<Frame>;
    fn map_frequencies<F>(self, fun: F) -> Self
    where
        F: Clone + FnMut(&Frequency) -> Frequency;
}
impl<'a, const N: usize> TriggerListExtension<'a> for [Trigger; N] {
    fn generate(
        &self,
        song: &Song,
        fun: &'a mut dyn FnMut(Frequency, usize) -> Vec<Frame>,
    ) -> Vec<Frame> {
        let mut vec: Vec<Frame> = Vec::new();
        for freq_length in self {
            let length = freq_length.length(song);

            if freq_length.is_empty() {
                silence().take_samples(length);
            } else {
                vec.append(&mut join_tracks(
                    freq_length
                        .freqs
                        .iter()
                        .map(|note| fun(*note, length))
                        .collect(),
                ));
            }
        }
        vec
    }

    fn map_frequencies<F>(mut self, fun: F) -> Self
    where
        F: Clone + FnMut(&Frequency) -> Frequency,
    {
        for freq_length in &mut self {
            freq_length.freqs = freq_length.freqs.iter().map(fun.clone()).collect();
        }
        self
    }
}
impl<'a> TriggerListExtension<'a> for Vec<Trigger> {
    fn generate(
        &self,
        song: &Song,
        fun: &'a mut dyn FnMut(Frequency, usize) -> Vec<Frame>,
    ) -> Vec<Frame> {
        let mut vec: Vec<Frame> = Vec::new();
        for freq_length in self {
            let length = freq_length.length(song);

            if freq_length.is_empty() {
                silence().take_samples(length);
            } else {
                vec.append(&mut join_tracks(
                    freq_length
                        .freqs
                        .iter()
                        .map(|note| fun(*note, length))
                        .collect(),
                ));
            }
        }
        vec
    }

    fn map_frequencies<F>(mut self, fun: F) -> Self
    where
        F: Clone + FnMut(&Frequency) -> Frequency,
    {
        for freq_length in &mut self {
            freq_length.freqs = freq_length.freqs.iter().map(fun.clone()).collect();
        }
        self
    }
}
