use core::fmt;

use serde::{
    de::Visitor,
    ser::{Serialize, Serializer},
    Deserialize, Deserializer,
};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Language {
    Chinese,
    English,
    Spanish,
    Hindi,
    Portuguese,
    Russian,
    French,
    Arabic,
    Japanese,
    Indonesian,
    Urdu,
    German,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Language::Chinese => "zh",
                Language::English => "en",
                Language::Spanish => "es",
                Language::Hindi => "hi",
                Language::Portuguese => "pt",
                Language::Russian => "ru",
                Language::French => "fr",
                Language::Arabic => "ar",
                Language::Japanese => "ja",
                Language::Indonesian => "id",
                Language::Urdu => "ur",
                Language::German => "de",
            }
        )
    }
}

pub trait StringExt {
    fn iso_639(&self) -> Option<Language>;
}

impl StringExt for String {
    fn iso_639(&self) -> Option<Language> {
        match self.as_str() {
            "zh" => Some(Language::Chinese),
            "en" => Some(Language::English),
            "es" => Some(Language::Spanish),
            "hi" => Some(Language::Hindi),
            "pt" => Some(Language::Portuguese),
            "ru" => Some(Language::Russian),
            "fr" => Some(Language::French),
            "ar" => Some(Language::Arabic),
            "ja" => Some(Language::Japanese),
            "id" => Some(Language::Indonesian),
            "ur" => Some(Language::Urdu),
            "de" => Some(Language::German),
            _ => None,
        }
    }
}

impl Serialize for Language {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

struct PhonemeVisitor;

impl<'de> Visitor<'de> for PhonemeVisitor {
    type Value = Language;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a phoneme")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        v.to_owned()
            .iso_639()
            .ok_or(serde::de::Error::invalid_value(
                serde::de::Unexpected::Str(v),
                &"a Phoneme",
            ))
    }
}

impl<'de> Deserialize<'de> for Language {
    fn deserialize<D>(deserializer: D) -> Result<Language, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_string(PhonemeVisitor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn language_test() {
        assert_eq!(Language::Chinese.to_string(), "zh");
        assert_eq!("zh".to_string().iso_639().unwrap(), Language::Chinese);
    }

    use crate::convert::{ipa_to_phonemes, phonemes_to_loan};
    #[test]
    fn test_language_to_latin() {
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ˈlæŋɡwɪd͡ʒ")), "langwidj");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ˈlẽŋ.ɡwa")), "lengwa");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("bʱɑː.ʂɑː")), "baca");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("lĩ.ɡwɐ")), "ligwa");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("(j)ɪˈzɨk")), "yizik");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("lɑ̃ɡ")), "lag");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ɡẽ̞ŋɡo̞")), "gengo");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("baˈhasa")), "bahasa");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("zʊ.bɑːn")), "zuban");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ˈʃpʁaːxə")), "cprax-");
    }

    #[test]
    fn test_cat_to_latin() {
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("māo")), "mao");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("kæt")), "kat");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ˈɡa.t̪o")), "gato");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("bɪl̪.l̪iː")), "billi");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("biṛal")), "biral");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ˈɡä.t̪ʊ")), "gatu");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ˈkoʂkə")), "kock-");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ʃa")), "ca");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("qiṭṭ")), "kitt");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ne̞ko̞")), "neko");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("kut͡ʃɪŋ")), "kutcin");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("billī")), "billi");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ˈkatsə")), "kats-");
    }

    #[test]
    fn test_to_latin() {
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("ʃpʁaːxə")), "cprax-");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("koʂkə")), "kock-");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("qitˤtˤ")), "kitt");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("jabɫəkə")), "yabl-k-");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("møːɡən")), "meg-n");
        assert_eq!(phonemes_to_loan(&ipa_to_phonemes("knʲiɡə")), "knig-");
    }
}
