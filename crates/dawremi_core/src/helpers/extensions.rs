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

    /// Joins two lists by mixing the last n elements of self and the beginning n elements of other
    fn overlap(self, other: Vec<Frame>, n: usize) -> Vec<Frame>;

    /// Removes leading and trailing 0s
    fn trim(self) -> Vec<Frame>;

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

    fn overlap(self, other: Vec<Frame>, n: usize) -> Vec<Frame> {
        assert!(self.len() > n);
        assert!(other.len() > n);

        let len = self.len() + other.len() - n;
        let mut output = Vec::with_capacity(len + 1);

        let self_len = self.len();
        let self_len_minus_n = self.len() - n;

        for i in 0..len {
            if i < self_len_minus_n {
                output.push(self[i]);
            } else if i >= self_len {
                output.push(other[i - self_len_minus_n]);
            } else {
                let a = (i - self_len_minus_n) as f64 / n as f64;
                let val = self[i] * (1. - a) + other[i - self_len_minus_n] * a;
                output.push(val);
            }
        }

        output
    }

    fn trim(self) -> Vec<Frame> {
        fn is_not_0(x: &Frame) -> bool {
            x.left.abs() > 0.000001 && x.right.abs() > 0.000001
        }

        let vec = if let Some(first) = self.iter().position(is_not_0) {
            if let Some(last) = self.iter().rposition(is_not_0) {
                &self[first..last + 1]
            } else {
                unreachable!();
            }
        } else {
            &[]
        };
        vec.to_vec()
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
    fun: &mut dyn FnMut(Frequency, usize) -> Vec<Frame>,
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
        fun: &'a mut dyn FnMut(Frequency, usize) -> Vec<Frame>,
        length: usize,
    ) -> Vec<Frame>;
}
impl<'a, T: Clone + Into<Frequency>> IntoFrequencyList<'a> for Vec<Vec<T>> {
    fn generate(
        &self,
        fun: &'a mut dyn FnMut(Frequency, usize) -> Vec<Frame>,
        length: usize,
    ) -> Vec<Frame> {
        let freqs = self.into_freqs();
        generate_frequency_list(&freqs, fun, length)
    }
}
impl<'a, T: Clone + Into<Frequency>> IntoFrequencyList<'a> for Vec<Option<T>> {
    fn generate(
        &self,
        fun: &'a mut dyn FnMut(Frequency, usize) -> Vec<Frame>,
        length: usize,
    ) -> Vec<Frame> {
        let freqs = self.into_freqs();
        generate_frequency_list(&freqs, fun, length)
    }
}

pub trait IntoVecVecFreqList {
    fn into_freqs(&self) -> Vec<Vec<Frequency>>;
}
impl<T: Clone + Into<Frequency>> IntoVecVecFreqList for Vec<Vec<T>> {
    fn into_freqs(&self) -> Vec<Vec<Frequency>> {
        self.iter()
            .map(|a| a.iter().map(|b| b.clone().into()).collect())
            .collect()
    }
}
impl<T: Clone + Into<Frequency>> IntoVecVecFreqList for Vec<Option<T>> {
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

    #[test]
    fn overlap_two_vectors() {
        let vec = vec![1., 1., 0.].into_frames();
        let vec2 = vec![0.5, 1., 1.].into_frames();

        assert_eq!(
            vec![1., 1.0, 0.5, 1.].into_frames(),
            vec.clone().overlap(vec2.clone(), 2)
        );
        assert_eq!(vec![1., 1., 0., 1., 1.].into_frames(), vec.overlap(vec2, 1));
    }

    #[test]
    #[should_panic]
    fn overlap_vectors_with_n_over_len() {
        let vec = vec![1., 1., 0.].into_frames();
        let vec2 = vec![0.5, 1., 1.].into_frames();

        vec.overlap(vec2, 4);
    }
}
