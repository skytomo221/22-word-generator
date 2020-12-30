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
