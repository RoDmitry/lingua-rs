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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, strum_macros::Display, fixed_map::Key)]
pub enum Alphabet {
    Cyrillic(CyrillicAlphabet),
    Han(HanAlphabet),
    Latin(LatinAlphabet),
    Script(Script),
}
// );

/* impl From<Alphabet> for Script {
    fn from(a: Alphabet) -> Self {
        match a {
            Alphabet::Cyrillic(_) => Script::Cyrillic,
            Alphabet::Han(_) => Script::Han,
            Alphabet::Latin(_) => Script::Latin,
            Alphabet::Script(s) => s,
        }
    }
} */

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
                    Malay => &[Language::Malay],
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
                        Language::AcehneseJawi,
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
                        Language::BanjarJawi,
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, strum_macros::Display, fixed_map::Key)]
pub enum HanAlphabet in Han {
    ChineseSimplified,
    ChineseTraditional,
    Hanja, // Korean
    Kanji, // Japanese
}
);

alphabet_wrapper!(
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, strum_macros::Display, fixed_map::Key)]
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
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, strum_macros::Display, fixed_map::Key)]
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

/* pub(crate) fn script_char_to_alphabets(script: Option<Script>, ch: char) -> ScriptAlphabets {
    let alphabets = script.map(|s| script_char_match(s, ch));
    ScriptAlphabets::from((script, alphabets))
} */

/// add all the leters of all the alphabets in the script group
/// or not all, only if it does not require to exclude letters
/// if the script group has only one language, then leave it empty
pub(crate) fn script_char_to_langs(script: Script, ch: char) -> &'static [Language] {
    use Script::*;
    match script {
        Adlam => &[Language::Fulani, Language::Pular],
        Ahom => &[Language::Ahom],
        AnatolianHieroglyphs => &[Language::Luwian],
        Arabic => &[
            Language::AcehneseJawi,
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
            Language::BanjarJawi,
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
        // if you want to add something here, validate that char's range is active in `ucd.rs`
        // during parsing these considered as connectors, not chars of the word
        // example1: `can't` for english is one word, for other lang it is two words: `can, t`
        //   because if Alphabets of all 3 chars do not intersect, it will be two words
        // example2: `word1' word2` for all langs will be parsed as two words without `'`,
        //   because next char after `'` is space, which is not a char of any language
        Common => match ch {
            '\'' => &[
                Language::Belarusian,
                Language::Acehnese,
                Language::Afrikaans,
                Language::AlbanianTosk,
                Language::AymaraCentral,
                Language::AzerbaijaniNorth,
                Language::Banjar,
                Language::Bemba,
                Language::Bokmal,
                Language::Buginese,
                Language::Catalan,
                Language::CreoleHaitian,
                Language::Danish,
                Language::Dyula,
                Language::English,
                Language::Fon,
                Language::French,
                Language::Friulian,
                Language::FulfuldeNigerian,
                Language::GaelicScottish,
                Language::Ganda,
                Language::Guarani,
                Language::MalgasyPlateau,
            ],
            '¡' => &[Language::Spanish],
            '¿' => &[Language::Spanish],
            'ʻ' => &[Language::UzbekNorthern],
            _ => &[], // must be always empty
        },
        Coptic => &[Language::Coptic],
        Cuneiform => &[Language::Akkadian, Language::Hittite, Language::Sumerian],
        Cypriot => &[Language::AncientGreek],
        CyproMinoan => &[Language::CyproMinoan],
        Cyrillic => alphabet_match!([
            // TODO: Lost some alphabets from CyrillicAlphabet
            (
                Language::Belarusian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'І', 'і', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ў', 'ў', 'Ф', 'ф', 'Х', 'х',
                    'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Language::Bulgarian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ж', 'ж', 'З', 'з',
                    'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о', 'П', 'п',
                    'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч',
                    'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ь', 'ь', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Language::Kazakh,
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
                Language::Macedonian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Ѓ', 'ѓ', 'Е', 'е', 'Ж', 'ж',
                    'З', 'з', 'Ѕ', 'ѕ', 'И', 'и', 'Ј', 'ј', 'К', 'к', 'Л', 'л', 'Љ', 'љ', 'М', 'м',
                    'Н', 'н', 'Њ', 'њ', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ќ', 'ќ',
                    'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Џ', 'џ', 'Ш', 'ш',
                ]
            ),
            (
                Language::MongolianHalh,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ү', 'ү', 'Ф', 'ф',
                    'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь',
                    'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Language::OldChurchSlavonic,
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
                Language::Russian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц',
                    'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю',
                    'Я', 'я',
                ],
            ),
            (
                Language::Serbian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Ђ', 'ђ', 'Е', 'е', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Ј', 'ј', 'К', 'к', 'Л', 'л', 'Љ', 'љ', 'М', 'м', 'Н', 'н',
                    'Њ', 'њ', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ћ', 'ћ', 'У', 'у',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Џ', 'џ', 'Ш', 'ш',
                ]
            ),
            (
                Language::Ukrainian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Ґ', 'ґ', 'Д', 'д', 'Е', 'е', 'Є', 'є',
                    'Ж', 'ж', 'З', 'з', 'И', 'и', 'І', 'і', 'Ї', 'ї', 'Й', 'й', 'К', 'к', 'Л', 'л',
                    'М', 'м', 'Н', 'н', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ь', 'ь', 'Ю', 'ю',
                    'Я', 'я',
                ],
            ),
        ]),
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
        Han => alphabet_match!([
            (
                Language::Chinese, // ChineseSimplified
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
            (Language::Chinese, []), // ChineseTraditional
            (Language::Korean, []),
            (
                // it also uses all Traditional Chinese characters
                Language::Japanese,
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
        Inherited => &[], /* match ch {
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
        Latin => alphabet_match!([
            (
                Language::Acehnese, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', /* 'F', 'f', */ 'G',
                    'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O',
                    'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    /* 'V', 'v', */ 'W', 'w', /* 'X', 'x', */ 'Y', 'y',
                    /* 'Z', 'z', */ 'É', 'é', 'È', 'è', 'Ë', 'ë', 'Ô', 'ô', 'Ö', 'ö'
                ]
            ),
            (
                Language::Afrikaans, //+
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
                Language::AlbanianTosk, //+
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
                Language::Asturian, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú',
                    'Ü', 'ü',
                ]
            ),
            (
                Language::AymaraCentral, //+
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
                Language::AzerbaijaniNorth, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'Ə', 'ə', 'F', 'f',
                    'G', 'g', 'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's',
                    'Ş', 'ş', 'T', 't', 'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Language::Balinese, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'É', 'é'
                ]
            ),
            (
                Language::Bambara, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                    'É', 'é', 'Ô', 'ô'
                ]
            ),
            (
                Language::Banjar, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'É', 'é'
                ]
            ),
            (
                Language::Basque, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'X', 'x', 'Z', 'z'
                ]
            ),
            (
                Language::Bemba, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::Bokmal, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å', 'Ç', 'ç', 'É', 'é'
                ]
            ),
            (
                Language::Bosnian, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž', 'Ç', 'ç',
                ]
            ),
            (
                Language::Buginese, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Catalan, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'Ç', 'ç', 'É', 'é', 'È', 'è', 'Ë', 'ë', 'Í', 'í',
                    'Ï', 'ï', 'Ó', 'ó', 'Ò', 'ò', 'Ü', 'ü', 'Ú', 'ú',
                ]
            ),
            (
                Language::Cebuano, //+
                [
                    'A', 'a', 'B', 'b', /* 'C', 'c', */ 'D', 'd', 'E', 'e',
                    /* 'F', 'f', */ 'G', 'g', 'H', 'h', 'I', 'i', /* 'J', 'j', */ 'K',
                    'k', 'L', 'l', 'M', 'm', 'N', 'n', /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', /* 'V', 'v', */ 'W', 'w', 'Y',
                    'y',
                ]
            ),
            (
                Language::Chokwe, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::CreoleHaitian, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                    'À', 'à', 'È', 'è', 'É', 'é', 'Ò', 'ò'
                ]
            ),
            (
                Language::Croatian, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž'
                ]
            ),
            (
                Language::Czech, //+
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
                Language::Danish, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Å', 'å', 'Æ', 'æ', 'É', 'é', 'Ø', 'ø',
                ]
            ),
            (
                Language::DinkaSouthwestern, //+
                [
                    'Ä', 'ä', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ë', 'ë', 'Ɛ', 'ɛ',
                    'G', 'g', 'Ɣ', 'ɣ', 'H', 'h', 'I', 'i', 'Ï', 'ï', 'J', 'j', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'Ŋ', 'ŋ', 'Ö', 'ö', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                ]
            ),
            (
                Language::Dutch, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'Ä', 'ä', 'È', 'è', 'Ë', 'ë', 'É', 'é', 'Ï', 'ï',
                    'Ĳ', 'ĳ', 'Ó', 'ó', 'Ö', 'ö', 'Ü', 'ü',
                ]
            ),
            (
                Language::Dyula, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ɲ', 'ɲ',
                    'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v',
                    'W', 'w', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Language::English,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::Esperanto, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ĉ', 'ĉ', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ĝ', 'ĝ', 'H', 'h', 'Ĥ', 'ĥ', 'I', 'i', 'J', 'j', 'Ĵ', 'ĵ', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'Ŝ', 'ŝ', 'T', 't',
                    'U', 'u', 'Ŭ', 'ŭ', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z', // XY?
                ]
            ),
            (
                Language::Estonian, //loanwords?
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Ä', 'ä', 'Ö', 'ö', 'Õ', 'õ', 'Š', 'š',
                    'Ü', 'ü', 'Ž', 'ž', 'Z', 'z',
                ]
            ),
            (
                Language::Ewe, //+
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'Ð', 'Ɖ', 'ɖ', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'Ƒ',
                    'ƒ', 'G', 'g', 'Ɣ', 'ɣ', 'H', 'h', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N',
                    'n', 'Ŋ', 'ŋ', 'O', 'o', 'Ɔ', 'ɔ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U',
                    'u', 'V', 'v', 'Ʋ', 'ʋ', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ã', 'ã', 'À',
                    'à', 'È', 'è', 'Ẽ', 'ẽ', 'É', 'é', 'Ĩ', 'ĩ', 'Í', 'í', 'Ò', 'ò',
                ]
            ),
            (
                Language::Faroese, //+
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'D', 'd', 'Ð', 'ð', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'Ú', 'ú',
                    'V', 'v', 'Y', 'y', 'Ý', 'ý', 'Æ', 'æ', 'Ø', 'ø'
                ]
            ),
            (
                Language::Fijian, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Finnish, //+
                [
                    'A', 'a', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k',
                    'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't',
                    'U', 'u', 'V', 'v', 'Y', 'y', 'Ä', 'ä', 'Ö', 'ö',
                ]
            ),
            (
                Language::Fon, //+
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
                Language::French, //+
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
                Language::Friulian, //+
                [
                    'A', 'a', 'Â', 'â', 'À', 'à', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'Ê', 'ê',
                    'È', 'è', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Ì', 'ì', 'Î', 'î',
                    'J', 'j', 'L', 'l', 'M', 'm', 'N', 'n', 'Ò', 'ò', 'Ô', 'ô', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'Ù', 'ù', 'Û', 'û', 'U', 'u', 'V', 'v', 'Z', 'z',
                ]
            ),
            (
                Language::FulfuldeNigerian, //+ added extra `ŋ`
                [
                    'A', 'a', 'B', 'b', 'Ɓ', 'ɓ', 'C', 'c', 'D', 'd', 'Ɗ', 'ɗ', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y',
                    'Ƴ', 'ƴ', 'Ŋ', 'ŋ',
                ],
            ),
            (
                Language::GaelicScottish, //+
                [
                    'À', 'à', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'È', 'è', 'F', 'f',
                    'G', 'g', 'H', 'h', 'Ì', 'ì', 'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ò', 'ò', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'Ù', 'ù', 'U', 'u',
                ],
            ),
            (
                Language::Galician, //+
                [
                    'Á', 'á', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'É', 'é', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'Í', 'í', 'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ',
                    'Ó', 'ó', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'Ú', 'ú', 'V', 'v', 'X', 'x', 'Z', 'z',
                ],
            ),
            (
                Language::Ganda, //+
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ŋ', 'ŋ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Language::German, //+
                [
                    'Ä', 'ä', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'ẞ', 'ß', 'T', 't', 'Ü', 'ü',
                    'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Language::Guarani, //+
                [
                    'Ã', 'ã', 'Á', 'á', 'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'Ẽ', 'ẽ', 'Ê', 'ê',
                    'E', 'e', 'É', 'é', 'G', 'g', 'H', 'h', 'Í', 'í', 'Ĩ', 'ĩ', 'Î', 'î', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'Ó', 'ó', 'O', 'o',
                    'Õ', 'õ', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'Ú', 'ú', 'U', 'u', 'Ũ', 'ũ',
                    'V', 'v', 'Ý', 'ý', 'Ỹ', 'ỹ', 'Y', 'y',
                ],
            ),
            (
                Language::Hausa,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Ɓ', 'ɓ', 'Ɗ', 'ɗ', 'Ƙ', 'ƙ',
                    'Ƴ', 'ƴ'
                ],
            ),
            (
                Language::Hungarian,
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
                Language::Icelandic,
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'D', 'd', 'Ð', 'ð', 'E', 'e', 'É', 'é', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'Ú', 'ú', 'V', 'v', 'X', 'x', 'Y', 'y', 'Ý', 'ý', 'Þ', 'þ', 'Æ', 'æ', 'Ö', 'ö',
                ]
            ),
            (
                Language::Igbo,
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
                Language::Ilocano,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'I', 'i', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'W', 'w', 'Y', 'y', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Ñ', 'ñ'
                ]
            ),
            (
                Language::Indonesian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::Irish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú'
                ]
            ),
            (
                Language::Italian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é',
                    'Ì', 'ì', 'Ò', 'ò', 'Ù', 'ù'
                ]
            ),
            (
                Language::Javanese,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::Jingpho,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::Kabiye,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Kabuverdianu,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Z', 'z',
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú', 'Õ', 'õ', 'Ç', 'ç', 'Ê', 'ê'
                ]
            ),
            (
                Language::Kabyle,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Č', 'č', 'Ɣ', 'ɣ'
                ]
            ),
            (
                Language::Kamba,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'S', 's',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::KanuriCentral,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Kikongo,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Kikuyu,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Í', 'í', 'Ú', 'ú'
                ]
            ),
            (
                Language::Kimbundu,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'S', 's', 'T', 't',
                    'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Kinyarwanda,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Í', 'í', 'Ú', 'ú'
                ]
            ),
            (
                Language::KurdishNorthern,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'Ê', 'ê', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Î', 'î', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Û', 'û', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::Latgalian,
                [
                    'A', 'a', 'Ā', 'ā', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ē', 'ē',
                    'F', 'f', 'G', 'g', 'Ģ', 'ģ', 'H', 'h', 'I', 'i', 'Ī', 'ī', 'J', 'j', 'K', 'k',
                    'Ķ', 'ķ', 'L', 'l', 'Ļ', 'ļ', 'M', 'm', 'N', 'n', 'Ņ', 'ņ', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'Ū', 'ū', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž'
                ]
            ),
            (
                Language::Latin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z',
                    'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū', 'Æ', 'æ', 'Œ', 'œ'
                ]
            ),
            (
                Language::Latvian,
                [
                    'A', 'a', 'Ā', 'ā', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ē', 'ē',
                    'F', 'f', 'G', 'g', 'Ģ', 'ģ', 'H', 'h', 'I', 'i', 'Ī', 'ī', 'J', 'j', 'K', 'k',
                    'Ķ', 'ķ', 'L', 'l', 'Ļ', 'ļ', 'M', 'm', 'N', 'n', 'Ņ', 'ņ', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'Ū', 'ū', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                Language::Ligurian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'É', 'é', 'Ç', 'ç'
                ]
            ),
            (
                Language::Limburgish,
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
                Language::Lingala,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Lithuanian,
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'Ė', 'ė', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Į', 'į', 'Y', 'y', 'J', 'j',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'Š', 'š', 'T', 't', 'U', 'u', 'Ų', 'ų', 'Ū', 'ū', 'V', 'v', 'Z', 'z', 'Ž', 'ž',
                ]
            ),
            (
                Language::Lombard,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'È', 'è', 'É', 'é', 'Ì', 'ì', 'Í', 'í', 'Ò', 'ò',
                    'Ó', 'ó', 'Ù', 'ù', 'Ú', 'ú'
                ]
            ),
            (
                Language::LubaKasai,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Luo,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Luxembourgish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ä', 'ä', 'É', 'é', 'Ë', 'ë', 'Ó', 'ó', 'Ö', 'ö', 'Ü', 'ü'
                ]
            ),
            (
                Language::Malay,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::MalgasyPlateau,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'V', 'v', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Language::Maltese,
                [
                    'A', 'a', 'B', 'b', 'Ċ', 'ċ', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    /* 'Għ', 'għ', */ 'H', 'h', 'Ħ', 'ħ', 'I', 'i',
                    /* 'Ie', 'ie', */ 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O',
                    'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W',
                    'w', 'X', 'x', 'Ż', 'ż', 'Z', 'z', 'Ġ', 'ġ',
                ],
            ),
            (
                Language::Maori,
                [
                    'A', 'a', 'E', 'e', 'H', 'h', 'I', 'i', 'K', 'k', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'T', 't', 'U', 'u', 'W', 'w', /* 'Ng', 'ng', */ 'G',
                    'g', /* 'Wh', 'wh' */ 'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū'
                ]
            ),
            (
                Language::Minangkabau,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Language::Mizo,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z'
                ],
            ),
            (
                Language::Mossi,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::Nuer,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ɛ', 'ɛ', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ŋ', 'ŋ',
                    'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::Nyanja,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Language::Nynorsk,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å',
                ]
            ),
            (
                Language::Occitan,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'È', 'è', 'É', 'é', 'Í', 'í', 'Ò', 'ò', 'Ó', 'ó',
                    'Ú', 'ú', 'Ç', 'ç'
                ],
            ),
            (
                Language::OromoWestCentral,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'X', 'x', 'Y', 'y'
                ],
            ),
            (
                Language::Pangasinan,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'I', 'i', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::Papiamento,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z', 'Á', 'á', 'É', 'é', 'È', 'è', 'Ó', 'ó', 'Ú', 'ú', 'Ü', 'ü', 'Ñ', 'ñ'
                ],
            ),
            (
                Language::Polish,
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Ć', 'ć', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'Ł', 'ł',
                    'M', 'm', 'N', 'n', 'Ń', 'ń', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's',
                    'Ś', 'ś', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Ź', 'ź', 'Ż', 'ż',
                ]
            ),
            (
                Language::Portuguese,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'Â', 'â', 'Ã', 'ã', 'À', 'à', 'Ç', 'ç', 'É', 'é',
                    'Ê', 'ê', 'Í', 'í', 'Ó', 'ó', 'Ô', 'ô', 'Õ', 'õ', 'Ú', 'ú', 'Ü', 'ü'
                ]
            ),
            (
                Language::QuechuaAyacucho,
                [
                    'A', 'a', /* 'Ch', 'ch', */ 'C', 'c', 'D', 'd', 'E', 'e', 'H', 'h', 'I',
                    'i', 'K', 'k', 'L', 'l', /* 'Ll', 'll', */ 'M', 'm', 'N', 'n', 'Ñ', 'ñ',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::Romanian,
                [
                    'A', 'a', 'Ă', 'ă', 'Â', 'â', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Î', 'î', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ș', 'ș', 'T', 't',
                    'Ț', 'ț', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ţ', 'ţ',
                ]
            ),
            (
                Language::Rundi,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::Samoan,
                [
                    'A', 'a', 'E', 'e', 'F', 'f', 'G', 'g', 'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v'
                ],
            ),
            (
                Language::Sango,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Language::Sardinian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'É', 'é', 'È', 'è', 'Í', 'í', 'Ì', 'ì', 'Ó', 'ó',
                    'Ò', 'ò', 'Ú', 'ú', 'Ù', 'ù'
                ],
            ),
            (
                Language::Sepedi,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'Ê', 'ê', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ô', 'ô', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Š', 'š'
                ]
            ),
            (
                Language::Sesotho,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Š', 'š'
                ]
            ),
            (
                Language::Shona,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú'
                ]
            ),
            (
                Language::Sicilian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é', 'Ì', 'ì', 'Ó', 'ó', 'Ù', 'ù'
                ],
            ),
            (
                Language::Silesian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'Ł', 'ł', 'M', 'm', 'N', 'n',
                    'Ň', 'ň', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'Ř', 'ř', 'S', 's', 'Š', 'š',
                    'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'Ž', 'ž'
                ],
            ),
            (
                Language::Slovak,
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
                Language::Slovene,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                Language::Somali,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Q', 'q',
                    'R', 'r', 'S', 's', /* 'Sh', 'sh', */ 'T', 't', 'U', 'u', 'W', 'w', 'X',
                    'x', 'Y', 'y'
                ]
            ),
            (
                Language::Spanish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú',
                    'Ü', 'ü'
                ]
            ),
            (
                Language::Sundanese,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Language::Swahili,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::Swati,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::Swedish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Å', 'å', 'Ä', 'ä', 'Ö', 'ö',
                ]
            ),
            (
                Language::Tagalog,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T',
                    't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Language::Tamasheq,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::TatarCrimean,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    'O', 'o', 'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z', 'Ğ', 'ğ', 'Ə', 'ə',
                ],
            ),
            (
                Language::TokPisin,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Language::Tsonga,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z'
                ]
            ),
            (
                Language::Tswana,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Language::Tumbuka,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Language::Turkish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Language::Turkmen,
                [
                    'A', 'a', 'B', 'b', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'ı', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ö', 'ö',
                    'P', 'p', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't', 'U', 'u', 'Ü', 'ü', 'W', 'w',
                    'Y', 'y', 'Z', 'z', 'Ä', 'ä', 'Ý', 'ý', 'Ž', 'ž',
                ],
            ),
            (
                Language::Twi,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::Umbundu,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y'
                ],
            ),
            (
                Language::UzbekNorthern,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z'
                ],
            ),
            (
                Language::Venetian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é', 'Ì', 'ì', 'Ó', 'ó', 'Ù', 'ù',
                    'Ç', 'ç'
                ],
            ),
            (
                Language::Vietnamese,
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
                Language::Waray,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'G', 'g', 'H', 'h', 'I', 'i', 'K', 'k',
                    'L', 'l', 'M', 'm', 'N', 'n', /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p', 'R',
                    'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Á', 'á', 'É', 'é', 'Í',
                    'í', 'Ó', 'ó', 'Ú', 'ú', 'Ñ', 'ñ'
                ],
            ),
            (
                Language::Welsh,
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
                Language::Wolof,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ŋ', 'ŋ'
                ],
            ),
            (
                Language::Xhosa,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z'
                ]
            ),
            (
                Language::Yoruba,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'Ẹ', 'ẹ', 'F', 'f', 'G', 'g',
                    /* 'Gb', 'gb', */ 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M',
                    'm', 'N', 'n', 'O', 'o', 'Ọ', 'ọ', 'P', 'p', 'R', 'r', 'S', 's', 'Ṣ', 'ṣ', 'T',
                    't', 'U', 'u', 'W', 'w', 'Y', 'y', 'À', 'à', 'Á', 'á', 'È', 'è', 'É', 'é', 'Ì',
                    'ì', 'Í', 'í', 'Ò', 'ò', 'Ó', 'ó', 'Ù', 'ù', 'Ú', 'ú',
                ]
            ),
            (
                Language::Zulu,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
        ]),
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
        // _ => None,
    }
}
