use crate::effects::Automation;
use crate::frame::*;
use crate::music_theory::notes::Note;
use crate::signals::adsr::*;

pub trait VecFrameExtension {
    // TODO This could be changed for as_mono(&self), or maybe we can even have both

    /// Joins both channels into one
    fn into_mono(self) -> Vec<f64>;

    /// Split the two channels into two vectors
    /// Returns first Left and then Right
    fn split(self) -> (Vec<f64>, Vec<f64>);
}

impl VecFrameExtension for Vec<Frame> {
    fn into_mono(self) -> Vec<f64> {
        self.iter().map(Frame::to_mono).collect()
    }

    fn split(self) -> (Vec<f64>, Vec<f64>) {
        let mut left = Vec::with_capacity(self.len());
        let mut right = Vec::with_capacity(self.len());

        for frame in self {
            left.push(frame.left);
            right.push(frame.right);
        }
        (left, right)
    }
}

pub trait VecExtension {
    type Item: Default;

    /// Makes a new Vec with `samples` number of samples. Fills with `Self::Item::default` if `samples > self.len()`
    fn take_samples(self, samples: usize) -> Vec<Self::Item>;

    /// Adds `samples` number of empty samples in front
    fn delay(self, samples: usize) -> Vec<Self::Item>;

    /// Repeats the list `times` times
    fn repeat(self, times: usize) -> Vec<Self::Item>;

    /// Joins two lists together
    fn chain(self, new: Vec<Self::Item>) -> Vec<Self::Item>;

    /// Joins two lists by mixing the last n elements of self and the beginning n elements of other
    fn overlap(self, other: Vec<Self::Item>, n: usize) -> Vec<Self::Item>;

    /// Removes leading and trailing 0s
    fn trim(self) -> Vec<Self::Item>;

    /// Repeats the list until `samples` number of samples are taken
    fn cycle_until_samples(self, samples: usize) -> Vec<Self::Item>;

    /// Adds the samples in each side
    fn add(self, other: &[Self::Item]) -> Vec<Self::Item>;

    /// Multiplies the samples in each side
    fn multiply(self, other: &[Self::Item]) -> Vec<Self::Item>;

    /// Mix both of the tracks
    ///
    /// If val is 0.0, only self will play
    /// If val is 1.0, only other will play
    /// It linearly mixes both
    fn mix(self, other: &[Self::Item], val: Automation<f64>) -> Vec<Self::Item>;

    /// Applies the Adsr envelope to the signal
    fn envelope(self, adsr: &Adsr) -> Vec<Self::Item>;

    /// Maps values from (-1., 1.) to (start, end)
    ///
    /// Used to easily map the values of a generated wave into another range, for example for frequencies
    fn map_to(self, start: Self::Item, end: Self::Item) -> Vec<Self::Item>;

    /// Maps values from `from` to `to`
    fn map_from_to(
        self,
        from: (Self::Item, Self::Item),
        to: (Self::Item, Self::Item),
    ) -> Vec<Self::Item>;
}

impl VecExtension for Vec<Frame> {
    type Item = Frame;

    fn take_samples(mut self, samples: usize) -> Vec<Self::Item> {
        self.resize(samples, Self::Item::default());
        self
    }

    fn delay(self, samples: usize) -> Vec<Self::Item> {
        vec![Self::Item::default(); samples].chain(self)
    }

    fn repeat(self, times: usize) -> Vec<Self::Item> {
        let len = self.len() * times;
        self.into_iter().cycle().take(len).collect()
    }

    fn chain(mut self, mut new: Vec<Self::Item>) -> Vec<Self::Item> {
        self.append(&mut new);
        self
    }

    fn overlap(self, other: Vec<Self::Item>, n: usize) -> Vec<Self::Item> {
        assert!(self.len() > n);
        assert!(other.len() > n);

        let len = self.len() + other.len() - n;
        let mut output = Self::with_capacity(len + 1);

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

    fn trim(self) -> Vec<Self::Item> {
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

    fn cycle_until_samples(self, samples: usize) -> Vec<Self::Item> {
        self.into_iter().cycle().take(samples).collect()
    }

    fn add(self, other: &[Self::Item]) -> Vec<Self::Item> {
        self.iter().zip(other.iter()).map(|(a, b)| a + b).collect()
    }

    fn multiply(self, other: &[Self::Item]) -> Vec<Self::Item> {
        self.iter().zip(other.iter()).map(|(a, b)| a * b).collect()
    }

    fn mix(self, other: &[Self::Item], val: Automation<f64>) -> Vec<Self::Item> {
        self.iter()
            .zip(other.iter())
            .enumerate()
            .map(|(idx, (a, b))| {
                let val = val.value(idx);
                a * (1. - val) + b * val
            })
            .collect()
    }

    fn envelope(self, adsr: &Adsr) -> Vec<Self::Item> {
        let length = self.len();
        self.multiply(&adsr.generate(length).as_frames())
    }

    fn map_to(self, start: Self::Item, end: Self::Item) -> Vec<Self::Item> {
        self.iter()
            .map(|val| map_from_to(*val, Frame::mono(-1.), Frame::mono(1.), start, end))
            .collect()
    }

    fn map_from_to(
        self,
        (from_start, from_end): (Self::Item, Self::Item),
        (to_start, to_end): (Self::Item, Self::Item),
    ) -> Vec<Self::Item> {
        self.iter()
            .map(|val| map_from_to(*val, from_start, from_end, to_start, to_end))
            .collect()
    }
}

fn map_from_to<T>(value: T, from_start: T, from_end: T, to_start: T, to_end: T) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Clone,
{
    to_start.clone()
        + (to_end - to_start) * ((value - from_start.clone()) / (from_end - from_start))
}

impl VecExtension for Vec<f64> {
    type Item = f64;

    fn take_samples(mut self, samples: usize) -> Vec<Self::Item> {
        self.resize(samples, Self::Item::default());
        self
    }

    fn delay(self, samples: usize) -> Vec<Self::Item> {
        vec![Self::Item::default(); samples].chain(self)
    }

    fn repeat(self, times: usize) -> Vec<Self::Item> {
        let len = self.len() * times;
        self.into_iter().cycle().take(len).collect()
    }

    fn chain(mut self, mut new: Vec<Self::Item>) -> Vec<Self::Item> {
        self.append(&mut new);
        self
    }

    fn overlap(self, other: Vec<Self::Item>, n: usize) -> Vec<Self::Item> {
        assert!(self.len() > n);
        assert!(other.len() > n);

        let len = self.len() + other.len() - n;
        let mut output = Self::with_capacity(len + 1);

        let self_len = self.len();
        let self_len_minus_n = self.len() - n;

        for i in 0..len {
            if i < self_len_minus_n {
                output.push(self[i]);
            } else if i >= self_len {
                output.push(other[i - self_len_minus_n]);
            } else {
                let a = (i - self_len_minus_n) as f64 / n as f64;
                let val = self[i].mul_add(1. - a, other[i - self_len_minus_n] * a);
                output.push(val);
            }
        }

        output
    }

    fn trim(self) -> Vec<Self::Item> {
        fn is_not_0(x: &f64) -> bool {
            x.abs() > 0.000001
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

    fn cycle_until_samples(self, samples: usize) -> Vec<Self::Item> {
        self.into_iter().cycle().take(samples).collect()
    }

    fn add(self, other: &[Self::Item]) -> Vec<Self::Item> {
        self.iter().zip(other.iter()).map(|(a, b)| a + b).collect()
    }

    fn multiply(self, other: &[Self::Item]) -> Vec<Self::Item> {
        self.iter().zip(other.iter()).map(|(a, b)| a * b).collect()
    }

    fn mix(self, other: &[Self::Item], val: Automation<f64>) -> Vec<Self::Item> {
        self.iter()
            .zip(other.iter())
            .enumerate()
            .map(|(idx, (a, b))| {
                let val = val.value(idx);
                a * (1. - val) + b * val
            })
            .collect()
    }

    fn envelope(self, adsr: &Adsr) -> Vec<Self::Item> {
        let length = self.len();
        self.multiply(&adsr.generate(length))
    }

    fn map_to(self, start: Self::Item, end: Self::Item) -> Vec<Self::Item> {
        self.iter()
            .map(|val| map_from_to(*val, -1., 1., start, end))
            .collect()
    }

    fn map_from_to(
        self,
        (from_start, from_end): (Self::Item, Self::Item),
        (to_start, to_end): (Self::Item, Self::Item),
    ) -> Vec<Self::Item> {
        self.iter()
            .map(|val| map_from_to(*val, from_start, from_end, to_start, to_end))
            .collect()
    }
}

pub trait NoteListExtension {
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self;
}
impl NoteListExtension for Vec<Option<Note>> {
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self {
        self.iter().map(|opt| opt.map(fun)).collect()
    }
}
impl NoteListExtension for Vec<Vec<Note>> {
    fn map_notes<U: Copy + Fn(Note) -> Note>(&self, fun: U) -> Self {
        self.iter()
            .map(|list| list.iter().map(|note| fun(*note)).collect())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn take_samples_returns_number_of_samples() {
        let vec = vec![1., 2., 3., 4., 5., 6.].as_frames();

        let expected = vec![1., 2., 3.].as_frames();

        assert_eq!(expected, vec.take_samples(3));
    }

    #[test]
    fn take_samples_pads_vec_with_0s() {
        let vec = vec![1., 2., 3.].as_frames();

        let expected = vec![1., 2., 3., 0., 0.].as_frames();

        assert_eq!(expected, vec.take_samples(5));
    }

    #[test]
    fn overlap_two_vectors() {
        let vec = vec![1., 1., 0.].as_frames();
        let vec2 = vec![0.5, 1., 1.].as_frames();

        assert_eq!(
            vec![1., 1.0, 0.5, 1.].as_frames(),
            vec.clone().overlap(vec2.clone(), 2)
        );
        assert_eq!(vec![1., 1., 0., 1., 1.].as_frames(), vec.overlap(vec2, 1));
    }

    #[test]
    #[should_panic]
    fn overlap_vectors_with_n_over_len() {
        let vec = vec![1., 1., 0.].as_frames();
        let vec2 = vec![0.5, 1., 1.].as_frames();

        vec.overlap(vec2, 4);
    }

    #[test]
    fn repeat_vec() {
        let vec = vec![1., 1., 0.].as_frames();
        let result = vec![1., 1., 0., 1., 1., 0., 1., 1., 0.].as_frames();

        assert_eq!(result, vec.repeat(3));
    }

    #[test]
    fn cycle_until_samples() {
        let vec = vec![1., 1., 0.].as_frames();
        let result = vec![1., 1., 0., 1., 1., 0., 1., 1.].as_frames();

        assert_eq!(result, vec.cycle_until_samples(8));
    }
}
