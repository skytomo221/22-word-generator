use std::{fs::File, io::BufReader};

use bacitit_word_generator::{recipe::Recipe, word_generator::WordGenerator};
use criterion::{criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
    let recipe_file = File::open("benches/bench.json").unwrap();
    let recipe_reader = BufReader::new(recipe_file);
    let recipe: Recipe = serde_json::from_reader(recipe_reader).unwrap();
    let recipe = recipe.complement();
    for super_word in recipe.super_words.clone() {
        let mut word_generator = WordGenerator {
            super_languages: &recipe.super_languages,
            super_word,
            words: Vec::new(),
            limit: 10000000,
            weight_sum: 0.0,
        };
        c.bench_function(
            &format!("generate {}", &word_generator.super_word.meaning),
            |b| b.iter(|| word_generator.generate()),
        );
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
