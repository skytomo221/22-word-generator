use crate::phoneme::{Phoneme, PhonemeExt};

pub trait PhonotacticsExt {
    // See https://skytomo221.github.io/22/doc/rfcs/rfc-9
    fn is_match_rfc_9_3(&self) -> bool;
    fn is_match_rfc_9_4(&self) -> bool;
    fn is_match_rfc_9_5(&self) -> bool;
    fn is_match_rfc_9_6(&self) -> bool;
    fn is_match_rfc_9_7(&self) -> bool;
}

impl PhonotacticsExt for Vec<Phoneme> {
    fn is_match_rfc_9_3(&self) -> bool {
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

    fn is_match_rfc_9_5(&self) -> bool {
        if self.len() < 2 {
            true
        } else {
            let len = self.len();
            match (self[len - 2], self[len - 1]) {
                (a, b) => !(a.is_consonant() && b.is_consonant()),
            }
        }
    }

    fn is_match_rfc_9_6(&self) -> bool {
        if self.len() < 1 {
            true
        } else {
            self[0].is_consonant()
        }
    }

    fn is_match_rfc_9_4(&self) -> bool {
        if self.len() < 1 {
            true
        } else {
            let len = self.len();
            self[len - 1].is_consonant()
        }
    }

    fn is_match_rfc_9_7(&self) -> bool {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::phoneme::Phoneme;

    #[test]
    fn test_w212() {
        assert!(vec![Phoneme::P, Phoneme::A].is_match_rfc_9_3());
        assert!(vec![Phoneme::A, Phoneme::P].is_match_rfc_9_3());
        assert!(!vec![Phoneme::A, Phoneme::A].is_match_rfc_9_3());
        assert!(vec![Phoneme::P, Phoneme::P].is_match_rfc_9_3());
        assert!(!vec![Phoneme::B, Phoneme::A, Phoneme::A].is_match_rfc_9_3());
        assert!(vec![Phoneme::A].is_match_rfc_9_3());
        assert!(vec![Phoneme::P].is_match_rfc_9_3());
        assert!(vec![].is_match_rfc_9_3());
    }

    #[test]
    fn test_w209() {
        assert!(vec![Phoneme::T].is_match_rfc_9_4());
        assert!(vec![Phoneme::A, Phoneme::T].is_match_rfc_9_4());
        assert!(!vec![Phoneme::A].is_match_rfc_9_4());
        assert!(vec![].is_match_rfc_9_4());
    }

    #[test]
    fn test_w213() {
        assert!(vec![Phoneme::P, Phoneme::A].is_match_rfc_9_5());
        assert!(vec![Phoneme::A, Phoneme::P].is_match_rfc_9_5());
        assert!(vec![Phoneme::A, Phoneme::A].is_match_rfc_9_5());
        assert!(!vec![Phoneme::P, Phoneme::P].is_match_rfc_9_5());
        assert!(!vec![Phoneme::A, Phoneme::B, Phoneme::B].is_match_rfc_9_5());
        assert!(vec![Phoneme::A].is_match_rfc_9_5());
        assert!(vec![Phoneme::P].is_match_rfc_9_5());
        assert!(vec![].is_match_rfc_9_5());
    }

    #[test]
    fn test_w214() {
        assert!(vec![Phoneme::P, Phoneme::A].is_match_rfc_9_6());
        assert!(!vec![Phoneme::A, Phoneme::P].is_match_rfc_9_6());
        assert!(!vec![Phoneme::A].is_match_rfc_9_6());
        assert!(vec![Phoneme::P].is_match_rfc_9_6());
        assert!(vec![].is_match_rfc_9_6());
    }

    #[test]
    fn test_w215() {
        assert!(vec![Phoneme::I, Phoneme::Y, Phoneme::I, Phoneme::A].is_match_rfc_9_7());
        assert!(vec![Phoneme::U, Phoneme::W, Phoneme::U, Phoneme::A].is_match_rfc_9_7());
        assert!(!vec![Phoneme::I, Phoneme::Y, Phoneme::I].is_match_rfc_9_7());
        assert!(!vec![Phoneme::U, Phoneme::W, Phoneme::U].is_match_rfc_9_7());
        assert!(!vec![Phoneme::A, Phoneme::I, Phoneme::Y, Phoneme::I].is_match_rfc_9_7());
        assert!(!vec![Phoneme::A, Phoneme::U, Phoneme::W, Phoneme::U].is_match_rfc_9_7());
        assert!(vec![].is_match_rfc_9_7());
    }
}
