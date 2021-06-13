use crate::music_theory::intervals::*;

macro_rules! scales {
    (
        $( $var:ident, $intervals:expr ; )*
    ) => {
        pub enum Scale {
            $( $var, )*
            /// Use Other for chords that aren't in this enum
            Other(&'static [ Interval ])
        }

        impl Scale {
            pub const fn intervals(&self) -> &[Interval] {
                use intervals_short::*;
                match self {
                    $( Self::$var => &$intervals, )*
                    Self::Other(int) => int,
                }
            }

            pub const fn len(&self) -> usize {
                self.intervals().len()
            }
        }
    };
}

scales! {
    MajorPentatonic, [ P1, M2, M3, P5, M6 ];
    MinorPentatonic, [ P1, m3, P4, P5, m7 ];
    ChromaticScale, [ P1, m2, M2, m3, M3, P4, A4, P5, m6, M6, m7, M7 ];
    DorianMode, [ P1, M2, m3, P4, P5, M6, m7 ];
    HarmonicMajor, [ P1, M2, M3, P4, P5, m6, M7 ];
    HarmonicMinor, [ P1, M2, m3, P4, P5, m6, M7 ];
    MajorScale, [ P1, M2, M3, P4, P5, M6, M7 ];
    LocrianMode, [ P1, m2, m3, P4, d5, m6, m7 ];
    LydianMode, [ P1, M2, M3, A4, P5, M6, M7 ];
    Myxolidian, [ P1, M2, M3, P4, P5, M6, m7 ];
    PersianScale, [ P1, m2, M3, P4, d5, m6, M7 ];
    PhyrigianMode, [ P1, m2, m3, P4, P5, m6, m7 ];
}

// There's some I haven't added, feel free to send a PR with more

use super::notes::Note;
use std::ops::Shr;

impl Shr<Scale> for Note {
    type Output = Vec<Self>;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn shr(self, rhs: Scale) -> Self::Output {
        rhs.intervals()
            .iter()
            .map(|interval| self + *interval)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scales() {
        assert_eq!(
            vec![Note::C4, Note::D4, Note::E4, Note::G4, Note::A4],
            Note::C4 >> Scale::MajorPentatonic
        );
        assert_eq!(
            vec![Note::A4, Note::C5, Note::D5, Note::E5, Note::G5],
            Note::A4 >> Scale::MinorPentatonic
        );
    }
    // C, E-flat, F, G, B-flat.
}
