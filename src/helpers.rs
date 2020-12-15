#![macro_use]
use dasp::signal::{self, Signal};

pub fn silence() -> signal::Equilibrium<f64> {
    signal::equilibrium()
}

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

macro_rules! sequence {
    // This a test to be able to do something with vocaloids down the line
    (@lyrics $self:ident, $len_id:ident : $len:expr, $sign_id:ident : $sign:expr, $( $([ $($lyrics:tt)* ],)? ( $($x:tt)* ),)*) => {
        sequence!($self,
                  $len_id: $len, $sign_id: $sign,
                  $($($x)*)*
        )
    };

    // With a signal
    ($self:ident, len: $len:expr, signal: $sign:expr, $($x:tt)*) => {
        {
            let mut vec: Vec<f64> = Vec::new();
            $(
                vec.append(
                    &mut sequence!(@map $self sign: $sign, $x)
                        .take_samples($self.beats($len))
                );
            )*
                vec
        }
    };
    // With a function that takes a note
    ($self:ident, len: $len:expr, fun: $fun:expr, $($x:tt)*) => {
        {
            // TODO We might want to use a different set of notes somewhere else.
            // Make something to abstract this or smth
            use crate::notes::Note::*;

            let mut vec: Vec<f64> = Vec::new();
                $(
                    vec.append(
                        &mut sequence!(@map $self fun: $fun, $x)
                            .take_samples($self.beats($len * sequence!(@unwrap_len $x)))
                    );
                )*
            vec
        }
    };
    // With default params
    ($self:ident, $($x:tt)*) => {
        sequence!($self, len: 1., fun: |note| $self.hz(note).square(), $( $x * 1. )*)
    };

    // Helpers

    (@unwrap_note, ($x:tt * $len:expr)) => { $x };
    (@unwrap_note, $x:tt) => { $x };
    (@unwrap_len ($x:tt * $len:expr)) => { $len };
    (@unwrap_len $x:tt) => { 1. };

    (@map $self:ident fun: $fun:expr, _) => { silence() };
    (@map $self:ident fun: $fun:expr, __) => { silence() };
    (@map $self:ident fun: $fun:expr, $x:tt) => { $fun(sequence!(@unwrap_note, $x)) };
    (@map $self:ident sign: $sign:expr, _) => { silence() };
    (@map $self:ident sign: $sign:expr, __) => { silence() };
    (@map $self:ident sign: $sign:expr, $_x:tt) => { $sign };
}

pub fn join_tracks(tracks: Vec<Vec<f64>>) -> Vec<f64> {
    let len = &tracks
        .iter()
        .map(|track| track.len())
        .max()
        .expect("There should be at least one track to join");

    (0..*len)
        .map(|i| {
            let mut val = 0.;
            let mut count = 0;
            for track in &tracks {
                if let Some(value) = track.get(i) {
                    val += value;
                    count += 1;
                }
            }
            val / count as f64
        })
        .collect()
}

macro_rules! pattern {
    // With a function that takes a note
    ($self:ident, repetitions: $rep:expr, $( beat: $beat:expr, fun: $fun:expr, pat: ( $($x:tt)* ), )* ) => {
        {
            join_tracks(
                vec![
                    $(
                        {
                            // TODO We might want to use a different set of notes somewhere else.
                            // Make something to abstract this or smth
                            use crate::notes::Note::*;

                            let mut vec: Vec<f64> = Vec::new();
                            $(
                                vec.append(
                                    &mut sequence!(@map $self fun: $fun, $x)
                                        .take_samples($self.beats($beat * sequence!(@unwrap_len $x)))
                                );
                            )*
                            vec
                        },
                    )*
                ]
            )
                .repeat($rep)
         }
    };
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn join_tracks_test() {
        let tracks = vec![vec![1., 1., 0., 0.5, 0.3], vec![0., 1., 0., 0.5, 0.5]];

        assert_eq!(vec![0.5, 1., 0., 0.5, 0.4], join_tracks(tracks))
    }
}
