//! For making patterns out of sounds
//! Specially useful for drum loops

use super::*;
use crate::frame::Frame;
use crate::song::Song;

/// Contains a pattern and a sound
#[derive(Clone)]
pub struct SoundPattern<const LEN: usize> {
    pub(crate) pattern: [bool; LEN],
    pub(crate) sound: Sound,
}
impl<const LEN: usize> SoundPattern<LEN> {
    /// Returns the opposite pattern
    pub fn neg(self) -> Self {
        let mut pattern = [false; LEN];
        for i in 0..LEN {
            pattern[i] = !self.pattern[i];
        }
        SoundPattern {
            sound: self.sound,
            pattern,
        }
    }
}

/// Used to denote when a step is on and when it's off
pub enum PatternStep {
    /// On
    X,
    /// Off
    O,
}

pub trait IntoSoundPattern<const LEN: usize> {
    /// Convert into a SoundPattern using the provided sound
    fn sound(self, sound: impl Into<Sound>) -> SoundPattern<LEN>;
}

impl<const LEN: usize> IntoSoundPattern<LEN> for [bool; LEN] {
    fn sound(self, sound: impl Into<Sound>) -> SoundPattern<LEN> {
        SoundPattern {
            pattern: self,
            sound: sound.into(),
        }
    }
}
impl<const LEN: usize> IntoSoundPattern<LEN> for [PatternStep; LEN] {
    fn sound(self, sound: impl Into<Sound>) -> SoundPattern<LEN> {
        SoundPattern {
            pattern: map_steps_to_bools(self),
            sound: sound.into(),
        }
    }
}
impl<const LEN: usize> IntoSoundPattern<LEN> for usize {
    fn sound(self, sound: impl Into<Sound>) -> SoundPattern<LEN> {
        SoundPattern {
            pattern: map_number_to_bools(self),
            sound: sound.into(),
        }
    }
}

pub trait SoundPatternListExtension<const LEN: usize> {
    /// Generates the drum patterns into audio
    /// Will loop `loops` times
    fn generate(&self, song: &Song, loops: usize) -> Vec<Frame>;
    /// Generates the drum patterns into audio with a beat of length `length`
    /// Will loop `loops` times
    fn generate_beat_len(&self, song: &Song, loops: usize, length: f64) -> Vec<Frame>;
}
impl<const N: usize, const LEN: usize> SoundPatternListExtension<LEN> for [SoundPattern<LEN>; N] {
    fn generate(&self, song: &Song, loops: usize) -> Vec<Frame> {
        self.generate_beat_len(song, loops, 1.)
    }
    fn generate_beat_len(&self, song: &Song, loops: usize, length: f64) -> Vec<Frame> {
        let beat = song.beats(length);
        let mut vec = vec![Frame::default(); beat * LEN];

        for pat in self {
            let sound = song.sound(pat.sound.clone());

            for (i, &on) in pat.pattern.iter().cycle().take(LEN * loops).enumerate() {
                if on {
                    let start = i * beat;
                    vec = add_vecs_starting_from(vec, start, &sound);
                }
            }
        }

        vec
    }
}
impl<const LEN: usize> SoundPatternListExtension<LEN> for Vec<SoundPattern<LEN>> {
    fn generate(&self, song: &Song, loops: usize) -> Vec<Frame> {
        self.generate_beat_len(song, loops, 1.)
    }
    fn generate_beat_len(&self, song: &Song, loops: usize, length: f64) -> Vec<Frame> {
        let beat = song.beats(length);
        let mut vec = vec![Frame::default(); beat * LEN];

        for pat in self {
            let sound = song.sound(pat.sound.clone());

            for (i, &on) in pat.pattern.iter().cycle().take(LEN * loops).enumerate() {
                if on {
                    let start = i * beat;
                    vec = add_vecs_starting_from(vec, start, &sound);
                }
            }
        }

        vec
    }
}

/// Adds two vectors, but starts `other` from `start`
fn add_vecs_starting_from(mut base: Vec<Frame>, start: usize, other: &[Frame]) -> Vec<Frame> {
    let end = start + other.len(); // Not the actual end, just of other
    let new_len = base.len().max(end);

    base.resize(new_len, Frame::default());

    for i in start..end {
        base[i] += other[i - start];
    }

    base
}
/// Maps array of steps to array of bools
fn map_steps_to_bools<const LEN: usize>(steps: [PatternStep; LEN]) -> [bool; LEN] {
    let mut out = [false; LEN];
    for i in 0..LEN {
        out[i] = matches!(steps[i], PatternStep::X);
    }
    out
}
/// Maps numbers to an array of bools
/// 01001 -> [false, true, false, false, true]
fn map_number_to_bools<const LEN: usize>(num: usize) -> [bool; LEN] {
    let num = num.to_le();
    let mut out = [false; LEN];
    for i in 0..LEN {
        out[LEN - i - 1] = num & (1 << i) > 0;
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_adding_vecs_from() {
        let base = vec![Frame::default(); 10];

        let other = vec![Frame::mono(1.); 5];

        let res = add_vecs_starting_from(base, 7, &other);

        assert_eq!(12, res.len());
        assert_eq!(Frame::default(), res[6]);
        assert_eq!(Frame::mono(1.), res[7]);
        assert_eq!(Frame::mono(1.), res[8]);
        assert_eq!(Frame::mono(1.), res[11]);
    }

    #[test]
    fn steps_to_bools() {
        use PatternStep::*;
        let pat = [O, O, O, X, O, O, X];
        let res = [false, false, false, true, false, false, true];

        assert_eq!(res, map_steps_to_bools(pat));
    }

    #[test]
    fn nums_to_bools() {
        let num = 0b01001usize;
        let res = [false, false, false, true, false, false, true];

        assert_eq!(res, map_number_to_bools(num));
    }
}
