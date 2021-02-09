use crate::helpers::join_tracks;
use crate::helpers::silence;
use crate::notes::{Frequency, Note};
use dasp::signal::Signal;

pub trait TakeSamplesExtension {
    fn take_samples(self, samples: usize) -> Vec<f64>;
}
impl<T: Signal<Frame = f64>> TakeSamplesExtension for T {
    fn take_samples(self, samples: usize) -> Vec<f64> {
        self.take(samples).collect()
    }
}

pub trait RepeatExtension {
    fn repeat(self, times: usize) -> Vec<f64>;
    fn collect(self) -> Self;

    // take_samples is here again so cause there's a conflicting implementation if
    // we try to impl TakeSamplesExtension for Vec<f64>
    fn take_samples(self, samples: usize) -> Vec<f64>;
    fn chain(self, new: Vec<f64>) -> Vec<f64>;
}

impl RepeatExtension for Vec<f64> {
    fn collect(self) -> Self {
        self
    }

    fn take_samples(mut self, samples: usize) -> Vec<f64> {
        self.resize(samples, 0.);
        self
    }

    fn repeat(self, times: usize) -> Vec<f64> {
        self.iter()
            .cloned()
            .cycle()
            .take(self.len() * times)
            .collect()
    }

    fn chain(mut self, mut new: Vec<f64>) -> Vec<f64> {
        self.append(&mut new);
        self
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
    fun: &dyn Fn(Frequency, usize) -> Vec<f64>,
    length: usize,
) -> Vec<f64> {
    let mut vec: Vec<f64> = Vec::new();
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
    fn generate(&self, fun: &'a dyn Fn(Frequency, usize) -> Vec<f64>, length: usize) -> Vec<f64>;
}
impl<'a, T: Clone + Into<Frequency>> IntoFrequencyList<'a> for Vec<Vec<T>> {
    fn generate(&self, fun: &'a dyn Fn(Frequency, usize) -> Vec<f64>, length: usize) -> Vec<f64> {
        let freqs = self.into_freqs();
        generate_frequency_list(&freqs, fun, length)
    }
}
impl<'a, T: Clone + Into<Frequency>> IntoFrequencyList<'a> for Vec<Option<T>> {
    fn generate(&self, fun: &'a dyn Fn(Frequency, usize) -> Vec<f64>, length: usize) -> Vec<f64> {
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
        let vec = vec![1., 2., 3., 4., 5., 6.];

        let expected = vec![1., 2., 3.];

        assert_eq!(expected, vec.take_samples(3));
    }

    #[test]
    fn take_samples_pads_vec_with_0s() {
        let vec = vec![1., 2., 3.];

        let expected = vec![1., 2., 3., 0., 0.];

        assert_eq!(expected, vec.take_samples(5));
    }
}
