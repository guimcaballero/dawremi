//! Different types that can be turned into `Frequency`

pub use std::convert::TryFrom;

use crate::frequency::Frequency;

pub mod n_tet {
    // Some microtonal stuff
    // https://en.wikipedia.org/wiki/31_equal_temperament
    // https://en.wikipedia.org/wiki/Quarter_tone

    // I might have been listening to too much KG&LW while working today...
    // Flying microtonal banana rocks

    #![allow(dead_code)]

    use super::*;

    /// I'm using Const generics to define the number of subdivisions, as I think it belongs more on the type
    /// Also, I just wanted to try it
    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub struct NTet<const N: u8>(i16);

    impl<const N: u8> NTet<N> {
        /// Octave: 4, Fraction: 0, always corresponds to C4
        /// The rest are generated accordingly
        pub fn new(octave: u8, fraction: u8) -> Self {
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
            let initial = 2.0_f64.powf(note.0 as f64 / N as f64);

            let c4f: Frequency = Note::C4.into();
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
}

impl From<Note> for Frequency {
    fn from(note: Note) -> Self {
        let n = note as i16;
        let a: f64 = 2.0_f64.powf(n as f64 / 12.);
        440. * a
    }
}

macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
            $vis enum $name {
                $($(#[$vmeta])* $vname $(= $val)?,)*
            }

        impl TryFrom<i16> for $name {
            type Error = ();

            fn try_from(v: i16) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as i16 => Ok($name::$vname),)*
                        _ => Err(()),
                }
            }
        }
    }
}

back_to_enum! {
    #[allow(dead_code)]
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Note {
        C0 = -57,
        Cs0,
        D0,
        Ds0,
        E0,
        F0,
        Fs0,
        G0,
        Gs0,
        A0 = -48,
        As0,
        B0,
        C1,
        Cs1,
        D1,
        Ds1,
        E1,
        F1,
        Fs1,
        G1,
        Gs1,
        A1 = -36,
        As1,
        B1,
        C2,
        Cs2,
        D2,
        Ds2,
        E2,
        F2,
        Fs2,
        G2,
        Gs2,
        A2 = -24,
        As2,
        B2,
        C3,
        Cs3,
        D3,
        Ds3,
        E3,
        F3,
        Fs3,
        G3,
        Gs3,
        A3 = -12,
        As3,
        B3,
        C4,
        Cs4,
        D4,
        Ds4,
        E4,
        F4,
        Fs4,
        G4,
        Gs4,
        A4 = 0,
        As4,
        B4,
        C5,
        Cs5,
        D5,
        Ds5,
        E5,
        F5,
        Fs5,
        G5,
        Gs5,
        A5 = 12,
        As5,
        B5,
        C6,
        Cs6,
        D6,
        Ds6,
        E6,
        F6,
        Fs6,
        G6,
        Gs6,
        A6 = 24,
        As6,
        B6,
        C7,
        Cs7,
        D7,
        Ds7,
        E7,
        F7,
        Fs7,
        G7,
        Gs7,
        A7 = 36,
        As7,
        B7,
        C8,
        Cs8,
        D8,
        Ds8,
        E8,
        F8,
        Fs8,
        G8,
        Gs8,
        A8,
        As8,
        B8,
    }
}

impl Note {
    // All of this functions can panic, this way we know at compile time if our note will exist or not
    // And if it doesn't we can add it to the enum

    pub fn up_an_octave(self) -> Note {
        let n = self as i16;
        Note::try_from(n + 12).expect("Raise note an octave")
    }
    pub fn down_an_octave(self) -> Note {
        let n = self as i16;
        Note::try_from(n - 12).expect("Lower note an octave")
    }
    pub fn up_a_note(self) -> Note {
        let n = self as i16;
        Note::try_from(n + 1).expect("Raise one note")
    }
    pub fn down_a_note(self) -> Note {
        let n = self as i16;
        Note::try_from(n - 1).expect("Lower one note")
    }

    pub fn closest_to_frequency(freq: f64) -> Note {
        let n = (12. * (freq / 440.).log2()).round() as i16;
        Note::try_from(n).expect("Couldn't find closest note")
    }
}

macro_rules! enum_to_note {
    ($vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident => $val:ident,)*
    }) => {
        #[derive(Clone, Copy, Debug, PartialEq)]
        $vis enum $name {
            $($(#[$vmeta])* $vname,)*
        }

        impl From<$name> for Note {
            fn from(f: $name) -> Note {
                match f {
                    $($name::$vname => Note::$val,)*
                }
            }
        }

        impl From<$name> for Frequency {
            fn from(f: $name) -> Frequency {
                let note: Note = f.into();
                note.into()
            }
        }
    }
}

enum_to_note! {
    // From https://i.redd.it/vabojwxo2yf31.jpg
    pub enum GuitarFretboard {
        E0  => E4,
        E1  => F4,
        E2  => Fs4,
        E3  => G4,
        E4  => Gs4,
        E5  => A4,
        E6  => As4,
        E7  => B4,
        E8  => C5,
        E9  => Cs5,
        E10 => D5,
        E11 => Ds5,
        E12 => E5,
        E13 => F5,
        E14 => Fs5,
        E15 => G5,
        E16 => Gs5,
        E17 => A5,
        E18 => As5,
        E19 => B5,
        E20 => C6,
        E21 => Cs6,
        E22 => D6,
        E23 => Ds6,
        E24 => E6,

        B0  => B3,
        B1  => C4,
        B2  => Cs4,
        B3  => D4,
        B4  => Ds4,
        B5  => E4,
        B6  => F4,
        B7  => Fs4,
        B8  => G4,
        B9  => Gs4,
        B10 => A4,
        B11 => As4,
        B12 => B4,
        B13 => C5,
        B14 => Cs5,
        B15 => D5,
        B16 => Ds5,
        B17 => E5,
        B18 => F5,
        B19 => Fs5,
        B20 => G5,
        B21 => Gs5,
        B22 => A5,
        B23 => As5,
        B24 => B5,

        G0  => G3,
        G1  => Gs3,
        G2  => A3,
        G3  => As3,
        G4  => B3,
        G5  => C4,
        G6  => Cs4,
        G7  => D4,
        G8  => Ds4,
        G9  => E4,
        G10 => F4,
        G11 => Fs4,
        G12 => G4,
        G13 => Gs4,
        G14 => A4,
        G15 => As4,
        G16 => B4,
        G17 => C5,
        G18 => Cs5,
        G19 => D5,
        G20 => Ds5,
        G21 => E5,
        G22 => F5,
        G23 => Fs5,
        G24 => G5,

        D0  => D3,
        D1  => Ds3,
        D2  => E3,
        D3  => F3,
        D4  => Fs3,
        D5  => G3,
        D6  => Gs3,
        D7  => A3,
        D8  => As3,
        D9  => B3,
        D10 => C4,
        D11 => Cs4,
        D12 => D4,
        D13 => Ds4,
        D14 => E4,
        D15 => F4,
        D16 => Fs4,
        D17 => G4,
        D18 => Gs4,
        D19 => A4,
        D20 => As4,
        D21 => B4,
        D22 => C5,
        D23 => Cs5,
        D24 => D5,

        A0  => A2,
        A1  => As2,
        A2  => B2,
        A3  => C3,
        A4  => Cs3,
        A5  => D3,
        A6  => Ds3,
        A7  => E3,
        A8  => F3,
        A9  => Fs3,
        A10 => G3,
        A11 => Gs3,
        A12 => A3,
        A13 => As3,
        A14 => B3,
        A15 => C4,
        A16 => Cs4,
        A17 => D4,
        A18 => Ds4,
        A19 => E4,
        A20 => F4,
        A21 => Fs4,
        A22 => G4,
        A23 => Gs4,
        A24 => A4,

        // L for low E
        L0  => E2,
        L1  => F2,
        L2  => Fs2,
        L3  => G2,
        L4  => Gs2,
        L5  => A2,
        L6  => As2,
        L7  => B2,
        L8  => C3,
        L9  => Cs3,
        L10 => D3,
        L11 => Ds3,
        L12 => E3,
        L13 => F3,
        L14 => Fs3,
        L15 => G3,
        L16 => Gs3,
        L17 => A3,
        L18 => As3,
        L19 => B3,
        L20 => C4,
        L21 => Cs4,
        L22 => D4,
        L23 => Ds4,
        L24 => E4,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_freq_note {
        ( $freq:expr, $note:tt) => {
            let note: Frequency = Note::$note.into();
            assert_eq!(format!("{:.2}", note), format!("{:.2}", $freq));
        };
    }

    #[test]
    fn check_frequencies_for_notes() {
        check_freq_note!(16.35, C0);
        check_freq_note!(98., G2);
        check_freq_note!(440., A4);
        check_freq_note!(293.66, D4);
        check_freq_note!(1046.5, C6);
        check_freq_note!(7902.13, B8);
    }

    #[test]
    fn going_up_octave() {
        let c5 = Note::C4.up_an_octave();
        assert_eq!(c5, Note::C5);

        let a3 = Note::A2.up_an_octave();
        assert_eq!(a3, Note::A3);
    }

    #[test]
    fn going_down_octave() {
        let c5 = Note::C6.down_an_octave();
        assert_eq!(c5, Note::C5);

        let a3 = Note::A4.down_an_octave();
        assert_eq!(a3, Note::A3);
    }

    #[test]
    fn closest_frequency() {
        let note = Note::closest_to_frequency(21.);
        assert_eq!(note, Note::E0);

        let note = Note::closest_to_frequency(21.3);
        assert_eq!(note, Note::F0);

        let note = Note::closest_to_frequency(90.);
        assert_eq!(note, Note::Fs2);

        let note = Note::closest_to_frequency(442.);
        assert_eq!(note, Note::A4);

        let note = Note::closest_to_frequency(486.);
        assert_eq!(note, Note::B4);

        let note = Note::closest_to_frequency(870.);
        assert_eq!(note, Note::A5);

        let note = Note::closest_to_frequency(1100.);
        assert_eq!(note, Note::Cs6);
    }
}
