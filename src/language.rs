/*
 * Copyright © 2020-present Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter, Result};
use std::hash::BuildHasher;

use ahash::AHashSet;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

use crate::alphabet::Script;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use crate::ExtraCheck;

/// This enum specifies the so far 75 supported languages which can be detected by *Lingua*.
#[derive(
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
    Eq,
    PartialEq,
    Hash,
    Ord,
    PartialOrd,
)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
#[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord, rename_all = "UPPERCASE")
)]
pub enum Language {
    Afrikaans,
    Ahom,
    Akkadian,
    Albanian,
    AlbanianHistorical,
    Amharic,
    AncientGreek,
    Arabic,
    Aramaic,
    Armenian,
    Assamese,
    Avestan,
    Azerbaijani,
    Balinese,
    Bamum,
    Basque,
    Bassa,
    Belarusian,
    Bengali,
    Berber,
    Bhaiksuki,
    Bhojpuri,
    BishnupriyaManipuri,
    Bokmal,
    Bosnian,
    Buginese,
    Buhid,
    Bulgarian,
    Burmese,
    Carian,
    Catalan,
    CaucasianAlbanian,
    Chakma,
    Cham,
    Cherokee,
    Chinese,
    Chorasmian,
    Coptic,
    Croatian,
    Czech,
    Danish,
    Dogri,
    Dutch,
    EgyptianAncient,
    Elymaic,
    English,
    EnglishMormon,
    EnglishPhonetic,
    Esperanto,
    Estonian,
    Etruscan,
    Finnish,
    French,
    Fulani,
    Ganda,
    Gandhari,
    Geez,
    Georgian,
    German,
    Gondi,
    Gothic,
    Greek,
    Gujarati,
    Gurung,
    Hanunoo,
    HatranAramaic,
    Hebrew,
    Hindi,
    Hittite,
    Hmong,
    HmongMiao,
    Ho,
    Hungarian,
    HungarianOld,
    Icelandic,
    Indonesian,
    Irish,
    Italian,
    Japanese,
    JapaneseKanji,
    Javanese,
    Kannada,
    Kashmiri,
    KayahLi,
    Kazakh,
    Khitan,
    Khmer,
    Khoja,
    KiratRai,
    Komi,
    Korean,
    KoreanHanja,
    Kurdish,
    KurdishYazidi,
    Lao,
    Latin,
    Latvian,
    Lepcha,
    Limbu,
    Lisu,
    Lithuanian,
    Luwian,
    Lycian,
    Lydian,
    Macedonian,
    Magahi,
    Maithili,
    Makasar,
    Makassarese,
    Malay,
    Malayalam,
    MaldivianDhivehi,
    Mandaic,
    MandarinChinese,
    ManipuriMeetei,
    Maori,
    Marathi,
    MarchenBuddhist,
    Marwari,
    Medefaidrin,
    Mende,
    Meroitic,
    MiddlePersian,
    Mongolian,
    Mro,
    Mundari,
    MycenaeanGreek,
    NabataeanAramaic,
    Nepali,
    Newari,
    NKoMandé,
    NorthernThai,
    NushuChina,
    Nynorsk,
    OldChurchSlavonic,
    OldEnglish,
    OldIrish,
    OldJavanese,
    OldNorse,
    OldNorthArabian,
    OldPersian,
    OldSouthArabian,
    OldTurkic,
    OldUyghur,
    OriyaOdia,
    Oromo,
    Osage,
    Oscan,
    PalmyreneAramaic,
    Parthian,
    Pashto,
    PauCinHauChin,
    Persian,
    Phoenician,
    Polish,
    Portuguese,
    Prakrit,
    Pular,
    Punjabi,
    Rejang,
    Rohingya,
    Romanian,
    Russian,
    SamaritanHebrew,
    Sanskrit,
    Santali,
    Saraiki,
    Saurashtra,
    Serbian,
    Shan,
    Shona,
    Signlanguages,
    Sindhi,
    Sinhala,
    Slovak,
    Slovene,
    Sogdian,
    Somali,
    Sora,
    Sotho,
    Spanish,
    Sumerian,
    Sundanese,
    Sunuwar,
    Swahili,
    Swedish,
    Sylheti,
    Syriac,
    Tagalog,
    Tagbanwa,
    TaiDam,
    TaiDón,
    TaiLe,
    TaiLue,
    Tamil,
    Tangsa,
    Tangut,
    Telugu,
    Thai,
    Tibetan,
    Tigrinya,
    Toto,
    Tsonga,
    Tswana,
    Tulu,
    Turkish,
    Ugaritic,
    Ukrainian,
    Umbrian,
    Urdu,
    Uyghur,
    Vai,
    Vietnamese,
    Wancho,
    Welsh,
    Wolof,
    Xhosa,
    Yi,
    Yiddish,
    Yoruba,
    Zulu,
}

impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr)
    }
}

impl ExtraCheck for Language {
    #[inline]
    fn modif_opt<S: BuildHasher>(_lang_counts: &mut HashMap<Option<Self>, usize, S>) {
        /* if cfg!(feature = "chinese") && cfg!(feature = "japanese") {
            let Some(j) = lang_counts
                .get_mut(&Some(Language::Japanese))
                .map(|j| j as *mut usize)
            else {
                return;
            };
            if let Some(c) = lang_counts.remove(&Some(Language::Chinese)) {
                unsafe {
                    *j += c;
                }
            }
        } */
    }
    #[inline]
    fn modif<S: BuildHasher>(_lang_counts: &mut HashMap<Self, usize, S>) {
        /* if cfg!(feature = "chinese") && cfg!(feature = "japanese") {
            let Some(j) = lang_counts
                .get_mut(&Language::Japanese)
                .map(|j| j as *mut usize)
            else {
                return;
            };
            if let Some(c) = lang_counts.remove(&Language::Chinese) {
                unsafe {
                    *j += c;
                }
            }
        } */
    }
}

impl Language {
    /// Returns a set of all supported languages.
    pub fn all() -> AHashSet<Language> {
        Language::iter().collect()
    }

    /// Returns a set of all supported spoken languages.
    pub fn all_spoken_ones() -> AHashSet<Language> {
        Language::iter()
            .filter(|it| {
                if cfg!(feature = "latin") {
                    it != &Language::Latin
                } else {
                    true
                }
            })
            .collect()
    }

    /// Returns a set of all languages supporting the Arabic script.
    pub fn all_with_arabic_script() -> AHashSet<Language> {
        Language::iter()
            .filter(|it| it.scripts().contains(&Script::Arabic))
            .collect()
    }

    /// Returns a set of all languages supporting the Cyrillic script.
    pub fn all_with_cyrillic_script() -> AHashSet<Language> {
        Language::iter()
            .filter(|it| it.scripts().contains(&Script::Cyrillic))
            .collect()
    }

    /// Returns a set of all languages supporting the Devanagari script.
    pub fn all_with_devanagari_script() -> AHashSet<Language> {
        Language::iter()
            .filter(|it| it.scripts().contains(&Script::Devanagari))
            .collect()
    }

    /// Returns a set of all languages supporting the Latin script.
    pub fn all_with_latin_script() -> AHashSet<Language> {
        Language::iter()
            .filter(|it| it.scripts().contains(&Script::Latin))
            .collect()
    }

    /// Returns the language associated with the ISO 639-1 code
    /// passed to this method.
    pub fn from_iso_code_639_1(iso_code: &IsoCode639_1) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_1() == iso_code)
            .unwrap()
    }

    /// Returns the language associated with the ISO 639-3 code
    /// passed to this method.
    pub fn from_iso_code_639_3(iso_code: &IsoCode639_3) -> Language {
        Language::iter()
            .find(|it| &it.iso_code_639_3() == iso_code)
            .unwrap()
    }

    /// Returns the ISO 639-1 code of this language.
    pub fn iso_code_639_1(&self) -> IsoCode639_1 {
        match self {
            #[cfg(feature = "afrikaans")]
            Language::Afrikaans => IsoCode639_1::AF,

            #[cfg(feature = "albanian")]
            Language::Albanian => IsoCode639_1::SQ,

            #[cfg(feature = "arabic")]
            Language::Arabic => IsoCode639_1::AR,

            #[cfg(feature = "armenian")]
            Language::Armenian => IsoCode639_1::HY,

            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => IsoCode639_1::AZ,

            #[cfg(feature = "basque")]
            Language::Basque => IsoCode639_1::EU,

            #[cfg(feature = "belarusian")]
            Language::Belarusian => IsoCode639_1::BE,

            #[cfg(feature = "bengali")]
            Language::Bengali => IsoCode639_1::BN,

            #[cfg(feature = "bokmal")]
            Language::Bokmal => IsoCode639_1::NB,

            #[cfg(feature = "bosnian")]
            Language::Bosnian => IsoCode639_1::BS,

            #[cfg(feature = "bulgarian")]
            Language::Bulgarian => IsoCode639_1::BG,

            #[cfg(feature = "catalan")]
            Language::Catalan => IsoCode639_1::CA,

            #[cfg(feature = "chinese")]
            Language::Chinese => IsoCode639_1::ZH,

            #[cfg(feature = "croatian")]
            Language::Croatian => IsoCode639_1::HR,

            #[cfg(feature = "czech")]
            Language::Czech => IsoCode639_1::CS,

            #[cfg(feature = "danish")]
            Language::Danish => IsoCode639_1::DA,

            #[cfg(feature = "dutch")]
            Language::Dutch => IsoCode639_1::NL,

            #[cfg(feature = "english")]
            Language::English => IsoCode639_1::EN,

            #[cfg(feature = "esperanto")]
            Language::Esperanto => IsoCode639_1::EO,

            #[cfg(feature = "estonian")]
            Language::Estonian => IsoCode639_1::ET,

            #[cfg(feature = "finnish")]
            Language::Finnish => IsoCode639_1::FI,

            #[cfg(feature = "french")]
            Language::French => IsoCode639_1::FR,

            #[cfg(feature = "ganda")]
            Language::Ganda => IsoCode639_1::LG,

            #[cfg(feature = "georgian")]
            Language::Georgian => IsoCode639_1::KA,

            #[cfg(feature = "german")]
            Language::German => IsoCode639_1::DE,

            #[cfg(feature = "greek")]
            Language::Greek => IsoCode639_1::EL,

            #[cfg(feature = "gujarati")]
            Language::Gujarati => IsoCode639_1::GU,

            #[cfg(feature = "hebrew")]
            Language::Hebrew => IsoCode639_1::HE,

            #[cfg(feature = "hindi")]
            Language::Hindi => IsoCode639_1::HI,

            #[cfg(feature = "hungarian")]
            Language::Hungarian => IsoCode639_1::HU,

            #[cfg(feature = "icelandic")]
            Language::Icelandic => IsoCode639_1::IS,

            #[cfg(feature = "indonesian")]
            Language::Indonesian => IsoCode639_1::ID,

            #[cfg(feature = "irish")]
            Language::Irish => IsoCode639_1::GA,

            #[cfg(feature = "italian")]
            Language::Italian => IsoCode639_1::IT,

            #[cfg(feature = "japanese")]
            Language::Japanese => IsoCode639_1::JA,

            #[cfg(feature = "kazakh")]
            Language::Kazakh => IsoCode639_1::KK,

            #[cfg(feature = "korean")]
            Language::Korean => IsoCode639_1::KO,

            #[cfg(feature = "latin")]
            Language::Latin => IsoCode639_1::LA,

            #[cfg(feature = "latvian")]
            Language::Latvian => IsoCode639_1::LV,

            #[cfg(feature = "lithuanian")]
            Language::Lithuanian => IsoCode639_1::LT,

            #[cfg(feature = "macedonian")]
            Language::Macedonian => IsoCode639_1::MK,

            #[cfg(feature = "malay")]
            Language::Malay => IsoCode639_1::MS,

            #[cfg(feature = "maori")]
            Language::Maori => IsoCode639_1::MI,

            #[cfg(feature = "marathi")]
            Language::Marathi => IsoCode639_1::MR,

            #[cfg(feature = "mongolian")]
            Language::Mongolian => IsoCode639_1::MN,

            #[cfg(feature = "nynorsk")]
            Language::Nynorsk => IsoCode639_1::NN,

            #[cfg(feature = "persian")]
            Language::Persian => IsoCode639_1::FA,

            #[cfg(feature = "polish")]
            Language::Polish => IsoCode639_1::PL,

            #[cfg(feature = "portuguese")]
            Language::Portuguese => IsoCode639_1::PT,

            #[cfg(feature = "punjabi")]
            Language::Punjabi => IsoCode639_1::PA,

            #[cfg(feature = "romanian")]
            Language::Romanian => IsoCode639_1::RO,

            #[cfg(feature = "russian")]
            Language::Russian => IsoCode639_1::RU,

            #[cfg(feature = "serbian")]
            Language::Serbian => IsoCode639_1::SR,

            #[cfg(feature = "shona")]
            Language::Shona => IsoCode639_1::SN,

            #[cfg(feature = "slovak")]
            Language::Slovak => IsoCode639_1::SK,

            #[cfg(feature = "slovene")]
            Language::Slovene => IsoCode639_1::SL,

            #[cfg(feature = "somali")]
            Language::Somali => IsoCode639_1::SO,

            #[cfg(feature = "sotho")]
            Language::Sotho => IsoCode639_1::ST,

            #[cfg(feature = "spanish")]
            Language::Spanish => IsoCode639_1::ES,

            #[cfg(feature = "swahili")]
            Language::Swahili => IsoCode639_1::SW,

            #[cfg(feature = "swedish")]
            Language::Swedish => IsoCode639_1::SV,

            #[cfg(feature = "tagalog")]
            Language::Tagalog => IsoCode639_1::TL,

            #[cfg(feature = "tamil")]
            Language::Tamil => IsoCode639_1::TA,

            #[cfg(feature = "telugu")]
            Language::Telugu => IsoCode639_1::TE,

            #[cfg(feature = "thai")]
            Language::Thai => IsoCode639_1::TH,

            #[cfg(feature = "tsonga")]
            Language::Tsonga => IsoCode639_1::TS,

            #[cfg(feature = "tswana")]
            Language::Tswana => IsoCode639_1::TN,

            #[cfg(feature = "turkish")]
            Language::Turkish => IsoCode639_1::TR,

            #[cfg(feature = "ukrainian")]
            Language::Ukrainian => IsoCode639_1::UK,

            #[cfg(feature = "urdu")]
            Language::Urdu => IsoCode639_1::UR,

            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => IsoCode639_1::VI,

            #[cfg(feature = "welsh")]
            Language::Welsh => IsoCode639_1::CY,

            #[cfg(feature = "xhosa")]
            Language::Xhosa => IsoCode639_1::XH,

            #[cfg(feature = "yoruba")]
            Language::Yoruba => IsoCode639_1::YO,

            #[cfg(feature = "zulu")]
            Language::Zulu => IsoCode639_1::ZU,

            _ => todo!(),
        }
    }

    /// Returns the ISO 639-3 code of this language.
    pub fn iso_code_639_3(&self) -> IsoCode639_3 {
        match self {
            #[cfg(feature = "afrikaans")]
            Language::Afrikaans => IsoCode639_3::AFR,

            #[cfg(feature = "albanian")]
            Language::Albanian => IsoCode639_3::SQI,

            #[cfg(feature = "arabic")]
            Language::Arabic => IsoCode639_3::ARA,

            #[cfg(feature = "armenian")]
            Language::Armenian => IsoCode639_3::HYE,

            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => IsoCode639_3::AZE,

            #[cfg(feature = "basque")]
            Language::Basque => IsoCode639_3::EUS,

            #[cfg(feature = "belarusian")]
            Language::Belarusian => IsoCode639_3::BEL,

            #[cfg(feature = "bengali")]
            Language::Bengali => IsoCode639_3::BEN,

            #[cfg(feature = "bokmal")]
            Language::Bokmal => IsoCode639_3::NOB,

            #[cfg(feature = "bosnian")]
            Language::Bosnian => IsoCode639_3::BOS,

            #[cfg(feature = "bulgarian")]
            Language::Bulgarian => IsoCode639_3::BUL,

            #[cfg(feature = "catalan")]
            Language::Catalan => IsoCode639_3::CAT,

            #[cfg(feature = "chinese")]
            Language::Chinese => IsoCode639_3::ZHO,

            #[cfg(feature = "croatian")]
            Language::Croatian => IsoCode639_3::HRV,

            #[cfg(feature = "czech")]
            Language::Czech => IsoCode639_3::CES,

            #[cfg(feature = "danish")]
            Language::Danish => IsoCode639_3::DAN,

            #[cfg(feature = "dutch")]
            Language::Dutch => IsoCode639_3::NLD,

            #[cfg(feature = "english")]
            Language::English => IsoCode639_3::ENG,

            #[cfg(feature = "esperanto")]
            Language::Esperanto => IsoCode639_3::EPO,

            #[cfg(feature = "estonian")]
            Language::Estonian => IsoCode639_3::EST,

            #[cfg(feature = "finnish")]
            Language::Finnish => IsoCode639_3::FIN,

            #[cfg(feature = "french")]
            Language::French => IsoCode639_3::FRA,

            #[cfg(feature = "ganda")]
            Language::Ganda => IsoCode639_3::LUG,

            #[cfg(feature = "georgian")]
            Language::Georgian => IsoCode639_3::KAT,

            #[cfg(feature = "german")]
            Language::German => IsoCode639_3::DEU,

            #[cfg(feature = "greek")]
            Language::Greek => IsoCode639_3::ELL,

            #[cfg(feature = "gujarati")]
            Language::Gujarati => IsoCode639_3::GUJ,

            #[cfg(feature = "hebrew")]
            Language::Hebrew => IsoCode639_3::HEB,

            #[cfg(feature = "hindi")]
            Language::Hindi => IsoCode639_3::HIN,

            #[cfg(feature = "hungarian")]
            Language::Hungarian => IsoCode639_3::HUN,

            #[cfg(feature = "icelandic")]
            Language::Icelandic => IsoCode639_3::ISL,

            #[cfg(feature = "indonesian")]
            Language::Indonesian => IsoCode639_3::IND,

            #[cfg(feature = "irish")]
            Language::Irish => IsoCode639_3::GLE,

            #[cfg(feature = "italian")]
            Language::Italian => IsoCode639_3::ITA,

            #[cfg(feature = "japanese")]
            Language::Japanese => IsoCode639_3::JPN,

            #[cfg(feature = "kazakh")]
            Language::Kazakh => IsoCode639_3::KAZ,

            #[cfg(feature = "korean")]
            Language::Korean => IsoCode639_3::KOR,

            #[cfg(feature = "latin")]
            Language::Latin => IsoCode639_3::LAT,

            #[cfg(feature = "latvian")]
            Language::Latvian => IsoCode639_3::LAV,

            #[cfg(feature = "lithuanian")]
            Language::Lithuanian => IsoCode639_3::LIT,

            #[cfg(feature = "macedonian")]
            Language::Macedonian => IsoCode639_3::MKD,

            #[cfg(feature = "malay")]
            Language::Malay => IsoCode639_3::MSA,

            #[cfg(feature = "maori")]
            Language::Maori => IsoCode639_3::MRI,

            #[cfg(feature = "marathi")]
            Language::Marathi => IsoCode639_3::MAR,

            #[cfg(feature = "mongolian")]
            Language::Mongolian => IsoCode639_3::MON,

            #[cfg(feature = "nynorsk")]
            Language::Nynorsk => IsoCode639_3::NNO,

            #[cfg(feature = "persian")]
            Language::Persian => IsoCode639_3::FAS,

            #[cfg(feature = "polish")]
            Language::Polish => IsoCode639_3::POL,

            #[cfg(feature = "portuguese")]
            Language::Portuguese => IsoCode639_3::POR,

            #[cfg(feature = "punjabi")]
            Language::Punjabi => IsoCode639_3::PAN,

            #[cfg(feature = "romanian")]
            Language::Romanian => IsoCode639_3::RON,

            #[cfg(feature = "russian")]
            Language::Russian => IsoCode639_3::RUS,

            #[cfg(feature = "serbian")]
            Language::Serbian => IsoCode639_3::SRP,

            #[cfg(feature = "shona")]
            Language::Shona => IsoCode639_3::SNA,

            #[cfg(feature = "slovak")]
            Language::Slovak => IsoCode639_3::SLK,

            #[cfg(feature = "slovene")]
            Language::Slovene => IsoCode639_3::SLV,

            #[cfg(feature = "somali")]
            Language::Somali => IsoCode639_3::SOM,

            #[cfg(feature = "sotho")]
            Language::Sotho => IsoCode639_3::SOT,

            #[cfg(feature = "spanish")]
            Language::Spanish => IsoCode639_3::SPA,

            #[cfg(feature = "swahili")]
            Language::Swahili => IsoCode639_3::SWA,

            #[cfg(feature = "swedish")]
            Language::Swedish => IsoCode639_3::SWE,

            #[cfg(feature = "tagalog")]
            Language::Tagalog => IsoCode639_3::TGL,

            #[cfg(feature = "tamil")]
            Language::Tamil => IsoCode639_3::TAM,

            #[cfg(feature = "telugu")]
            Language::Telugu => IsoCode639_3::TEL,

            #[cfg(feature = "thai")]
            Language::Thai => IsoCode639_3::THA,

            #[cfg(feature = "tsonga")]
            Language::Tsonga => IsoCode639_3::TSO,

            #[cfg(feature = "tswana")]
            Language::Tswana => IsoCode639_3::TSN,

            #[cfg(feature = "turkish")]
            Language::Turkish => IsoCode639_3::TUR,

            #[cfg(feature = "ukrainian")]
            Language::Ukrainian => IsoCode639_3::UKR,

            #[cfg(feature = "urdu")]
            Language::Urdu => IsoCode639_3::URD,

            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => IsoCode639_3::VIE,

            #[cfg(feature = "welsh")]
            Language::Welsh => IsoCode639_3::CYM,

            #[cfg(feature = "xhosa")]
            Language::Xhosa => IsoCode639_3::XHO,

            #[cfg(feature = "yoruba")]
            Language::Yoruba => IsoCode639_3::YOR,

            #[cfg(feature = "zulu")]
            Language::Zulu => IsoCode639_3::ZUL,

            _ => todo!(),
        }
    }

    pub(crate) fn scripts(&self) -> &[Script] {
        unreachable!();
        match self {
            #[cfg(feature = "afrikaans")]
            Language::Afrikaans => &[Script::Latin],

            #[cfg(feature = "albanian")]
            Language::Albanian => &[Script::Latin],

            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => &[Script::Latin],

            #[cfg(feature = "basque")]
            Language::Basque => &[Script::Latin],

            #[cfg(feature = "bokmal")]
            Language::Bokmal => &[Script::Latin],

            #[cfg(feature = "bosnian")]
            Language::Bosnian => &[Script::Latin],

            #[cfg(feature = "catalan")]
            Language::Catalan => &[Script::Latin],

            #[cfg(feature = "croatian")]
            Language::Croatian => &[Script::Latin],

            #[cfg(feature = "czech")]
            Language::Czech => &[Script::Latin],

            #[cfg(feature = "danish")]
            Language::Danish => &[Script::Latin],

            #[cfg(feature = "dutch")]
            Language::Dutch => &[Script::Latin],

            #[cfg(feature = "english")]
            Language::English => &[Script::Latin],

            #[cfg(feature = "esperanto")]
            Language::Esperanto => &[Script::Latin],

            #[cfg(feature = "estonian")]
            Language::Estonian => &[Script::Latin],

            #[cfg(feature = "finnish")]
            Language::Finnish => &[Script::Latin],

            #[cfg(feature = "french")]
            Language::French => &[Script::Latin],

            #[cfg(feature = "ganda")]
            Language::Ganda => &[Script::Latin],

            #[cfg(feature = "german")]
            Language::German => &[Script::Latin],

            #[cfg(feature = "hungarian")]
            Language::Hungarian => &[Script::Latin],

            #[cfg(feature = "icelandic")]
            Language::Icelandic => &[Script::Latin],

            #[cfg(feature = "indonesian")]
            Language::Indonesian => &[Script::Latin],

            #[cfg(feature = "irish")]
            Language::Irish => &[Script::Latin],

            #[cfg(feature = "italian")]
            Language::Italian => &[Script::Latin],

            #[cfg(feature = "latin")]
            Language::Latin => &[Script::Latin],

            #[cfg(feature = "latvian")]
            Language::Latvian => &[Script::Latin],

            #[cfg(feature = "lithuanian")]
            Language::Lithuanian => &[Script::Latin],

            #[cfg(feature = "malay")]
            Language::Malay => &[Script::Latin],

            #[cfg(feature = "maori")]
            Language::Maori => &[Script::Latin],

            #[cfg(feature = "nynorsk")]
            Language::Nynorsk => &[Script::Latin],

            #[cfg(feature = "polish")]
            Language::Polish => &[Script::Latin],

            #[cfg(feature = "portuguese")]
            Language::Portuguese => &[Script::Latin],

            #[cfg(feature = "romanian")]
            Language::Romanian => &[Script::Latin],

            #[cfg(feature = "shona")]
            Language::Shona => &[Script::Latin],

            #[cfg(feature = "slovak")]
            Language::Slovak => &[Script::Latin],

            #[cfg(feature = "slovene")]
            Language::Slovene => &[Script::Latin],

            #[cfg(feature = "somali")]
            Language::Somali => &[Script::Latin],

            #[cfg(feature = "sotho")]
            Language::Sotho => &[Script::Latin],

            #[cfg(feature = "spanish")]
            Language::Spanish => &[Script::Latin],

            #[cfg(feature = "swahili")]
            Language::Swahili => &[Script::Latin],

            #[cfg(feature = "swedish")]
            Language::Swedish => &[Script::Latin],

            #[cfg(feature = "tagalog")]
            Language::Tagalog => &[Script::Latin],

            #[cfg(feature = "tsonga")]
            Language::Tsonga => &[Script::Latin],

            #[cfg(feature = "tswana")]
            Language::Tswana => &[Script::Latin],

            #[cfg(feature = "turkish")]
            Language::Turkish => &[Script::Latin],

            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => &[Script::Latin],

            #[cfg(feature = "welsh")]
            Language::Welsh => &[Script::Latin],

            #[cfg(feature = "xhosa")]
            Language::Xhosa => &[Script::Latin],

            #[cfg(feature = "yoruba")]
            Language::Yoruba => &[Script::Latin],

            #[cfg(feature = "zulu")]
            Language::Zulu => &[Script::Latin],

            #[cfg(feature = "belarusian")]
            Language::Belarusian => &[Script::Cyrillic],

            #[cfg(feature = "bulgarian")]
            Language::Bulgarian => &[Script::Cyrillic],

            #[cfg(feature = "kazakh")]
            Language::Kazakh => &[Script::Cyrillic],

            #[cfg(feature = "macedonian")]
            Language::Macedonian => &[Script::Cyrillic],

            #[cfg(feature = "mongolian")]
            Language::Mongolian => &[Script::Cyrillic],

            #[cfg(feature = "russian")]
            Language::Russian => &[Script::Cyrillic],

            #[cfg(feature = "serbian")]
            Language::Serbian => &[Script::Cyrillic],

            #[cfg(feature = "ukrainian")]
            Language::Ukrainian => &[Script::Cyrillic],

            #[cfg(feature = "arabic")]
            Language::Arabic => &[Script::Arabic],

            #[cfg(feature = "persian")]
            Language::Persian => &[Script::Arabic],

            #[cfg(feature = "urdu")]
            Language::Urdu => &[Script::Arabic],

            #[cfg(feature = "hindi")]
            Language::Hindi => &[Script::Devanagari],

            #[cfg(feature = "marathi")]
            Language::Marathi => &[Script::Devanagari],

            #[cfg(feature = "armenian")]
            Language::Armenian => &[Script::Armenian],

            #[cfg(feature = "bengali")]
            Language::Bengali => &[Script::Bengali],

            #[cfg(feature = "chinese")]
            Language::Chinese => &[Script::Han],

            #[cfg(feature = "georgian")]
            Language::Georgian => &[Script::Georgian],

            #[cfg(feature = "greek")]
            Language::Greek => &[Script::Greek],

            #[cfg(feature = "gujarati")]
            Language::Gujarati => &[Script::Gujarati],

            #[cfg(feature = "hebrew")]
            Language::Hebrew => &[Script::Hebrew],

            #[cfg(feature = "japanese")]
            Language::Japanese => &[Script::Hiragana, Script::Katakana, Script::Han],

            #[cfg(feature = "korean")]
            Language::Korean => &[Script::Hangul],

            #[cfg(feature = "punjabi")]
            Language::Punjabi => &[Script::Gurmukhi],

            #[cfg(feature = "tamil")]
            Language::Tamil => &[Script::Tamil],

            #[cfg(feature = "telugu")]
            Language::Telugu => &[Script::Telugu],

            #[cfg(feature = "thai")]
            Language::Thai => &[Script::Thai],

            _ => todo!(),
        }
    }

    pub(crate) fn unique_characters(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => Some("Əə"),

            #[cfg(feature = "catalan")]
            Language::Catalan => Some("Ïï"),

            #[cfg(feature = "czech")]
            Language::Czech => Some("ĚěŘřŮů"),

            #[cfg(feature = "esperanto")]
            Language::Esperanto => Some("ĈĉĜĝĤĥĴĵŜŝŬŭ"),

            #[cfg(feature = "german")]
            Language::German => Some("ß"),

            #[cfg(feature = "hungarian")]
            Language::Hungarian => Some("ŐőŰű"),

            #[cfg(feature = "kazakh")]
            Language::Kazakh => Some("ҒғҚқҢңҰұ"),

            #[cfg(feature = "latvian")]
            Language::Latvian => Some("ĢģĶķĻļŅņ"),

            #[cfg(feature = "lithuanian")]
            Language::Lithuanian => Some("ĖėĮįŲų"),

            #[cfg(feature = "macedonian")]
            Language::Macedonian => Some("ЃѓЅѕЌќЏџ"),

            #[cfg(feature = "marathi")]
            Language::Marathi => Some("ळ"),

            #[cfg(feature = "polish")]
            Language::Polish => Some("ŁłŃńŚśŹź"),

            #[cfg(feature = "romanian")]
            Language::Romanian => Some("Țţ"),

            #[cfg(feature = "serbian")]
            Language::Serbian => Some("ЂђЋћ"),

            #[cfg(feature = "slovak")]
            Language::Slovak => Some("ĹĺĽľŔŕ"),

            #[cfg(feature = "spanish")]
            Language::Spanish => Some("¿¡"),

            #[cfg(feature = "ukrainian")]
            Language::Ukrainian => Some("ҐґЄєЇї"),

            #[cfg(feature = "vietnamese")]
            Language::Vietnamese => Some("ẰằẦầẲẳẨẩẴẵẪẫẮắẤấẠạẶặẬậỀềẺẻỂểẼẽỄễẾếỆệỈỉĨĩỊịƠơỒồỜờỎỏỔổỞởỖỗỠỡỐốỚớỘộỢợƯưỪừỦủỬửŨũỮữỨứỤụỰựỲỳỶỷỸỹỴỵ"),

            #[cfg(feature = "yoruba")]
            Language::Yoruba => Some("Ṣṣ"),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::language::Language::*;

    use super::*;

    #[test]
    fn assert_language_string_representation_is_correct() {
        assert_eq!(English.to_string(), "English");
    }

    #[test]
    fn test_language_serializer() {
        let serialized = serde_json::to_string(&English).unwrap();
        assert_eq!(serialized, "\"ENGLISH\"");
    }

    #[test]
    fn test_language_deserializer() {
        let deserialized = serde_json::from_str::<Language>("\"ENGLISH\"").unwrap();
        assert_eq!(deserialized, English);
    }

    #[test]
    fn test_from_str() {
        let language = Language::from_str("english").unwrap();
        assert_eq!(language, English);
    }

    #[test]
    fn assert_all_languages_are_available() {
        assert_eq!(
            Language::all(),
            ahashset!(
                Afrikaans,
                Albanian,
                Arabic,
                Armenian,
                Azerbaijani,
                Basque,
                Belarusian,
                Bengali,
                Bokmal,
                Bosnian,
                Bulgarian,
                Catalan,
                Chinese,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                Georgian,
                German,
                Greek,
                Gujarati,
                Hebrew,
                Hindi,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Japanese,
                Kazakh,
                Korean,
                Latin,
                Latvian,
                Lithuanian,
                Macedonian,
                Malay,
                Maori,
                Marathi,
                Mongolian,
                Nynorsk,
                Persian,
                Polish,
                Portuguese,
                Punjabi,
                Romanian,
                Russian,
                Serbian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tamil,
                Telugu,
                Thai,
                Tsonga,
                Tswana,
                Turkish,
                Ukrainian,
                Urdu,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }

    #[test]
    fn assert_all_spoken_languages_are_available() {
        assert_eq!(
            Language::all_spoken_ones(),
            ahashset!(
                Afrikaans,
                Albanian,
                Arabic,
                Armenian,
                Azerbaijani,
                Basque,
                Belarusian,
                Bengali,
                Bokmal,
                Bosnian,
                Bulgarian,
                Catalan,
                Chinese,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                Georgian,
                German,
                Greek,
                Gujarati,
                Hebrew,
                Hindi,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Japanese,
                Kazakh,
                Korean,
                Latvian,
                Lithuanian,
                Macedonian,
                Malay,
                Maori,
                Marathi,
                Mongolian,
                Nynorsk,
                Persian,
                Polish,
                Portuguese,
                Punjabi,
                Romanian,
                Russian,
                Serbian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tamil,
                Telugu,
                Thai,
                Tsonga,
                Tswana,
                Turkish,
                Ukrainian,
                Urdu,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }

    #[test]
    fn assert_certain_languages_support_arabic_script() {
        assert_eq!(
            Language::all_with_arabic_script(),
            ahashset!(Arabic, Persian, Urdu)
        );
    }

    #[test]
    fn assert_certain_languages_support_cyrillic_script() {
        assert_eq!(
            Language::all_with_cyrillic_script(),
            ahashset!(
                Belarusian, Bulgarian, Kazakh, Macedonian, Mongolian, Russian, Serbian, Ukrainian
            )
        );
    }

    #[test]
    fn assert_certain_languages_support_devanagari_script() {
        assert_eq!(
            Language::all_with_devanagari_script(),
            ahashset!(Hindi, Marathi)
        );
    }

    #[test]
    fn assert_certain_languages_support_latin_script() {
        assert_eq!(
            Language::all_with_latin_script(),
            ahashset!(
                Afrikaans,
                Albanian,
                Azerbaijani,
                Basque,
                Bokmal,
                Bosnian,
                Catalan,
                Croatian,
                Czech,
                Danish,
                Dutch,
                English,
                Esperanto,
                Estonian,
                Finnish,
                French,
                Ganda,
                German,
                Hungarian,
                Icelandic,
                Indonesian,
                Irish,
                Italian,
                Latin,
                Latvian,
                Lithuanian,
                Malay,
                Maori,
                Nynorsk,
                Polish,
                Portuguese,
                Romanian,
                Shona,
                Slovak,
                Slovene,
                Somali,
                Sotho,
                Spanish,
                Swahili,
                Swedish,
                Tagalog,
                Tsonga,
                Tswana,
                Turkish,
                Vietnamese,
                Welsh,
                Xhosa,
                Yoruba,
                Zulu
            )
        );
    }
}
