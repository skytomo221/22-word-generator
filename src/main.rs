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
            || *self == 'l'
    }
}

pub trait StringExt {
    fn nth(&self, n: usize) -> Option<char>;
    fn rev_nth(&self, n: usize) -> Option<char>;
    fn is_match_w212(&self) -> bool;
    fn is_match_w209(&self) -> bool;
    fn is_match_w213(&self) -> bool;
    fn is_match_w214(&self) -> bool;
    fn is_match_w215(&self) -> bool;
}

impl StringExt for String {
    fn nth(&self, n: usize) -> Option<char> {
        self.chars().nth(n)
    }

    fn rev_nth(&self, n: usize) -> Option<char> {
        self.chars().rev().nth(n)
    }

    fn is_match_w212(&self) -> bool {
        match (self.rev_nth(1), self.rev_nth(0)) {
            (Some('i'), Some(_)) => true,
            (Some('u'), Some(_)) => true,
            (Some(a), Some(b)) => !(a.is_vowel() && b.is_vowel()),
            _ => true,
        }
    }

    fn is_match_w213(&self) -> bool {
        match (self.rev_nth(1), self.rev_nth(0)) {
            (Some(a), Some(b)) => !(a.is_consonant() && b.is_consonant()),
            _ => true,
        }
    }

    fn is_match_w214(&self) -> bool {
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

    fn is_match_w215(&self) -> bool {
        match (self.rev_nth(2), self.rev_nth(1), self.rev_nth(0)) {
            (Some('i'), Some('i'), Some('i')) => false,
            (Some('u'), Some('u'), Some('u')) => false,
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
    let mut generated = BTreeMap::new();
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
            let mut f = fs::File::create(format!("./export/dic/{}.md", best_word.word)).unwrap();
            f.write_all(output.as_bytes()).unwrap();
            generated.insert(
                best_word.word.clone(),
                candidate_words.super_word.meaning.clone(),
            );
        }
        {
            let mut f = fs::File::create("./export/word-list.md").unwrap();
            let output = {
                let mut s = "# Word List\n\n|Spell|Meaning|\n|:-:|:-:|\n".to_string();
                for x in &generated {
                    s.push_str(&format!("|[{0}](./dic/{0}.md)|{1}|\n", x.0, x.1));
                }
                s
            };
            f.write_all(output.as_bytes()).unwrap();
        }
    }
}
