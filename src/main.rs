use bacitit_word_generator::{CandidateWords, Recipe, SuperWord};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufReader;
use std::{fs, io::Write};

use bacitit_word_generator::convert;

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
                    match Some(convert::to_latin(&origin.ipa.as_ref().unwrap())) {
                        Some(loan) => origin.loan = Some(loan),
                        None => panic!(
                            "借用語に変換できませんでした。 Word: {} Language: {} IPA: {:?}",
                            origin.word, origin.language, origin.ipa
                        ),
                    }
                }
                if origin.loan.as_ref().unwrap().contains("ə") {
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
                        origin.loan.as_ref().unwrap(),
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
            let mut f = fs::File::create(format!("./export/dic/{}.md", best_word.word)).unwrap();
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
            serde_json::to_writer_pretty(&File::create("./data/result.json").unwrap(), &recipe);
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
