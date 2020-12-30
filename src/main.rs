use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use std::fs::File;
use std::io::BufReader;

#[derive(Serialize, Deserialize, Debug)]
pub struct SuperLanguages {
    languages: HashMap<String, f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SuperLanguage {
    language: String,
    weight: f64,
}

fn main() {
    let file = File::open("data/language_weight.json").unwrap();
    let reader = BufReader::new(file);
    let super_languages: SuperLanguages = serde_json::from_reader(reader).unwrap();
    println!("{:?}", super_languages);
}
