use crate::helpers::join_tracks;
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

pub trait NoteList<'a> {
    fn generate(&self, fun: &'a dyn Fn(Note, usize) -> Vec<f64>, length: usize) -> Vec<f64>;
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self;
}

impl<'a> NoteList<'a> for Vec<Option<Note>> {
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

impl<'a> NoteList<'a> for Vec<Vec<Note>> {
    fn generate(&self, fun: &'a dyn Fn(Note, usize) -> Vec<f64>, length: usize) -> Vec<f64> {
        let mut vec: Vec<f64> = Vec::new();
        for note_list in self {
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
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self {
        self.iter()
            .map(|list| list.iter().map(|note| fun(*note)).collect())
            .collect()
    }
}

pub trait IntoNoteList {
    fn into_notes(self) -> Vec<Vec<Note>>;
}
impl<T: Clone + Into<Note>> IntoNoteList for Vec<Vec<T>> {
    fn into_notes(self) -> Vec<Vec<Note>> {
        self.iter()
            .map(|a| a.iter().map(|b| b.clone().into()).collect())
            .collect()
    }
}
