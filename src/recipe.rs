use serde::de::{self, Deserializer, MapAccess, SeqAccess, Visitor};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::convert;
use crate::phoneme::Phoneme;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    pub super_languages: Vec<SuperLanguage>,
    pub super_words: Vec<SuperWord>,
}

impl Recipe {
    pub fn complement(self) -> Self {
        Recipe {
            super_words: self
                .super_words
                .iter()
                .map(|super_word| super_word.clone().complement())
                .collect(),
            ..self
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuperLanguage {
    pub language: String,
    pub population: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SuperWord {
    pub id: usize,
    pub meaning: String,
    pub origins: Vec<Origin>,
}

impl SuperWord {
    pub fn complement(self) -> Self {
        SuperWord {
            origins: self
                .origins
                .iter()
                .map(|origin| {
                    origin
                        .clone()
                        .complement_ipa()
                        .complement_loan()
                        .check_complement()
                })
                .collect(),
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub struct Origin {
    pub language: String,
    pub word: String,
    pub ipa: Option<String>,
    pub loan: Option<Vec<Phoneme>>,
}

impl Origin {
    pub fn complement_ipa(self) -> Self {
        match self.ipa {
            Some(_) => self,
            None => match convert::to_ipa(&self.word, &self.language) {
                Some(ipa) => Origin {
                    ipa: Some(ipa),
                    ..self.clone()
                },
                None => panic!(
                    "IPAに変換できませんでした。 Word: {} Language: {}",
                    self.word, self.language
                ),
            },
        }
    }

    pub fn complement_loan(self) -> Self {
        match self.loan {
            Some(_) => self,
            None => match Some(convert::ipa_to_phonemes(&self.ipa.as_ref().unwrap())) {
                Some(loan) => Origin {
                    loan: Some(loan),
                    ..self
                },
                None => panic!(
                    "借用語に変換できませんでした。 Word: {} Language: {} IPA: {:?}",
                    self.word, self.language, self.ipa
                ),
            },
        }
    }

    fn contains_schwa(&self) -> bool {
        self.loan.as_ref().unwrap().contains(&Phoneme::SCHWA)
    }

    pub fn check_complement(self) -> Self {
        if self.contains_schwa() {
            panic!(
                "əが含まれています。 Word: {} Language: {} IPA: {:?}, loan {:?}",
                self.word, self.language, self.ipa, self.loan
            );
        }
        self
    }
}

impl Serialize for Origin {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Origin", 4)?;
        state.serialize_field("language", &self.language)?;
        state.serialize_field("word", &self.word)?;
        state.serialize_field("ipa", &self.ipa)?;
        state.serialize_field(
            "loan",
            &match &self.loan {
                Some(a) => Some(convert::phonemes_to_loan(a)),
                None => None,
            },
        )?;
        state.end()
    }
}

impl<'de> serde::de::Deserialize<'de> for Origin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field {
            Language,
            Word,
            IPA,
            Loan,
        }

        impl<'de> serde::de::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`language`, `word`, `ipa` or `loan`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "language" => Ok(Field::Language),
                            "word" => Ok(Field::Word),
                            "ipa" => Ok(Field::IPA),
                            "loan" => Ok(Field::Loan),
                            _ => Err(de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct DurationVisitor;

        impl<'de> Visitor<'de> for DurationVisitor {
            type Value = Origin;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Duration")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Origin, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let language = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                let word = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;
                let ipa = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;
                let loan = seq
                    .next_element()?
                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;
                Ok(Origin {
                    language,
                    word,
                    ipa,
                    loan,
                })
            }

            fn visit_map<V>(self, mut map: V) -> Result<Origin, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut language = None;
                let mut word = None;
                let mut ipa = None;
                let mut loan = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Language => {
                            if language.is_some() {
                                return Err(de::Error::duplicate_field("language"));
                            }
                            language = Some(map.next_value()?);
                        }
                        Field::Word => {
                            if word.is_some() {
                                return Err(de::Error::duplicate_field("word"));
                            }
                            word = Some(map.next_value()?);
                        }
                        Field::IPA => {
                            if ipa.is_some() {
                                return Err(de::Error::duplicate_field("ipa"));
                            }
                            ipa = Some(map.next_value()?);
                        }
                        Field::Loan => {
                            if loan.is_some() {
                                return Err(de::Error::duplicate_field("loan"));
                            }
                            loan = Some(match map.next_value::<Option<String>>()? {
                                Some(s) => Some(convert::loan_to_phonemes(&s)),
                                None => None,
                            });
                        }
                    }
                }
                let language = language.ok_or_else(|| de::Error::missing_field("language"))?;
                let word = word.ok_or_else(|| de::Error::missing_field("word"))?;
                let ipa = match ipa {
                    Some(s) => Some(s),
                    None => None,
                };
                let loan = match loan {
                    Some(s) => s,
                    None => None,
                };
                Ok(Origin {
                    language,
                    word,
                    ipa,
                    loan,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["language", "word", "ipa", "loan"];
        deserializer.deserialize_struct("Duration", FIELDS, DurationVisitor)
    }
}
