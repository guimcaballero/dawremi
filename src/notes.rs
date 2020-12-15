use std::convert::TryFrom;

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
    pub fn up_an_octave(&self) -> Option<Note> {
        let n = *self as i16;
        Note::try_from(n + 12).ok()
    }
    pub fn down_an_octave(&self) -> Option<Note> {
        let n = *self as i16;
        Note::try_from(n - 12).ok()
    }
}

pub struct Frequency(pub f64);
impl From<Note> for Frequency {
    fn from(note: Note) -> Self {
        let n = note as i16;
        let a: f64 = 2.0_f64.powf(n as f64 / 12.);
        Self(440.0 * a)
    }
}
impl From<f64> for Frequency {
    fn from(freq: f64) -> Self {
        Self(freq)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! check_freq_note {
        ( $freq:expr, $note:tt) => {
            let note: Frequency = Note::$note.into();
            assert_eq!(format!("{:.2}", note.0), format!("{:.2}", $freq));
        };
    }

    #[test]
    fn check_frequencies_for_notes() {
        check_freq_note!(16.35, C0);
        check_freq_note!(98., G2);
        check_freq_note!(440., A4);
        check_freq_note!(1046.5, C6);
        check_freq_note!(7902.13, B8);
    }

    #[test]
    fn going_up_octave() {
        let c5 = Note::C4.up_an_octave();
        assert_eq!(c5, Some(Note::C5));

        let a3 = Note::A2.up_an_octave();
        assert_eq!(a3, Some(Note::A3));

        let none = Note::B8.up_an_octave();
        assert_eq!(none, None);
    }

    #[test]
    fn going_down_octave() {
        let c5 = Note::C6.down_an_octave();
        assert_eq!(c5, Some(Note::C5));

        let a3 = Note::A4.down_an_octave();
        assert_eq!(a3, Some(Note::A3));

        let none = Note::A0.down_an_octave();
        assert_eq!(none, None);
    }
}
