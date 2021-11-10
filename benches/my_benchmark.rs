use std::{fs::File, io::BufReader};

use bacitit_word_generator::{CandidateWords, Recipe, SuperWord, convert};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let recipe_file = File::open("benches/bench.json").unwrap();
    let recipe_reader = BufReader::new(recipe_file);
    let recipe: Recipe = serde_json::from_reader(recipe_reader).unwrap();
    let super_languages = recipe.super_languages;
    let super_words = recipe.super_words;
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
        c.bench_function(
            &format!("generate {}", &candidate_words.super_word.meaning),
            |b| b.iter(|| candidate_words.generate()),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
