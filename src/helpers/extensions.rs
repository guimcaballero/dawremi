use crate::helpers::silence;
use crate::notes::Note;
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
    fn take_samples(self, samples: usize) -> Vec<f64>;
    fn chain(self, new: &mut Vec<f64>) -> Vec<f64>;
}

impl RepeatExtension for Vec<f64> {
    fn collect(self) -> Self {
        self
    }

    fn take_samples(self, samples: usize) -> Vec<f64> {
        self.iter().cloned().take(samples).collect()
    }

    fn repeat(self, times: usize) -> Vec<f64> {
        self.iter()
            .cloned()
            .cycle()
            .take(self.len() * times)
            .collect()
    }

    fn chain(mut self, new: &mut Vec<f64>) -> Vec<f64> {
        self.append(new);
        self
    }
}

pub trait VecOptionNote<'a> {
    fn generate(&self, fun: &'a dyn Fn(Note, usize) -> Vec<f64>, length: usize) -> Vec<f64>;
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self;
}

impl<'a> VecOptionNote<'a> for Vec<Option<Note>> {
    fn generate(&self, fun: &'a dyn Fn(Note, usize) -> Vec<f64>, length: usize) -> Vec<f64> {
        let mut vec: Vec<f64> = Vec::new();
        for opt_note in self {
            if let Some(note) = opt_note {
                vec.append(&mut fun(*note, length));
            } else {
                silence().take_samples(length);
            }
        }
        vec
    }
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self {
        self.iter().map(|opt| opt.map(fun)).collect()
    }
}

pub trait IntoVecOptionNote {
    fn into_notes(self) -> Vec<Option<Note>>;
}
impl<T: Clone + Into<Note>> IntoVecOptionNote for Vec<Option<T>> {
    fn into_notes(self) -> Vec<Option<Note>> {
        self.iter()
            .map(|a| a.as_ref().map(|b| b.clone().into()))
            .collect()
    }
}
