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
            _ => None
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
