use std::{
    cmp::{self, Ordering},
    collections::BTreeSet,
};

use crate::{
    phoneme::{Phoneme, PhonemeExt},
    phonotactics::PhonotacticsExt,
    recipe::{SuperLanguage, SuperWord},
};

#[derive(Debug)]
pub struct CandidateWord {
    pub score: f64,
    pub word: Vec<Phoneme>,
}

impl PartialEq for CandidateWord {
    fn eq(&self, other: &Self) -> bool {
        (self.word.len() == other.word.len()) &&  // zip stops at the shortest
        self.word.iter()
       .zip(other.word.iter())
       .all(|(a,b)| a == b)
    }

    fn ne(&self, other: &Self) -> bool {
        self.score == other.score && self.word == other.word
    }
}

impl Eq for CandidateWord {}

impl PartialOrd for CandidateWord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.score.partial_cmp(&other.score) {
            Some(Ordering::Less) => Some(Ordering::Greater),
            Some(Ordering::Equal) => self.word.partial_cmp(&other.word),
            Some(Ordering::Greater) => Some(Ordering::Less),
            x => x,
        }
    }
}

impl Ord for CandidateWord {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.partial_cmp(&other) {
            Some(x) => x,
            _ => self.word.cmp(&other.word),
        }
    }
}

pub struct WordGenerator<'a> {
    pub super_languages: &'a Vec<SuperLanguage>,
    pub super_word: SuperWord,
    pub words: Vec<CandidateWord>,
    pub limit: usize,
    pub weight_sum: f64,
}

impl WordGenerator<'_> {
    fn calc_weight_sum(&mut self) {
        self.weight_sum = self
            .super_languages
            .iter()
            .filter(|super_language| {
                self.super_word
                    .origins
                    .iter()
                    .any(|origin| origin.language == super_language.language)
            })
            .map(|super_language| super_language.population)
            .sum()
    }

    fn get_super_language(&self, language: &str) -> &SuperLanguage {
        self.super_languages
            .iter()
            .find(|super_language| super_language.language == language)
            .unwrap()
    }

    pub fn get_population(&self, language: &str) -> f64 {
        self.get_super_language(language).population
    }

    fn candidate_length(&self) -> i32 {
        let mut sum = 0.0;
        for origin in &self.super_word.origins {
            sum += origin.loan.as_ref().unwrap().len() as f64
                * self.get_population(&origin.language)
                / self.weight_sum;
        }
        let sum = sum.ceil() as i32;
        if sum % 2 == 0 {
            sum + 1
        } else {
            sum
        }
    }

    fn cadidate_phonemes(&self, n: i32) -> BTreeSet<Phoneme> {
        let c_len = self.candidate_length();
        let mut set = BTreeSet::new();
        for origin in &self.super_word.origins {
            let loan = origin.loan.as_ref().unwrap();
            let len = loan.len() as i32;
            if len < c_len {
                let start = cmp::max(0, n - (c_len - len)) as usize;
                let end = cmp::min(len, n + 1) as usize;
                loan[start..end]
                    .iter()
                    .filter(|c| {
                        if n % 2 == 0 {
                            c.is_consonant() || (c == &&Phoneme::I) || (c == &&Phoneme::U)
                        } else {
                            c.is_vowel()
                        }
                    })
                    .for_each(|c| {
                        set.insert(c.clone());
                    });
            } else if len > c_len {
                let start = n as usize;
                let end = (n + len - c_len + 1) as usize;
                loan[start..end]
                    .iter()
                    .filter(|c| {
                        if n % 2 == 0 {
                            c.is_consonant() || (c == &&Phoneme::I) || (c == &&Phoneme::U)
                        } else {
                            c.is_vowel()
                        }
                    })
                    .for_each(|c| {
                        set.insert(c.clone());
                    });
            } else {
                set.insert(loan[n as usize]);
            }
        }
        set
    }

    pub fn generate(&mut self) {
        self.calc_weight_sum();
        let vec = vec![CandidateWord {
            score: 0.0,
            word: vec![],
        }];
        self.words = self.generate_rec(0, self.candidate_length(), vec);
    }

    fn generate_rec(&self, n: i32, len: i32, last_vec: Vec<CandidateWord>) -> Vec<CandidateWord> {
        if n >= len {
            last_vec
        } else if self.super_word.origins.len() == 1 {
            let mut vec = Vec::new();
            for origin in &self.super_word.origins {
                let loan = origin.loan.as_ref().unwrap();
                let ncw = CandidateWord {
                    score: self.score(&loan),
                    word: loan.clone(),
                };
                vec.push(ncw);
            }
            vec
        } else {
            let mut vec = Vec::new();
            let cps = self.cadidate_phonemes(n);
            for cw in last_vec {
                for cp in &cps {
                    let mut ncww = cw.word.clone();
                    ncww.push(*cp);
                    if ncww.is_match_w212()
                        && ncww.is_match_w213()
                        && ncww.is_match_w215()
                        && ((n == 0 && ncww.is_match_w214())
                            || (n == len - 1 && ncww.is_match_w209())
                            || (n != 0 && n != len - 1))
                    {
                        let ncw = CandidateWord {
                            score: self.score(&ncww),
                            word: ncww,
                        };
                        vec.push(ncw);
                    }
                }
            }
            vec.sort_unstable();
            vec.dedup();
            if vec.len() > self.limit {
                vec = vec.into_iter().take(self.limit).collect();
            }
            println!("n = {}, len = {}, set.len() = {}", n, len, vec.len());
            self.generate_rec(n + 1, len, vec)
        }
    }

    fn score(&self, word: &Vec<Phoneme>) -> f64 {
        if word.len() < 2 {
            0.0
        } else {
            let mut score = 0.0;
            for origin in &self.super_word.origins {
                let loanword = origin.loan.as_ref().unwrap();
                let language = &origin.language;
                'search: for i in (1..word.len()).rev() {
                    for j in 0..=word.len() - i {
                        let subword = &word[j..j + i];
                        if array_in_array(subword, &loanword) {
                            score += i as f64 * self.get_population(&language) / self.weight_sum
                                * (if i == 1 { 0.001 } else { 1.0 });
                            break 'search;
                        }
                    }
                }
            }
            score
        }
    }
}

pub fn array_in_array(needle: &[Phoneme], haystack: &[Phoneme]) -> bool {
    if needle.len() > haystack.len() {
        false
    } else {
        'outer: for i in 0..=(haystack.len() - needle.len()) {
            for j in 0..needle.len() {
                if needle[j] != haystack[i + j] {
                    continue 'outer;
                }
            }
            return true;
        }
        false
    }
}

#[test]
fn test_array_in_array() {
    assert_eq!(
        array_in_array(&vec![Phoneme::A, Phoneme::B], &vec![Phoneme::A, Phoneme::B]),
        true
    );
    assert_eq!(
        array_in_array(
            &vec![Phoneme::A, Phoneme::B, Phoneme::C],
            &vec![Phoneme::A, Phoneme::B]
        ),
        false
    );
    assert_eq!(
        array_in_array(
            &vec![Phoneme::A, Phoneme::B],
            &vec![Phoneme::C, Phoneme::A, Phoneme::B]
        ),
        true
    );
}
