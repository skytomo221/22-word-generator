use std::{collections::HashMap, fs};

fn language_to_filename() -> HashMap<&'static str, &'static str> {
    let mut filenames = HashMap::new();
    filenames.insert("zh", "cmn_hani_broad.tsv");
    filenames.insert("en", "eng_latn_uk_broad.tsv");
    filenames.insert("es", "spa_latn_ca_broad_filtered.tsv");
    filenames.insert("hi", "hin_deva_broad.tsv");
    filenames.insert("bn", "ben_beng_broad.tsv");
    filenames.insert("pt", "por_latn_bz_broad.tsv");
    filenames.insert("ru", "rus_cyrl_narrow.tsv");
    filenames.insert("fr", "fre_latn_broad.tsv");
    filenames.insert("ar", "ara_arab_broad.tsv");
    filenames.insert("ja", "jpn_hira_narrow.tsv");
    filenames.insert("id", "ind_latn_narrow.tsv");
    filenames.insert("ur", "urd_arab_broad.tsv");
    filenames.insert("de", "ger_latn_broad.tsv");
    filenames
}

pub fn to_ipa(word: &str, lang: &str) -> Option<String> {
    let dir = "./library/wikipron/data/scrape/tsv/";
    let filenames = language_to_filename();
    let filename = dir.to_string() + filenames.get(lang).unwrap();
    let dictionary = fs::read_to_string(filename).unwrap();
    for line in dictionary.lines() {
        let (orig, ipa) = {
            let mut iter = line.split("\t").map(|i| i.parse::<String>().unwrap());
            (iter.next().unwrap(), iter.next().unwrap().replace(" ", ""))
        };
        if orig == word {
            return Some(ipa);
        }
    }
    None
}

fn ipa_to_alphabets() -> HashMap<&'static str, &'static str> {
    let mut alphabets = HashMap::new();
    alphabets.insert("p", "p");
    alphabets.insert("b", "b");
    alphabets.insert("t", "t");
    alphabets.insert("d", "d");
    alphabets.insert("ʈ", "t");
    alphabets.insert("ɖ", "d");
    alphabets.insert("c", "k");
    alphabets.insert("ɟ", "g");
    alphabets.insert("k", "k");
    alphabets.insert("ɡ", "g");
    alphabets.insert("q", "k");
    alphabets.insert("ɢ", "g");
    alphabets.insert("m", "m");
    alphabets.insert("ɱ", "m");
    alphabets.insert("n", "n");
    alphabets.insert("ɳ", "n");
    alphabets.insert("ɲ", "n");
    alphabets.insert("ŋ", "n");
    alphabets.insert("ɴ", "n");
    alphabets.insert("ʙ", "b");
    alphabets.insert("r", "r");
    alphabets.insert("ʀ", "r");
    alphabets.insert("ⱱ", "v");
    alphabets.insert("ɾ", "r");
    alphabets.insert("ɽ", "r");
    alphabets.insert("ɸ", "f");
    alphabets.insert("β", "v");
    alphabets.insert("f", "f");
    alphabets.insert("v", "v");
    alphabets.insert("θ", "s");
    alphabets.insert("ð", "z");
    alphabets.insert("s", "s");
    alphabets.insert("z", "z");
    alphabets.insert("ʃ", "c");
    alphabets.insert("ʒ", "j");
    alphabets.insert("ʂ", "c");
    alphabets.insert("ʐ", "j");
    alphabets.insert("ç", "x");
    alphabets.insert("ʝ", "i");
    alphabets.insert("x", "x");
    alphabets.insert("ɣ", "g");
    alphabets.insert("χ", "x");
    alphabets.insert("ʁ", "r");
    alphabets.insert("ħ", "x");
    alphabets.insert("ʕ", "u");
    alphabets.insert("ʜ", "x");
    alphabets.insert("ʢ", "u");
    alphabets.insert("h", "h");
    alphabets.insert("ɦ", "x");
    alphabets.insert("ʋ", "w");
    alphabets.insert("ɹ", "r");
    alphabets.insert("ɻ", "r");
    alphabets.insert("j", "y");
    alphabets.insert("ɰ", "w");
    alphabets.insert("ɬ", "c");
    alphabets.insert("ɮ", "j");
    alphabets.insert("l", "l");
    alphabets.insert("ɭ", "l");
    alphabets.insert("ʎ", "w");
    alphabets.insert("ʟ", "w");
    alphabets.insert("ɺ", "r");
    alphabets.insert("ɕ", "c");
    alphabets.insert("ʑ", "j");
    alphabets.insert("ɧ", "c");
    alphabets.insert("ɫ", "l");
    alphabets.insert("ɥ", "w");
    alphabets.insert("ʍ", "w");
    alphabets.insert("w", "w");
    alphabets.insert("a", "a");
    alphabets.insert("ɑ", "a");
    alphabets.insert("ʌ", "a");
    alphabets.insert("æ", "a");
    alphabets.insert("ɐ", "a");
    alphabets.insert("ɶ", "a");
    alphabets.insert("ä", "a");
    alphabets.insert("ɛ", "e");
    alphabets.insert("e", "e");
    alphabets.insert("ø", "e");
    alphabets.insert("ɘ", "e");
    alphabets.insert("i", "i");
    alphabets.insert("y", "i");
    alphabets.insert("ɨ", "i");
    alphabets.insert("ɪ", "i");
    alphabets.insert("ʏ", "i");
    alphabets.insert("ɪ̈", "i");
    alphabets.insert("o", "o");
    alphabets.insert("ɔ", "o");
    alphabets.insert("ɵ", "o");
    alphabets.insert("ɤ", "o");
    alphabets.insert("ɒ", "o");
    alphabets.insert("u", "u");
    alphabets.insert("ɯ", "u");
    alphabets.insert("ʉ", "u");
    alphabets.insert("ʊ̈", "u");
    alphabets.insert("ɯ̽", "u");
    alphabets.insert("ʊ", "u");
    alphabets.insert("ə", "ə");
    alphabets.insert("ʦ", "ts");
    alphabets.insert("ʣ", "dz");
    alphabets.insert("ʧ", "tc");
    alphabets.insert("ʤ", "dj");
    alphabets.insert("ʨ", "tc");
    alphabets.insert("ʥ", "dz");
    alphabets
}

pub fn to_latin(word: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    let ipa_to_alphabets = ipa_to_alphabets();
    let word = word.nfd().to_string();
    let mut latins = String::new();
    for c in word.chars() {
        match ipa_to_alphabets.get(c.to_string().as_str()) {
            Some(x) => latins += x,
            None => {}
        }
    }
    latins
}
