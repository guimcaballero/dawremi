#![macro_use]
use dasp::signal;

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

pub trait RepeatExtension {
    fn repeat(self, times: usize) -> Vec<f64>;
}
impl RepeatExtension for Vec<f64> {
    fn repeat(self, times: usize) -> Vec<f64> {
        self.iter()
            .cloned()
            .cycle()
            .take(self.len() * times)
            .collect()
    }
}

#[allow(dead_code)]
pub enum Note {
    A = 0,
    As,
    B,
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
}

pub struct Frequency(pub f64);
impl From<Note> for Frequency {
    fn from(note: Note) -> Self {
        let n = note as u8;
        let a: f64 = 2.0_f64.powf(n as f64 / 12.);
        Self(440.0 * a)
    }
}
impl From<f64> for Frequency {
    fn from(freq: f64) -> Self {
        Self(freq)
    }
}

macro_rules! sequence {
    // This a test to be able to do something with vocaloids down the line
    (@lyrics $self:ident, $len_id:ident : $len:expr, $sign_id:ident : $sign:expr, $(enum: $enum:ident,)? $( $([ $($lyrics:tt)* ],)? ( $($x:tt)* ),)*) => {
        sequence!($self,
                  $len_id: $len, $sign_id: $sign, $(enum: $enum,)?
                  $($($x)*)*
        )
    };

    // With a signal
    ($self:ident, len: $len:expr, signal: $sign:expr, $($x:tt)*) => {
        {
            let mut vec = Vec::new();
            $(
                vec.append(
                    &mut sequence!(@map $self sign: $sign, $x)
                        .take($self.beats($len))
                        .collect::<Vec<f64>>()
                );
            )*
                vec
        }
    };
    // With a function that takes a note
    ($self:ident, len: $len:expr, fun: $fun:expr, enum: $enum:ident, $($x:tt)*) => {
        {
            let mut vec = Vec::new();
                $(
                    vec.append(
                        &mut sequence!(@map $self fun: $fun, enum: $enum, $x)
                            .take($self.beats($len))
                            .collect::<Vec<f64>>()
                    );
                )*
            vec
        }
    };
    // With default params
    ($self:ident, $($x:tt)*) => {
        sequence!($self, len: 1., fun: |note| $self.hz(note).square(), enum: Note, $( $x )*)
    };
    (@map $self:ident fun: $fun:expr,enum: $enum:ident, _) => { silence() };
    (@map $self:ident fun: $fun:expr,enum: $enum:ident, $x:tt) => { $fun($enum::$x) };
    (@map $self:ident sign: $sign:expr, _) => { silence() };
    (@map $self:ident sign: $sign:expr, $_x:tt) => { $sign };
}
