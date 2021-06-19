// Some microtonal stuff
// https://en.wikipedia.org/wiki/31_equal_temperament
// https://en.wikipedia.org/wiki/Quarter_tone

// I might have been listening to too much KG&LW while working today...
// Flying microtonal banana rocks

#![allow(dead_code)]

use crate::music_theory::notes::*;
use crate::trigger::Frequency;

/// I'm using Const generics to define the number of subdivisions, as I think it belongs more on the type
/// Also, I just wanted to try it
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct NTet<const N: u8>(i16);

impl<const N: u8> NTet<N> {
    /// Octave: 4, Fraction: 0, always corresponds to C4
    /// The rest are generated accordingly
    pub const fn new(octave: u8, fraction: u8) -> Self {
        Self((octave as i16 - 4) * N as i16 + fraction as i16)
    }
}

pub type Tet7 = NTet<7>;
pub type Tet12 = NTet<12>;
pub type Tet19 = NTet<19>;
pub type Tet24 = NTet<24>;
pub type Tet31 = NTet<31>;
pub type Tet41 = NTet<41>;
pub type Tet53 = NTet<53>;
pub type Tet72 = NTet<72>;

impl<const N: u8> From<NTet<N>> for Frequency {
    fn from(note: NTet<N>) -> Self {
        let initial = (note.0 as Self / N as Self).exp2();

        let c4f: Self = Note::C4.into();
        c4f * initial
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_freq_equivals_note {
        ($freq:expr, $note:tt) => {
            let note: Frequency = Note::$note.into();
            let freq: Frequency = $freq.into();
            assert_eq!(format!("{:.4}", note), format!("{:.4}", freq));
        };
    }

    #[test]
    fn check_some_values() {
        check_freq_equivals_note!(Tet12::new(2, 0), C2);
        check_freq_equivals_note!(Tet12::new(4, 0), C4);
        check_freq_equivals_note!(Tet12::new(4, 1), Cs4);
        check_freq_equivals_note!(Tet12::new(4, 2), D4);
        check_freq_equivals_note!(Tet12::new(4, 7), G4);
        check_freq_equivals_note!(Tet12::new(7, 2), D7);

        check_freq_equivals_note!(Tet24::new(4, 0), C4);
        check_freq_equivals_note!(Tet24::new(4, 2), Cs4);
        check_freq_equivals_note!(Tet24::new(4, 4), D4);
        check_freq_equivals_note!(Tet24::new(4, 14), G4);
        check_freq_equivals_note!(Tet24::new(7, 4), D7);

        check_freq_equivals_note!(Tet72::new(4, 0), C4);
        check_freq_equivals_note!(Tet72::new(4, 6), Cs4);
        check_freq_equivals_note!(Tet72::new(4, 12), D4);
        check_freq_equivals_note!(Tet72::new(4, 42), G4);
        check_freq_equivals_note!(Tet72::new(7, 6), Cs7);
    }

    #[test]
    fn equalities() {
        assert_eq!(Tet24::new(4, 2), Tet24::new(3, 26));
        assert_eq!(Tet24::new(5, 1), Tet24::new(3, 49));
    }
}
