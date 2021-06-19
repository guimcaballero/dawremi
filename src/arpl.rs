use crate::music_theory::chords::*;
use crate::music_theory::notes::*;
use crate::music_theory::scales::*;

pub use arpl::Pattern;

pub trait PatternExtension {
    fn scale(self, root: Note, scale: Scale) -> ScalePattern;
    fn chord(self, root: Note, chord: Chord) -> ChordPattern;
    fn notes(self, notes: &[Note]) -> NotePattern;
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
