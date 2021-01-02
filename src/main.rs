use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::{cmp, fs, io::Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuperLanguages {
    languages: BTreeMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuperWords {
    words: Vec<SuperWord>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SuperWord {
    id: usize,
    meaning: String,
    loans: BTreeMap<String, String>,
    origins: BTreeMap<String, String>,
    note: Option<String>,
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
    fn is_match_w211(&self) -> bool;
}

impl StringExt for String {
    fn nth(&self, n: usize) -> Option<char> {
        self.chars().nth(n)
    }

    fn rev_nth(&self, n: usize) -> Option<char> {
        self.chars().rev().nth(n)
    }

    fn is_match_w202_and_w203(&self) -> bool {
        match (self.rev_nth(1), self.rev_nth(0)) {
            (Some(a), Some(b)) => a != b,
            _ => true,
        }
    }

    fn is_match_w204(&self) -> bool {
        match (self.rev_nth(1), self.rev_nth(0)) {
            (Some(a), Some(b)) => {
                !(a.is_voiced() && b.is_unvoiced() || a.is_unvoiced() && b.is_voiced())
            }
            _ => true,
        }
    }

    fn is_match_w205(&self) -> bool {
        match (self.rev_nth(1), self.rev_nth(0)) {
            (Some(a), Some(b)) => {
                !(a.is_xhyw() && b.is_consonant() || a.is_consonant() && b.is_xhyw())
            }
            _ => true,
        }
    }

    fn is_match_w206(&self) -> bool {
        match (self.rev_nth(2), self.rev_nth(1), self.rev_nth(0)) {
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

    fn is_match_w211(&self) -> bool {
        match (self.rev_nth(2), self.rev_nth(1), self.rev_nth(0)) {
            (Some(a), Some(b), Some(c)) => !(a.is_vowel() && b.is_vowel() && c.is_vowel()),
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct CandidateWord {
    pub score: f64,
    pub word: String,
}

impl PartialEq for CandidateWord {
    fn eq(&self, other: &Self) -> bool {
        self.word == other.word
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

pub struct CandidateWords<'a> {
    pub super_languages: &'a SuperLanguages,
    pub super_word: SuperWord,
    pub words: Vec<CandidateWord>,
    pub limit: usize,
    pub weight_sum: f64,
}

impl CandidateWords<'_> {
    fn calc_weight_sum(&mut self) {
        self.weight_sum = self
            .super_languages
            .languages
            .iter()
            .filter(|(language, _)| self.super_word.loans.contains_key(*language))
            .map(|(_, score)| score)
            .sum()
    }

    fn cadidate_length(&self) -> i32 {
        let mut sum = 0.0;
        for (language, loan) in &self.super_word.loans {
            sum += loan.len() as f64 * self.super_languages.languages[language] / self.weight_sum;
        }
        sum.ceil() as i32
    }

    fn cadidate_phonemes(&self, n: i32) -> BTreeSet<char> {
        let c_len = self.cadidate_length();
        let mut set = BTreeSet::new();
        for (_, loan) in &self.super_word.loans {
            let len = loan.len() as i32;
            if len < c_len {
                let start = cmp::max(0, n - (c_len - len)) as usize;
                let end = cmp::min(len, n + 1) as usize;
                &loan[start..end].chars().for_each(|c| {
                    set.insert(c);
                });
            } else if len > c_len {
                let start = n as usize;
                let end = (n + len - c_len + 1) as usize;
                &loan[start..end].chars().for_each(|c| {
                    set.insert(c);
                });
            } else {
                if let Some(c) = loan.nth(n as usize) {
                    set.insert(c);
                }
            }
        }
        set
    }

    fn generate(&mut self) {
        self.calc_weight_sum();
        let vec = vec![CandidateWord {
            score: 0.0,
            word: "".to_string(),
        }];
        self.words = self.generate_rec(0, self.cadidate_length(), vec);
    }

    fn generate_rec(&self, n: i32, len: i32, last_vec: Vec<CandidateWord>) -> Vec<CandidateWord> {
        if n >= len {
            last_vec
        } else if self.super_word.loans.len() == 1 {
            let mut vec = Vec::new();
            for (_, loan) in &self.super_word.loans {
                let ncw = CandidateWord {
                    score: self.score(loan),
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
                    if ncww.is_match_w202_and_w203()
                        && ncww.is_match_w204()
                        && ncww.is_match_w205()
                        && ncww.is_match_w206()
                        && ncww.is_match_w207()
                        && ncww.is_match_w211()
                        && ((n == 0 && ncww.is_match_w208() && ncww.is_match_w210())
                            || (n == len - 1 && ncww.is_match_w209())
                            || (n != 0 && n != len - 1))
                    {
                        assert!(ncww.is_match_w202_and_w203());
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

    fn score(&self, word: &str) -> f64 {
        if word.len() < 2 {
            0.0
        } else {
            let mut score = 0.0;
            for (language, loanword) in &self.super_word.loans {
                'search: for i in (1..word.len()).rev() {
                    for j in 0..=word.len() - i {
                        let subword = &word[j..j + i];
                        if loanword.contains(subword) {
                            score += i as f64 * self.super_languages.languages[language]
                                / self.weight_sum
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

pub fn main() {
    let file1 = File::open("data/language_weight.json").unwrap();
    let file2 = File::open("data/super_words.json").unwrap();
    let reader1 = BufReader::new(file1);
    let reader2 = BufReader::new(file2);
    let super_languages: SuperLanguages = serde_json::from_reader(reader1).unwrap();
    let super_words: SuperWords = serde_json::from_reader(reader2).unwrap();
    println!("super_words.words.len() = {}", super_words.words.len());
    for super_word in super_words.words {
        let mut candidate_words = CandidateWords {
            super_languages: &super_languages,
            super_word: super_word,
            words: Vec::new(),
            limit: 100000,
            weight_sum: 0.0,
        };
        println!(
            "\nGenerating a word meaning '{}'...",
            candidate_words.super_word.meaning
        );
        candidate_words.generate();
        {
            let best_word = &candidate_words.words[0];
            let mut output = format!(
                "# {}\n\n## Meaning\n\n{}",
                best_word.word, candidate_words.super_word.meaning
            );
            let candidates_info = {
                let mut s = "|Word|Score|\n|:-:|:-:|\n".to_string();
                let b = candidate_words.words.iter().take(10);
                for c in b {
                    println!("{:?}", c);
                    s.push_str(&format!("|{}|{:.6}|\n", c.word, c.score));
                }
                s
            };
            let langs_info = {
                let mut s = "|ISO 639-1|Weight|Regular weight|Origin word|Loanword|\n|:-:|:-:|:-:|:-:|:-:|\n"
                    .to_string();
                for (language, origin) in candidate_words.super_word.origins {
                    s.push_str(&format!(
                        "|{}|{}|{:.4}|{}|{}|\n",
                        language,
                        candidate_words.super_languages.languages[&language],
                        candidate_words.super_languages.languages[&language]
                            / candidate_words.weight_sum,
                        origin,
                        candidate_words.super_word.loans[&language]
                    ));
                }
                s
            };
            output.push_str(&format!(
                "\n\n## Candidates\n\n{}\n## Origins\n\nWeight sum: {}\n{}",
                candidates_info, candidate_words.weight_sum, langs_info
            ));
            if let Some(note) = candidate_words.super_word.note {
                output.push_str(&format!("\n## Note\n\n{}\n", &note));
            }
            let mut f = fs::File::create(format!("./export/{}.md", best_word.word)).unwrap();
            f.write_all(output.as_bytes()).unwrap();
        }
    }
}
