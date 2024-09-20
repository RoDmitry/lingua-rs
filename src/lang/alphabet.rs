use super::{Language, Script};
use ::std::fmt::Debug;
use alphabet_match_macro::alphabet_match;
/* use std::str::FromStr;
use std::string::ToString;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::IntoStaticStr; */

/* macro_rules! alphabets_filter {
    ($var:ident $inner:ident) => {
        None
    };
    ($var:ident) => {
        Some($var)
    };
}
macro_rules! main_alphabet {
    (
        $( #[$meta:meta] )*
        $vis:vis enum $name:ident {
            $($var:ident$(($inner:ident))?),* $(,)?
        }
    ) => {
        $( #[$meta] )*
        $vis enum $name {
            $($var$(($inner))?,)*
        }
        impl $name {
            pub const ALL: &'static [Option<$name>] = {
                use $name::*;
                &[
                    $(
                        alphabets_filter!($var $($inner)?),
                    )*
                ]
            };
        }
    };
} */

// main_alphabet!(
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, strum_macros::Display)]
pub enum Alphabet {
    Cyrillic(CyrillicAlphabet),
    Han(HanAlphabet),
    Latin(LatinAlphabet),
    Script(Script),
}
// );

#[derive(Clone, Copy, Debug)]
pub enum ScriptAlphabets {
    Alphabets(&'static [Alphabet]),
    Script(Option<Script>),
}

impl From<(Option<Script>, Option<&'static [Alphabet]>)> for ScriptAlphabets {
    fn from((os, oa): (Option<Script>, Option<&'static [Alphabet]>)) -> Self {
        if let Some(a) = oa {
            Self::Alphabets(a)
        } else {
            Self::Script(os)
        }
    }
}

impl ScriptAlphabets {
    pub fn iter(&self) -> ScriptAlphabetIter {
        match self {
            Self::Alphabets(a) => ScriptAlphabetIter::Iter(a.iter().copied()),
            Self::Script(s) => ScriptAlphabetIter::Script(*s),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Alphabets(a) => a.is_empty(),
            Self::Script(s) => s.is_none(),
        }
    }
}

pub enum ScriptAlphabetIter {
    Iter(::std::iter::Copied<::std::slice::Iter<'static, Alphabet>>),
    Script(Option<Script>),
}

impl Iterator for ScriptAlphabetIter {
    type Item = Alphabet;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Iter(iter) => iter.next(),
            Self::Script(s) => s.take().map(|s| Alphabet::Script(s)),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            Self::Iter(iter) => iter.size_hint(),
            Self::Script(s) => {
                if s.is_some() {
                    (1, Some(1))
                } else {
                    (0, Some(0))
                }
            }
        }
    }
}

fn type_of<T>(_: &T) -> &str {
    std::any::type_name::<T>()
}

impl Alphabet {
    /* #[allow(dead_code)]
    fn iter() -> impl Iterator<Item = &'static Self> {
        Self::ALL
            .iter()
            .filter_map(|a| a.as_ref())
            .chain(LatinAlphabet::ALPHABETS)
    } */

    pub fn to_full_dbg(&self) -> String {
        match self {
            Self::Latin(v) => {
                self.to_string()
                    + "("
                    + type_of(v).split("::").last().unwrap()
                    + "::"
                    + &v.to_string()
                    + ")"
            }
            v => v.to_string(),
        }
    }
}

impl From<Alphabet> for &'static [Language] {
    fn from(a: Alphabet) -> Self {
        match a {
            Alphabet::Cyrillic(ca) => {
                use CyrillicAlphabet::*;
                match ca {
                    Bashkir => &[Language::Bashkir],
                    Belarusian => &[Language::Belarusian],
                    Bulgarian => &[Language::Bulgarian],
                    Kazakh => &[Language::Kazakh],
                    Kyrgyz => &[Language::Kyrgyz],
                    Macedonian => &[Language::Macedonian],
                    MongolianHalh => &[Language::MongolianHalh],
                    OldChurchSlavonic => &[Language::OldChurchSlavonic],
                    Russian => &[Language::Russian],
                    Serbian => &[Language::Serbian],
                    Tajik => &[Language::Tajik],
                    Tatar => &[Language::Tatar],
                    Ukrainian => &[Language::Ukrainian],
                }
            }
            Alphabet::Latin(la) => {
                use LatinAlphabet::*;
                match la {
                    Acehnese => &[Language::Acehnese],
                    Afrikaans => &[Language::Afrikaans],
                    Albanian => &[Language::AlbanianTosk],
                    Asturian => &[Language::Asturian],
                    AymaraCentral => &[Language::AymaraCentral],
                    Azerbaijani => &[Language::AzerbaijaniNorth],
                    Balinese => &[Language::Balinese],
                    Bambara => &[Language::Bambara],
                    Banjar => &[Language::Banjar],
                    Basque => &[Language::Basque],
                    Bemba => &[Language::Bemba],
                    Bokmal => &[Language::Bokmal],
                    Bosnian => &[Language::Bosnian],
                    Buginese => &[Language::Buginese],
                    Catalan => &[Language::Catalan],
                    Cebuano => &[Language::Cebuano],
                    Chokwe => &[Language::Chokwe],
                    CreoleHaitian => &[Language::CreoleHaitian],
                    Croatian => &[Language::Croatian],
                    Czech => &[Language::Czech],
                    Danish => &[Language::Danish],
                    DinkaSouthwestern => &[Language::DinkaSouthwestern],
                    Dutch => &[Language::Dutch],
                    Dyula => &[Language::Dyula],
                    English => &[Language::English],
                    Esperanto => &[Language::Esperanto],
                    Estonian => &[Language::Estonian],
                    Ewe => &[Language::Ewe],
                    Faroese => &[Language::Faroese],
                    Fijian => &[Language::Fijian],
                    Finnish => &[Language::Finnish],
                    Fon => &[Language::Fon],
                    French => &[Language::French],
                    Friulian => &[Language::Friulian],
                    FulfuldeNigerian => &[Language::FulfuldeNigerian],
                    GaelicScottish => &[Language::GaelicScottish],
                    Galician => &[Language::Galician],
                    Ganda => &[Language::Ganda],
                    German => &[Language::German],
                    Guarani => &[Language::Guarani],
                    Hausa => &[Language::Hausa],
                    Hungarian => &[Language::Hungarian],
                    Icelandic => &[Language::Icelandic],
                    Igbo => &[Language::Igbo],
                    Ilocano => &[Language::Ilocano],
                    Indonesian => &[Language::Indonesian],
                    Irish => &[Language::Irish],
                    Italian => &[Language::Italian],
                    Javanese => &[Language::Javanese],
                    Jingpho => &[Language::Jingpho],
                    Kabiye => &[Language::Kabiye],
                    Kabuverdianu => &[Language::Kabuverdianu],
                    Kabyle => &[Language::Kabyle],
                    Kamba => &[Language::Kamba],
                    KanuriCentral => &[Language::KanuriCentral],
                    Kikongo => &[Language::Kikongo],
                    Kikuyu => &[Language::Kikuyu],
                    Kimbundu => &[Language::Kimbundu],
                    Kinyarwanda => &[Language::Kinyarwanda],
                    KurdishNorthern => &[Language::KurdishNorthern],
                    Latgalian => &[Language::Latgalian],
                    Latin => &[Language::Latin],
                    Latvian => &[Language::Latvian],
                    Ligurian => &[Language::Ligurian],
                    Limburgish => &[Language::Limburgish],
                    Lingala => &[Language::Lingala],
                    Lithuanian => &[Language::Lithuanian],
                    Lombard => &[Language::Lombard],
                    LubaKasai => &[Language::LubaKasai],
                    Luo => &[Language::Luo],
                    Luxembourgish => &[Language::Luxembourgish],
                    Malay => &[Language::Malay, Language::MalayStandard],
                    MalgasyPlateau => &[Language::MalgasyPlateau],
                    Maltese => &[Language::Maltese],
                    Maori => &[Language::Maori],
                    Minangkabau => &[Language::Minangkabau],
                    Mizo => &[Language::Mizo],
                    Mossi => &[Language::Mossi],
                    Nuer => &[Language::Nuer],
                    Nyanja => &[Language::Nyanja],
                    Nynorsk => &[Language::Nynorsk],
                    Occitan => &[Language::Occitan],
                    OromoWestCentral => &[Language::OromoWestCentral],
                    Pangasinan => &[Language::Pangasinan],
                    Papiamento => &[Language::Papiamento],
                    Polish => &[Language::Polish],
                    Portuguese => &[Language::Portuguese],
                    QuechuaAyacucho => &[Language::QuechuaAyacucho],
                    Romanian => &[Language::Romanian],
                    Rundi => &[Language::Rundi],
                    Samoan => &[Language::Samoan],
                    Sango => &[Language::Sango],
                    Sardinian => &[Language::Sardinian],
                    Sepedi => &[Language::Sepedi],
                    Sesotho => &[Language::Sesotho],
                    Shona => &[Language::Shona],
                    Sicilian => &[Language::Sicilian],
                    Silesian => &[Language::Silesian],
                    Slovak => &[Language::Slovak],
                    Slovene => &[Language::Slovene],
                    Somali => &[Language::Somali],
                    Spanish => &[Language::Spanish],
                    Sundanese => &[Language::Sundanese],
                    Swahili => &[Language::Swahili],
                    Swati => &[Language::Swati],
                    Swedish => &[Language::Swedish],
                    Tagalog => &[Language::Tagalog],
                    Tamasheq => &[Language::Tamasheq],
                    TatarCrimean => &[Language::TatarCrimean],
                    TokPisin => &[Language::TokPisin],
                    Tsonga => &[Language::Tsonga],
                    Tswana => &[Language::Tswana],
                    Tumbuka => &[Language::Tumbuka],
                    Turkish => &[Language::Turkish],
                    Turkmen => &[Language::Turkmen],
                    Twi => &[Language::Twi],
                    Umbundu => &[Language::Umbundu],
                    UzbekNorthern => &[Language::UzbekNorthern],
                    Venetian => &[Language::Venetian],
                    Vietnamese => &[Language::Vietnamese],
                    Waray => &[Language::Waray],
                    Welsh => &[Language::Welsh],
                    Wolof => &[Language::Wolof],
                    Xhosa => &[Language::Xhosa],
                    Yoruba => &[Language::Yoruba],
                    Zulu => &[Language::Zulu],
                }
            }
            Alphabet::Han(ha) => {
                use HanAlphabet::*;
                match ha {
                    ChineseSimplified => &[
                        Language::Chinese,
                        Language::ChineseCantonese,
                        Language::ChineseMandarin,
                    ],
                    ChineseTraditional => &[
                        Language::Chinese,
                        Language::ChineseCantonese,
                        Language::ChineseMandarin,
                    ],
                    Hanja => &[Language::Korean],
                    Kanji => &[Language::Japanese],
                }
            }
            Alphabet::Script(s) => {
                use Script::*;
                match s {
                    Adlam => &[Language::Fulani, Language::Pular],
                    Ahom => &[Language::Ahom],
                    AnatolianHieroglyphs => &[Language::Luwian],
                    Arabic => &[
                        Language::Acehnese,
                        Language::Arabic,
                        Language::ArabicEgyptian,
                        Language::ArabicMesopotamian,
                        Language::ArabicMoroccan,
                        Language::ArabicNajdi,
                        Language::ArabicNorthLevantine,
                        Language::ArabicSouthernYemeni,
                        Language::ArabicSouthLevantine,
                        Language::ArabicTunisian,
                        Language::AzerbaijaniSouth,
                        Language::Banjar,
                        Language::Dari,
                        Language::KanuriCentral,
                        Language::Kashmiri,
                        Language::Kurdish,
                        Language::KurdishCentral,
                        Language::Pashto,
                        Language::PastoSouthern,
                        Language::Persian,
                        Language::PersianWestern,
                        Language::Sindhi,
                        Language::Urdu,
                        Language::Uyghur,
                    ],
                    Armenian => &[Language::Armenian],
                    Avestan => &[Language::Avestan],
                    Balinese => &[Language::Balinese],
                    Bamum => &[Language::Bamum],
                    BassaVah => &[Language::Bassa],
                    Batak => &[
                        Language::Angkola,
                        Language::Karo,
                        Language::Mandailing,
                        Language::Pakpak,
                        Language::Simalungun,
                        Language::Toba,
                    ],
                    Bengali => &[
                        Language::Assamese,
                        Language::Bengali,
                        Language::BishnupriyaManipuri,
                        Language::Meitei,
                    ],
                    Bhaiksuki => &[Language::Bhaiksuki],
                    Bopomofo => &[Language::ChineseMandarin],
                    Brahmi => &[Language::Sanskrit, Language::Prakrit],
                    Braille => &[Language::Braille],
                    Buginese => &[Language::Buginese, Language::Makassarese],
                    Buhid => &[Language::Buhid],
                    CanadianAboriginal => &[Language::Cree, Language::Inuktitut, Language::Ojibwe],
                    Carian => &[Language::Carian],
                    CaucasianAlbanian => &[Language::CaucasianAlbanian],
                    Chakma => &[Language::Chakma],
                    Cham => &[Language::Cham],
                    Cherokee => &[Language::Cherokee],
                    Nushu => &[Language::ChineseTuhua],
                    Chorasmian => &[Language::Chorasmian],
                    Coptic => &[Language::Coptic],
                    Cuneiform => &[Language::Akkadian, Language::Hittite, Language::Sumerian],
                    Cypriot => &[Language::AncientGreek],
                    CyproMinoan => &[Language::CyproMinoan],
                    Deseret => &[Language::EnglishMormon],
                    Devanagari => &[
                        Language::Awadhi,
                        Language::Bhojpuri,
                        Language::Chhattisgarhi,
                        Language::Hindi,
                        Language::Kashmiri,
                        Language::Magahi,
                        Language::Maithili,
                        Language::Marathi,
                        Language::Marathi,
                        Language::Nepali,
                        Language::Sanskrit,
                    ],
                    DivesAkuru => &[Language::MaldivianDhivehi],
                    Dogra => &[Language::Dogri],
                    Duployan => &[Language::EnglishDuployan, Language::FrenchDuployan],
                    EgyptianHieroglyphs => &[Language::EgyptianHieroglyphs],
                    Elbasan => &[Language::AlbanianHistorical],
                    Elymaic => &[Language::Elymaic],
                    Ethiopic => &[
                        Language::Amharic,
                        Language::Geez,
                        Language::Oromo,
                        Language::Tigrinya,
                    ],
                    Garay => &[Language::Wolof],
                    Georgian => &[Language::Georgian],
                    Glagolitic => &[Language::OldChurchSlavonic],
                    Gothic => &[Language::Gothic],
                    Grantha => &[Language::Sanskrit, Language::Tamil],
                    Greek => &[Language::Greek],
                    Gujarati => &[Language::Gujarati],
                    GunjalaGondi => &[Language::Gondi],
                    Gurmukhi => &[Language::PunjabiEastern],
                    GurungKhema => &[Language::Gurung],
                    Hangul => &[Language::Korean],
                    HanifiRohingya => &[Language::Rohingya],
                    Hanunoo => &[Language::Hanunoo],
                    Hatran => &[Language::Aramaic],
                    Hebrew => &[
                        Language::Hebrew,
                        Language::Yiddish,
                        Language::YiddishEastern,
                    ],
                    Hiragana => &[Language::Japanese],
                    ImperialAramaic => &[Language::Aramaic],
                    InscriptionalPahlavi => &[Language::MiddlePersian],
                    InscriptionalParthian => &[Language::Parthian],
                    Javanese => &[Language::Javanese],
                    Kaithi => &[Language::Bhojpuri, Language::Magahi, Language::Maithili],
                    Kannada => &[Language::Kannada],
                    Katakana => &[Language::Japanese],
                    Kawi => &[Language::OldJavanese, Language::Sanskrit],
                    KayahLi => &[Language::KayahLi],
                    Kharoshthi => &[Language::Gandhari],
                    KhitanSmallScript => &[Language::Khitan],
                    Khmer => &[Language::Khmer],
                    Khojki => &[Language::Sindhi, Language::Khoja],
                    Khudawadi => &[Language::Sindhi],
                    KiratRai => &[Language::Bantawa],
                    Lao => &[Language::Lao],
                    Lepcha => &[Language::Lepcha],
                    Limbu => &[Language::Limbu],
                    LinearA => &[Language::Minoan],
                    LinearB => &[Language::MycenaeanGreek],
                    Lisu => &[Language::Lisu],
                    Lycian => &[Language::Lycian],
                    Lydian => &[Language::Lydian],
                    Mahajani => &[Language::Hindi, Language::Marwari, Language::PunjabiEastern],
                    Makasar => &[Language::Makasar],
                    Malayalam => &[Language::Malayalam],
                    Mandaic => &[Language::Mandaic, Language::Aramaic],
                    Manichaean => &[Language::MiddlePersian, Language::Sogdian],
                    Marchen => &[Language::BuddhistMarchen],
                    MasaramGondi => &[Language::Gondi],
                    Medefaidrin => &[Language::Medefaidrin],
                    MeeteiMayek => &[Language::ManipuriMeetei],
                    MendeKikakui => &[Language::Mende],
                    MeroiticCursive => &[Language::Meroitic],
                    MeroiticHieroglyphs => &[Language::Meroitic],
                    Miao => &[Language::Hmong],
                    Modi => &[Language::Marathi],
                    Mongolian => &[Language::MongolianHalh],
                    Mro => &[Language::Mro],
                    Multani => &[Language::Saraiki],
                    NagMundari => &[Language::Mundari],
                    Myanmar => &[Language::Burmese, Language::Shan],
                    Nabataean => &[Language::Aramaic],
                    Nandinagari => &[Language::Sanskrit],
                    Newa => &[Language::Newari],
                    NewTaiLue => &[Language::TaiLue],
                    Nko => &[Language::Mande],
                    NyiakengPuachueHmong => &[Language::Hmong],
                    Ogham => &[Language::OldIrish],
                    OlChiki => &[Language::Santali],
                    OldHungarian => &[Language::OldHungarian],
                    OldItalic => &[Language::Etruscan, Language::Oscan, Language::Umbrian],
                    OldPermic => &[Language::OldKomi],
                    OldNorthArabian => &[Language::OldNorthArabian],
                    OldPersian => &[Language::OldPersian],
                    OldSogdian => &[Language::OldSogdian],
                    OldSouthArabian => &[Language::OldSouthArabian],
                    OldTurkic => &[Language::OldTurkic],
                    OldUyghur => &[Language::OldUyghur],
                    OlOnal => &[Language::Ho],
                    Oriya => &[Language::Odia],
                    Osage => &[Language::Osage],
                    Osmanya => &[Language::Somali],
                    PahawhHmong => &[Language::Hmong],
                    Palmyrene => &[Language::Aramaic],
                    PauCinHau => &[Language::ZoLanguages],
                    PhagsPa => &[Language::MongolianHalh, Language::Tibetan],
                    Phoenician => &[Language::Phoenician],
                    PsalterPahlavi => &[Language::MiddlePersian],
                    Rejang => &[Language::Rejang],
                    Runic => &[Language::OldNorse, Language::OldEnglish],
                    Samaritan => &[Language::Hebrew],
                    Saurashtra => &[Language::Saurashtra],
                    Sharada => &[Language::Sanskrit, Language::Kashmiri],
                    Shavian => &[Language::EnglishPhonetic],
                    Siddham => &[Language::Sanskrit],
                    SignWriting => &[Language::Signlanguages],
                    Sinhala => &[Language::Sinhala],
                    Sogdian => &[Language::Sogdian],
                    SoraSompeng => &[Language::Sora],
                    Soyombo => &[
                        Language::MongolianHalh,
                        Language::Sanskrit,
                        Language::Tibetan,
                    ],
                    Sundanese => &[Language::Sundanese],
                    Sunuwar => &[Language::Sunuwar],
                    SylotiNagri => &[Language::Sylheti],
                    Syriac => &[Language::Syriac, Language::Aramaic],
                    Tagalog => &[Language::Tagalog],
                    Tagbanwa => &[Language::Tagbanwa],
                    TaiLe => &[Language::TaiLe],
                    TaiTham => &[Language::Lao, Language::NorthernThai, Language::TaiLue],
                    TaiViet => &[Language::TaiDam, Language::TaiDon],
                    Takri => &[Language::Dogri, Language::Kashmiri],
                    Tamil => &[Language::Tamil],
                    Tangsa => &[Language::Tangsa],
                    Tangut => &[Language::Tangut],
                    Telugu => &[Language::Telugu],
                    Thaana => &[Language::MaldivianDhivehi],
                    Thai => &[Language::Thai],
                    Tibetan => &[Language::Dzongkha, Language::Tibetan],
                    Tifinagh => &[
                        Language::Berber,
                        Language::Tamasheq,
                        Language::TamazightCentralAtlas,
                    ],
                    Tirhuta => &[Language::Maithili],
                    Todhri => &[Language::AlbanianHistorical],
                    Toto => &[Language::Toto],
                    TuluTigalari => &[Language::Sanskrit, Language::Tulu, Language::Kannada],
                    Ugaritic => &[Language::Ugaritic],
                    Vai => &[Language::Vai],
                    Vithkuqi => &[Language::AlbanianTosk],
                    Wancho => &[Language::Wancho],
                    WarangCiti => &[Language::Ho],
                    Yezidi => &[Language::Kurdish],
                    Yi => &[Language::Yi],
                    ZanabazarSquare => &[
                        Language::MongolianHalh,
                        Language::Sanskrit,
                        Language::Tibetan,
                    ],
                    _ => &[],
                }
            }
        }
    }
}

macro_rules! alphabet_wrapper {
    (
        $( #[$meta:meta] )*
        $vis:vis enum $name:ident in $parent:ident {
            $($var:ident),* $(,)?
        }
    ) => {
        $( #[$meta] )*
        $vis enum $name {
            $($var,)*
        }
        impl $name {
            /* pub const ALL: &'static [Self] = {
                use $name::*;
                &[ $($var),* ]
            }; */
            pub const ALPHABETS: &'static [Alphabet] = {
                use $name::*;
                &[ $(Alphabet::$parent($var)),* ]
            };
        }
    };
}

alphabet_wrapper!(
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, strum_macros::Display)]
pub enum HanAlphabet in Han {
    ChineseSimplified,
    ChineseTraditional,
    Hanja, // Korean
    Kanji, // Japanese
}
);

alphabet_wrapper!(
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, strum_macros::Display)]
pub enum CyrillicAlphabet in Cyrillic {
    Bashkir,
    Belarusian,
    Bulgarian,
    Kazakh,
    Kyrgyz,
    Macedonian,
    MongolianHalh,
    OldChurchSlavonic,
    Russian,
    Serbian,
    Tajik,
    Tatar,
    Ukrainian,
}
);

alphabet_wrapper!(
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, strum_macros::Display)]
pub enum LatinAlphabet in Latin {
    Acehnese,
    Afrikaans,
    Albanian,
    Asturian,
    AymaraCentral,
    Azerbaijani,
    Balinese,
    Bambara,
    Banjar,
    Basque,
    Bemba,
    Bokmal,
    Bosnian,
    Buginese,
    Catalan,
    Cebuano,
    Chokwe,
    CreoleHaitian,
    Croatian,
    Czech,
    Danish,
    DinkaSouthwestern,
    Dutch,
    Dyula,
    English,
    Esperanto,
    Estonian,
    Ewe,
    Faroese,
    Fijian,
    Finnish,
    Fon,
    French,
    Friulian,
    FulfuldeNigerian,
    GaelicScottish,
    Galician,
    Ganda,
    German,
    Guarani,
    Hausa,
    Hungarian,
    Icelandic,
    Igbo,
    Ilocano,
    Indonesian,
    Irish,
    Italian,
    Javanese,
    Jingpho,
    Kabiye,
    Kabuverdianu,
    Kabyle,
    Kamba,
    KanuriCentral,
    Kikongo,
    Kikuyu,
    Kimbundu,
    Kinyarwanda,
    KurdishNorthern,
    Latgalian,
    Latin,
    Latvian,
    Ligurian,
    Limburgish,
    Lingala,
    Lithuanian,
    Lombard,
    LubaKasai,
    Luo,
    Luxembourgish,
    Malay,
    MalgasyPlateau,
    Maltese,
    Maori,
    Minangkabau,
    Mizo,
    Mossi,
    Nuer,
    Nyanja,
    Nynorsk,
    Occitan,
    OromoWestCentral,
    Pangasinan,
    Papiamento,
    Polish,
    Portuguese,
    QuechuaAyacucho,
    Romanian,
    Rundi,
    Samoan,
    Sango,
    Sardinian,
    Sepedi,
    Sesotho,
    Shona,
    Sicilian,
    Silesian,
    Slovak,
    Slovene,
    Somali,
    Spanish,
    Sundanese,
    Swahili,
    Swati,
    Swedish,
    Tagalog,
    Tamasheq,
    TatarCrimean,
    TokPisin,
    Tsonga,
    Tswana,
    Tumbuka,
    Turkish,
    Turkmen,
    Twi,
    Umbundu,
    UzbekNorthern,
    Venetian,
    Vietnamese,
    Waray,
    Welsh,
    Wolof,
    Xhosa,
    Yoruba,
    Zulu,
}
);

/*
ace_Arab
hye_Armn
asm_Beng
bak_Cyrl
awa_Deva
amh_Ethi
kat_Geor
ell_Grek
guj_Gujr
pan_Guru
kor_Hang
zho_Hans
yue_Hant
heb_Hebr
jpn_Jpan
khm_Khmr
kan_Knda
lao_Laoo
ace_Latn
mal_Mlym
mya_Mymr
sat_Olck
ory_Orya
sin_Sinh
tam_Taml
tel_Telu
taq_Tfng
tha_Thai
bod_Tibt
*/

// static ASD: Vec<Alphabet> = LatinAlphabet::iter().map(|la| Alphabet::Latin(la)).collect::<Vec<_>>();

pub fn str_to_alphabets(s: &str) -> &[Alphabet] {
    // use Alphabet::*;
    match s {
        "Arab" => &[Alphabet::Script(Script::Arabic)],
        "Armn" => &[Alphabet::Script(Script::Armenian)],
        "Beng" => &[Alphabet::Script(Script::Bengali)],
        "Cyrl" => CyrillicAlphabet::ALPHABETS,
        "Deva" => &[Alphabet::Script(Script::Devanagari)],
        "Ethi" => &[Alphabet::Script(Script::Ethiopic)],
        "Geor" => &[Alphabet::Script(Script::Georgian)],
        "Grek" => &[Alphabet::Script(Script::Greek)],
        "Gujr" => &[Alphabet::Script(Script::Gujarati)],
        "Guru" => &[Alphabet::Script(Script::Gurmukhi)],
        "Hang" => &[Alphabet::Script(Script::Hangul)],
        "Hans" => &[Alphabet::Han(HanAlphabet::ChineseSimplified)],
        "Hant" => &[Alphabet::Han(HanAlphabet::ChineseTraditional)],
        "Hebr" => &[Alphabet::Script(Script::Hebrew)],
        "JpanKanji" => &[Alphabet::Han(HanAlphabet::Kanji)],
        "Khmr" => &[Alphabet::Script(Script::Khmer)],
        "Knda" => &[Alphabet::Script(Script::Kannada)],
        "Laoo" => &[Alphabet::Script(Script::Lao)],
        "Latn" => LatinAlphabet::ALPHABETS,
        "Mlym" => &[Alphabet::Script(Script::Malayalam)],
        "Mymr" => &[Alphabet::Script(Script::Myanmar)],
        "Olck" => &[Alphabet::Script(Script::OlChiki)],
        "Orya" => &[Alphabet::Script(Script::Oriya)],
        "Sinh" => &[Alphabet::Script(Script::Sinhala)],
        "Taml" => &[Alphabet::Script(Script::Tamil)],
        "Telu" => &[Alphabet::Script(Script::Telugu)],
        "Tfng" => &[Alphabet::Script(Script::Tifinagh)],
        "Thai" => &[Alphabet::Script(Script::Thai)],
        "Tibt" => &[Alphabet::Script(Script::Tibetan)],
        _ => &[],
    }
}

/* #[cfg(test)]
mod test {
    use super::*;
    use ahash::AHashSet;
    use strum::IntoEnumIterator;

    #[test]
    fn test_language_missing_in_from_alphabet_to_language() {
        let lang_len = Language::iter().size_hint().0;
        let mut langs_set: AHashSet<Language> = AHashSet::with_capacity(lang_len);
        for &alphabet in Alphabet::iter() {
            let langs: &[Language] = alphabet.into();
            langs_set.extend(langs);
        }
        if langs_set.len() != lang_len {
            let missing: Vec<_> = Language::iter().filter(|l| !langs_set.remove(&l)).collect();
            panic!(
                "Not all languages used in `from alphabet to language`, missing: {:?}",
                missing
            );
        }
    }

    #[test]
    fn test_alphabets_missing_in_fn_script_alphabets() {
        let alphabets_len = Alphabet::iter().count();
        let mut alphabets_set: AHashSet<Alphabet> = AHashSet::with_capacity(alphabets_len);
        for script in Script::iter() {
            let alphabets = script_char_to_alphabets(script, '\0');
            alphabets_set.extend(alphabets);
        }
        if alphabets_set.len() != alphabets_len {
            let missing: Vec<_> = Alphabet::iter()
                .filter(|a| !alphabets_set.remove(a))
                .collect();
            panic!(
                "Not all alphabets used in `script_alphabets`, missing: {:?}",
                missing
            );
        }
    }
} */

pub(crate) fn char_combine(s: Option<Script>, ch: char, extra: char) -> char {
    match s {
        Some(Script::Latin) => match extra {
            '\u{300}' | '\u{340}' => match ch {
                'a' => 'á',
                'A' => 'Á',
                'c' => 'ć',
                'C' => 'Ć',
                'e' => 'é',
                'E' => 'É',
                'i' => 'í',
                'I' => 'Í',
                'l' => 'ĺ',
                'L' => 'Ĺ',
                'n' => 'ń',
                'N' => 'Ń',
                'o' => 'ó',
                'O' => 'Ó',
                'r' => 'ŕ',
                'R' => 'Ŕ',
                's' => 'ś',
                'S' => 'Ś',
                'u' => 'ú',
                'U' => 'Ú',
                'y' => 'ý',
                'Y' => 'Ý',
                'z' => 'ź',
                'Z' => 'Ź',
                _ => ch,
            },
            '\u{301}' | '\u{341}' => match ch {
                'a' => 'à',
                'A' => 'À',
                'e' => 'è',
                'E' => 'È',
                'i' => 'ì',
                'I' => 'Ì',
                'o' => 'ò',
                'O' => 'Ò',
                'u' => 'ù',
                'U' => 'Ù',
                _ => ch,
            },
            '\u{302}' => match ch {
                'a' => 'â',
                'A' => 'Â',
                'c' => 'ĉ',
                'C' => 'Ĉ',
                'e' => 'ê',
                'E' => 'Ê',
                'g' => 'ĝ',
                'G' => 'Ĝ',
                'h' => 'ĥ',
                'H' => 'Ĥ',
                'i' => 'î',
                'I' => 'Î',
                'j' => 'ĵ',
                'J' => 'Ĵ',
                'o' => 'ô',
                'O' => 'Ô',
                's' => 'ŝ',
                'S' => 'Ŝ',
                'u' => 'û',
                'U' => 'Û',
                'w' => 'ŵ',
                'W' => 'Ŵ',
                'y' => 'ŷ',
                'Y' => 'Ŷ',
                _ => ch,
            },
            '\u{308}' => match ch {
                'a' => 'ä',
                'A' => 'Ä',
                'e' => 'ë',
                'E' => 'Ë',
                'i' => 'ï',
                'I' => 'Ï',
                'o' => 'ö',
                'O' => 'Ö',
                'u' => 'ü',
                'U' => 'Ü',
                'y' => 'ÿ',
                'Y' => 'Ÿ',
                _ => ch,
            },
            '\u{303}' | '\u{342}' => match ch {
                'a' => 'ã',
                'A' => 'Ã',
                'e' => 'ẽ',
                'E' => 'Ẽ',
                'i' => 'ĩ',
                'I' => 'Ĩ',
                'n' => 'ñ',
                'N' => 'Ñ',
                'o' => 'õ',
                'O' => 'Õ',
                'y' => 'ỹ',
                'Y' => 'Ỹ',
                _ => ch,
            },
            '\u{327}' => match ch {
                'c' => 'ç',
                'C' => 'Ç',
                'g' => 'ģ',
                'G' => 'Ģ',
                'k' => 'ķ',
                'K' => 'Ķ',
                'l' => 'ļ',
                'L' => 'Ļ',
                'n' => 'ņ',
                'N' => 'Ņ',
                's' => 'ş',
                'S' => 'Ş',
                't' => 'ţ',
                'T' => 'Ţ',
                _ => ch,
            },
            '\u{30A}' => match ch {
                'a' => 'å',
                'A' => 'Å',
                'u' => 'ů',
                'U' => 'Ů',
                _ => ch,
            },
            '\u{30C}' => match ch {
                'a' => 'ǎ',
                'A' => 'Ǎ',
                'c' => 'č',
                'C' => 'Č',
                'd' => 'ď',
                'D' => 'Ď',
                'e' => 'ě',
                'E' => 'Ě',
                'g' => 'ğ',
                'G' => 'Ğ',
                'o' => 'ǒ',
                'O' => 'Ǒ',
                'r' => 'ř',
                'R' => 'Ř',
                's' => 'š',
                'S' => 'Š',
                't' => 'ť',
                'T' => 'Ť',
                'u' => 'ŭ',
                'U' => 'Ŭ',
                'z' => 'ž',
                'Z' => 'Ž',
                _ => ch,
            },
            '\u{30B}' => match ch {
                'u' => 'Ű',
                'U' => 'ű',
                _ => ch,
            },
            '\u{304}' => match ch {
                'a' => 'ā',
                'A' => 'Ā',
                'e' => 'ē',
                'E' => 'Ē',
                'i' => 'ī',
                'I' => 'Ī',
                'o' => 'ō',
                'O' => 'Ō',
                'u' => 'ū',
                'U' => 'Ū',
                _ => ch,
            },
            '\u{328}' => match ch {
                'a' => 'ą',
                'A' => 'Ą',
                'e' => 'ę',
                'E' => 'Ę',
                'u' => 'ų',
                'U' => 'Ų',
                _ => ch,
            },
            '\u{307}' => match ch {
                'c' => 'ċ',
                'C' => 'Ċ',
                'g' => 'ġ',
                'G' => 'Ġ',
                'I' => 'İ',
                'z' => 'ż',
                'Z' => 'Ż',
                _ => ch,
            },
            '\u{326}' => match ch {
                's' => 'ș',
                'S' => 'Ș',
                't' => 'ț',
                'T' => 'Ț',
                _ => ch,
            },
            '\u{323}' => match ch {
                'a' => 'ạ',
                'A' => 'Ạ',
                'ǎ' => 'ặ',
                'Ǎ' => 'Ặ',
                'â' => 'ậ',
                'Â' => 'Ậ',
                'e' => 'ẹ',
                'E' => 'Ẹ',
                'ê' => 'ệ',
                'Ê' => 'Ệ',
                'i' => 'ị',
                'I' => 'Ị',
                'ơ' => 'ợ',
                'Ơ' => 'Ợ',
                'o' => 'ọ',
                'O' => 'Ọ',
                'ô' => 'ộ',
                'Ô' => 'Ộ',
                's' => 'ṣ',
                'S' => 'Ṣ',
                'u' => 'ụ',
                'U' => 'Ụ',
                'y' => 'ỵ',
                'Y' => 'Ỵ',
                _ => ch,
            },
            '\u{309}' => match ch {
                'a' => 'ả',
                'A' => 'Ả',
                _ => ch,
            },
            '\u{31B}' => match ch {
                'o' => 'ơ',
                'O' => 'Ơ',
                'ó' => 'ớ',
                'Ó' => 'Ớ',
                'u' => 'ư',
                'U' => 'Ư',
                'ù' => 'ừ',
                'Ù' => 'Ừ',
                // 'ữ',
                // 'Ữ',
                _ => ch,
            },
            _ => ch,
            /* 'Ħ', 'ħ', 'Ł', 'ł',*/
        },
        _ => ch,
    }
}

/// add all the leters of all the alphabets in the script group
/// or not all, only if it does not require to exclude letters
/// if the script group has only one language, then leave it empty
pub(crate) fn script_char_to_alphabets(script: Script, ch: char) -> Option<&'static [Alphabet]> {
    match script {
        /* Script::Adlam => alphabet_match!([(Alphabet::Adlam, [])]),
        Script::Ahom => alphabet_match!([(Alphabet::Ahom, [])]),
        Script::AnatolianHieroglyphs => {
            alphabet_match!([(Alphabet::AnatolianHieroglyphs, [])])
        }
        Script::Arabic => alphabet_match!([(Alphabet::Arabic, []),]),
        Script::Armenian => alphabet_match!([(Alphabet::Armenian, [])]),
        Script::Avestan => alphabet_match!([(Alphabet::Avestan, [])]),
        Script::Balinese => alphabet_match!([(Alphabet::Balinese, [])]),
        Script::Bamum => alphabet_match!([(Alphabet::Bamum, [])]),
        Script::BassaVah => alphabet_match!([(Alphabet::BassaVah, [])]),
        Script::Batak => {
            alphabet_match!([(Alphabet::Batak, []),])
        }
        Script::Bengali => alphabet_match!([(Alphabet::Bengali, []),]),
        Script::Bhaiksuki => alphabet_match!([(Alphabet::Bhaiksuki, [])]),
        Script::Bopomofo => alphabet_match!([(Alphabet::Bopomofo, [])]),
        Script::Brahmi => alphabet_match!([(Alphabet::Brahmi, []),]),
        Script::Braille => alphabet_match!([(Alphabet::Braille, [])]),
        Script::Buginese => {
            alphabet_match!([(Alphabet::Buginese, []),])
        }
        Script::Buhid => alphabet_match!([(Alphabet::Buhid, [])]),
        Script::CanadianAboriginal => alphabet_match!([(Alphabet::CanadianAboriginal, []),]),
        Script::Carian => alphabet_match!([(Alphabet::Carian, [])]),
        Script::CaucasianAlbanian => alphabet_match!([(Alphabet::CaucasianAlbanian, [])]),
        Script::Chakma => alphabet_match!([(Alphabet::Chakma, [])]),
        Script::Cham => alphabet_match!([(Alphabet::Cham, [])]),
        Script::Cherokee => alphabet_match!([(Alphabet::Cherokee, [])]),
        Script::Chorasmian => alphabet_match!([(Alphabet::Chorasmian, [])]), */
        // if you want to add something here, validate that char's range is active in `ucd.rs`
        // during parsing these considered as connectors, not chars of the word
        // example1: `can't` for english is one word, for other lang it is two words: `can, t`
        //   because if Alphabets of all 3 chars do not intersect, it will be two words
        // example2: `word1' word2` for all langs will be parsed as two words without `'`,
        //   because next char after `'` is space, which is not a char of any language
        Script::Common => Some(match ch {
            '\'' => &[
                Alphabet::Cyrillic(CyrillicAlphabet::Belarusian),
                Alphabet::Latin(LatinAlphabet::Acehnese),
                Alphabet::Latin(LatinAlphabet::Afrikaans),
                Alphabet::Latin(LatinAlphabet::Albanian),
                Alphabet::Latin(LatinAlphabet::AymaraCentral),
                Alphabet::Latin(LatinAlphabet::Azerbaijani),
                Alphabet::Latin(LatinAlphabet::Banjar),
                Alphabet::Latin(LatinAlphabet::Bemba),
                Alphabet::Latin(LatinAlphabet::Bokmal),
                Alphabet::Latin(LatinAlphabet::Buginese),
                Alphabet::Latin(LatinAlphabet::Catalan),
                Alphabet::Latin(LatinAlphabet::CreoleHaitian),
                Alphabet::Latin(LatinAlphabet::Danish),
                Alphabet::Latin(LatinAlphabet::Dyula),
                Alphabet::Latin(LatinAlphabet::English),
                Alphabet::Latin(LatinAlphabet::Fon),
                Alphabet::Latin(LatinAlphabet::French),
                Alphabet::Latin(LatinAlphabet::Friulian),
                Alphabet::Latin(LatinAlphabet::FulfuldeNigerian),
                Alphabet::Latin(LatinAlphabet::GaelicScottish),
                Alphabet::Latin(LatinAlphabet::Ganda),
                Alphabet::Latin(LatinAlphabet::Guarani),
                Alphabet::Latin(LatinAlphabet::MalgasyPlateau),
            ],
            '¡' => &[Alphabet::Latin(LatinAlphabet::Spanish)],
            '¿' => &[Alphabet::Latin(LatinAlphabet::Spanish)],
            'ʻ' => &[Alphabet::Latin(LatinAlphabet::UzbekNorthern)],
            _ => &[], // must be always empty
        }),
        /* Script::Coptic => alphabet_match!([(Alphabet::Coptic, [])]),
        Script::Cuneiform => alphabet_match!([(Alphabet::Cuneiform, []),]),
        Script::Cypriot => alphabet_match!([(Alphabet::Cypriot, [])]),
        Script::CyproMinoan => alphabet_match!([(Alphabet::CyproMinoan, [])]), */
        Script::Cyrillic => alphabet_match!([
            // TODO: Lost some alphabets from CyrillicAlphabet
            (
                Alphabet::Cyrillic(CyrillicAlphabet::Belarusian),
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'І', 'і', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ў', 'ў', 'Ф', 'ф', 'Х', 'х',
                    'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Alphabet::Cyrillic(CyrillicAlphabet::Bulgarian),
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ж', 'ж', 'З', 'з',
                    'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о', 'П', 'п',
                    'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч',
                    'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ь', 'ь', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Alphabet::Cyrillic(CyrillicAlphabet::Kazakh),
                [
                    'А', 'а', 'Ә', 'ә', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Ғ', 'ғ', 'Д', 'д', 'Е', 'е',
                    'Ё', 'ё', 'Ж', 'ж', 'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Қ', 'қ', 'Л', 'л',
                    'М', 'м', 'Н', 'н', 'Ң', 'ң', 'О', 'о', 'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с',
                    'Т', 'т', 'У', 'у', 'Ұ', 'ұ', 'Ү', 'ү', 'Ф', 'ф', 'Х', 'х', 'Һ', 'һ', 'Ц', 'ц',
                    'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'І', 'і', 'Ь', 'ь', 'Э', 'э',
                    'Ю', 'ю', 'Я', 'я',
                ],
            ),
            (
                Alphabet::Cyrillic(CyrillicAlphabet::Macedonian),
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Ѓ', 'ѓ', 'Е', 'е', 'Ж', 'ж',
                    'З', 'з', 'Ѕ', 'ѕ', 'И', 'и', 'Ј', 'ј', 'К', 'к', 'Л', 'л', 'Љ', 'љ', 'М', 'м',
                    'Н', 'н', 'Њ', 'њ', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ќ', 'ќ',
                    'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Џ', 'џ', 'Ш', 'ш',
                ]
            ),
            (
                Alphabet::Cyrillic(CyrillicAlphabet::MongolianHalh),
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ү', 'ү', 'Ф', 'ф',
                    'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь',
                    'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Alphabet::Cyrillic(CyrillicAlphabet::OldChurchSlavonic),
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Є', 'є', 'Ж', 'ж', 'Ѕ', 'ѕ',
                    'З', 'з', 'Ꙁ', 'ꙁ', 'И', 'и', 'І', 'і', 'Ї', 'ї', 'К', 'к', 'Л', 'л', 'М', 'м',
                    'Н', 'н', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ѹ', 'ѹ', 'Ꙋ', 'ꙋ',
                    'Ф', 'ф', 'Х', 'х', 'Ѡ', 'ѡ', 'Ѿ', 'ѿ', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ',
                    'Ъ', 'ъ', 'Ꙑ', 'ꙑ', 'Ь', 'ь', 'Ѣ', 'ѣ', 'Ꙗ', 'ꙗ', 'Ѥ', 'ѥ', 'Ю', 'ю', 'Ѫ', 'ѫ',
                    'Ѭ', 'ѭ', 'Ѧ', 'ѧ', 'Ѩ', 'ѩ', 'Ѯ', 'ѯ', 'Ѱ', 'ѱ', 'Ѳ', 'ѳ', 'Ѵ', 'ѵ', 'Ҁ', 'ҁ',
                ]
            ),
            (
                Alphabet::Cyrillic(CyrillicAlphabet::Russian),
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц',
                    'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю',
                    'Я', 'я',
                ],
            ),
            (
                Alphabet::Cyrillic(CyrillicAlphabet::Serbian),
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Ђ', 'ђ', 'Е', 'е', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Ј', 'ј', 'К', 'к', 'Л', 'л', 'Љ', 'љ', 'М', 'м', 'Н', 'н',
                    'Њ', 'њ', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ћ', 'ћ', 'У', 'у',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Џ', 'џ', 'Ш', 'ш',
                ]
            ),
            (
                Alphabet::Cyrillic(CyrillicAlphabet::Ukrainian),
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Ґ', 'ґ', 'Д', 'д', 'Е', 'е', 'Є', 'є',
                    'Ж', 'ж', 'З', 'з', 'И', 'и', 'І', 'і', 'Ї', 'ї', 'Й', 'й', 'К', 'к', 'Л', 'л',
                    'М', 'м', 'Н', 'н', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ь', 'ь', 'Ю', 'ю',
                    'Я', 'я',
                ],
            ),
        ]),
        /* Script::Deseret => alphabet_match!([(Alphabet::Deseret, [])]),
        Script::Devanagari => alphabet_match!([(Alphabet::Devanagari, []),]),
        Script::DivesAkuru => alphabet_match!([(Alphabet::DivesAkuru, [])]),
        Script::Dogra => alphabet_match!([(Alphabet::Dogra, [])]),
        Script::Duployan => alphabet_match!([(Alphabet::Duployan, []),]),
        Script::EgyptianHieroglyphs => alphabet_match!([(Alphabet::EgyptianHieroglyphs, [])]),
        Script::Elbasan => alphabet_match!([(Alphabet::Elbasan, [])]),
        Script::Elymaic => alphabet_match!([(Alphabet::Elymaic, [])]),
        Script::Ethiopic => alphabet_match!([(Alphabet::Ethiopic, []),]),
        Script::Garay => alphabet_match!([(Alphabet::Garay, [])]),
        Script::Georgian => alphabet_match!([(Alphabet::Georgian, [])]),
        Script::Glagolitic => alphabet_match!([(Alphabet::Glagolitic, [])]),
        Script::Gothic => alphabet_match!([(Alphabet::Gothic, [])]),
        Script::Grantha => alphabet_match!([(Alphabet::Grantha, [])]),
        Script::Greek => alphabet_match!([(Alphabet::Greek, [])]),
        Script::Gujarati => alphabet_match!([(Alphabet::Gujarati, [])]),
        Script::GunjalaGondi => alphabet_match!([(Alphabet::GunjalaGondi, [])]),
        Script::Gurmukhi => alphabet_match!([(Alphabet::Gurmukhi, [])]),
        Script::GurungKhema => alphabet_match!([(Alphabet::GurungKhema, [])]), */
        Script::Han => alphabet_match!([
            (
                Alphabet::Han(HanAlphabet::ChineseSimplified),
                [
                    // https://en.wikisource.org/wiki/Translation:List_of_Frequently_Used_Characters_in_Modern_Chinese
                    // https://www.tutormandarin.net/en/list-of-different-simplified-and-traditional-characters/
                    '劳', '学', '绅', '浑', '么', '轮', '达', '韦', '边', '详', '练', '养', '颜',
                    '篱', '伞', '泄', '筛', '牵', '谐', '备', '捞', '国', '厅', '尔', '级', '饼',
                    '仓', '骑', '没', '蕴', '仅', '炮', '势', '冻', '纲', '厉', '决', '账', '内',
                    '极', '灵', '统', '临', '莲', '橱', '块', '战', '础', '广', '蜡', '证', '匀',
                    '争', '腾', '机', '营', '妇', '权', '记', '梦', '页', '鸭', '从', '铲', '楼',
                    '龄', '启', '涝', '简', '诀', '扑', '资', '灾', '浆', '俭', '剧', '锐', '华',
                    '垦', '压', '转', '盖', '炼', '红', '骚', '谬', '蓝', '矿', '聋', '谎', '为',
                    '枣', '爷', '雾', '瞒', '腊', '径', '纽', '挣', '缅', '辅', '贞', '後', '询',
                    '赔', '吗', '丢', '迁', '麦', '穷', '鲁', '贪', '窝', '设', '闸', '闲', '综',
                    '祸', '栏', '较', '缴', '钢', '二', '冈', '厂', '沉', '盏', '亚', '滩', '号',
                    '贼', '凉', '数', '敌', '伟', '监', '狱', '卫', '饭', '纯', '俩', '惊', '庄',
                    '鹤', '协', '尽', '脑', '关', '济', '桨', '濒', '污', '迈', '猎', '续', '厢',
                    '韧', '侨', '帐', '裤', '温', '区', '彻', '写', '宽', '陆', '赖', '饲', '优',
                    '绸', '惭', '谋', '睁', '爱', '喷', '满', '聪', '杰', '药', '铃', '缰', '赐',
                    '译', '驰', '诚', '绒', '仙', '胁', '庙', '窜', '枢', '犹', '泽', '鹅', '帘',
                    '倾', '频', '台', '饥', '讼', '绿', '纳', '赛', '沦', '癞', '谢', '洁', '终',
                    '会', '举', '业', '侣', '归', '婴', '亏', '输', '伤', '潇', '继', '夸', '轴',
                    '凭', '弃', '汇', '灿', '岭', '诱', '调', '叙', '缓', '症', '镇', '谣', '烁',
                    '闯', '蚂', '独', '据', '雕', '欢', '浊', '钩', '贸', '硕', '阳', '壶', '缘',
                    '绰', '缚', '齐', '乐', '荣', '诅', '歼', '汤', '发', '丽', '衔', '谨', '办',
                    '摇', '夺', '严', '岂', '闭', '阵', '这', '胶', '厦', '杨', '妆', '额', '毁',
                    '经', '删', '眯', '疗', '问', '陕', '忧', '无', '饱', '绘', '龟', '艺', '迹',
                    '铸', '坠', '仑', '岁', '驶', '择', '摆', '够', '卧', '萝', '缝', '园', '离',
                    '弯', '奖', '谦', '哑', '挂', '桥', '垄', '粪', '锣', '荫', '颗', '滞', '录',
                    '嚣', '态', '斋', '晋', '镶', '辽', '庞', '戏', '纹', '听', '洒', '纪', '赢',
                    '伦', '讹', '题', '说', '啸', '搅', '霉', '显', '渊', '践', '气', '舱', '宾',
                    '坚', '凛', '携', '馋', '载', '偿', '蝇', '响', '皱', '键', '谴', '壮', '识',
                    '嘱', '潜', '铭', '凯', '误', '谱', '励', '怀', '峡', '类', '贺', '谓', '异',
                    '帅', '鸠', '刹', '艰', '郑', '只', '脸', '闹', '络', '赞', '贫', '档', '鹰',
                    '三', '阁', '晓', '锋', '绵', '审', '荡', '滚', '坝', '让', '热', '师', '线',
                    '侠', '沟', '秆', '试', '规', '读', '净', '买', '篮', '纫', '烫', '泻', '实',
                    '筝', '锄', '缠', '袄', '时', '秃', '诉', '礼', '趋', '错', '摄', '锤', '彦',
                    '虽', '乡', '丛', '阶', '认', '拦', '须', '什', '亿', '执', '锡', '侧', '鳞',
                    '谭', '鉴', '滤', '锻', '过', '咏', '烧', '鲸', '讲', '阅', '罚', '紧', '维',
                    '义', '编', '损', '陈', '领', '纠', '贾', '箩', '锦', '论', '馒', '闻', '阔',
                    '圆', '盗', '吴', '检', '画', '呐', '捣', '吃', '窃', '绕', '绢', '泪', '厨',
                    '讽', '宝', '网', '骂', '挤', '誉', '囱', '哟', '屡', '佣', '递', '桩', '弹',
                    '务', '盘', '渗', '蛮', '虑', '庐', '呜', '谅', '晒', '猪', '献', '孙', '顷',
                    '赠', '冲', '坏', '苏', '体', '贵', '习', '党', '脱', '湾', '鳄', '阐', '门',
                    '劝', '兰', '恶', '脚', '崭', '贴', '晕', '运', '医', '辈', '凑', '违', '专',
                    '话', '挥', '灭', '条', '顽', '涌', '轨', '葱', '粮', '种', '际', '两', '宪',
                    '琼', '才', '赏', '觉', '涂', '坟', '状', '鸡', '毙', '庆', '狭', '昼', '扬',
                    '莱', '万', '计', '坛', '贡', '笋', '费', '该', '摊', '凫', '处', '肠', '伙',
                    '苹', '鹦', '职', '垒', '对', '样', '顿', '恋', '单', '挡', '谁', '纸', '泼',
                    '柜', '诵', '隶', '邻', '乱', '寻', '悦', '纬', '舆', '难', '遗', '萨', '责',
                    '辫', '军', '哗', '项', '训', '声', '钞', '览', '称', '龙', '苍', '驻', '馆',
                    '飘', '辉', '电', '镰', '愤', '邓', '织', '贩', '盐', '念', '萧', '宁', '驴',
                    '亲', '撑', '现', '鸦', '拢', '八', '颠', '茧', '则', '贱', '五', '呛', '视',
                    '释', '钻', '兴', '鸟', '拥', '侄', '鸽', '碍', '拟', '吓', '栈', '屿', '钓',
                    '浓', '开', '六', '剥', '玛', '绳', '书', '颊', '扫', '刘', '畅', '鹊', '肤',
                    '渐', '饿', '旷', '千', '翘', '烦', '抡', '复', '胜', '购', '静', '茎', '护',
                    '缎', '农', '辩', '驳', '稳', '组', '劲', '蚁', '牺', '链', '胀', '缕', '斩',
                    '笔', '兽', '芜', '讨', '汉', '邮', '帜', '鬓', '惧', '吁', '闪', '税', '诺',
                    '虏', '诗', '韵', '烛', '验', '触', '掷', '娇', '顺', '脉', '鸿', '却', '滨',
                    '胆', '饰', '沪', '稣', '绑', '鲍', '乌', '缩', '讶', '刚', '儿', '亩', '细',
                    '跃', '残', '镜', '还', '叶', '浅', '疮', '马', '应', '产', '换', '预', '衬',
                    '壳', '挠', '锈', '狈', '骤', '抬', '员', '顶', '们', '舰', '齿', '贤', '捡',
                    '构', '夹', '轻', '叠', '滥', '逊', '铺', '剑', '报', '颈', '铅', '鳖', '瘾',
                    '纷', '竞', '册', '觅', '饺', '赌', '烂', '库', '懒', '惩', '恼', '搁', '驾',
                    '蒋', '蚀', '肿', '狮', '赵', '妈', '给', '风', '张', '枪', '总', '驮', '缆',
                    '远', '与', '阀', '笼', '遥', '间', '双', '绍', '忆', '标', '谊', '痒', '七',
                    '拨', '辙', '图', '击', '许', '弥', '厌', '朴', '侦', '钟', '猫', '烟', '镖',
                    '扎', '剂', '质', '险', '头', '悬', '县', '软', '诊', '耻', '惯', '酿', '户',
                    '杀', '环', '贝', '顾', '宫', '颁', '货', '强', '呕', '厕', '颂', '卢', '迟',
                    '闰', '针', '败', '砖', '进', '锹', '参', '纵', '艳', '驯', '颇', '诈', '讯',
                    '创', '卖', '个', '语', '娱', '层', '适', '贿', '销', '饮', '团', '癣', '骄',
                    '躏', '墙', '鹏', '溃', '测', '芦', '辞', '十', '订', '尸', '评', '抢', '访',
                    '当', '九', '车', '它', '馅', '卤', '筑', '变', '唤', '年', '财', '诸', '绣',
                    '隐', '旧', '锁', '横', '鳍', '疯', '补', '沧', '堕', '长', '纱', '众', '耸',
                    '属', '脏', '钦', '树', '罢', '惨', '轧', '岛', '怜', '婶', '传', '苇', '词',
                    '带', '负', '确', '恳', '废', '纤', '鸣', '樱', '纺', '冯', '驱', '坞', '着',
                    '岖', '赋', '颖', '逻', '价', '骆', '辆', '恒', '黄', '尝', '动', '钳', '汹',
                    '储', '讥', '奋', '枫', '虾', '竖', '抠', '栋', '润', '谈', '钥', '况', '断',
                    '联', '装', '债', '禅', '抚', '丝', '将', '圣', '垫', '银', '术', '炉', '袜',
                    '轿', '拣', '选', '结', '辑', '铜', '兹', '队', '锯', '吕', '伪', '观', '尘',
                    '巩', '吨', '鱼', '围', '赶', '蚕', '岗', '厘', '贯', '积', '涛', '暂', '讳',
                    '肾', '罗', '别', '涨', '诞', '届', '锅', '赡', '韩', '鲜', '赚', '腻', '连',
                    '减', '来', '肃', '贷', '奥', '湿', '请', '轩', '签', '浇', '仪', '胧', '躯',
                    '钉', '导', '寿', '见', '点', '筹', '渔', '饶', '阴', '丧', '课', '搂', '酱',
                    '币', '骡', '欧', '宠', '获', '兑', '凤', '东', '担', '帮', '毕', '约', '攒',
                    '沥', '百', '扰', '骗', '抛', '杂', '铁', '轰', '绝', '闷', '踪', '虚', '灯',
                    '驼', '颤', '荐', '节', '绞', '谜', '绩', '叹', '议', '袭', '钱', '场', '码',
                    '窑', '乔', '四', '随', '历', '咽', '扩', '绪', '飞', '愿', '虫'
                ]
            ),
            (Alphabet::Han(HanAlphabet::ChineseTraditional), []),
            (Alphabet::Han(HanAlphabet::Hanja), []),
            (
                // it also uses all Traditional Chinese characters
                Alphabet::Han(HanAlphabet::Kanji),
                [
                    // Jōyō kanji (https://en.wikipedia.org/wiki/List_of_j%C5%8Dy%C5%8D_kanji)
                    '亜', '悪', '圧', '囲', '医', '為', '壱', '逸', '隠', '栄', '営', '衛', '駅',
                    '謁', '円', '塩', '縁', '艶', '応', '欧', '殴', '桜', '奥', '横', '温', '穏',
                    '仮', '価', '禍', '画', '会', '悔', '海', '絵', '壊', '懐', '慨', '概', '拡',
                    '殻', '覚', '学', '岳', '楽', '喝', '渇', '褐', '缶', '巻', '陥', '勧', '寛',
                    '漢', '関', '歓', '観', '気', '祈', '既', '帰', '亀', '器', '偽', '戯', '犠',
                    '旧', '拠', '挙', '虚', '峡', '挟', '狭', '郷', '響', '暁', '勤', '謹', '区',
                    '駆', '勲', '薫', '径', '茎', '恵', '掲', '渓', '経', '蛍', '軽', '継', '鶏',
                    '芸', '撃', '欠', '研', '県', '倹', '剣', '険', '圏', '検', '献', '権', '顕',
                    '験', '厳', '広', '効', '恒', '黄', '鉱', '号', '国', '黒', '穀', '砕', '済',
                    '斎', '剤', '殺', '雑', '参', '桟', '蚕', '惨', '賛', '残', '糸', '祉', '視',
                    '歯', '児', '辞', '湿', '実', '写', '社', '舎', '者', '煮', '釈', '寿', '収',
                    '臭', '従', '渋', '獣', '縦', '祝', '粛', '処', '暑', '署', '緒', '諸', '叙',
                    '将', '祥', '称', '渉', '焼', '証', '奨', '条', '状', '乗', '浄', '剰', '畳',
                    '縄', '壌', '嬢', '譲', '醸', '触', '嘱', '神', '真', '寝', '慎', '尽', '図',
                    '粋', '酔', '穂', '随', '髄', '枢', '数', '瀬', '声', '斉', '静', '窃', '摂',
                    '節', '絶', '専', '浅', '戦', '践', '銭', '潜', '繊', '禅', '祖', '双', '壮',
                    '争', '荘', '捜', '挿', '巣', '曽', '痩', '装', '僧', '層', '総', '騒', '増',
                    '憎', '蔵', '贈', '臓', '即', '属', '続', '堕', '対', '体', '帯', '滞', '台',
                    '滝', '択', '沢', '担', '単', '胆', '嘆', '団', '断', '弾', '遅', '痴', '虫',
                    '昼', '鋳', '著', '庁', '徴', '聴', '懲', '勅', '鎮', '塚', '逓', '鉄', '点',
                    '転', '伝', '都', '灯', '当', '党', '盗', '稲', '闘', '徳', '独', '読', '突',
                    '届', '難', '弐', '悩', '脳', '覇', '拝', '廃', '売', '梅', '麦', '発', '髪',
                    '抜', '繁', '晩', '蛮', '卑', '秘', '碑', '浜', '賓', '頻', '敏', '瓶', '侮',
                    '福', '払', '仏', '併', '並', '塀', '餅', '辺', '変', '弁', '勉', '歩', '宝',
                    '豊', '褒', '墨', '没', '翻', '毎', '万', '満', '免', '麺', '黙', '弥', '訳',
                    '薬', '与', '予', '余', '誉', '揺', '様', '謡', '来', '頼', '乱', '覧', '欄',
                    '竜', '隆', '虜', '両', '猟', '緑', '涙', '塁', '類', '礼', '励', '戻', '霊',
                    '齢', '暦', '歴', '恋', '練', '錬', '炉', '労', '郎', '朗', '廊', '楼', '録',
                    '湾',
                    // Jinmeiyō Kanji (https://en.wikipedia.org/wiki/Ky%C5%ABjitai)
                    '亘', '凜', '尭', '巌', '晃', '桧', '槙', '渚', '猪', '琢', '祢', '祐', '祷',
                    '禄', '禎', '穣', '萌', '遥',
                    // Hyōgai kanji (https://en.wikipedia.org/wiki/Ky%C5%ABjitai)
                    '唖', '頴', '鴎', '躯', '撹', '麹', '鹸', '噛', '繍', '蒋', '醤', '掻', '屏',
                    '并', '沪', '芦', '蝋', '弯', '焔', '砿', '讃', '顛', '醗', '溌', '輌', '繋',
                    // Kokuji (https://en.wikipedia.org/wiki/Kokuji)
                    // Jōyō Kanji
                    '働', '匂', '峠', '枠', '栃', '畑', '込', '腺', '搾',
                    // Jinmeiyō Kanji (https://en.wikipedia.org/wiki/Ky%C5%ABjitai)
                    '勺', '銑', '脹', '錘', '匁', '俣', '凧', '凪', '喰', '柾', '椛', '榊', '樫',
                    '畠', '笹', '籾', '辻', '雫', '鰯', '麿', //
                    // Hyōgaiji
                    '躾', //
                    // https://en.wikipedia.org/wiki/Extended_shinjitai
                    '鴬', '掴', '箪', '涜', '侭', '薮', '篭', //
                    // others
                    '醖',
                ]
            ),
        ]),
        /* Script::Hangul => alphabet_match!([(Alphabet::Hangul, [])]),
        Script::HanifiRohingya => alphabet_match!([(Alphabet::HanifiRohingya, [])]),
        Script::Hanunoo => alphabet_match!([(Alphabet::Hanunoo, [])]),
        Script::Hatran => alphabet_match!([(Alphabet::Hatran, [])]),
        Script::Hebrew => alphabet_match!([(Alphabet::Hebrew, []),]),
        Script::Hiragana => alphabet_match!([(Alphabet::Hiragana, [])]),
        Script::ImperialAramaic => alphabet_match!([(Alphabet::ImperialAramaic, [])]), */
        Script::Inherited => Some(&[]), /* match ch {
        /* '\u{307}' => &[
                // Alphabet::ChechenLatin,
                // Alphabet::OldIrishLatin,
                Alphabet::LithuanianLatin,
                // Alphabet::LivonianLatin,
                // Alphabet::MalteseLatin,
                // Alphabet::OldEnglishLatin,
                Alphabet::PolishLatin,
            ], */
            _ => &[], // must be always empty
        }, */
        /* Script::InscriptionalPahlavi => {
            alphabet_match!([(Alphabet::InscriptionalPahlavi, [])])
        }
        Script::InscriptionalParthian => alphabet_match!([(Alphabet::InscriptionalParthian, [])]),
        Script::Javanese => alphabet_match!([(Alphabet::Javanese, [])]),
        Script::Kaithi => alphabet_match!([(Alphabet::Kaithi, []),]),
        Script::Kannada => alphabet_match!([(Alphabet::Kannada, [])]),
        Script::Katakana => alphabet_match!([(Alphabet::Katakana, [])]),
        Script::Kawi => alphabet_match!([(Alphabet::Kawi, []),]),
        Script::KayahLi => alphabet_match!([(Alphabet::KayahLi, [])]),
        Script::Kharoshthi => alphabet_match!([(Alphabet::Kharoshthi, [])]),
        Script::KhitanSmallScript => alphabet_match!([(Alphabet::KhitanSmallScript, [])]),
        Script::Khmer => alphabet_match!([(Alphabet::Khmer, [])]),
        Script::Khojki => alphabet_match!([(Alphabet::Khojki, [])]),
        Script::Khudawadi => alphabet_match!([(Alphabet::Khudawadi, [])]),
        Script::KiratRai => alphabet_match!([(Alphabet::KiratRai, [])]),
        Script::Lao => alphabet_match!([(Alphabet::Lao, [])]), */
        Script::Latin => alphabet_match!([
            (
                Alphabet::Latin(LatinAlphabet::Acehnese), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', /* 'F', 'f', */ 'G',
                    'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O',
                    'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    /* 'V', 'v', */ 'W', 'w', /* 'X', 'x', */ 'Y', 'y',
                    /* 'Z', 'z', */ 'É', 'é', 'È', 'è', 'Ë', 'ë', 'Ô', 'ô', 'Ö', 'ö'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Afrikaans), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ê', 'ê', 'Ë', 'ë', 'É', 'é', 'Ï', 'ï', 'Ô', 'ô', 'Á', 'á',
                    'Ä', 'ä', 'È', 'è', 'Í', 'í', 'Î', 'î', 'Ó', 'ó', 'Ö', 'ö', 'Ú', 'ú', 'Û', 'û',
                    'Ü', 'ü', 'Ý', 'ý',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Albanian), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', /* 'Dh', 'dh', */ 'E',
                    'e', 'Ë', 'ë', 'F', 'f', 'G', 'g', /* 'Gj', 'gj', */ 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', /* 'Ll', 'll', */ 'M', 'm', 'N', 'n',
                    /* 'Nj', 'nj', */ 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r',
                    /* 'Rr', 'rr', */ 'S', 's', /* 'Sh', 'sh', */ 'T', 't',
                    /* 'Th', 'th', */ 'U', 'u', 'V', 'v', 'X', 'x',
                    /* 'Xh', 'xh', */ 'Y', 'y', 'Z', 'z', /* 'Zh', 'zh', */
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Asturian), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú',
                    'Ü', 'ü',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::AymaraCentral), //+
                [
                    'A', 'a', 'B', 'b', /* 'Ch', 'ch', */ 'C', 'c', 'D', 'd', 'E', 'e', 'F',
                    'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Ll', 'll', */ 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o', 'P', 'p', 'Q',
                    'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Ä',
                    'ä', 'Ï', 'ï', 'Á', 'á', 'Ó', 'ó', 'É', 'é', 'Ü', 'ü', 'Í', 'í', 'Z', 'z', 'Ë',
                    'ë', 'Ú', 'ú', 'Ö', 'ö',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Azerbaijani), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'Ə', 'ə', 'F', 'f',
                    'G', 'g', 'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's',
                    'Ş', 'ş', 'T', 't', 'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Balinese), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'É', 'é'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Bambara), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                    'É', 'é', 'Ô', 'ô'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Banjar), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'É', 'é'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Basque), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'X', 'x', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Bemba), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Bokmal), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å', 'Ç', 'ç', 'É', 'é'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Bosnian), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž', 'Ç', 'ç',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Buginese), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Catalan), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'Ç', 'ç', 'É', 'é', 'È', 'è', 'Ë', 'ë', 'Í', 'í',
                    'Ï', 'ï', 'Ó', 'ó', 'Ò', 'ò', 'Ü', 'ü', 'Ú', 'ú',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Cebuano), //+
                [
                    'A', 'a', 'B', 'b', /* 'C', 'c', */ 'D', 'd', 'E', 'e',
                    /* 'F', 'f', */ 'G', 'g', 'H', 'h', 'I', 'i', /* 'J', 'j', */ 'K',
                    'k', 'L', 'l', 'M', 'm', 'N', 'n', /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', /* 'V', 'v', */ 'W', 'w', 'Y',
                    'y',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Chokwe), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::CreoleHaitian), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                    'À', 'à', 'È', 'è', 'É', 'é', 'Ò', 'ò'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Croatian), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Czech), //+
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'Ď', 'ď', 'E', 'e',
                    'È', 'è', 'É', 'é', 'Ě', 'ě', 'F', 'f', 'G', 'g', 'H', 'h',
                    /* 'Ch', 'ch', */ 'I', 'i', 'Ì', 'ì', 'Í', 'í', 'J', 'j', 'K', 'k', 'L',
                    'l', 'M', 'm', 'N', 'n', 'Ň', 'ň', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'Ř',
                    'ř', 'S', 's', 'Š', 'š', 'T', 't', 'Ť', 'ť', 'U', 'u', 'Ù', 'ù', 'Ú', 'ú', 'Ů',
                    'ů', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Ý', 'ý', 'Z', 'z', 'Ž', 'ž', 'Ø',
                    'ø',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Danish), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Å', 'å', 'Æ', 'æ', 'É', 'é', 'Ø', 'ø',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::DinkaSouthwestern), //+
                [
                    'Ä', 'ä', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ë', 'ë', 'Ɛ', 'ɛ',
                    'G', 'g', 'Ɣ', 'ɣ', 'H', 'h', 'I', 'i', 'Ï', 'ï', 'J', 'j', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'Ŋ', 'ŋ', 'Ö', 'ö', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Dutch), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'Ä', 'ä', 'È', 'è', 'Ë', 'ë', 'É', 'é', 'Ï', 'ï',
                    'Ĳ', 'ĳ', 'Ó', 'ó', 'Ö', 'ö', 'Ü', 'ü',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Dyula), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ɲ', 'ɲ',
                    'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v',
                    'W', 'w', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::English),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Esperanto), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ĉ', 'ĉ', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ĝ', 'ĝ', 'H', 'h', 'Ĥ', 'ĥ', 'I', 'i', 'J', 'j', 'Ĵ', 'ĵ', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'Ŝ', 'ŝ', 'T', 't',
                    'U', 'u', 'Ŭ', 'ŭ', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z', // XY?
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Estonian), //loanwords?
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Ä', 'ä', 'Ö', 'ö', 'Õ', 'õ', 'Š', 'š',
                    'Ü', 'ü', 'Ž', 'ž', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Ewe), //+
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'Ð', 'Ɖ', 'ɖ', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'Ƒ',
                    'ƒ', 'G', 'g', 'Ɣ', 'ɣ', 'H', 'h', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N',
                    'n', 'Ŋ', 'ŋ', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U',
                    'u', 'V', 'v', 'Ʋ', 'ʋ', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ã', 'ã', 'À',
                    'à', 'È', 'è', 'Ẽ', 'ẽ', 'É', 'é', 'Ĩ', 'ĩ', 'Í', 'í', 'Ò', 'ò',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Faroese), //+
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'D', 'd', 'Ð', 'ð', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'Ú', 'ú',
                    'V', 'v', 'Y', 'y', 'Ý', 'ý', 'Æ', 'æ', 'Ø', 'ø'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Fijian), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Finnish), //+
                [
                    'A', 'a', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k',
                    'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't',
                    'U', 'u', 'V', 'v', 'Y', 'y', 'Ä', 'ä', 'Ö', 'ö',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Fon), //+
                [
                    'A', 'a', 'À', 'à', 'Á', 'á', 'Ǎ', 'ǎ', 'B', 'b', 'C', 'c', 'D', 'd', 'Ð', 'Ɖ',
                    'ɖ', 'Ě', 'ě', 'É', 'é', 'È', 'è', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g', 'H',
                    'h', 'Ǐ', 'ǐ', 'Í', 'í', 'Ì', 'ì', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M',
                    'm', 'N', 'n', 'Ò', 'ò', 'Ǒ', 'ǒ', 'O', 'o', 'Ó', 'ó', 'Ɔ', 'ɔ', 'P', 'p', 'R',
                    'r', 'S', 's', 'T', 't', 'Ú', 'ú', 'Ǔ', 'ǔ', 'Ù', 'ù', 'U', 'u', 'V', 'v', 'W',
                    'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::French), //+
                [
                    'Æ', 'æ', 'À', 'à', 'Â', 'â', 'A', 'a', 'B', 'b', 'Ç', 'ç', 'C', 'c', 'D', 'd',
                    'É', 'é', 'E', 'e', 'Ê', 'ê', 'È', 'è', 'Ë', 'ë', 'F', 'f', 'G', 'g', 'H', 'h',
                    'Î', 'î', 'Ï', 'ï', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'Ô', 'ô', 'O', 'o', 'Œ', 'œ', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't',
                    'Û', 'û', 'Ù', 'ù', 'Ü', 'ü', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Ÿ', 'ÿ', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Friulian), //+
                [
                    'A', 'a', 'Â', 'â', 'À', 'à', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'Ê', 'ê',
                    'È', 'è', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Ì', 'ì', 'Î', 'î',
                    'J', 'j', 'L', 'l', 'M', 'm', 'N', 'n', 'Ò', 'ò', 'Ô', 'ô', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'Ù', 'ù', 'Û', 'û', 'U', 'u', 'V', 'v', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::FulfuldeNigerian), //+ added extra `ŋ`
                [
                    'A', 'a', 'B', 'b', 'Ɓ', 'ɓ', 'C', 'c', 'D', 'd', 'Ɗ', 'ɗ', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                    'Ƴ', 'ƴ', 'Ŋ', 'ŋ',
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::GaelicScottish), //+
                [
                    'À', 'à', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'È', 'è', 'F', 'f',
                    'G', 'g', 'H', 'h', 'Ì', 'ì', 'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ò', 'ò', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'Ù', 'ù', 'U', 'u',
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Galician), //+
                [
                    'Á', 'á', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'É', 'é', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'Í', 'í', 'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ',
                    'Ó', 'ó', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'Ú', 'ú', 'V', 'v', 'X', 'x', 'Z', 'z',
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Ganda), //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ŋ', 'ŋ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::German), //+
                [
                    'Ä', 'ä', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'ẞ', 'ß', 'T', 't', 'Ü', 'ü',
                    'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Guarani), //+
                [
                    'Ã', 'ã', 'Á', 'á', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'Ẽ', 'ẽ', 'Ê', 'ê',
                    'E', 'e', 'É', 'é', 'G', 'g', 'H', 'h', 'Í', 'í', 'Ĩ', 'ĩ', 'Î', 'î', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'Ó', 'ó', 'O', 'o',
                    'Õ', 'õ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'Ú', 'ú', 'U', 'u', 'Ũ', 'ũ',
                    'V', 'v', 'Ý', 'ý', 'Ỹ', 'ỹ', 'Y', 'y',
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Hausa),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Ɓ', 'ɓ', 'Ɗ', 'ɗ', 'Ƙ', 'ƙ',
                    'Ƴ', 'ƴ'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Hungarian),
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'C', 'c', /* 'Cs', 'cs', */ 'D', 'd',
                    /* 'Dz', 'dz', 'Dzs', 'dzs', */ 'E', 'e', 'É', 'é', 'F', 'f', 'G', 'g',
                    /* 'Gy', 'gy', */ 'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L',
                    'l', /* 'Ly', 'ly', */ 'M', 'm', 'N', 'n', /* 'Ny', 'ny', */ 'O',
                    'o', 'Ó', 'ó', 'Ö', 'ö', 'Ő', 'ő', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's',
                    /* 'Sz', 'sz', */ 'T', 't', /* 'Ty', 'ty', */ 'U', 'u', 'Ú', 'ú',
                    'Ü', 'ü', 'Ű', 'ű', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z',
                    'z', /* 'Zs', 'zs' */
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Icelandic),
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'D', 'd', 'Ð', 'ð', 'E', 'e', 'É', 'é', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'Ú', 'ú', 'V', 'v', 'X', 'x', 'Y', 'y', 'Ý', 'ý', 'Þ', 'þ', 'Æ', 'æ', 'Ö', 'ö',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Igbo),
                [
                    'A', 'a', 'B', 'b', /* 'Ch', 'ch', */ 'C', 'c', 'D', 'd', 'E', 'e', 'F',
                    'f', 'G', 'g', /* 'Gb', 'gb', 'Gh', 'gh', */ 'H', 'h', 'I', 'i', 'Ị', 'ị',
                    'J', 'j', 'K', 'k', /* 'Kw', 'kw', */ 'L', 'l', 'M', 'm', 'N', 'n',
                    /* 'Nw', 'nw', */ 'O', 'o', 'Ọ', 'ọ', 'P', 'p', 'R', 'r', 'S', 's', 'T',
                    't', 'U', 'u', 'Ụ', 'ụ', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É',
                    'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Ṅ', 'ṅ', 'Ŋ', 'ŋ'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Ilocano),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'I', 'i', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'W', 'w', 'Y', 'y', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Ñ', 'ñ'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Indonesian),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Irish),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Italian),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é',
                    'Ì', 'ì', 'Ò', 'ò', 'Ù', 'ù'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Javanese),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Jingpho),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Kabiye),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Kabuverdianu),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Z', 'z',
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Õ', 'õ', 'Ç', 'ç', 'Ê', 'ê'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Kabyle),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Č', 'č', 'Ɣ', 'ɣ'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Kamba),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'S', 's',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::KanuriCentral),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Kikongo),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Kikuyu),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Í', 'í', 'Ú', 'ú'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Kimbundu),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'S', 's', 'T', 't',
                    'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Kinyarwanda),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Í', 'í', 'Ú', 'ú'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::KurdishNorthern),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'Ê', 'ê', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Î', 'î', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Û', 'û', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Latgalian),
                [
                    'A', 'a', 'Ā', 'ā', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ē', 'ē',
                    'F', 'f', 'G', 'g', 'Ģ', 'ģ', 'H', 'h', 'I', 'i', 'Ī', 'ī', 'J', 'j', 'K', 'k',
                    'Ķ', 'ķ', 'L', 'l', 'Ļ', 'ļ', 'M', 'm', 'N', 'n', 'Ņ', 'ņ', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'Ū', 'ū', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Latin),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z',
                    'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū', 'Æ', 'æ', 'Œ', 'œ'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Latvian),
                [
                    'A', 'a', 'Ā', 'ā', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ē', 'ē',
                    'F', 'f', 'G', 'g', 'Ģ', 'ģ', 'H', 'h', 'I', 'i', 'Ī', 'ī', 'J', 'j', 'K', 'k',
                    'Ķ', 'ķ', 'L', 'l', 'Ļ', 'ļ', 'M', 'm', 'N', 'n', 'Ņ', 'ņ', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'Ū', 'ū', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Ligurian),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'É', 'é', 'Ç', 'ç'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Limburgish),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'È', 'è', 'Ë', 'ë', 'Ì', 'ì', 'Í', 'í',
                    'Î', 'î', 'Ó', 'ó', 'Ô', 'ô', 'Ò', 'ò', 'Ú', 'ú', 'Ù', 'ù', 'Û', 'û', 'Ä', 'ä',
                    'Ö', 'ö', 'Ü', 'ü'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Lingala),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Lithuanian),
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'Ė', 'ė', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Į', 'į', 'Y', 'y', 'J', 'j',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'Š', 'š', 'T', 't', 'U', 'u', 'Ų', 'ų', 'Ū', 'ū', 'V', 'v', 'Z', 'z', 'Ž', 'ž',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Lombard),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'È', 'è', 'É', 'é', 'Ì', 'ì', 'Í', 'í', 'Ò', 'ò',
                    'Ó', 'ó', 'Ù', 'ù', 'Ú', 'ú'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::LubaKasai),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Luo),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Luxembourgish),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ä', 'ä', 'É', 'é', 'Ë', 'ë', 'Ó', 'ó', 'Ö', 'ö', 'Ü', 'ü'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Malay),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::MalgasyPlateau),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'V', 'v', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Maltese),
                [
                    'A', 'a', 'B', 'b', 'Ċ', 'ċ', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    /* 'Għ', 'għ', */ 'H', 'h', 'Ħ', 'ħ', 'I', 'i',
                    /* 'Ie', 'ie', */ 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O',
                    'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W',
                    'w', 'X', 'x', 'Ż', 'ż', 'Z', 'z', 'Ġ', 'ġ',
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Maori),
                [
                    'A', 'a', 'E', 'e', 'H', 'h', 'I', 'i', 'K', 'k', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'T', 't', 'U', 'u', 'W', 'w', /* 'Ng', 'ng', */ 'G',
                    'g', /* 'Wh', 'wh' */ 'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Minangkabau),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Mizo),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Mossi),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Nuer),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ŋ', 'ŋ',
                    'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Nyanja),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Nynorsk),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Occitan),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'È', 'è', 'É', 'é', 'Í', 'í', 'Ò', 'ò', 'Ó', 'ó',
                    'Ú', 'ú', 'Ç', 'ç'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::OromoWestCentral),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'X', 'x', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Pangasinan),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'I', 'i', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Papiamento),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z', 'Á', 'á', 'É', 'é', 'È', 'è', 'Ó', 'ó', 'Ú', 'ú', 'Ü', 'ü', 'Ñ', 'ñ'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Polish),
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Ć', 'ć', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'Ł', 'ł',
                    'M', 'm', 'N', 'n', 'Ń', 'ń', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's',
                    'Ś', 'ś', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Ź', 'ź', 'Ż', 'ż',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Portuguese),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'Â', 'â', 'Ã', 'ã', 'À', 'à', 'Ç', 'ç', 'É', 'é',
                    'Ê', 'ê', 'Í', 'í', 'Ó', 'ó', 'Ô', 'ô', 'Õ', 'õ', 'Ú', 'ú', 'Ü', 'ü'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::QuechuaAyacucho),
                [
                    'A', 'a', /* 'Ch', 'ch', */ 'C', 'c', 'D', 'd', 'E', 'e', 'H', 'h', 'I',
                    'i', 'K', 'k', 'L', 'l', /* 'Ll', 'll', */ 'M', 'm', 'N', 'n', 'Ñ', 'ñ',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Romanian),
                [
                    'A', 'a', 'Ă', 'ă', 'Â', 'â', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Î', 'î', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ș', 'ș', 'T', 't',
                    'Ț', 'ț', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ţ', 'ţ',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Rundi),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Samoan),
                [
                    'A', 'a', 'E', 'e', 'F', 'f', 'G', 'g', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Sango),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Sardinian),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'É', 'é', 'È', 'è', 'Í', 'í', 'Ì', 'ì', 'Ó', 'ó',
                    'Ò', 'ò', 'Ú', 'ú', 'Ù', 'ù'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Sepedi),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ê', 'ê', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ô', 'ô', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Š', 'š'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Sesotho),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Š', 'š'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Shona),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Sicilian),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é', 'Ì', 'ì', 'Ó', 'ó', 'Ù', 'ù'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Silesian),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'Ł', 'ł', 'M', 'm', 'N', 'n',
                    'Ň', 'ň', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'Ř', 'ř', 'S', 's', 'Š', 'š',
                    'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ž', 'ž'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Slovak),
                [
                    'A', 'a', 'Á', 'á', 'Ä', 'ä', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'Ď', 'ď',
                    'E', 'e', 'É', 'é', 'F', 'f', 'G', 'g', 'H', 'h', /* 'Ch', 'ch', */ 'I',
                    'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L', 'l', 'Ľ', 'ľ', 'M', 'm', 'N', 'n', 'Ň',
                    'ň', 'O', 'o', 'Ó', 'ó', 'Ô', 'ô', 'P', 'p', 'R', 'r', 'Ŕ', 'ŕ', 'S', 's', 'Š',
                    'š', 'T', 't', 'Ť', 'ť', 'U', 'u', 'Ú', 'ú', 'V', 'v', 'Y', 'y', 'Ý', 'ý', 'Z',
                    'z', 'Ž', 'ž', 'Ĺ', 'ĺ',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Slovene),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Somali),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Q', 'q',
                    'R', 'r', 'S', 's', /* 'Sh', 'sh', */ 'T', 't', 'U', 'u', 'W', 'w', 'X',
                    'x', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Spanish),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú',
                    'Ü', 'ü'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Sundanese),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Swahili),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Swati),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Swedish),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Å', 'å', 'Ä', 'ä', 'Ö', 'ö',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Tagalog),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T',
                    't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Tamasheq),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::TatarCrimean),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z', 'Ğ', 'ğ', 'Ə', 'ə',
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::TokPisin),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Tsonga),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Tswana),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Tumbuka),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Turkish),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Turkmen),
                [
                    'A', 'a', 'B', 'b', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'ı', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ö', 'ö',
                    'P', 'p', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't', 'U', 'u', 'Ü', 'ü', 'W', 'w',
                    'Y', 'y', 'Z', 'z', 'Ä', 'ä', 'Ý', 'ý', 'Ž', 'ž',
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Twi),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Umbundu),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::UzbekNorthern),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Venetian),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é', 'Ì', 'ì', 'Ó', 'ó', 'Ù', 'ù',
                    'Ç', 'ç'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Vietnamese),
                [
                    'A', 'a', 'Ă', 'ă', 'Â', 'â', 'B', 'b', 'C', 'c', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'Ê', 'ê', 'G', 'g', 'H', 'h', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'Ô', 'ô', 'Ơ', 'ơ', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't',
                    'U', 'u', 'Ư', 'ư', 'V', 'v', 'X', 'x', 'Y', 'y', 'Á', 'á', 'À', 'à', 'Ả', 'ả',
                    'Ã', 'ã', 'Ạ', 'ạ', 'Ắ', 'ắ', 'Ằ', 'ằ', 'Ẳ', 'ẳ', 'Ẵ', 'ẵ', 'Ặ', 'ặ', 'Ấ', 'ấ',
                    'Ầ', 'ầ', 'Ẩ', 'ẩ', 'Ẫ', 'ẫ', 'Ậ', 'ậ', 'É', 'é', 'È', 'è', 'Ẻ', 'ẻ', 'Ẽ', 'ẽ',
                    'Ẹ', 'ẹ', 'Ế', 'ế', 'Ề', 'ề', 'Ể', 'ể', 'Ễ', 'ễ', 'Ệ', 'ệ', 'Í', 'í', 'Ì', 'ì',
                    'Ỉ', 'ỉ', 'Ĩ', 'ĩ', 'Ị', 'ị', 'Ó', 'ó', 'Ò', 'ò', 'Ỏ', 'ỏ', 'Õ', 'õ', 'Ọ', 'ọ',
                    'Ố', 'ố', 'Ồ', 'ồ', 'Ổ', 'ổ', 'Ỗ', 'ỗ', 'Ộ', 'ộ', 'Ớ', 'ớ', 'Ờ', 'ờ', 'Ở', 'ở',
                    'Ỡ', 'ỡ', 'Ợ', 'ợ', 'Ú', 'ú', 'Ù', 'ù', 'Ủ', 'ủ', 'Ũ', 'ũ', 'Ụ', 'ụ', 'Ứ', 'ứ',
                    'Ừ', 'ừ', 'Ử', 'ử', 'Ữ', 'ữ', 'Ự', 'ự', 'Ý', 'ý', 'Ỳ', 'ỳ', 'Ỷ', 'ỷ', 'Ỹ', 'ỹ',
                    'Ỵ', 'ỵ'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Waray),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i', 'K', 'k',
                    'L', 'l', 'M', 'm', 'N', 'n', /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p', 'R',
                    'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Á', 'á', 'É', 'é', 'Í',
                    'í', 'Ó', 'ó', 'Ú', 'ú', 'Ñ', 'ñ'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Welsh),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', /* 'Ch', 'ch', */ 'D', 'd',
                    /* 'Dd', 'dd', */ 'E', 'e', 'F', 'f', /* 'Ff', 'ff', */ 'G', 'g',
                    /* 'Ng', 'ng', */ 'H', 'h', 'I', 'i', 'L', 'l',
                    /* 'Ll', 'll', */ 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    /* 'Rh', 'rh', */ 'S', 's', 'T', 't', /* 'Th', 'th', */ 'U', 'u',
                    'W', 'w', 'Y', 'y', 'Â', 'â', 'Ê', 'ê', 'Î', 'î', 'Ô', 'ô', 'Û', 'û', 'Ŵ', 'ŵ',
                    'Ŷ', 'ŷ'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Wolof),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ŋ', 'ŋ'
                ],
            ),
            (
                Alphabet::Latin(LatinAlphabet::Xhosa),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z'
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Yoruba),
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'Ẹ', 'ẹ', 'F', 'f', 'G', 'g',
                    /* 'Gb', 'gb', */ 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M',
                    'm', 'N', 'n', 'O', 'o', 'Ọ', 'ọ', 'P', 'p', 'R', 'r', 'S', 's', 'Ṣ', 'ṣ', 'T',
                    't', 'U', 'u', 'W', 'w', 'Y', 'y', 'À', 'à', 'Á', 'á', 'È', 'è', 'É', 'é', 'Ì',
                    'ì', 'Í', 'í', 'Ò', 'ò', 'Ó', 'ó', 'Ù', 'ù', 'Ú', 'ú',
                ]
            ),
            (
                Alphabet::Latin(LatinAlphabet::Zulu),
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
        ]),
        /* Script::Lepcha => alphabet_match!([(Alphabet::Lepcha, [])]),
        Script::Limbu => alphabet_match!([(Alphabet::Limbu, [])]),
        Script::LinearA => alphabet_match!([(Alphabet::LinearA, [])]),
        Script::LinearB => alphabet_match!([(Alphabet::LinearB, [])]),
        Script::Lisu => alphabet_match!([(Alphabet::Lisu, [])]),
        Script::Lycian => alphabet_match!([(Alphabet::Lycian, [])]),
        Script::Lydian => alphabet_match!([(Alphabet::Lydian, [])]), */
        /* Script::Mahajani => alphabet_match!([
            (Alphabet::MahajaniHindi, []),
            (Alphabet::MahajaniMarwari, []),
            (Alphabet::MahajaniPunjabi, []),
        ]), */
        /* Script::Makasar => alphabet_match!([(Alphabet::Makasar, [])]),
        Script::Malayalam => alphabet_match!([(Alphabet::Malayalam, [])]),
        Script::Mandaic => alphabet_match!([(Alphabet::Mandaic, [])]),
        Script::Manichaean => alphabet_match!([(Alphabet::Manichaean, []),]),
        Script::Marchen => alphabet_match!([(Alphabet::Marchen, [])]),
        Script::MasaramGondi => alphabet_match!([(Alphabet::MasaramGondi, [])]),
        Script::Medefaidrin => alphabet_match!([(Alphabet::Medefaidrin, [])]),
        Script::MeeteiMayek => alphabet_match!([(Alphabet::MeeteiMayek, [])]),
        Script::MendeKikakui => alphabet_match!([(Alphabet::MendeKikakui, [])]),
        Script::MeroiticCursive => alphabet_match!([(Alphabet::MeroiticCursive, [])]),
        Script::MeroiticHieroglyphs => alphabet_match!([(Alphabet::MeroiticHieroglyphs, [])]),
        Script::Miao => alphabet_match!([(Alphabet::Miao, [])]),
        Script::Modi => alphabet_match!([(Alphabet::Modi, [])]),
        Script::Mongolian => alphabet_match!([(Alphabet::Mongolian, [])]),
        Script::Mro => alphabet_match!([(Alphabet::Mro, [])]),
        Script::Multani => alphabet_match!([(Alphabet::Multani, [])]),
        Script::Myanmar => alphabet_match!([(Alphabet::Myanmar, [])]),
        Script::Nabataean => alphabet_match!([(Alphabet::Nabataean, [])]),
        Script::NagMundari => alphabet_match!([(Alphabet::MundariNag, [])]),
        Script::Nandinagari => alphabet_match!([(Alphabet::Nandinagari, [])]),
        Script::NewTaiLue => alphabet_match!([(Alphabet::NewTaiLue, [])]),
        Script::Newa => alphabet_match!([(Alphabet::Newa, [])]),
        Script::Nko => alphabet_match!([(Alphabet::NKo, [])]),
        Script::Nushu => alphabet_match!([(Alphabet::ChineseNushu, [])]),
        Script::NyiakengPuachueHmong => alphabet_match!([(Alphabet::NyiakengPuachueHmong, [])]),
        Script::Ogham => alphabet_match!([(Alphabet::Ogham, [])]),
        Script::OlChiki => alphabet_match!([(Alphabet::OlChiki, [])]),
        Script::OlOnal => alphabet_match!([(Alphabet::OlOnal, [])]),
        Script::OldHungarian => alphabet_match!([(Alphabet::OldHungarian, [])]),
        Script::OldItalic => alphabet_match!([(Alphabet::OldItalic, []),]),
        Script::OldNorthArabian => alphabet_match!([(Alphabet::OldNorthArabian, [])]),
        Script::OldPermic => alphabet_match!([(Alphabet::OldKomiPermic, [])]),
        Script::OldPersian => alphabet_match!([(Alphabet::OldPersian, [])]),
        Script::OldSogdian => alphabet_match!([(Alphabet::OldSogdian, [])]),
        Script::OldSouthArabian => alphabet_match!([(Alphabet::OldSouthArabian, [])]),
        Script::OldTurkic => alphabet_match!([(Alphabet::OldTurkic, [])]),
        Script::OldUyghur => alphabet_match!([(Alphabet::OldUyghur, [])]),
        Script::Oriya => alphabet_match!([(Alphabet::Oriya, [])]),
        Script::Osage => alphabet_match!([(Alphabet::Osage, [])]),
        Script::Osmanya => alphabet_match!([(Alphabet::Osmanya, [])]),
        Script::PahawhHmong => alphabet_match!([(Alphabet::PahawhHmong, [])]),
        Script::Palmyrene => alphabet_match!([(Alphabet::Palmyrene, [])]),
        Script::PauCinHau => alphabet_match!([(Alphabet::PauCinHau, [])]),
        Script::PhagsPa => alphabet_match!([(Alphabet::PhagsPa, []),]),
        Script::Phoenician => alphabet_match!([(Alphabet::Phoenician, [])]),
        Script::PsalterPahlavi => alphabet_match!([(Alphabet::PsalterPahlavi, [])]),
        Script::Rejang => alphabet_match!([(Alphabet::Rejang, [])]),
        Script::Runic => alphabet_match!([(Alphabet::Runic, []),]),
        Script::Samaritan => alphabet_match!([(Alphabet::Samaritan, [])]),
        Script::Saurashtra => alphabet_match!([(Alphabet::Saurashtra, [])]),
        Script::Sharada => alphabet_match!([(Alphabet::Sharada, []),]),
        Script::Shavian => alphabet_match!([(Alphabet::Shavian, [])]),
        Script::Siddham => alphabet_match!([(Alphabet::Siddham, [])]),
        Script::SignWriting => alphabet_match!([(Alphabet::SignlanguageWriting, [])]),
        Script::Sinhala => alphabet_match!([(Alphabet::Sinhala, [])]),
        Script::Sogdian => alphabet_match!([(Alphabet::Sogdian, [])]),
        Script::SoraSompeng => alphabet_match!([(Alphabet::SoraSompeng, [])]),
        Script::Soyombo => alphabet_match!([(Alphabet::Soyombo, []),]),
        Script::Sundanese => alphabet_match!([(Alphabet::Sundanese, [])]),
        Script::Sunuwar => alphabet_match!([(Alphabet::Sunuwar, [])]),
        Script::SylotiNagri => alphabet_match!([(Alphabet::SylotiNagri, [])]),
        Script::Syriac => alphabet_match!([(Alphabet::Syriac, [])]),
        Script::Tagalog => alphabet_match!([(Alphabet::Tagalog, [])]),
        Script::Tagbanwa => alphabet_match!([(Alphabet::Tagbanwa, [])]),
        Script::TaiLe => alphabet_match!([(Alphabet::TaiLe, [])]),
        Script::TaiTham => alphabet_match!([(Alphabet::TaiTham, []),]),
        Script::TaiViet => alphabet_match!([(Alphabet::TaiViet, [])]),
        Script::Takri => alphabet_match!([(Alphabet::Takri, [])]),
        Script::Tamil => alphabet_match!([(Alphabet::Tamil, [])]),
        Script::Tangsa => alphabet_match!([(Alphabet::Tangsa, [])]),
        Script::Tangut => alphabet_match!([(Alphabet::Tangut, [])]),
        Script::Telugu => alphabet_match!([(Alphabet::Telugu, [])]),
        Script::Thaana => alphabet_match!([(Alphabet::Thaana, [])]),
        Script::Thai => alphabet_match!([(Alphabet::Thai, [])]),
        Script::Tibetan => alphabet_match!([(Alphabet::Tibetan, [])]),
        Script::Tifinagh => alphabet_match!([(Alphabet::Tifinagh, [])]),
        Script::Tirhuta => alphabet_match!([(Alphabet::Tirhuta, [])]),
        Script::Todhri => alphabet_match!([(Alphabet::Todhri, [])]),
        Script::Toto => alphabet_match!([(Alphabet::Toto, [])]),
        Script::TuluTigalari => alphabet_match!([(Alphabet::TuluTigalari, []),]),
        Script::Ugaritic => alphabet_match!([(Alphabet::Ugaritic, [])]),
        Script::Vai => alphabet_match!([(Alphabet::Vai, [])]),
        Script::Vithkuqi => alphabet_match!([(Alphabet::Vithkuqi, [])]),
        Script::Wancho => alphabet_match!([(Alphabet::Wancho, [])]),
        Script::WarangCiti => alphabet_match!([(Alphabet::WarangCiti, [])]),
        Script::Yezidi => alphabet_match!([(Alphabet::Yezidi, [])]),
        Script::Yi => alphabet_match!([(Alphabet::Yi, [])]),
        Script::ZanabazarSquare => alphabet_match!([(Alphabet::ZanabazarSquare, []),]), */
        _ => None,
    }
}
