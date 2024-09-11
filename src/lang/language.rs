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

use super::Script;
use crate::isocode::{IsoCode639_1, IsoCode639_3};
use crate::ExtraCheck;
use ::std::collections::HashMap;
use ::std::fmt::{Debug, Display, Formatter, Result};
use ::std::hash::BuildHasher;
use ahash::AHashSet;
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumString};

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
    EnumIter,
    EnumString,
)]
#[serde(rename_all(serialize = "UPPERCASE", deserialize = "UPPERCASE"))]
#[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord, rename_all = "UPPERCASE")
)]
pub enum Language {
    Acehnese,  // #[strum(serialize = "ace")]
    Afrikaans, // #[strum(serialize = "afr")]
    Ahom,
    Akkadian,
    Albanian,
    AlbanianHistorical,
    AlbanianTosk, // #[strum(serialize = "als")]
    Amharic,      // #[strum(serialize = "amh")]
    AncientGreek,
    Angkola,
    Arabic,               // #[strum(serialize = "arb")]
    ArabicEgyptian,       // #[strum(serialize = "arz")]
    ArabicMesopotamian,   // #[strum(serialize = "acm")]
    ArabicMoroccan,       // #[strum(serialize = "ary")]
    ArabicNajdi,          // #[strum(serialize = "ars")]
    ArabicNorthLevantine, // #[strum(serialize = "apc")]
    ArabicSouthernYemeni, // #[strum(serialize = "acq")]
    ArabicSouthLevantine, // #[strum(serialize = "ajp")]
    ArabicTunisian,       // #[strum(serialize = "aeb")]
    Aramaic,
    Armenian,      // #[strum(serialize = "hye")]
    Assamese,      // #[strum(serialize = "asm")]
    Asturian,      // #[strum(serialize = "ast")]
    Avestan,
    Awadhi,        // #[strum(serialize = "awa")]
    AymaraCentral, // #[strum(serialize = "ayr")]
    Azerbaijani,
    AzerbaijaniNorth, // #[strum(serialize = "azj")]
    AzerbaijaniSouth, // #[strum(serialize = "azb")]
    Balinese, // #[strum(serialize = "ban")]
    Bambara, // #[strum(serialize = "bam")]
    Bamum,
    Banjar,     // #[strum(serialize = "bjn")]
    Bantawa,
    Bashkir, // #[strum(serialize = "bak")]
    Basque,  // #[strum(serialize = "eus")]
    Bassa,
    Belarusian, // #[strum(serialize = "bel")]
    Bemba, // #[strum(serialize = "bem")]
    Bengali,    // #[strum(serialize = "ben")]
    Berber,
    Bhaiksuki,
    Bhojpuri, // #[strum(serialize = "bho")]
    BishnupriyaManipuri,
    Bokmal,  // #[strum(serialize = "nob")]
    Bosnian, // #[strum(serialize = "bos")]
    Braille, // Any language adapted to Braille
    BuddhistMarchen,
    Buginese, // #[strum(serialize = "bug")]
    Buhid,
    Bulgarian, // #[strum(serialize = "bul")]
    Burmese,   // #[strum(serialize = "mya")]
    Carian,
    Catalan, // #[strum(serialize = "cat")]
    CaucasianAlbanian,
    Cebuano, // #[strum(serialize = "ceb")]
    Chakma,
    Cham,
    Cherokee,
    Chhattisgarhi,    // #[strum(serialize = "hne")]
    Chinese,          // #[strum(serialize = "zho")]
    ChineseCantonese, // #[strum(serialize = "yue")]
    ChineseMandarin,
    ChineseTuhua,
    Chokwe,        // #[strum(serialize = "cjk")]
    Chorasmian,
    Coptic,
    Cree,
    CreoleHaitian, // #[strum(serialize = "hat")]
    Croatian,      // #[strum(serialize = "hrv")]
    CyproMinoan,   // used in ancient Cyprus
    Czech,         // #[strum(serialize = "ces")]
    Danish,        // #[strum(serialize = "dan")]
    Dari,              // #[strum(serialize = "prs")]
    DinkaSouthwestern, // #[strum(serialize = "dik")]
    Dogri,
    Dutch,             // #[strum(serialize = "nld")]
    Dyula,             // #[strum(serialize = "dyu")]
    Dzongkha,          // #[strum(serialize = "dzo")]
    EgyptianHieroglyphs,
    Elymaic,
    English,         // #[strum(serialize = "eng")]
    EnglishDuployan, //Shorthand systems for English
    EnglishMormon,
    EnglishPhonetic,
    Esperanto, // #[strum(serialize = "epo")]
    Estonian,  // #[strum(serialize = "est")]
    Etruscan,
    Ewe,       // #[strum(serialize = "ewe")]
    Faroese,          // #[strum(serialize = "fao")]
    Fijian,           // #[strum(serialize = "fij")]
    Finnish, // #[strum(serialize = "fin")]
    Fon,              // #[strum(serialize = "fon")]
    French,  // #[strum(serialize = "fra")]
    FrenchDuployan,
    Friulian,         // #[strum(serialize = "fur")]
    Fulani,
    FulfuldeNigerian, // #[strum(serialize = "fuv")]
    GaelicScottish, // #[strum(serialize = "gla")]
    Galician,       // #[strum(serialize = "glg")]
    Ganda,            // #[strum(serialize = "lug")]
    Gandhari,
    Geez,
    Georgian, // #[strum(serialize = "kat")]
    German,   // #[strum(serialize = "deu")]
    Gondi,
    Gothic,
    Greek,    // #[strum(serialize = "ell")]
    Guarani,        // #[strum(serialize = "grn")]
    Gujarati, // #[strum(serialize = "guj")]
    Gurung,
    Hanunoo,
    Hausa,  // #[strum(serialize = "hau")]
    Hebrew, // #[strum(serialize = "heb")]
    Hindi,  // #[strum(serialize = "hin")]
    Hittite,
    Hmong,
    Ho,
    Hungarian,  // #[strum(serialize = "hun")]
    Icelandic,  // #[strum(serialize = "isl")]
    Igbo,     // #[strum(serialize = "ibo")]
    Ilocano,  // #[strum(serialize = "ilo")]
    Indonesian, // #[strum(serialize = "ind")]
    Inuktitut,
    Irish,    // #[strum(serialize = "gle")]
    Italian,  // #[strum(serialize = "ita")]
    Japanese, // #[strum(serialize = "jpn")]
    Javanese, // #[strum(serialize = "jav")]
    Jingpho,  // #[strum(serialize = "kac")]
    Kabiye, // #[strum(serialize = "kbp")]
    Kabuverdianu, // #[strum(serialize = "kea")]
    Kabyle,   // #[strum(serialize = "kab")]
    Kamba,  // #[strum(serialize = "kam")]
    Kannada,  // #[strum(serialize = "kan")]
    KanuriCentral,   // #[strum(serialize = "knc")]
    Karo,
    Kashmiri, // #[strum(serialize = "kas")]
    KayahLi,
    Kazakh,
    Khitan,
    Khmer,       // #[strum(serialize = "khm")]
    Khoja,
    Kikongo,         // #[strum(serialize = "kon")]
    Kikuyu,      // #[strum(serialize = "kik")]
    Kimbundu,    // #[strum(serialize = "kmb")]
    Kinyarwanda, // #[strum(serialize = "kin")]
    Korean, // #[strum(serialize = "kor")]
    Kurdish,
    KurdishCentral,  // #[strum(serialize = "ckb")]
    KurdishNorthern, // #[strum(serialize = "kmr")]
    Kyrgyz,      // #[strum(serialize = "kir")]
    Lao,             // #[strum(serialize = "lao")]
    Latgalian,     // #[strum(serialize = "ltg")]
    Latin,
    Latvian, // #[strum(serialize = "lvs")]
    Lepcha,
    Ligurian,   // #[strum(serialize = "lij")]
    Limbu,
    Limburgish, // #[strum(serialize = "lim")]
    Lingala,    // #[strum(serialize = "lin")]
    Lisu,
    Lithuanian,    // #[strum(serialize = "lit")]
    Lombard,       // #[strum(serialize = "lmo")]
    LubaKasai,     // #[strum(serialize = "lua")]
    Luo,           // #[strum(serialize = "luo")]
    Luwian,
    Luxembourgish, // #[strum(serialize = "ltz")]
    Lycian,
    Lydian,
    Macedonian, // #[strum(serialize = "mkd")]
    Magahi,     // #[strum(serialize = "mag")]
    Maithili,   // #[strum(serialize = "mai")]
    Makasar,
    Makassarese,
    Malay,
    Malayalam,     // #[strum(serialize = "mal")]
    MalayStandard, // #[strum(serialize = "zsm")]
    MaldivianDhivehi,
    MalgasyPlateau, // #[strum(serialize = "plt")]
    Maltese,    // #[strum(serialize = "mlt")]
    Mandaic,
    Mandailing,
    Mande,
    ManipuriMeetei,
    Maori,       // #[strum(serialize = "mri")]
    Marathi,     // #[strum(serialize = "mar")]
    Marwari,
    Medefaidrin,
    Meitei,      // #[strum(serialize = "mni")]
    Mende,
    Meroitic,
    MiddlePersian,
    Minangkabau, // #[strum(serialize = "min")]
    Minoan,
    Mizo,           // #[strum(serialize = "lus")]
    MongolianHalh,  // #[strum(serialize = "khk")]
    Mossi,          // #[strum(serialize = "mos")]
    Mro,
    Mundari,
    MycenaeanGreek,
    Nepali, // #[strum(serialize = "npi")]
    Newari,
    NorthernThai,
    Nuer,   // #[strum(serialize = "nus")]
    Nyanja,  // #[strum(serialize = "nya")]
    Nynorsk, // #[strum(serialize = "nno")]
    Occitan,          // #[strum(serialize = "oci")]
    Odia,    // #[strum(serialize = "ory")]
    Ojibwe,
    OldChurchSlavonic,
    OldEnglish,
    OldHungarian,
    OldIrish,
    OldJavanese,
    OldKomi,
    OldNorse,
    OldNorthArabian,
    OldPersian,
    OldSogdian,
    OldSouthArabian,
    OldTurkic,
    OldUyghur,
    Oromo,
    OromoWestCentral, // #[strum(serialize = "gaz")]
    Osage,
    Oscan,
    Pakpak,
    Pangasinan, // #[strum(serialize = "pag")]
    Papiamento,     // #[strum(serialize = "pap")]
    Parthian,
    Pashto,
    PastoSouthern,  // #[strum(serialize = "pbt")]
    Persian,
    PersianWestern, // #[strum(serialize = "pes")]
    Phoenician,
    Polish,     // #[strum(serialize = "pol")]
    Portuguese, // #[strum(serialize = "por")]
    Prakrit,
    Pular,
    PunjabiEastern, // #[strum(serialize = "pan")]
    QuechuaAyacucho, // #[strum(serialize = "quy")]
    Rejang,
    Rohingya,
    Romanian, // #[strum(serialize = "ron")]
    Rundi,    // #[strum(serialize = "run")]
    Russian,  // #[strum(serialize = "rus")]
    Samoan,  // #[strum(serialize = "smo")]
    Sango,    // #[strum(serialize = "sag")]
    Sanskrit, // #[strum(serialize = "san")]
    Santali,  // #[strum(serialize = "sat")]
    Saraiki,
    Sardinian,  // #[strum(serialize = "srd")]
    Saurashtra,
    Sepedi,     // #[strum(serialize = "nso")]
    Serbian,    // #[strum(serialize = "srp")]
    Sesotho,    // #[strum(serialize = "sot")]
    Shan,       // #[strum(serialize = "shn")]
    Shona,      // #[strum(serialize = "sna")]
    Sicilian, // #[strum(serialize = "scn")]
    Signlanguages,
    Silesian, // #[strum(serialize = "szl")]
    Simalungun,
    Sindhi,  // #[strum(serialize = "snd")]
    Sinhala, // #[strum(serialize = "sin")]
    Slovak,  // #[strum(serialize = "slk")]
    Slovene, // #[strum(serialize = "slv")]
    Sogdian,
    Somali, // #[strum(serialize = "som")]
    Sora,
    Spanish,  // #[strum(serialize = "spa")]
    Sumerian,
    Sundanese, // #[strum(serialize = "sun")]
    Sunuwar,
    Swahili, // #[strum(serialize = "swh")]
    Swati,    // #[strum(serialize = "ssw")]
    Swedish, // #[strum(serialize = "swe")]
    Sylheti,
    Syriac,
    Tagalog, // #[strum(serialize = "tgl")]
    Tagbanwa,
    TaiDam,
    TaiDon,
    TaiLe,
    TaiLue,
    Tajik,    // #[strum(serialize = "tgk")]
    Tamasheq, // #[strum(serialize = "taq")]
    TamazightCentralAtlas, // #[strum(serialize = "tzm")]
    Tamil,    // #[strum(serialize = "tam")]
    Tangsa,
    Tangut,
    Tatar,         // #[strum(serialize = "tat")]
    TatarCrimean,  // #[strum(serialize = "crh")]
    Telugu,          // #[strum(serialize = "tel")]
    Thai,            // #[strum(serialize = "tha")]
    Tibetan,         // #[strum(serialize = "bod")]
    Tigrinya,        // #[strum(serialize = "tir")]
    Toba,
    TokPisin,        // #[strum(serialize = "tpi")]
    Toto,
    Tsonga,  // #[strum(serialize = "tso")]
    Tswana,  // #[strum(serialize = "tsn")]
    Tulu,
    Tumbuka, // #[strum(serialize = "tum")]
    Turkish,               // #[strum(serialize = "tur")]
    Turkmen, // #[strum(serialize = "tuk")]
    Twi,                   // #[strum(serialize = "twi")]
    Ugaritic,
    Ukrainian, // #[strum(serialize = "ukr")]
    Umbrian,
    Umbundu,   // #[strum(serialize = "umb")]
    Urdu,          // #[strum(serialize = "urd")]
    Uyghur,        // #[strum(serialize = "uig")]
    UzbekNorthern, // #[strum(serialize = "uzn")]
    Vai,
    Venetian,      // #[strum(serialize = "vec")]
    Vietnamese, // #[strum(serialize = "vie")]
    Wancho,
    Waray, // #[strum(serialize = "war")]
    Welsh, // #[strum(serialize = "cym")]
    Wolof, // #[strum(serialize = "wol")]
    Xhosa, // #[strum(serialize = "xho")]
    Yi,
    Yiddish,
    YiddishEastern, // #[strum(serialize = "ydd")]
    Yoruba,         // #[strum(serialize = "yor")]
    ZoLanguages,
    Zulu, // #[strum(serialize = "zul")]
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
        use Language::*;
        match self {
            Afrikaans => IsoCode639_1::AF,
            Albanian => IsoCode639_1::SQ,
            Arabic => IsoCode639_1::AR,
            Armenian => IsoCode639_1::HY,
            Azerbaijani => IsoCode639_1::AZ,
            Basque => IsoCode639_1::EU,
            Belarusian => IsoCode639_1::BE,
            Bengali => IsoCode639_1::BN,
            Bokmal => IsoCode639_1::NB,
            Bosnian => IsoCode639_1::BS,
            Bulgarian => IsoCode639_1::BG,
            Catalan => IsoCode639_1::CA,
            Chinese => IsoCode639_1::ZH,
            Croatian => IsoCode639_1::HR,
            Czech => IsoCode639_1::CS,
            Danish => IsoCode639_1::DA,
            Dutch => IsoCode639_1::NL,
            English => IsoCode639_1::EN,
            Esperanto => IsoCode639_1::EO,
            Estonian => IsoCode639_1::ET,
            Finnish => IsoCode639_1::FI,
            French => IsoCode639_1::FR,
            Ganda => IsoCode639_1::LG,
            Georgian => IsoCode639_1::KA,
            German => IsoCode639_1::DE,
            Greek => IsoCode639_1::EL,
            Gujarati => IsoCode639_1::GU,
            Hebrew => IsoCode639_1::HE,
            Hindi => IsoCode639_1::HI,
            Hungarian => IsoCode639_1::HU,
            Icelandic => IsoCode639_1::IS,
            Indonesian => IsoCode639_1::ID,
            Irish => IsoCode639_1::GA,
            Italian => IsoCode639_1::IT,
            Japanese => IsoCode639_1::JA,
            Kazakh => IsoCode639_1::KK,
            Korean => IsoCode639_1::KO,
            Latin => IsoCode639_1::LA,
            Latvian => IsoCode639_1::LV,
            Lithuanian => IsoCode639_1::LT,
            Macedonian => IsoCode639_1::MK,
            Malay => IsoCode639_1::MS,
            Maori => IsoCode639_1::MI,
            Marathi => IsoCode639_1::MR,
            MongolianHalh => IsoCode639_1::MN,
            Nynorsk => IsoCode639_1::NN,
            Persian => IsoCode639_1::FA,
            Polish => IsoCode639_1::PL,
            Portuguese => IsoCode639_1::PT,
            PunjabiEastern => IsoCode639_1::PA,
            Romanian => IsoCode639_1::RO,
            Russian => IsoCode639_1::RU,
            Serbian => IsoCode639_1::SR,
            Shona => IsoCode639_1::SN,
            Slovak => IsoCode639_1::SK,
            Slovene => IsoCode639_1::SL,
            Somali => IsoCode639_1::SO,
            Sesotho => IsoCode639_1::ST,
            Spanish => IsoCode639_1::ES,
            Swahili => IsoCode639_1::SW,
            Swedish => IsoCode639_1::SV,
            Tagalog => IsoCode639_1::TL,
            Tamil => IsoCode639_1::TA,
            Telugu => IsoCode639_1::TE,
            Thai => IsoCode639_1::TH,
            Tsonga => IsoCode639_1::TS,
            Tswana => IsoCode639_1::TN,
            Turkish => IsoCode639_1::TR,
            Ukrainian => IsoCode639_1::UK,
            Urdu => IsoCode639_1::UR,
            Vietnamese => IsoCode639_1::VI,
            Welsh => IsoCode639_1::CY,
            Xhosa => IsoCode639_1::XH,
            Yoruba => IsoCode639_1::YO,
            Zulu => IsoCode639_1::ZU,

            _ => todo!(),
        }
    }

    /// Returns the ISO 639-3 code of this language.
    pub fn iso_code_639_3(&self) -> IsoCode639_3 {
        use Language::*;
        match self {
            Afrikaans => IsoCode639_3::AFR,
            Albanian => IsoCode639_3::SQI,
            Arabic => IsoCode639_3::ARA,
            Armenian => IsoCode639_3::HYE,
            Azerbaijani => IsoCode639_3::AZE,
            Basque => IsoCode639_3::EUS,
            Belarusian => IsoCode639_3::BEL,
            Bengali => IsoCode639_3::BEN,
            Bokmal => IsoCode639_3::NOB,
            Bosnian => IsoCode639_3::BOS,
            Bulgarian => IsoCode639_3::BUL,
            Catalan => IsoCode639_3::CAT,
            Chinese => IsoCode639_3::ZHO,
            Croatian => IsoCode639_3::HRV,
            Czech => IsoCode639_3::CES,
            Danish => IsoCode639_3::DAN,
            Dutch => IsoCode639_3::NLD,
            English => IsoCode639_3::ENG,
            Esperanto => IsoCode639_3::EPO,
            Estonian => IsoCode639_3::EST,
            Finnish => IsoCode639_3::FIN,
            French => IsoCode639_3::FRA,
            Ganda => IsoCode639_3::LUG,
            Georgian => IsoCode639_3::KAT,
            German => IsoCode639_3::DEU,
            Greek => IsoCode639_3::ELL,
            Gujarati => IsoCode639_3::GUJ,
            Hebrew => IsoCode639_3::HEB,
            Hindi => IsoCode639_3::HIN,
            Hungarian => IsoCode639_3::HUN,
            Icelandic => IsoCode639_3::ISL,
            Indonesian => IsoCode639_3::IND,
            Irish => IsoCode639_3::GLE,
            Italian => IsoCode639_3::ITA,
            Japanese => IsoCode639_3::JPN,
            Kazakh => IsoCode639_3::KAZ,
            Korean => IsoCode639_3::KOR,
            Latin => IsoCode639_3::LAT,
            Latvian => IsoCode639_3::LAV,
            Lithuanian => IsoCode639_3::LIT,
            Macedonian => IsoCode639_3::MKD,
            Malay => IsoCode639_3::MSA,
            Maori => IsoCode639_3::MRI,
            Marathi => IsoCode639_3::MAR,
            MongolianHalh => IsoCode639_3::MON,
            Nynorsk => IsoCode639_3::NNO,
            Persian => IsoCode639_3::FAS,
            Polish => IsoCode639_3::POL,
            Portuguese => IsoCode639_3::POR,
            PunjabiEastern => IsoCode639_3::PAN,
            Romanian => IsoCode639_3::RON,
            Russian => IsoCode639_3::RUS,
            Serbian => IsoCode639_3::SRP,
            Shona => IsoCode639_3::SNA,
            Slovak => IsoCode639_3::SLK,
            Slovene => IsoCode639_3::SLV,
            Somali => IsoCode639_3::SOM,
            Sesotho => IsoCode639_3::SOT,
            Spanish => IsoCode639_3::SPA,
            Swahili => IsoCode639_3::SWA,
            Swedish => IsoCode639_3::SWE,
            Tagalog => IsoCode639_3::TGL,
            Tamil => IsoCode639_3::TAM,
            Telugu => IsoCode639_3::TEL,
            Thai => IsoCode639_3::THA,
            Tsonga => IsoCode639_3::TSO,
            Tswana => IsoCode639_3::TSN,
            Turkish => IsoCode639_3::TUR,
            Ukrainian => IsoCode639_3::UKR,
            Urdu => IsoCode639_3::URD,
            Vietnamese => IsoCode639_3::VIE,
            Welsh => IsoCode639_3::CYM,
            Xhosa => IsoCode639_3::XHO,
            Yoruba => IsoCode639_3::YOR,
            Zulu => IsoCode639_3::ZUL,

            _ => todo!(),
        }
    }

    pub(crate) fn scripts(&self) -> &[Script] {
        match self {
            Language::Afrikaans => &[Script::Latin],
            Language::Albanian => &[Script::Latin],
            Language::Azerbaijani => &[Script::Latin],
            Language::Basque => &[Script::Latin],
            Language::Bokmal => &[Script::Latin],
            Language::Bosnian => &[Script::Latin],
            Language::Catalan => &[Script::Latin],
            Language::Croatian => &[Script::Latin],
            Language::Czech => &[Script::Latin],
            Language::Danish => &[Script::Latin],
            Language::Dutch => &[Script::Latin],
            Language::English => &[Script::Latin],
            Language::Esperanto => &[Script::Latin],
            Language::Estonian => &[Script::Latin],
            Language::Finnish => &[Script::Latin],
            Language::French => &[Script::Latin],
            Language::Ganda => &[Script::Latin],
            Language::German => &[Script::Latin],
            Language::Hungarian => &[Script::Latin],
            Language::Icelandic => &[Script::Latin],
            Language::Indonesian => &[Script::Latin],
            Language::Irish => &[Script::Latin],
            Language::Italian => &[Script::Latin],
            Language::Latin => &[Script::Latin],
            Language::Latvian => &[Script::Latin],
            Language::Lithuanian => &[Script::Latin],
            Language::Malay => &[Script::Latin],
            Language::Maori => &[Script::Latin],
            Language::Nynorsk => &[Script::Latin],
            Language::Polish => &[Script::Latin],
            Language::Portuguese => &[Script::Latin],
            Language::Romanian => &[Script::Latin],
            Language::Shona => &[Script::Latin],
            Language::Slovak => &[Script::Latin],
            Language::Slovene => &[Script::Latin],
            Language::Somali => &[Script::Latin],
            Language::Sesotho => &[Script::Latin],
            Language::Spanish => &[Script::Latin],
            Language::Swahili => &[Script::Latin],
            Language::Swedish => &[Script::Latin],
            Language::Tagalog => &[Script::Latin],
            Language::Tsonga => &[Script::Latin],
            Language::Tswana => &[Script::Latin],
            Language::Turkish => &[Script::Latin],
            Language::Vietnamese => &[Script::Latin],
            Language::Welsh => &[Script::Latin],
            Language::Xhosa => &[Script::Latin],
            Language::Yoruba => &[Script::Latin],
            Language::Zulu => &[Script::Latin],
            Language::Belarusian => &[Script::Cyrillic],
            Language::Bulgarian => &[Script::Cyrillic],
            Language::Kazakh => &[Script::Cyrillic],
            Language::Macedonian => &[Script::Cyrillic],
            Language::MongolianHalh => &[Script::Cyrillic],
            Language::Russian => &[Script::Cyrillic],
            Language::Serbian => &[Script::Cyrillic],
            Language::Ukrainian => &[Script::Cyrillic],
            Language::Arabic => &[Script::Arabic],
            Language::Persian => &[Script::Arabic],
            Language::Urdu => &[Script::Arabic],
            Language::Hindi => &[Script::Devanagari],
            Language::Marathi => &[Script::Devanagari],
            Language::Armenian => &[Script::Armenian],
            Language::Bengali => &[Script::Bengali],
            Language::Chinese => &[Script::Han],
            Language::Georgian => &[Script::Georgian],
            Language::Greek => &[Script::Greek],
            Language::Gujarati => &[Script::Gujarati],
            Language::Hebrew => &[Script::Hebrew],
            Language::Japanese => &[Script::Hiragana, Script::Katakana, Script::Han],
            Language::Korean => &[Script::Hangul],
            Language::PunjabiEastern => &[Script::Gurmukhi],
            Language::Tamil => &[Script::Tamil],
            Language::Telugu => &[Script::Telugu],
            Language::Thai => &[Script::Thai],

            _ => &[],
        }
    }

    pub(crate) fn unique_characters(&self) -> Option<&str> {
        match self {
            #[cfg(feature = "azerbaijani")]
            Language::Azerbaijani => Some("Əə"),
            Language::Catalan => Some("Ïï"),
            Language::Czech => Some("ĚěŘřŮů"),
            Language::Esperanto => Some("ĈĉĜĝĤĥĴĵŜŝŬŭ"),
            Language::German => Some("ß"),
            Language::Hungarian => Some("ŐőŰű"),
            Language::Kazakh => Some("ҒғҚқҢңҰұ"),
            Language::Latvian => Some("ĢģĶķĻļŅņ"),
            Language::Lithuanian => Some("ĖėĮįŲų"),
            Language::Macedonian => Some("ЃѓЅѕЌќЏџ"),
            Language::Marathi => Some("ळ"),
            Language::Polish => Some("ŁłŃńŚśŹź"),
            Language::Romanian => Some("Țţ"),
            Language::Serbian => Some("ЂђЋћ"),
            Language::Slovak => Some("ĹĺĽľŔŕ"),
            Language::Spanish => Some("¿¡"),
            Language::Ukrainian => Some("ҐґЄєЇї"),
            Language::Vietnamese => Some("ẰằẦầẲẳẨẩẴẵẪẫẮắẤấẠạẶặẬậỀềẺẻỂểẼẽỄễẾếỆệỈỉĨĩỊịƠơỒồỜờỎỏỔổỞởỖỗỠỡỐốỚớỘộỢợƯưỪừỦủỬửŨũỮữỨứỤụỰựỲỳỶỷỸỹỴỵ"),
            Language::Yoruba => Some("Ṣṣ"),

            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Language::*;
    use ::std::str::FromStr;

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

    /* #[test]
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
                Sesotho,
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
                Sesotho,
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
    } */

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
                Belarusian,
                Bulgarian,
                Kazakh,
                Macedonian,
                MongolianHalh,
                Russian,
                Serbian,
                Ukrainian
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
                Sesotho,
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
