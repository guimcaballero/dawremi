#![macro_use]
use dasp::signal::{self, Signal};

/// Adds the expressions to a vector if they're not None
macro_rules! some_vec {
    ($($x:expr),* $(,)?) => (
        {
            let mut temp = Vec::new();
            $(
                if let Some(val) = $x {
                    temp.push(val);
                }
            )*
                temp
        }
    );
}

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

/// Joins multiple functions that return Option<Vec<f64>> into a single averaged signal
macro_rules! join_tracks {
    (duration: $duration:expr, $($x:expr),* $(,)?) => {
        {
            let duration = $duration;
            let mut tracks = some_vec![
                $( $x, )*
            ];
            let number_of_tracks = tracks.len();

            // Join all of the tracks into one
            let track = silence()
            // Add the track or an empty Signal
            $(
                .add_amp(signal::from_iter(
                    tracks
                        .pop()
                        .unwrap_or_else(|| {
                            // To loop we need to use $x, so we just ignore it
                            join_tracks!(@ignore $x);
                            silence().take_samples(duration)
                        })
                ))
            )*
            ;

             track
                .map(move |s| s / (number_of_tracks as f64))
        }
    };
    (@ignore $x:expr) => {()}
}

macro_rules! pattern {
    // With a function that takes a note
    ($self:ident, repetitions: $rep:expr, $( beat: $beat:expr, fun: $fun:expr, pat: ( $($x:tt)* ), )* ) => {
        {
            let mut take_duration = 0.;

            join_tracks![
                duration: $self.duration(),
                $(
                    {
                        // TODO We might want to use a different set of notes somewhere else.
                        // Make something to abstract this or smth
                        use crate::notes::Note::*;

                        // Basically a thing to get unused_assignments to shut up
                        let _ = format!("{}", take_duration);

                        take_duration = 0.;
                        let mut vec: Vec<f64> = Vec::new();
                        $(
                            take_duration += $beat * sequence!(@unwrap_len $x);
                            vec.append(
                                &mut sequence!(@map $self fun: $fun, $x)
                                    .take_samples($self.beats($beat * sequence!(@unwrap_len $x)))
                            );
                        )*
                        Some(vec)
                    },
                )*
            ]
                .take_samples($self.beats(take_duration))
                .repeat($rep)
         }
    };
}
