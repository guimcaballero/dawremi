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
    A = 1,
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
    ($self:ident, len: $len:expr, fun: $fun:ident, $($x:tt)*) => {
        silence().take($self.beats(0.))
            $(
                .chain(sequence!(@map $self $fun $x).take($self.beats($len)))
            )*
    };
    ($self:ident, $($x:tt)*) => {
        sequence!($self, len: 1., fun: sine, $( $x )*)
    };
    (@map $self:ident $fun:ident _) => { silence() };
    (@map $self:ident $fun:ident $x:tt) => { $self.hz(Note::$x).$fun() };
}
