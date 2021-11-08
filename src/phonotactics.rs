use crate::phoneme::{Phoneme, PhonemeExt};

pub trait PhonotacticsExt {
    fn is_match_w212(&self) -> bool;
    fn is_match_w209(&self) -> bool;
    fn is_match_w213(&self) -> bool;
    fn is_match_w214(&self) -> bool;
    fn is_match_w215(&self) -> bool;
}

impl PhonotacticsExt for Vec<Phoneme> {
    fn is_match_w212(&self) -> bool {
        if self.len() < 2 {
            true
        } else {
            let len = self.len();
            match (self[len - 2], self[len - 1]) {
                (Phoneme::Y, _) | (Phoneme::W, _) => true,
                (a, b) => !(a.is_vowel() && b.is_vowel()),
            }
        }
    }

    fn is_match_w213(&self) -> bool {
        if self.len() < 2 {
            true
        } else {
            let len = self.len();
            match (self[len - 2], self[len - 1]) {
                (a, b) => !(a.is_consonant() && b.is_consonant()),
            }
        }
    }

    fn is_match_w214(&self) -> bool {
        if self.len() < 1 {
            true
        } else {
            self[0].is_consonant()
        }
    }

    fn is_match_w209(&self) -> bool {
        if self.len() < 1 {
            true
        } else {
            let len = self.len();
            self[len - 1].is_consonant()
        }
    }

    fn is_match_w215(&self) -> bool {
        if self.len() < 3 {
            true
        } else {
            let len = self.len();
            match (self[len - 3], self[len - 2], self[len - 1]) {
                (Phoneme::I, Phoneme::Y, Phoneme::I) => false,
                (Phoneme::U, Phoneme::W, Phoneme::U) => false,
                _ => true,
            }
        }
    }
}
