use crate::music_theory::chords::*;
use crate::music_theory::notes::*;
use crate::music_theory::scales::*;
use crate::sound_files::{pattern::*, Sound};

pub use arpl::Pattern;

pub trait PatternExtension {
    fn scale(self, root: Note, scale: Scale) -> ScalePattern;
    fn chord(self, root: Note, chord: Chord) -> ChordPattern;
    fn notes(self, notes: &[Note]) -> NotePattern;
    fn sound(self, sound: impl Into<Sound>) -> SoundPatternGenerator;
}
impl PatternExtension for Pattern {
    fn scale(self, root: Note, scale: Scale) -> ScalePattern {
        ScalePattern(self, root, scale)
    }
    fn chord(self, root: Note, chord: Chord) -> ChordPattern {
        ChordPattern(self, root, chord)
    }
    fn notes<'a>(self, notes: &'a [Note]) -> NotePattern<'a> {
        NotePattern(self, notes)
    }
    fn sound(self, sound: impl Into<Sound>) -> SoundPatternGenerator {
        SoundPatternGenerator(self, sound.into())
    }
}

pub struct ScalePattern(Pattern, Note, Scale);
impl Iterator for ScalePattern {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|idx| self.1 + self.2.intervals()[idx as usize % self.2.len()])
    }
}
pub struct ChordPattern(Pattern, Note, Chord);
impl Iterator for ChordPattern {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|idx| self.1 + self.2.intervals()[idx as usize % self.2.len()])
    }
}
pub struct NotePattern<'a>(Pattern, &'a [Note]);
impl Iterator for NotePattern<'_> {
    type Item = Note;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|idx| self.1[idx as usize % self.1.len()])
    }
}
pub struct SoundPatternGenerator(Pattern, Sound);
impl SoundPatternGenerator {
    pub fn get<const LEN: usize>(self) -> SoundPattern<LEN> {
        let mut pattern = [false; LEN];
        for (i, step) in self.0.take(LEN).enumerate() {
            pattern[i] = step % 2 == 0;
        }
        SoundPattern {
            sound: self.1,
            pattern,
        }
    }
    pub fn with<const LEN: usize>(self, f: &dyn Fn(u64) -> bool) -> SoundPattern<LEN> {
        let mut pattern = [false; LEN];
        for (i, step) in self.0.take(LEN).enumerate() {
            pattern[i] = f(step);
        }
        SoundPattern {
            sound: self.1,
            pattern,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_scale_pattern() {
        let pat = Pattern::new("A&OH3+45kdn+jAJ!K247[N,J|hy]");
        let mut pat = pat.scale(Note::C4, Scale::MajorPentatonic);

        assert_eq!(pat.next(), Some(Note::C4));
        assert_eq!(pat.next(), Some(Note::C4));
        assert_eq!(pat.next(), Some(Note::C4));
        assert_eq!(pat.next(), Some(Note::D4));
        assert_eq!(pat.next(), Some(Note::A4));
        assert_eq!(pat.next(), Some(Note::D4));
    }

    #[test]
    fn test_notes_pattern() {
        let pat = Pattern::new("A&OH3+45kdn+jAJ!K247[N,J|hy]");
        let mut pat = pat.notes(&[Note::C4, Note::D4]);

        assert_eq!(pat.next(), Some(Note::C4));
        assert_eq!(pat.next(), Some(Note::C4));
        assert_eq!(pat.next(), Some(Note::C4));
        assert_eq!(pat.next(), Some(Note::D4));
    }
}
