use std::cmp::Ordering;

use strum_macros::EnumIter;

use crate::{ExtraCheck, Language};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, EnumIter)]
pub enum Script {
    Adlam,
    Ahom,
    Anatolian_Hieroglyphs,
    Arabic,
    Armenian,
    Avestan,
    Balinese,
    Bamum,
    Bassa_Vah,
    // Batak,
    Bengali,
    Bhaiksuki,
    Bopomofo,
    Brahmi,
    // Braille,
    Buginese,
    Buhid,
    // Canadian_Aboriginal,
    Carian,
    Caucasian_Albanian,
    Chakma,
    Cham,
    Cherokee,
    Chorasmian,
    // Common,
    Coptic,
    Cuneiform,
    Cypriot,
    // Cypro_Minoan,
    Cyrillic,
    Deseret,
    Devanagari,
    Dives_Akuru,
    Dogra,
    // Duployan,
    Egyptian_Hieroglyphs,
    Elbasan,
    Elymaic,
    Ethiopic,
    Garay,
    Georgian,
    Glagolitic,
    Gothic,
    Grantha,
    Greek,
    Gujarati,
    Gunjala_Gondi,
    Gurmukhi,
    Gurung_Khema,
    Han,
    Hangul,
    Hanifi_Rohingya,
    Hanunoo,
    Hatran,
    Hebrew,
    Hiragana,
    Imperial_Aramaic,
    // Inherited,
    Inscriptional_Pahlavi,
    Inscriptional_Parthian,
    Javanese,
    Kaithi,
    Kannada,
    Katakana,
    Kawi,
    Kayah_Li,
    Kharoshthi,
    Khitan_Small_Script,
    Khmer,
    Khojki,
    Khudawadi,
    Kirat_Rai,
    Lao,
    Latin,
    Lepcha,
    Limbu,
    // Linear_A,
    Linear_B,
    Lisu,
    Lycian,
    Lydian,
    Mahajani,
    Makasar,
    Malayalam,
    Mandaic,
    Manichaean,
    Marchen,
    Masaram_Gondi,
    Medefaidrin,
    Meetei_Mayek,
    Mende_Kikakui,
    Meroitic_Cursive,
    Meroitic_Hieroglyphs,
    Miao,
    Modi,
    Mongolian,
    Mro,
    Multani,
    Myanmar,
    Nabataean,
    Nag_Mundari,
    Nandinagari,
    New_Tai_Lue,
    Newa,
    Nko,
    Nushu,
    Nyiakeng_Puachue_Hmong,
    Ogham,
    Ol_Chiki,
    Ol_Onal,
    Old_Hungarian,
    Old_Italic,
    Old_North_Arabian,
    Old_Permic,
    Old_Persian,
    Old_Sogdian,
    Old_South_Arabian,
    Old_Turkic,
    Old_Uyghur,
    Oriya,
    Osage,
    Osmanya,
    Pahawh_Hmong,
    Palmyrene,
    Pau_Cin_Hau,
    Phags_Pa,
    Phoenician,
    Psalter_Pahlavi,
    Rejang,
    Runic,
    Samaritan,
    Saurashtra,
    Sharada,
    Shavian,
    Siddham,
    SignWriting,
    Sinhala,
    Sogdian,
    Sora_Sompeng,
    Soyombo,
    Sundanese,
    Sunuwar,
    Syloti_Nagri,
    Syriac,
    Tagalog,
    Tagbanwa,
    Tai_Le,
    Tai_Tham,
    Tai_Viet,
    Takri,
    Tamil,
    Tangsa,
    Tangut,
    Telugu,
    Thaana,
    Thai,
    Tibetan,
    Tifinagh,
    Tirhuta,
    Todhri,
    Toto,
    Tulu_Tigalari,
    Ugaritic,
    Vai,
    Vithkuqi,
    Wancho,
    Warang_Citi,
    Yezidi,
    Yi,
    Zanabazar_Square,
    /* Arabic,
    Armenian,
    Bengali,
    Cyrillic,
    Devanagari,
    Georgian,
    Greek,
    Gujarati,
    Gurmukhi,
    Han,
    Hangul,
    Hebrew,
    Hiragana,
    Katakana,
    Latin,
    Tamil,
    Telugu,
    Thai, */
}

impl ExtraCheck for Script {}

const fn char_ranges_count() -> usize {
    let mut i = 0;
    let mut cnt = 0;
    while i < crate::script::BY_NAME.len() {
        cnt += crate::script::BY_NAME[i].1.len();
        i += 1;
    }

    cnt
}
const LEN: usize = char_ranges_count();

#[derive(Clone, Copy, Debug)]
struct RangeScript {
    range_start: char,
    range_end: char,
    script: Script,
}
const RANGE_SCRIPT_DEFAULT: RangeScript = RangeScript {
    range_start: char::MAX,
    range_end: char::MAX,
    script: Script::Latin,
};

/* #[const_trait]
trait ConstDefault {
    fn default<const RUNTIME: bool>() -> Self;
}
impl const ConstDefault for RangeScript {
    fn default() -> Self {
        RangeScript {
            range_start: char::MAX,
            range_end: char::MAX,
            script: Script::Latin,
        }
    }
} */

const fn char_ranges_array_sorted() -> [RangeScript; LEN] {
    let mut res: [RangeScript; LEN] = [RANGE_SCRIPT_DEFAULT; LEN];

    // foreach BY_NAME
    let mut i = 0;
    while i < crate::script::BY_NAME.len() {
        let (script, ranges) = crate::script::BY_NAME[i];
        // foreach charset
        let mut j = 0;
        while j < ranges.len() {
            let range = ranges[j];
            // looking for insertion
            let mut ins = 0;
            while ins < LEN {
                let mut prev = res[ins];
                if range.0 < prev.range_start {
                    res[ins] = RangeScript {
                        range_start: range.0,
                        range_end: range.1,
                        script,
                    };
                    if prev.range_start == char::MAX {
                        break;
                    }
                    // shifts all elements right
                    let mut next_ins = ins + 1;
                    while next_ins < LEN {
                        let next = res[next_ins];
                        res[next_ins] = prev;
                        if next.range_start == char::MAX {
                            break;
                        }
                        prev = next;
                        next_ins += 1;
                    }
                    break;
                }
                ins += 1;
            }
            j += 1;
        }
        i += 1;
    }

    res
}
const CHAR_RANGES_SORTED: [RangeScript; LEN] = char_ranges_array_sorted();

/* #[test]
fn testing() {
    panic!("{:?}", CHAR_RANGES_SORTED);
} */

#[inline(always)]
fn compare(ra: &RangeScript, ch: char) -> Ordering {
    if ch < ra.range_start {
        Ordering::Greater
    } else if ch > ra.range_end {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

pub(crate) fn find_script(ch: char) -> Option<Script> {
    CHAR_RANGES_SORTED
        .binary_search_by(|ra| compare(ra, ch))
        .ok()
        .map(|i| unsafe { CHAR_RANGES_SORTED.get(i).unwrap() }.script) // todo: unchecked
}

/* pub(crate) fn find_script(ch: char) -> Option<Script> {
    crate::script::BY_SCRIPT
        .iter()
        .find(|(_, chars)| chars.iter().any(|cs| ch > cs.0 || ch <= cs.1))
        .map(|v| v.0)
}

pub(crate) fn script_same(script: Script, ch: char) -> bool {
    crate::script::BY_SCRIPT
        .iter()
        .find(|(a, _)| a == &script)
        .map(|(_, chars)| chars.iter().any(|cs| ch > cs.0 || ch <= cs.1))
        .unwrap_or_default()
} */
/* const fn insertion_sort<const N: usize>(mut arr: [u32; N]) -> [u32; N] {
    let mut i = 1;
    while i < N {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }

    arr
} */

/// add all the leters of all the alphabets in the script group
/// or only specific to the lang (only if it does not require to exclude letters)
/// if the script group has only one language, then leave it empty
fn script_langs_alphabet(a: Script) -> &'static [(Language, &'static [char])] {
    match a {
        Script::Adlam => &[(Language::Fulani, &[]), (Language::Pular, &[])],
        Script::Ahom => &[(Language::Ahom, &[])],
        Script::Anatolian_Hieroglyphs => &[(Language::Luwian, &[])],
        Script::Arabic => &[
            (Language::Arabic, &[]),
            (Language::Kurdish, &[]),
            (Language::Pashto, &[]),
            (Language::Persian, &[]),
            (Language::Sindhi, &[]),
            (Language::Urdu, &[]),
            (Language::Uyghur, &[]),
        ],
        Script::Armenian => &[(Language::Armenian, &[])],
        Script::Avestan => &[(Language::Avestan, &[])],
        Script::Balinese => &[(Language::Balinese, &[])],
        Script::Bamum => &[(Language::Bamum, &[])],
        Script::Bassa_Vah => &[(Language::Bassa, &[])],
        // Script::Batak => &[Language::Batak Toba, Language::Batak Karo, Language::Batak Mandailing, Language::Batak Pakpak, Language::Batak Simalungun, Language::Batak Angkola],
        Script::Bengali => &[
            (Language::Assamese, &[]),
            (Language::Bengali, &[]),
            (Language::BishnupriyaManipuri, &[]),
        ],
        Script::Bhaiksuki => &[(Language::Bhaiksuki, &[])],
        Script::Bopomofo => &[(Language::MandarinChinese, &[])],
        Script::Brahmi => &[(Language::Sanskrit, &[]), (Language::Prakrit, &[])],
        // Script::Braille => &[(Language::Any language adapted to Braille,&[])],
        Script::Buginese => &[(Language::Buginese, &[]), (Language::Makassarese, &[])],
        Script::Buhid => &[(Language::Buhid, &[])],
        // Script::Canadian_Aboriginal => &[(Language::Cree,&[]),(Language::Inuktitut,&[]),(Language::Ojibwe,&[])],
        Script::Carian => &[(Language::Carian, &[])],
        Script::Caucasian_Albanian => &[(Language::CaucasianAlbanian, &[])],
        Script::Chakma => &[(Language::Chakma, &[])],
        Script::Cham => &[(Language::Cham, &[])],
        Script::Cherokee => &[(Language::Cherokee, &[])],
        Script::Chorasmian => &[(Language::Chorasmian, &[])],
        Script::Coptic => &[(Language::Coptic, &[])],
        Script::Cuneiform => &[
            (Language::Akkadian, &[]),
            (Language::Hittite, &[]),
            (Language::Sumerian, &[]),
        ],
        Script::Cypriot => &[(Language::AncientGreek, &[])],
        // Script::Cypro_Minoan => &[(Language::Unknown (used in ancient Cyprus),&[])],
        Script::Cyrillic => &[
            (Language::Belarusian, &[]),
            (Language::Bulgarian, &[]),
            (
                Language::Kazakh,
                &[
                    'А', 'Ә', 'Б', 'В', 'Г', 'Ғ', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Қ', 'Л',
                    'М', 'Н', 'Ң', 'О', 'Ө', 'П', 'Р', 'С', 'Т', 'У', 'Ұ', 'Ү', 'Ф', 'Х', 'Һ', 'Ц',
                    'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'І', 'Ь', 'Э', 'Ю', 'Я', 'а', 'ә', 'б', 'в', 'г', 'ғ',
                    'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'қ', 'л', 'м', 'н', 'ң', 'о', 'ө', 'п',
                    'р', 'с', 'т', 'у', 'ұ', 'ү', 'ф', 'х', 'һ', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'і',
                    'ь', 'э', 'ю', 'я',
                ],
            ),
            (Language::Macedonian, &[]),
            (Language::Mongolian, &[]),
            (
                Language::Russian,
                &[
                    'А', 'Б', 'В', 'Г', 'Д', 'Е', 'Ё', 'Ж', 'З', 'И', 'Й', 'К', 'Л', 'М', 'Н', 'О',
                    'П', 'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ъ', 'Ы', 'Ь', 'Э', 'Ю',
                    'Я', 'а', 'б', 'в', 'г', 'д', 'е', 'ё', 'ж', 'з', 'и', 'й', 'к', 'л', 'м', 'н',
                    'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ъ', 'ы', 'ь', 'э',
                    'ю', 'я',
                ],
            ),
            (Language::Serbian, &[]),
            (
                Language::Ukrainian,
                &[
                    'А', 'Б', 'В', 'Г', 'Ґ', 'Д', 'Е', 'Є', 'Ж', 'З', 'И', 'І', 'Ї', 'Й', 'К', 'Л',
                    'М', 'Н', 'О', 'П', 'Р', 'С', 'Т', 'У', 'Ф', 'Х', 'Ц', 'Ч', 'Ш', 'Щ', 'Ь', 'Ю',
                    'Я', 'а', 'б', 'в', 'г', 'ґ', 'д', 'е', 'є', 'ж', 'з', 'и', 'і', 'ї', 'й', 'к',
                    'л', 'м', 'н', 'о', 'п', 'р', 'с', 'т', 'у', 'ф', 'х', 'ц', 'ч', 'ш', 'щ', 'ь',
                    'ю', 'я',
                ],
            ),
        ],
        Script::Deseret => &[(Language::EnglishMormon, &[])],
        Script::Devanagari => &[
            (Language::Hindi, &[]),
            (Language::Marathi, &[]),
            (Language::Nepali, &[]),
            (Language::Sanskrit, &[]),
        ],
        Script::Dives_Akuru => &[(Language::MaldivianDhivehi, &[])],
        Script::Dogra => &[(Language::Dogri, &[])],
        // Script::Duployan => &[Language::Shorthand systems for English, Language::French],
        Script::Egyptian_Hieroglyphs => &[(Language::EgyptianAncient, &[])],
        Script::Elbasan => &[(Language::AlbanianHistorical, &[])],
        Script::Elymaic => &[(Language::Elymaic, &[])],
        Script::Ethiopic => &[
            (Language::Amharic, &[]),
            (Language::Geez, &[]),
            (Language::Oromo, &[]),
            (Language::Tigrinya, &[]),
        ],
        Script::Garay => &[(Language::Wolof, &[])],
        Script::Georgian => &[(Language::Georgian, &[])],
        Script::Glagolitic => &[(Language::OldChurchSlavonic, &[])],
        Script::Gothic => &[(Language::Gothic, &[])],
        Script::Grantha => &[(Language::Sanskrit, &[]), (Language::Tamil, &[])],
        Script::Greek => &[(Language::Greek, &[])],
        Script::Gujarati => &[(Language::Gujarati, &[])],
        Script::Gunjala_Gondi => &[(Language::Gondi, &[])],
        Script::Gurmukhi => &[(Language::Punjabi, &[])],
        Script::Gurung_Khema => &[(Language::Gurung, &[])],
        Script::Han => &[
            (Language::Chinese, &[]),
            (Language::JapaneseKanji, &[]), //???????????
            (Language::KoreanHanja, &[]),
        ],
        Script::Hangul => &[(Language::Korean, &[])],
        Script::Hanifi_Rohingya => &[(Language::Rohingya, &[])],
        Script::Hanunoo => &[(Language::Hanunoo, &[])],
        Script::Hatran => &[(Language::HatranAramaic, &[])],
        Script::Hebrew => &[(Language::Hebrew, &[]), (Language::Yiddish, &[])],
        Script::Hiragana => &[(Language::Japanese, &[])],
        Script::Imperial_Aramaic => &[(Language::Aramaic, &[])],
        Script::Inscriptional_Pahlavi => &[(Language::MiddlePersian, &[])],
        Script::Inscriptional_Parthian => &[(Language::Parthian, &[])],
        Script::Javanese => &[(Language::Javanese, &[])],
        Script::Kaithi => &[
            (Language::Bhojpuri, &[]),
            (Language::Magahi, &[]),
            (Language::Maithili, &[]),
        ],
        Script::Kannada => &[(Language::Kannada, &[])],
        Script::Katakana => &[(Language::Japanese, &[])],
        Script::Kawi => &[(Language::OldJavanese, &[]), (Language::Sanskrit, &[])],
        Script::Kayah_Li => &[(Language::KayahLi, &[])],
        Script::Kharoshthi => &[(Language::Gandhari, &[])],
        Script::Khitan_Small_Script => &[(Language::Khitan, &[])],
        Script::Khmer => &[(Language::Khmer, &[])],
        Script::Khojki => &[(Language::Sindhi, &[]), (Language::Khoja, &[])],
        Script::Khudawadi => &[(Language::Sindhi, &[])],
        Script::Kirat_Rai => &[(Language::KiratRai, &[])],
        Script::Lao => &[(Language::Lao, &[])],
        Script::Latin => &[
            (Language::Afrikaans, &[]),
            (Language::Albanian, &[]),
            (Language::Azerbaijani, &[]),
            (Language::Basque, &[]),
            (Language::Bokmal, &[]),
            (Language::Bosnian, &[]),
            (Language::Catalan, &[]),
            (Language::Croatian, &[]),
            (Language::Czech, &[]),
            (Language::Danish, &[]),
            (Language::Dutch, &[]),
            (Language::English, &[]),
            (Language::Esperanto, &[]),
            (Language::Estonian, &[]),
            (Language::Finnish, &[]),
            (Language::French, &[]),
            (Language::Ganda, &[]),
            (Language::German, &[]),
            (Language::Hungarian, &[]),
            (Language::Icelandic, &[]),
            (Language::Indonesian, &[]),
            (Language::Irish, &[]),
            (Language::Italian, &[]),
            (Language::Latin, &[]),
            (Language::Latvian, &[]),
            (Language::Lithuanian, &[]),
            (Language::Malay, &[]),
            (Language::Maori, &[]),
            (Language::Nynorsk, &[]),
            (Language::Polish, &[]),
            (Language::Portuguese, &[]),
            (Language::Romanian, &[]),
            (Language::Shona, &[]),
            (Language::Slovak, &[]),
            (Language::Slovene, &[]),
            (Language::Somali, &[]),
            (Language::Sotho, &[]),
            (Language::Spanish, &[]),
            (Language::Swahili, &[]),
            (Language::Swedish, &[]),
            (Language::Tagalog, &[]),
            (Language::Tsonga, &[]),
            (Language::Tswana, &[]),
            (Language::Turkish, &[]),
            (Language::Vietnamese, &[]),
            (Language::Welsh, &[]),
            (Language::Xhosa, &[]),
            (Language::Yoruba, &[]),
            (Language::Zulu, &[]),
        ],
        Script::Lepcha => &[(Language::Lepcha, &[])],
        Script::Limbu => &[(Language::Limbu, &[])],
        // Script::Linear_A => &[(Language::Unknown (Minoan civilization),&[])],
        Script::Linear_B => &[(Language::MycenaeanGreek, &[])],
        Script::Lisu => &[(Language::Lisu, &[])],
        Script::Lycian => &[(Language::Lycian, &[])],
        Script::Lydian => &[(Language::Lydian, &[])],
        Script::Mahajani => &[
            (Language::Hindi, &[]),
            (Language::Marwari, &[]),
            (Language::Punjabi, &[]),
        ],
        Script::Makasar => &[(Language::Makasar, &[])],
        Script::Malayalam => &[(Language::Malayalam, &[])],
        Script::Mandaic => &[(Language::Mandaic, &[]), (Language::Aramaic, &[])],
        Script::Manichaean => &[(Language::MiddlePersian, &[]), (Language::Sogdian, &[])],
        Script::Marchen => &[(Language::MarchenBuddhist, &[])],
        Script::Masaram_Gondi => &[(Language::Gondi, &[])],
        Script::Medefaidrin => &[(Language::Medefaidrin, &[])],
        Script::Meetei_Mayek => &[(Language::ManipuriMeetei, &[])],
        Script::Mende_Kikakui => &[(Language::Mende, &[])],
        Script::Meroitic_Cursive => &[(Language::Meroitic, &[])],
        Script::Meroitic_Hieroglyphs => &[(Language::Meroitic, &[])],
        Script::Miao => &[(Language::HmongMiao, &[])],
        Script::Modi => &[(Language::Marathi, &[])],
        Script::Mongolian => &[(Language::Mongolian, &[])],
        Script::Mro => &[(Language::Mro, &[])],
        Script::Multani => &[(Language::Saraiki, &[])],
        Script::Myanmar => &[(Language::Burmese, &[]), (Language::Shan, &[])],
        Script::Nabataean => &[(Language::NabataeanAramaic, &[])],
        Script::Nag_Mundari => &[(Language::Mundari, &[])],
        Script::Nandinagari => &[(Language::Sanskrit, &[])],
        Script::New_Tai_Lue => &[(Language::TaiLue, &[])],
        Script::Newa => &[(Language::Newari, &[])],
        Script::Nko => &[(Language::NKoMandé, &[])],
        Script::Nushu => &[(Language::NushuChina, &[])],
        Script::Nyiakeng_Puachue_Hmong => &[(Language::Hmong, &[])],
        Script::Ogham => &[(Language::OldIrish, &[])],
        Script::Ol_Chiki => &[(Language::Santali, &[])],
        Script::Ol_Onal => &[(Language::Ho, &[])],
        Script::Old_Hungarian => &[(Language::HungarianOld, &[])],
        Script::Old_Italic => &[
            (Language::Etruscan, &[]),
            (Language::Oscan, &[]),
            (Language::Umbrian, &[]),
        ],
        Script::Old_North_Arabian => &[(Language::OldNorthArabian, &[])],
        Script::Old_Permic => &[(Language::Komi, &[])],
        Script::Old_Persian => &[(Language::OldPersian, &[])],
        Script::Old_Sogdian => &[(Language::Sogdian, &[])],
        Script::Old_South_Arabian => &[(Language::OldSouthArabian, &[])],
        Script::Old_Turkic => &[(Language::OldTurkic, &[])],
        Script::Old_Uyghur => &[(Language::OldUyghur, &[])],
        Script::Oriya => &[(Language::OriyaOdia, &[])],
        Script::Osage => &[(Language::Osage, &[])],
        Script::Osmanya => &[(Language::Somali, &[])],
        Script::Pahawh_Hmong => &[(Language::Hmong, &[])],
        Script::Palmyrene => &[(Language::PalmyreneAramaic, &[])],
        Script::Pau_Cin_Hau => &[(Language::PauCinHauChin, &[])],
        Script::Phags_Pa => &[(Language::Mongolian, &[]), (Language::Tibetan, &[])],
        Script::Phoenician => &[(Language::Phoenician, &[])],
        Script::Psalter_Pahlavi => &[(Language::MiddlePersian, &[])],
        Script::Rejang => &[(Language::Rejang, &[])],
        Script::Runic => &[(Language::OldNorse, &[]), (Language::OldEnglish, &[])],
        Script::Samaritan => &[(Language::SamaritanHebrew, &[])],
        Script::Saurashtra => &[(Language::Saurashtra, &[])],
        Script::Sharada => &[(Language::Sanskrit, &[]), (Language::Kashmiri, &[])],
        Script::Shavian => &[(Language::EnglishPhonetic, &[])],
        Script::Siddham => &[(Language::Sanskrit, &[])],
        Script::SignWriting => &[(Language::Signlanguages, &[])],
        Script::Sinhala => &[(Language::Sinhala, &[])],
        Script::Sogdian => &[(Language::Sogdian, &[])],
        Script::Sora_Sompeng => &[(Language::Sora, &[])],
        Script::Soyombo => &[
            (Language::Mongolian, &[]),
            (Language::Sanskrit, &[]),
            (Language::Tibetan, &[]),
        ],
        Script::Sundanese => &[(Language::Sundanese, &[])],
        Script::Sunuwar => &[(Language::Sunuwar, &[])],
        Script::Syloti_Nagri => &[(Language::Sylheti, &[])],
        Script::Syriac => &[(Language::Syriac, &[]), (Language::Aramaic, &[])],
        Script::Tagalog => &[(Language::Tagalog, &[])],
        Script::Tagbanwa => &[(Language::Tagbanwa, &[])],
        Script::Tai_Le => &[(Language::TaiLe, &[])],
        Script::Tai_Tham => &[
            (Language::Lao, &[]),
            (Language::NorthernThai, &[]),
            (Language::TaiLue, &[]),
        ],
        Script::Tai_Viet => &[(Language::TaiDam, &[]), (Language::TaiDón, &[])],
        Script::Takri => &[(Language::Dogri, &[]), (Language::Kashmiri, &[])],
        Script::Tamil => &[(Language::Tamil, &[])],
        Script::Tangsa => &[(Language::Tangsa, &[])],
        Script::Tangut => &[(Language::Tangut, &[])],
        Script::Telugu => &[(Language::Telugu, &[])],
        Script::Thaana => &[(Language::MaldivianDhivehi, &[])],
        Script::Thai => &[(Language::Thai, &[])],
        Script::Tibetan => &[(Language::Tibetan, &[])],
        Script::Tifinagh => &[(Language::Berber, &[])],
        Script::Tirhuta => &[(Language::Maithili, &[])],
        Script::Todhri => &[(Language::AlbanianHistorical, &[])],
        Script::Toto => &[(Language::Toto, &[])],
        Script::Tulu_Tigalari => &[(Language::Sanskrit, &[]), (Language::Tulu, &[])],
        Script::Ugaritic => &[(Language::Ugaritic, &[])],
        Script::Vai => &[(Language::Vai, &[])],
        Script::Vithkuqi => &[(Language::Albanian, &[])],
        Script::Wancho => &[(Language::Wancho, &[])],
        Script::Warang_Citi => &[(Language::Ho, &[])],
        Script::Yezidi => &[(Language::KurdishYazidi, &[])],
        Script::Yi => &[(Language::Yi, &[])],
        Script::Zanabazar_Square => &[
            (Language::Mongolian, &[]),
            (Language::Sanskrit, &[]),
            (Language::Tibetan, &[]),
        ],
    }
}
