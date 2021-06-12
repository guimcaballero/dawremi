macro_rules! intervals {
    (
        $( $var:ident, $func:ident, $semi:expr ; )*
    ) => {
        #[derive(Copy, Clone, Debug)]
        pub enum Interval {
            $( $var, )*
        }

        impl Interval {
            pub const fn semitones(&self) -> u8 {
                match self {
                    $( Self::$var => $semi, )*
                }
            }
        }

        pub mod intervals_short {
            #![allow(non_upper_case_globals)]

            use super::*;

            $(
                pub const $func: Interval = Interval::$var;
            )*
        }
    };
}

// From https://en.wikipedia.org/wiki/Interval_(music)

intervals! {
    PerfectUnison, P1, 0;
    DiminishedSecond, d2, 0;
    MinorSecond, m2, 1;
    AugmentedUnison, A1, 1;
    MajorSecond, M2, 2;
    DiminishedThird, d3, 2;
    MinorThird, m3, 3;
    AugmentedSecond, A2, 3;
    MajorThird, M3, 4;
    DimishedFourth, d4, 4;
    PerfectFourth, P4, 5;
    AugmentedThird, A3, 5;
    DimishedFifth, d5, 6;
    AugmentedFourth, A4, 6;
    PerfectFifth, P5, 7;
    DimishedSixth, d6, 7;
    MinorSixth, m6, 8;
    AugmentedFifth, A5, 8;
    MajorSixth, M6, 9;
    DimishedSeventh, d7, 9;
    MinorSeventh, m7, 10;
    AugmentedSixth, A6, 10;
    MajorSevent, M7, 11;
    DiminishedOctave, d8, 11;
    PerfectOctave, P8, 12;
    AugmentedSeventh, A7, 12;
    DiminishedNinth, d9, 12;
    MinorNinth, m9, 13;
    AugmentedOctave, A8, 13;
    MajorNinth, M9, 14;
    DiminishedTenth, d10, 14;
    MinorTenth, m10, 15;
    AugmentedNinth, A9, 15;
    MajorTenth, M10, 16;
    DiminishedEleventh, d11, 16;
    PerfectEleventh, P11, 17;
    AugmentedTenth, A10, 17;
    DiminishedTwelfth, d12, 18;
    AugmentedEleventh, A11, 18;
    PefectTwelfth, P12, 19;
    DminishedThirteenth, d13, 19;
    MinorThirteenth, m13, 20;
    AugmentedTwelfth, A12, 20;
    MajorThirteenth, M13, 21;
    DiminishedForteenth, d14, 21;
    MinorFourteenth, m14, 22;
    AugmentedThirteenth, A13, 22;
    MajorFourteenth, M14, 23;
    DiminishedFifteenth, d15, 23;
    PerfectFifteenth, P15, 24;
    AugmentedFourteenth, A14, 24;
    AugmentedFifteenth, A15, 25;
}

use super::notes::Note;
pub use std::convert::TryFrom;
use std::ops::{Add, Sub};

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, interval: Interval) -> Self {
        let n = self as i16;
        let semi = interval.semitones() as i16;

        Self::try_from(n + semi).expect("Couldn't add Interval to Note")
    }
}
impl Add<Note> for Interval {
    type Output = Note;

    fn add(self, note: Note) -> Note {
        let n = note as i16;
        let semi = self.semitones() as i16;

        Note::try_from(n + semi).expect("Couldn't add Interval to Note")
    }
}
impl Sub<Interval> for Note {
    type Output = Self;

    fn sub(self, interval: Interval) -> Self {
        let n = self as i16;
        let semi = interval.semitones() as i16;

        Self::try_from(n - semi).expect("Couldn't subtract Interval from Note")
    }
}
impl Sub<Note> for Interval {
    type Output = Note;

    fn sub(self, note: Note) -> Note {
        let n = note as i16;
        let semi = self.semitones() as i16;

        Note::try_from(n - semi).expect("Couldn't subtract Interval from Note")
    }
}

#[cfg(test)]
mod test {
    use super::intervals_short::*;
    use super::*;

    #[test]
    fn add_interval_to_note() {
        assert_eq!(Note::C5, Note::A4 + m3);
        assert_eq!(Note::A4, Note::C5 - m3);

        assert_eq!(Note::A4, Note::A5 - P8);
        assert_eq!(Note::A4, Note::A5 - A7);
        assert_eq!(Note::A5, Note::A4 + P8);
        assert_eq!(Note::A5, Note::A4 + A7);
    }

    #[test]
    #[should_panic]
    fn adding_to_max_note_panics() {
        let _ = Note::B8 + m7;
    }
}
