use crate::frame::*;
use crate::helpers::join_tracks;
use crate::helpers::silence;
use crate::notes::{Frequency, Note};

pub trait TakeSamplesExtension {
    fn take_samples(self, samples: usize) -> Vec<f64>;
}
impl TakeSamplesExtension for Vec<f64> {
    fn take_samples(mut self, samples: usize) -> Vec<f64> {
        self.resize(samples, 0.);
        self
    }
}

pub trait VecFrameExtension {
    // TODO Can we remove this? Here for legacy reasons I think
    fn collect(self) -> Self;

    // take_samples is here again so cause there's a conflicting implementation if
    // we try to impl TakeSamplesExtension for Vec<f64>

    /// Makes a new Vec with `samples` number of samples. Fills with `Frame::default` if `samples > self.len()`
    fn take_samples(self, samples: usize) -> Vec<Frame>;

    /// Adds `samples` number of empty samples in front
    fn delay(self, samples: usize) -> Vec<Frame>;

    /// Repeats the list `times` times
    fn repeat(self, times: usize) -> Vec<Frame>;

    /// Joins two lists together
    fn chain(self, new: Vec<Frame>) -> Vec<Frame>;

    /// Repeats the list until `samples` number of samples are taken
    fn cycle_until_samples(self, samples: usize) -> Vec<Frame>;

    // Adds the samples in each side
    fn add(self, other: Vec<Frame>) -> Vec<Frame>;

    // Multiplies the samples in each side
    fn multiply(self, other: Vec<Frame>) -> Vec<Frame>;

    // Multiplies the samples in each side
    fn to_mono(self) -> Vec<f64>;
}

impl VecFrameExtension for Vec<Frame> {
    fn collect(self) -> Self {
        self
    }

    fn take_samples(mut self, samples: usize) -> Vec<Frame> {
        self.resize(samples, Frame::default());
        self
    }

    fn delay(self, samples: usize) -> Vec<Frame> {
        vec![Frame::default(); samples].chain(self)
    }

    fn repeat(self, times: usize) -> Vec<Frame> {
        self.iter()
            .cloned()
            .cycle()
            .take(self.len() * times)
            .collect()
    }

    fn chain(mut self, mut new: Vec<Frame>) -> Vec<Frame> {
        self.append(&mut new);
        self
    }

    fn cycle_until_samples(self, samples: usize) -> Vec<Frame> {
        self.iter().cloned().cycle().take(samples).collect()
    }

    fn add(self, other: Vec<Frame>) -> Vec<Frame> {
        self.iter().zip(other.iter()).map(|(a, b)| a + b).collect()
    }

    fn multiply(self, other: Vec<Frame>) -> Vec<Frame> {
        self.iter().zip(other.iter()).map(|(a, b)| a * b).collect()
    }

    fn to_mono(self) -> Vec<f64> {
        self.iter().map(Frame::to_mono).collect()
    }
}

pub trait NoteList<'a> {
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self;
}
impl<'a> NoteList<'a> for Vec<Option<Note>> {
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self {
        self.iter().map(|opt| opt.map(fun)).collect()
    }
}
impl<'a> NoteList<'a> for Vec<Vec<Note>> {
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self {
        self.iter()
            .map(|list| list.iter().map(|note| fun(*note)).collect())
            .collect()
    }
}

fn generate_frequency_list(
    list: &[Vec<Frequency>],
    fun: &dyn Fn(Frequency, usize) -> Vec<Frame>,
    length: usize,
) -> Vec<Frame> {
    let mut vec: Vec<Frame> = Vec::new();
    for note_list in list {
        if note_list.is_empty() {
            silence().take_samples(length);
        } else {
            vec.append(&mut join_tracks(
                note_list.iter().map(|note| fun(*note, length)).collect(),
            ));
        }
    }
    vec
}

pub trait IntoFrequencyList<'a> {
    fn generate(
        &self,
        fun: &'a dyn Fn(Frequency, usize) -> Vec<Frame>,
        length: usize,
    ) -> Vec<Frame>;
}
impl<'a, T: Clone + Into<Frequency>> IntoFrequencyList<'a> for Vec<Vec<T>> {
    fn generate(
        &self,
        fun: &'a dyn Fn(Frequency, usize) -> Vec<Frame>,
        length: usize,
    ) -> Vec<Frame> {
        let freqs = self.into_freqs();
        generate_frequency_list(&freqs, fun, length)
    }
}
impl<'a, T: Clone + Into<Frequency>> IntoFrequencyList<'a> for Vec<Option<T>> {
    fn generate(
        &self,
        fun: &'a dyn Fn(Frequency, usize) -> Vec<Frame>,
        length: usize,
    ) -> Vec<Frame> {
        let freqs = self.into_freqs();
        generate_frequency_list(&freqs, fun, length)
    }
}

pub trait IntoFreqList {
    fn into_freqs(&self) -> Vec<Vec<Frequency>>;
}
impl<T: Clone + Into<Frequency>> IntoFreqList for Vec<Vec<T>> {
    fn into_freqs(&self) -> Vec<Vec<Frequency>> {
        self.iter()
            .map(|a| a.iter().map(|b| b.clone().into()).collect())
            .collect()
    }
}
impl<T: Clone + Into<Frequency>> IntoFreqList for Vec<Option<T>> {
    fn into_freqs(&self) -> Vec<Vec<Frequency>> {
        self.iter()
            .map(|a| {
                if let Some(val) = a {
                    vec![val.clone().into()]
                } else {
                    vec![]
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn take_samples_returns_number_of_samples() {
        let vec = vec![1., 2., 3., 4., 5., 6.].into_frames();

        let expected = vec![1., 2., 3.].into_frames();

        assert_eq!(expected, vec.take_samples(3));
    }

    #[test]
    fn take_samples_pads_vec_with_0s() {
        let vec = vec![1., 2., 3.].into_frames();

        let expected = vec![1., 2., 3., 0., 0.].into_frames();

        assert_eq!(expected, vec.take_samples(5));
    }
}
