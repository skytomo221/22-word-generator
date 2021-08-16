use std::fs;

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

fn ipa_to_alphabets(c: &char) -> Option<String> {
    match c {
        'p' => Some("p".into()),
        'b' => Some("b".into()),
        't' => Some("t".into()),
        'd' => Some("d".into()),
        'ʈ' => Some("t".into()),
        'ɖ' => Some("d".into()),
        'c' => Some("k".into()),
        'ɟ' => Some("g".into()),
        'k' => Some("k".into()),
        'ɡ' => Some("g".into()),
        'q' => Some("k".into()),
        'ɢ' => Some("g".into()),
        'm' => Some("m".into()),
        'ɱ' => Some("m".into()),
        'n' => Some("n".into()),
        'ɳ' => Some("n".into()),
        'ɲ' => Some("n".into()),
        'ŋ' => Some("n".into()),
        'ɴ' => Some("n".into()),
        'ʙ' => Some("b".into()),
        'r' => Some("r".into()),
        'ʀ' => Some("r".into()),
        'ⱱ' => Some("v".into()),
        'ɾ' => Some("r".into()),
        'ɽ' => Some("r".into()),
        'ɸ' => Some("f".into()),
        'β' => Some("v".into()),
        'f' => Some("f".into()),
        'v' => Some("v".into()),
        'θ' => Some("s".into()),
        'ð' => Some("z".into()),
        's' => Some("s".into()),
        'z' => Some("z".into()),
        'ʃ' => Some("c".into()),
        'ʒ' => Some("j".into()),
        'ʂ' => Some("c".into()),
        'ʐ' => Some("j".into()),
        'ç' => Some("x".into()),
        'ʝ' => Some("i".into()),
        'x' => Some("x".into()),
        'ɣ' => Some("g".into()),
        'χ' => Some("x".into()),
        'ʁ' => Some("r".into()),
        'ħ' => Some("x".into()),
        'ʕ' => Some("u".into()),
        'ʜ' => Some("x".into()),
        'ʢ' => Some("u".into()),
        'h' => Some("h".into()),
        'ɦ' => Some("x".into()),
        'ʋ' => Some("w".into()),
        'ɹ' => Some("r".into()),
        'ɻ' => Some("r".into()),
        'j' => Some("y".into()),
        'ɰ' => Some("w".into()),
        'ɬ' => Some("c".into()),
        'ɮ' => Some("j".into()),
        'l' => Some("l".into()),
        'ɭ' => Some("l".into()),
        'ʎ' => Some("w".into()),
        'ʟ' => Some("w".into()),
        'ɺ' => Some("r".into()),
        'ɕ' => Some("c".into()),
        'ʑ' => Some("j".into()),
        'ɧ' => Some("c".into()),
        'ɫ' => Some("l".into()),
        'ɥ' => Some("w".into()),
        'ʍ' => Some("w".into()),
        'w' => Some("w".into()),
        'a' => Some("a".into()),
        'ɑ' => Some("a".into()),
        'ʌ' => Some("a".into()),
        'æ' => Some("a".into()),
        'ɐ' => Some("a".into()),
        'ɶ' => Some("a".into()),
        'ä' => Some("a".into()),
        'ɛ' => Some("e".into()),
        'e' => Some("e".into()),
        'ø' => Some("e".into()),
        'ɘ' => Some("e".into()),
        'i' => Some("i".into()),
        'y' => Some("i".into()),
        'ɨ' => Some("i".into()),
        'ɪ' => Some("i".into()),
        'ʏ' => Some("i".into()),
        'o' => Some("o".into()),
        'ɔ' => Some("o".into()),
        'ɵ' => Some("o".into()),
        'ɤ' => Some("o".into()),
        'ɒ' => Some("o".into()),
        'u' => Some("u".into()),
        'ɯ' => Some("u".into()),
        'ʉ' => Some("u".into()),
        'ʊ' => Some("u".into()),
        'ə' => Some("ə".into()),
        'ʦ' => Some("ts".into()),
        'ʣ' => Some("dz".into()),
        'ʧ' => Some("tc".into()),
        'ʤ' => Some("dj".into()),
        'ʨ' => Some("tc".into()),
        'ʥ' => Some("dz".into()),
        _ => None,
    }
}

pub fn to_latin(word: &str) -> String {
    use unicode_normalization::UnicodeNormalization;
    let word = word.nfd().to_string();
    let mut latins = String::new();
    for c in word.chars() {
        match ipa_to_alphabets(&c) {
            Some(x) => latins += &x,
            None => {}
        }
    }
    latins
}
