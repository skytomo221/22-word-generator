use bacitit_word_generator::convert;
use bacitit_word_generator::phoneme::{Phoneme, PhonemeExt};
use bacitit_word_generator::phonotactics::PhonotacticsExt;
use bacitit_word_generator::recipe::{Recipe, SuperLanguage, SuperWord};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::BufReader;
use std::{fs, io::Write};

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

pub struct CandidateWords<'a> {
    pub super_languages: &'a Vec<SuperLanguage>,
    pub super_word: SuperWord,
    pub words: Vec<CandidateWord>,
    pub limit: usize,
    pub weight_sum: f64,
}

impl CandidateWords<'_> {
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

    fn get_population(&self, language: &str) -> f64 {
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
                loan.iter()
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
                loan.iter()
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

    fn generate(&mut self) {
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

pub fn main() {
    let recipe_file = File::open("data/recipe.json").unwrap();
    let recipe_reader = BufReader::new(recipe_file);
    let recipe: Recipe = serde_json::from_reader(recipe_reader).unwrap();
    let super_languages = recipe.super_languages;
    let super_words = recipe.super_words;
    let mut generated = BTreeMap::new();
    println!("super_words.words.len() = {}", super_words.len());
    let super_words = {
        let mut new_super_words: Vec<SuperWord> = Vec::new();
        for super_word in super_words {
            let mut new_super_word = super_word;
            for origin in &mut new_super_word.origins {
                if origin.ipa == None {
                    match convert::to_ipa(&origin.word, &origin.language) {
                        Some(ipa) => origin.ipa = Some(ipa),
                        None => panic!(
                            "IPAに変換できませんでした。 Word: {} Language: {}",
                            origin.word, origin.language
                        ),
                    }
                }
                if origin.loan == None {
                    match Some(convert::ipa_to_phonemes(&origin.ipa.as_ref().unwrap())) {
                        Some(loan) => origin.loan = Some(loan),
                        None => panic!(
                            "借用語に変換できませんでした。 Word: {} Language: {} IPA: {:?}",
                            origin.word, origin.language, origin.ipa
                        ),
                    }
                }
                if origin.loan.as_ref().unwrap().contains(&Phoneme::SCHWA) {
                    panic!(
                        "əが含まれています。 Word: {} Language: {} IPA: {:?}, loan {:?}",
                        origin.word, origin.language, origin.ipa, origin.loan
                    );
                }
            }
            new_super_words.push(new_super_word);
        }
        new_super_words
    };
    for super_word in super_words.clone() {
        let mut candidate_words = CandidateWords {
            super_languages: &super_languages,
            super_word,
            words: Vec::new(),
            limit: 1000000,
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
                convert::phonemes_to_loan(&best_word.word),
                candidate_words.super_word.meaning
            );
            let candidates_info = {
                let mut s = "|Word|Score|\n|:-:|:-:|\n".to_string();
                let b = candidate_words.words.iter().take(10);
                for c in b {
                    println!("{:?}", c);
                    s.push_str(&format!(
                        "|{}|{:.6}|\n",
                        convert::phonemes_to_loan(&c.word),
                        c.score
                    ));
                }
                s
            };
            let langs_info = {
                let mut s = "|ISO 639-1|Weight|Regular weight|Origin word|IPA|Loanword|\n|:-:|:-:|:-:|:-:|:-:|:-:|\n"
                    .to_string();
                for origin in &candidate_words.super_word.origins {
                    let language = &origin.language;
                    s.push_str(&format!(
                        "|{}|{}|{:.4}|{}|{}|{}|\n",
                        language,
                        candidate_words.get_population(&language),
                        candidate_words.get_population(&language) / candidate_words.weight_sum,
                        origin.word,
                        origin.ipa.as_ref().unwrap(),
                        convert::phonemes_to_loan(origin.loan.as_ref().unwrap()),
                    ));
                }
                s
            };
            output.push_str(&format!(
                "\n\n## Candidates\n\n{}\n## Origins\n\nWeight sum: {}\n{}",
                candidates_info, candidate_words.weight_sum, langs_info
            ));
            // if let Some(note) = candidate_words.super_word.note {
            //     output.push_str(&format!("\n## Note\n\n{}\n", &note));
            // }
            let mut f = fs::File::create(format!(
                "./export/dic/{}.md",
                convert::phonemes_to_loan(&best_word.word)
            ))
            .unwrap();
            f.write_all(output.as_bytes()).unwrap();
            generated.insert(
                best_word.word.clone(),
                candidate_words.super_word.meaning.clone(),
            );
        }
        {
            let recipe = Recipe {
                super_languages: super_languages.clone(),
                super_words: super_words.clone(),
            };
            serde_json::to_writer_pretty(&File::create("./data/result.json").unwrap(), &recipe)
                .unwrap();
        }
        {
            let mut f = fs::File::create("./export/word-list.md").unwrap();
            let output = {
                let mut s = "# Word List\n\n|Spell|Meaning|\n|:-:|:-:|\n".to_string();
                for x in &generated {
                    s.push_str(&format!(
                        "|[{0}](./dic/{0}.md)|{1}|\n",
                        convert::phonemes_to_loan(x.0),
                        x.1
                    ));
                }
                s
            };
            f.write_all(output.as_bytes()).unwrap();
        }
    }
}
