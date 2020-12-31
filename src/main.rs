use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct SuperLanguages {
    languages: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuperWords {
    words: Vec<SuperWord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuperWord {
    id: usize,
    meaning: String,
    loans: HashMap<String, String>,
    origins: HashMap<String, String>,
}

trait CharExt {
    fn is_vowel(&self) -> bool;
    fn is_consonant(&self) -> bool;
    fn is_voiced(&self) -> bool;
    fn is_unvoiced(&self) -> bool;
    fn is_xhyw(&self) -> bool;
    fn is_cjsz(&self) -> bool;
}

impl CharExt for char {
    fn is_vowel(&self) -> bool {
        *self == 'i' || *self == 'e' || *self == 'a' || *self == 'o' || *self == 'u'
    }

    fn is_consonant(&self) -> bool {
        *self == 'p'
            || *self == 'b'
            || *self == 't'
            || *self == 'd'
            || *self == 'k'
            || *self == 'g'
            || *self == 'm'
            || *self == 'n'
            || *self == 'r'
            || *self == 'f'
            || *self == 'v'
            || *self == 's'
            || *self == 'z'
            || *self == 'c'
            || *self == 'j'
            || *self == 'x'
            || *self == 'h'
            || *self == 'y'
            || *self == 'l'
            || *self == 'w'
    }

    fn is_voiced(&self) -> bool {
        *self == 'b' || *self == 'd' || *self == 'g' || *self == 'v' || *self == 'z' || *self == 'j'
    }

    fn is_unvoiced(&self) -> bool {
        *self == 'p' || *self == 't' || *self == 'k' || *self == 'f' || *self == 's' || *self == 'c'
    }

    fn is_xhyw(&self) -> bool {
        *self == 'x' || *self == 'h' || *self == 'y' || *self == 'w'
    }

    fn is_cjsz(&self) -> bool {
        *self == 'c' || *self == 'j' || *self == 's' || *self == 'z'
    }
}

pub trait StringExt {
    fn nth(&self, n: usize) -> Option<char>;
    fn rev_nth(&self, n: usize) -> Option<char>;
    fn is_match_w202_and_w203(&self) -> bool;
    fn is_match_w204(&self) -> bool;
    fn is_match_w205(&self) -> bool;
    fn is_match_w206(&self) -> bool;
    fn is_match_w207(&self) -> bool;
    fn is_match_w208(&self) -> bool;
    fn is_match_w209(&self) -> bool;
    fn is_match_w210(&self) -> bool;
}

impl StringExt for String {
    fn nth(&self, n: usize) -> Option<char> {
        self.chars().nth(n)
    }

    fn rev_nth(&self, n: usize) -> Option<char> {
        self.chars().rev().nth(n)
    }

    fn is_match_w202_and_w203(&self) -> bool {
        match (self.nth(0), self.nth(1)) {
            (Some(a), Some(b)) => a != b,
            _ => true,
        }
    }

    fn is_match_w204(&self) -> bool {
        match (self.nth(0), self.nth(1)) {
            (Some(a), Some(b)) => {
                !(a.is_voiced() && b.is_unvoiced() || a.is_unvoiced() && b.is_voiced())
            }
            _ => true,
        }
    }

    fn is_match_w205(&self) -> bool {
        match (self.nth(0), self.nth(1)) {
            (Some(a), Some(b)) => {
                !(a.is_xhyw() && b.is_consonant() || a.is_consonant() && b.is_xhyw())
            }
            _ => true,
        }
    }

    fn is_match_w206(&self) -> bool {
        match (self.nth(0), self.nth(1), self.nth(2)) {
            (Some('t'), Some('s'), Some(_)) => true,
            (Some('t'), Some('c'), Some(_)) => true,
            (Some('d'), Some('z'), Some(_)) => true,
            (Some('d'), Some('j'), Some(_)) => true,
            (Some(_), Some('t'), Some('s')) => true,
            (Some(_), Some('t'), Some('c')) => true,
            (Some(_), Some('d'), Some('z')) => true,
            (Some(_), Some('d'), Some('j')) => true,
            (Some(a), Some(b), Some(c)) => {
                !(a.is_consonant() && b.is_consonant() && c.is_consonant())
            }
            _ => true,
        }
    }

    fn is_match_w207(&self) -> bool {
        match (self.nth(0), self.nth(1)) {
            (Some(a), Some(b)) => !(a.is_cjsz() && b.is_cjsz()),
            _ => true,
        }
    }

    fn is_match_w208(&self) -> bool {
        match self.nth(0) {
            Some(a) => a.is_consonant(),
            _ => true,
        }
    }

    fn is_match_w209(&self) -> bool {
        match self.rev_nth(0) {
            Some(a) => a.is_consonant(),
            _ => true,
        }
    }

    fn is_match_w210(&self) -> bool {
        match (self.nth(0), self.nth(1)) {
            (Some('t'), Some('s')) => true,
            (Some('t'), Some('c')) => true,
            (Some('t'), Some(_)) => false,
            (Some('d'), Some('z')) => true,
            (Some('d'), Some('j')) => true,
            (Some('d'), Some(_)) => false,
            (Some(a), Some(b)) => !(a.is_consonant() && b.is_consonant()),
            _ => true,
        }
    }
}

pub struct CandidateWord {
    pub word: String,
    pub score: f64,
}

#[warn(dead_code)]
fn main() {
    let file1 = File::open("data/language_weight.json").unwrap();
    let file2 = File::open("data/super_words.json").unwrap();
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);
    let super_languages: SuperLanguages = serde_json::from_reader(reader1).unwrap();
    let super_words: SuperWords = serde_json::from_reader(reader2).unwrap();
    println!("{:?}", super_languages);
    println!("{:?}", super_words);
}
