use std::fs;

use crate::phoneme::Phoneme;

fn language_to_filename(language: &str) -> Option<String> {
    match language {
        "zh" => Some("cmn_hani_broad.tsv".into()),
        "en" => Some("eng_latn_uk_broad.tsv".into()),
        "es" => Some("spa_latn_ca_broad_filtered.tsv".into()),
        "hi" => Some("hin_deva_broad.tsv".into()),
        "bn" => Some("ben_beng_broad.tsv".into()),
        "pt" => Some("por_latn_bz_broad.tsv".into()),
        "ru" => Some("rus_cyrl_narrow.tsv".into()),
        "fr" => Some("fre_latn_broad.tsv".into()),
        "ar" => Some("ara_arab_broad.tsv".into()),
        "ja" => Some("jpn_hira_narrow.tsv".into()),
        "id" => Some("ind_latn_narrow.tsv".into()),
        "ur" => Some("urd_arab_broad.tsv".into()),
        "de" => Some("ger_latn_broad.tsv".into()),
        _ => None,
    }
}

pub fn to_ipa(word: &str, lang: &str) -> Option<String> {
    let dir = "./library/wikipron/data/scrape/tsv/";
    let filename = dir.to_string() + &language_to_filename(lang).unwrap();
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

fn ipa_to_phoneme(c: &char) -> Option<Phoneme> {
    match c {
        'p' => Some(Phoneme::P),
        'b' => Some(Phoneme::B),
        't' => Some(Phoneme::T),
        'd' => Some(Phoneme::D),
        'ʈ' => Some(Phoneme::T),
        'ɖ' => Some(Phoneme::D),
        'c' => Some(Phoneme::K),
        'ɟ' => Some(Phoneme::G),
        'k' => Some(Phoneme::K),
        'ɡ' => Some(Phoneme::G),
        'q' => Some(Phoneme::K),
        'ɢ' => Some(Phoneme::G),
        'm' => Some(Phoneme::M),
        'ɱ' => Some(Phoneme::M),
        'n' => Some(Phoneme::N),
        'ɳ' => Some(Phoneme::N),
        'ɲ' => Some(Phoneme::N),
        'ŋ' => Some(Phoneme::N),
        'ɴ' => Some(Phoneme::N),
        'ʙ' => Some(Phoneme::B),
        'r' => Some(Phoneme::R),
        'ʀ' => Some(Phoneme::R),
        'ⱱ' => Some(Phoneme::V),
        'ɾ' => Some(Phoneme::R),
        'ɽ' => Some(Phoneme::R),
        'ɸ' => Some(Phoneme::F),
        'β' => Some(Phoneme::V),
        'f' => Some(Phoneme::F),
        'v' => Some(Phoneme::V),
        'θ' => Some(Phoneme::S),
        'ð' => Some(Phoneme::Z),
        's' => Some(Phoneme::S),
        'z' => Some(Phoneme::Z),
        'ʃ' => Some(Phoneme::C),
        'ʒ' => Some(Phoneme::J),
        'ʂ' => Some(Phoneme::C),
        'ʐ' => Some(Phoneme::J),
        'ç' => Some(Phoneme::X),
        'ʝ' => Some(Phoneme::I),
        'x' => Some(Phoneme::X),
        'ɣ' => Some(Phoneme::G),
        'χ' => Some(Phoneme::X),
        'ʁ' => Some(Phoneme::R),
        'ħ' => Some(Phoneme::X),
        'ʕ' => Some(Phoneme::U),
        'ʜ' => Some(Phoneme::X),
        'ʢ' => Some(Phoneme::U),
        'h' => Some(Phoneme::H),
        'ɦ' => Some(Phoneme::X),
        'ʋ' => Some(Phoneme::W),
        'ɹ' => Some(Phoneme::R),
        'ɻ' => Some(Phoneme::R),
        'j' => Some(Phoneme::Y),
        'ɰ' => Some(Phoneme::W),
        'ɬ' => Some(Phoneme::C),
        'ɮ' => Some(Phoneme::J),
        'l' => Some(Phoneme::L),
        'ɭ' => Some(Phoneme::L),
        'ʎ' => Some(Phoneme::W),
        'ʟ' => Some(Phoneme::W),
        'ɺ' => Some(Phoneme::R),
        'ɕ' => Some(Phoneme::C),
        'ʑ' => Some(Phoneme::J),
        'ɧ' => Some(Phoneme::C),
        'ɫ' => Some(Phoneme::L),
        'ɥ' => Some(Phoneme::W),
        'ʍ' => Some(Phoneme::W),
        'w' => Some(Phoneme::W),
        'a' => Some(Phoneme::A),
        'ɑ' => Some(Phoneme::A),
        'ʌ' => Some(Phoneme::A),
        'æ' => Some(Phoneme::A),
        'ɐ' => Some(Phoneme::A),
        'ɶ' => Some(Phoneme::A),
        'ä' => Some(Phoneme::A),
        'ɛ' => Some(Phoneme::E),
        'e' => Some(Phoneme::E),
        'ø' => Some(Phoneme::E),
        'ɘ' => Some(Phoneme::E),
        'i' => Some(Phoneme::I),
        'y' => Some(Phoneme::I),
        'ɨ' => Some(Phoneme::I),
        'ɪ' => Some(Phoneme::I),
        'ʏ' => Some(Phoneme::I),
        'o' => Some(Phoneme::O),
        'ɔ' => Some(Phoneme::O),
        'ɵ' => Some(Phoneme::O),
        'ɤ' => Some(Phoneme::O),
        'ɒ' => Some(Phoneme::O),
        'u' => Some(Phoneme::U),
        'ɯ' => Some(Phoneme::U),
        'ʉ' => Some(Phoneme::U),
        'ʊ' => Some(Phoneme::U),
        'ə' => Some(Phoneme::SCHWA),
        _ => None,
    }
}

pub fn ipa_to_phonemes(word: &str) -> Vec<Phoneme> {
    use unicode_normalization::UnicodeNormalization;
    let word = word
        .nfd()
        .to_string()
        .replace("ʦ", "ts")
        .replace("ʣ", "dz")
        .replace("ʧ", "tʃ")
        .replace("ʤ", "dʒ")
        .replace("ʨ", "tɕ")
        .replace("ʥ", "dʑ");
    let mut alphabets = vec![];
    for c in word.chars() {
        match ipa_to_phoneme(&c) {
            Some(x) => alphabets.push(x),
            None => {}
        }
    }
    alphabets
}

pub fn phoneme_to_string(phoneme: &Phoneme) -> String {
    match phoneme {
        Phoneme::P => "p".into(),
        Phoneme::B => "b".into(),
        Phoneme::T => "t".into(),
        Phoneme::D => "d".into(),
        Phoneme::K => "k".into(),
        Phoneme::G => "g".into(),
        Phoneme::M => "m".into(),
        Phoneme::N => "n".into(),
        Phoneme::R => "r".into(),
        Phoneme::F => "f".into(),
        Phoneme::V => "v".into(),
        Phoneme::S => "s".into(),
        Phoneme::Z => "z".into(),
        Phoneme::C => "c".into(),
        Phoneme::J => "j".into(),
        Phoneme::X => "x".into(),
        Phoneme::H => "h".into(),
        Phoneme::Y => "y".into(),
        Phoneme::L => "l".into(),
        Phoneme::W => "w".into(),
        Phoneme::I => "i".into(),
        Phoneme::U => "u".into(),
        Phoneme::E => "e".into(),
        Phoneme::SCHWA => "-".into(),
        Phoneme::O => "o".into(),
        Phoneme::A => "a".into(),
    }
}

pub fn phonemes_to_loan(phonemes: &Vec<Phoneme>) -> String {
    phonemes
        .iter()
        .map(|p| phoneme_to_string(p))
        .collect::<Vec<String>>()
        .join("")
}

pub fn loan_to_phoneme(c: &char) -> Option<Phoneme> {
    match c {
        'p' => Some(Phoneme::P),
        'b' => Some(Phoneme::B),
        't' => Some(Phoneme::T),
        'd' => Some(Phoneme::D),
        'k' => Some(Phoneme::K),
        'g' => Some(Phoneme::G),
        'm' => Some(Phoneme::M),
        'n' => Some(Phoneme::N),
        'r' => Some(Phoneme::R),
        'ⱱ' => Some(Phoneme::V),
        'f' => Some(Phoneme::F),
        'v' => Some(Phoneme::V),
        's' => Some(Phoneme::S),
        'z' => Some(Phoneme::Z),
        'c' => Some(Phoneme::C),
        'j' => Some(Phoneme::Y),
        'x' => Some(Phoneme::X),
        'h' => Some(Phoneme::H),
        'y' => Some(Phoneme::Y),
        'l' => Some(Phoneme::L),
        'w' => Some(Phoneme::W),
        'a' => Some(Phoneme::A),
        'e' => Some(Phoneme::E),
        'i' => Some(Phoneme::I),
        'o' => Some(Phoneme::O),
        'u' => Some(Phoneme::U),
        '-' => Some(Phoneme::SCHWA),
        _ => None,
    }
}

pub fn loan_to_phonemes(word: &str) -> Vec<Phoneme> {
    let mut alphabets = vec![];
    for c in word.chars() {
        match loan_to_phoneme(&c) {
            Some(x) => alphabets.push(x),
            None => {}
        }
    }
    alphabets
}
