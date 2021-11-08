use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Phoneme {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    SCHWA,
}

pub trait PhonemeExt {
    fn is_vowel(&self) -> bool;
    fn is_consonant(&self) -> bool;
}

impl PhonemeExt for Phoneme {
    fn is_vowel(&self) -> bool {
        match *self {
            Self::A | Self::E | Self::I | Self::O | Self::U => true,
            _ => false,
        }
    }

    fn is_consonant(&self) -> bool {
        match *self {
            Self::P
            | Self::B
            | Self::T
            | Self::D
            | Self::K
            | Self::G
            | Self::M
            | Self::N
            | Self::R
            | Self::F
            | Self::V
            | Self::S
            | Self::Z
            | Self::C
            | Self::J
            | Self::X
            | Self::H
            | Self::L
            | Self::Y
            | Self::W => true,
            _ => false,
        }
    }
}
