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

use ::std::fmt::{Debug, Display, Formatter, Result};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};

/// This enum specifies the ISO 639-1 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
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
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord)
)]
pub enum IsoCode639_1 {
    /// The ISO 639-1 code for [`Afrikaans`](crate::language::Language::Afrikaans)
    AF,
    /// The ISO 639-1 code for [`Arabic`](crate::language::Language::Arabic)
    AR,
    /// The ISO 639-1 code for [`Azerbaijani`](crate::language::Language::Azerbaijani)
    AZ,
    /// The ISO 639-1 code for [`Belarusian`](crate::language::Language::Belarusian)
    BE,
    /// The ISO 639-1 code for [`Bulgarian`](crate::language::Language::Bulgarian)
    BG,
    /// The ISO 639-1 code for [`Bengali`](crate::language::Language::Bengali)
    BN,
    /// The ISO 639-1 code for [`Bosnian`](crate::language::Language::Bosnian)
    BS,
    /// The ISO 639-1 code for [`Catalan`](crate::language::Language::Catalan)
    CA,
    /// The ISO 639-1 code for [`Czech`](crate::language::Language::Czech)
    CS,
    /// The ISO 639-1 code for [`Welsh`](crate::language::Language::Welsh)
    CY,
    /// The ISO 639-1 code for [`Danish`](crate::language::Language::Danish)
    DA,
    /// The ISO 639-1 code for [`German`](crate::language::Language::German)
    DE,
    /// The ISO 639-1 code for [`Greek`](crate::language::Language::Greek)
    EL,
    /// The ISO 639-1 code for [`English`](crate::language::Language::English)
    EN,
    /// The ISO 639-1 code for [`Esperanto`](crate::language::Language::Esperanto)
    EO,
    /// The ISO 639-1 code for [`Spanish`](crate::language::Language::Spanish)
    ES,
    /// The ISO 639-1 code for [`Estonian`](crate::language::Language::Estonian)
    ET,
    /// The ISO 639-1 code for [`Basque`](crate::language::Language::Basque)
    EU,
    /// The ISO 639-1 code for [`Persian`](crate::language::Language::Persian)
    FA,
    /// The ISO 639-1 code for [`Finnish`](crate::language::Language::Finnish)
    FI,
    /// The ISO 639-1 code for [`French`](crate::language::Language::French)
    FR,
    /// The ISO 639-1 code for [`Irish`](crate::language::Language::Irish)
    GA,
    /// The ISO 639-1 code for [`Gujarati`](crate::language::Language::Gujarati)
    GU,
    /// The ISO 639-1 code for [`Hebrew`](crate::language::Language::Hebrew)
    HE,
    /// The ISO 639-1 code for [`Hindi`](crate::language::Language::Hindi)
    HI,
    /// The ISO 639-1 code for [`Croatian`](crate::language::Language::Croatian)
    HR,
    /// The ISO 639-1 code for [`Hungarian`](crate::language::Language::Hungarian)
    HU,
    /// The ISO 639-1 code for [`Armenian`](crate::language::Language::Armenian)
    HY,
    /// The ISO 639-1 code for [`Indonesian`](crate::language::Language::Indonesian)
    ID,
    /// The ISO 639-1 code for [`Icelandic`](crate::language::Language::Icelandic)
    IS,
    /// The ISO 639-1 code for [`Italian`](crate::language::Language::Italian)
    IT,
    /// The ISO 639-1 code for [`Japanese`](crate::language::Language::Japanese)
    JA,
    /// The ISO 639-1 code for [`Georgian`](crate::language::Language::Georgian)
    KA,
    /// The ISO 639-1 code for [`Kazakh`](crate::language::Language::Kazakh)
    KK,
    /// The ISO 639-1 code for [`Korean`](crate::language::Language::Korean)
    KO,
    /// The ISO 639-1 code for [`Latin`](crate::language::Language::Latin)
    LA,
    /// The ISO 639-1 code for [`Ganda`](crate::language::Language::Ganda)
    LG,
    /// The ISO 639-1 code for [`Lithuanian`](crate::language::Language::Lithuanian)
    LT,
    /// The ISO 639-1 code for [`Latvian`](crate::language::Language::Latvian)
    LV,
    /// The ISO 639-1 code for [`Maori`](crate::language::Language::Maori)
    MI,
    /// The ISO 639-1 code for [`Macedonian`](crate::language::Language::Macedonian)
    MK,
    /// The ISO 639-1 code for [`Mongolian`](crate::language::Language::Mongolian)
    MN,
    /// The ISO 639-1 code for [`Marathi`](crate::language::Language::Marathi)
    MR,
    /// The ISO 639-1 code for [`Malay`](crate::language::Language::Malay)
    MS,
    /// The ISO 639-1 code for [`Norwegian Bokmal`](crate::language::Language::Bokmal)
    NB,
    /// The ISO 639-1 code for [`Dutch`](crate::language::Language::Dutch)
    NL,
    /// The ISO 639-1 code for [`Norwegian Nynorsk`](crate::language::Language::Nynorsk)
    NN,
    /// The ISO 639-1 code for [`Punjabi`](crate::language::Language::Punjabi)
    PA,
    /// The ISO 639-1 code for [`Polish`](crate::language::Language::Polish)
    PL,
    /// The ISO 639-1 code for [`Portuguese`](crate::language::Language::Portuguese)
    PT,
    /// The ISO 639-1 code for [`Romanian`](crate::language::Language::Romanian)
    RO,
    /// The ISO 639-1 code for [`Russian`](crate::language::Language::Russian)
    RU,
    /// The ISO 639-1 code for [`Slovak`](crate::language::Language::Slovak)
    SK,
    /// The ISO 639-1 code for [`Slovene`](crate::language::Language::Slovene)
    SL,
    /// The ISO 639-1 code for [`Shona`](crate::language::Language::Shona)
    SN,
    /// The ISO 639-1 code for [`Somali`](crate::language::Language::Somali)
    SO,
    /// The ISO 639-1 code for [`Albanian`](crate::language::Language::Albanian)
    SQ,
    /// The ISO 639-1 code for [`Serbian`](crate::language::Language::Serbian)
    SR,
    /// The ISO 639-1 code for [`Sesotho`](crate::language::Language::Sesotho)
    ST,
    /// The ISO 639-1 code for [`Swedish`](crate::language::Language::Swedish)
    SV,
    /// The ISO 639-1 code for [`Swahili`](crate::language::Language::Swahili)
    SW,
    /// The ISO 639-1 code for [`Tamil`](crate::language::Language::Tamil)
    TA,
    /// The ISO 639-1 code for [`Telugu`](crate::language::Language::Telugu)
    TE,
    /// The ISO 639-1 code for [`Thai`](crate::language::Language::Thai)
    TH,
    /// The ISO 639-1 code for [`Tagalog`](crate::language::Language::Tagalog)
    TL,
    /// The ISO 639-1 code for [`Tswana`](crate::language::Language::Tswana)
    TN,
    /// The ISO 639-1 code for [`Turkish`](crate::language::Language::Turkish)
    TR,
    /// The ISO 639-1 code for [`Tsonga`](crate::language::Language::Tsonga)
    TS,
    /// The ISO 639-1 code for [`Ukrainian`](crate::language::Language::Ukrainian)
    UK,
    /// The ISO 639-1 code for [`Urdu`](crate::language::Language::Urdu)
    UR,
    /// The ISO 639-1 code for [`Vietnamese`](crate::language::Language::Vietnamese)
    VI,
    /// The ISO 639-1 code for [`Xhosa`](crate::language::Language::Xhosa)
    XH,
    /// The ISO 639-1 code for [`Yoruba`](crate::language::Language::Yoruba)
    YO,
    /// The ISO 639-1 code for [`Chinese`](crate::language::Language::Chinese)
    ZH,
    /// The ISO 639-1 code for [`Zulu`](crate::language::Language::Zulu)
    ZU,
}

/// This enum specifies the ISO 639-3 code representations for the supported languages.
///
/// ISO 639 is a standardized nomenclature used to classify languages.
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
#[allow(clippy::upper_case_acronyms)]
#[strum(ascii_case_insensitive)]
#[cfg_attr(
    feature = "python",
    pyo3::prelude::pyclass(eq, eq_int, frozen, hash, ord)
)]
pub enum IsoCode639_3 {
    /// The ISO 639-3 code for [`Afrikaans`](crate::language::Language::Afrikaans)
    AFR,
    /// The ISO 639-3 code for [`Arabic`](crate::language::Language::Arabic)
    ARA,
    /// The ISO 639-3 code for [`Azerbaijani`](crate::language::Language::Azerbaijani)
    AZE,
    /// The ISO 639-3 code for [`Belarusian`](crate::language::Language::Belarusian)
    BEL,
    /// The ISO 639-3 code for [`Bengali`](crate::language::Language::Bengali)
    BEN,
    /// The ISO 639-3 code for [`Bosnian`](crate::language::Language::Bosnian)
    BOS,
    /// The ISO 639-3 code for [`Bulgarian`](crate::language::Language::Bulgarian)
    BUL,
    /// The ISO 639-3 code for [`Catalan`](crate::language::Language::Catalan)
    CAT,
    /// The ISO 639-3 code for [`Czech`](crate::language::Language::Czech)
    CES,
    /// The ISO 639-3 code for [`Welsh`](crate::language::Language::Welsh)
    CYM,
    /// The ISO 639-3 code for [`Danish`](crate::language::Language::Danish)
    DAN,
    /// The ISO 639-3 code for [`German`](crate::language::Language::German)
    DEU,
    /// The ISO 639-3 code for [`Greek`](crate::language::Language::Greek)
    ELL,
    /// The ISO 639-3 code for [`English`](crate::language::Language::English)
    ENG,
    /// The ISO 639-3 code for [`Esperanto`](crate::language::Language::Esperanto)
    EPO,
    /// The ISO 639-3 code for [`Estonian`](crate::language::Language::Estonian)
    EST,
    /// The ISO 639-3 code for [`Basque`](crate::language::Language::Basque)
    EUS,
    /// The ISO 639-3 code for [`Persian`](crate::language::Language::Persian)
    FAS,
    /// The ISO 639-3 code for [`Finnish`](crate::language::Language::Finnish)
    FIN,
    /// The ISO 639-3 code for [`French`](crate::language::Language::French)
    FRA,
    /// The ISO 639-3 code for [`Irish`](crate::language::Language::Irish)
    GLE,
    /// The ISO 639-3 code for [`Gujarati`](crate::language::Language::Gujarati)
    GUJ,
    /// The ISO 639-3 code for [`Hebrew`](crate::language::Language::Hebrew)
    HEB,
    /// The ISO 639-3 code for [`Hindi`](crate::language::Language::Hindi)
    HIN,
    /// The ISO 639-3 code for [`Croatian`](crate::language::Language::Croatian)
    HRV,
    /// The ISO 639-3 code for [`Hungarian`](crate::language::Language::Hungarian)
    HUN,
    /// The ISO 639-3 code for [`Armenian`](crate::language::Language::Armenian)
    HYE,
    /// The ISO 639-3 code for [`Indonesian`](crate::language::Language::Indonesian)
    IND,
    /// The ISO 639-3 code for [`Icelandic`](crate::language::Language::Icelandic)
    ISL,
    /// The ISO 639-3 code for [`Italian`](crate::language::Language::Italian)
    ITA,
    /// The ISO 639-3 code for [`Japanese`](crate::language::Language::Japanese)
    JPN,
    /// The ISO 639-3 code for [`Georgian`](crate::language::Language::Georgian)
    KAT,
    /// The ISO 639-3 code for [`Kazakh`](crate::language::Language::Kazakh)
    KAZ,
    /// The ISO 639-3 code for [`Korean`](crate::language::Language::Korean)
    KOR,
    /// The ISO 639-3 code for [`Latin`](crate::language::Language::Latin)
    LAT,
    /// The ISO 639-3 code for [`Latvian`](crate::language::Language::Latvian)
    LAV,
    /// The ISO 639-3 code for [`Lithuanian`](crate::language::Language::Lithuanian)
    LIT,
    /// The ISO 639-3 code for [`Ganda`](crate::language::Language::Ganda)
    LUG,
    /// The ISO 639-3 code for [`Marathi`](crate::language::Language::Marathi)
    MAR,
    /// The ISO 639-3 code for [`Macedonian`](crate::language::Language::Macedonian)
    MKD,
    /// The ISO 639-3 code for [`Mongolian`](crate::language::Language::Mongolian)
    MON,
    /// The ISO 639-3 code for [`Maori`](crate::language::Language::Maori)
    MRI,
    /// The ISO 639-3 code for [`Malay`](crate::language::Language::Malay)
    MSA,
    /// The ISO 639-3 code for [`Dutch`](crate::language::Language::Dutch)
    NLD,
    /// The ISO 639-3 code for [`Norwegian Nynorsk`](crate::language::Language::Nynorsk)
    NNO,
    /// The ISO 639-3 code for [`Norwegian Bokmal`](crate::language::Language::Bokmal)
    NOB,
    /// The ISO 639-3 code for [`Punjabi`](crate::language::Language::Punjabi)
    PAN,
    /// The ISO 639-3 code for [`Polish`](crate::language::Language::Polish)
    POL,
    /// The ISO 639-3 code for [`Portuguese`](crate::language::Language::Portuguese)
    POR,
    /// The ISO 639-3 code for [`Romanian`](crate::language::Language::Romanian)
    RON,
    /// The ISO 639-3 code for [`Russian`](crate::language::Language::Russian)
    RUS,
    /// The ISO 639-3 code for [`Slovak`](crate::language::Language::Slovak)
    SLK,
    /// The ISO 639-3 code for [`Slovene`](crate::language::Language::Slovene)
    SLV,
    /// The ISO 639-3 code for [`Shona`](crate::language::Language::Shona)
    SNA,
    /// The ISO 639-3 code for [`Somali`](crate::language::Language::Somali)
    SOM,
    /// The ISO 639-3 code for [`Sesotho`](crate::language::Language::Sesotho)
    SOT,
    /// The ISO 639-3 code for [`Spanish`](crate::language::Language::Spanish)
    SPA,
    /// The ISO 639-3 code for [`Albanian`](crate::language::Language::Albanian)
    SQI,
    /// The ISO 639-3 code for [`Serbian`](crate::language::Language::Serbian)
    SRP,
    /// The ISO 639-3 code for [`Swahili`](crate::language::Language::Swahili)
    SWA,
    /// The ISO 639-3 code for [`Swedish`](crate::language::Language::Swedish)
    SWE,
    /// The ISO 639-3 code for [`Tamil`](crate::language::Language::Tamil)
    TAM,
    /// The ISO 639-3 code for [`Telugu`](crate::language::Language::Telugu)
    TEL,
    /// The ISO 639-3 code for [`Tagalog`](crate::language::Language::Tagalog)
    TGL,
    /// The ISO 639-3 code for [`Thai`](crate::language::Language::Thai)
    THA,
    /// The ISO 639-3 code for [`Tswana`](crate::language::Language::Tswana)
    TSN,
    /// The ISO 639-3 code for [`Tsonga`](crate::language::Language::Tsonga)
    TSO,
    /// The ISO 639-3 code for [`Turkish`](crate::language::Language::Turkish)
    TUR,
    /// The ISO 639-3 code for [`Ukrainian`](crate::language::Language::Ukrainian)
    UKR,
    /// The ISO 639-3 code for [`Urdu`](crate::language::Language::Urdu)
    URD,
    /// The ISO 639-3 code for [`Vietnamese`](crate::language::Language::Vietnamese)
    VIE,
    /// The ISO 639-3 code for [`Xhosa`](crate::language::Language::Xhosa)
    XHO,
    /// The ISO 639-3 code for [`Yoruba`](crate::language::Language::Yoruba)
    YOR,
    /// The ISO 639-3 code for [`Chinese`](crate::language::Language::Chinese)
    ZHO,
    /// The ISO 639-3 code for [`Zulu`](crate::language::Language::Zulu)
    ZUL,
}

impl Display for IsoCode639_1 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

impl Display for IsoCode639_3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let debug_repr = format!("{self:?}");
        write!(f, "{}", debug_repr.to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::str::FromStr;

    #[test]
    fn assert_iso_code_639_1_string_representation_is_correct() {
        assert_eq!(IsoCode639_1::EN.to_string(), "en");
    }

    #[test]
    fn assert_iso_code_639_3_string_representation_is_correct() {
        assert_eq!(IsoCode639_3::ENG.to_string(), "eng");
    }

    #[test]
    fn assert_string_to_iso_code_639_1_is_correct() {
        assert_eq!(IsoCode639_1::from_str("en").unwrap(), IsoCode639_1::EN);
    }

    #[test]
    fn assert_string_to_iso_code_639_3_is_correct() {
        assert_eq!(IsoCode639_3::from_str("eng").unwrap(), IsoCode639_3::ENG);
    }
}
