use crate::music_theory::intervals::*;

macro_rules! chords {
    (
        $( $var:ident, $func:ident, $intervals:expr ; )*
    ) => {
        pub enum Chord {
            $( $var, )*
            /// Use Other for chords that aren't in this enum
            Other(&'static [ Interval ])
        }

        impl Chord {
            pub const fn intervals(&self) -> &[Interval] {
                use intervals_short::*;
                match self {
                    $( Self::$var => &$intervals, )*
                    Self::Other(int) => int,
                }
            }
        }

        pub mod chords_short {
            #![allow(non_upper_case_globals)]

            use super::*;

            $(
                pub const $func: Chord = Chord::$var;
            )*
        }
    };
}

// From https://en.wikibooks.org/wiki/Music_Theory/Complete_List_of_Chord_Patterns

chords! {
    Major, maj, [ P1, M3, P5 ];
    MajorSeventh, maj7, [ P1, M3, P5, M7 ];
    MajorNinth, maj9, [ P1, M3, P5, M7, M9 ];
    Sixth, add6, [ P1, M3, P5, M6 ];
    SixthNinth, six9, [ P1, M3, P5, M6, M9 ];
    Lydian, lydian, [ P1, M3, P5, M7, M9, A11 ];
    MajorSeventhB6, maj7b6, [ P1, M3, P5, M7, M9, A11, m13 ];

    Fifth, fifth, [ P1, P5 ];
    Augmented, aug, [ P1, M3, A5 ];
    AugmentedSeventh, aug7, [ P1, M3, A5, M7 ];

    Minor, min, [ P1, m3, P5 ];
    MinorSeventh, min7, [ P1, m3, P5, m7 ];
    MinorMajorSeventh, minmaj7, [ P1, m3, P5, M7 ];
    MinorSixth, min6, [ P1, m3, P5, M6 ];
    MinorNinth, min9, [ P1, m3, P5, m7, M9 ];
    MinorEleventh, min11, [ P1, m3, P5, m7, M9, P11 ];
    MinorThirteenth, min13, [ P1, m3, P5, m7, M9, P11, M13 ];

    DominantSeventh, dom7, [ P1, M3, P5, m7 ];
    DominantNinth, dom9, [ P1, M3, P5, m7, M9 ];
    DominantThirteenth, dom13, [ P1, M3, P5, m7, M9, M13 ];
    Altered, alt7, [ P1, M3, m7, m9 ];
    SuspendedSecond, sus2, [ P1, M2, P5 ];
    SuspendedFourth, sus4, [ P1, P4, P5 ];
}

// There's some I haven't added, feel free to send a PR with more

// TODO Implement something to do
// C4 >> Chord::Major = [C4, E4, G4]

use super::notes::Note;
use std::ops::Shr;

impl Shr<Chord> for Note {
    type Output = Vec<Note>;

    fn shr(self, rhs: Chord) -> Self::Output {
        rhs.intervals()
            .iter()
            .map(|interval| self + *interval)
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::chords_short::*;
    use super::*;

    #[test]
    fn add_interval_to_note() {
        assert_eq!(vec![Note::C4, Note::E4, Note::G4], Note::C4 >> maj);
    }
}
