use std::cmp::Ordering;

use strum_macros::EnumIter;

use crate::{ExtraCheck, Language};

#[allow(non_camel_case_types)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, EnumIter)]
pub enum Alphabet {
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

impl ExtraCheck for Alphabet {}

const fn char_ranges_count() -> usize {
    let mut i = 0;
    let mut cnt = 0;
    while i < crate::script::BY_ALPHABET.len() {
        cnt += crate::script::BY_ALPHABET[i].1.len();
        i += 1;
    }

    cnt
}
const LEN: usize = char_ranges_count();

#[derive(Clone, Copy, Debug)]
struct RangeAlphabet {
    range_start: char,
    range_end: char,
    alphabet: Alphabet,
}
const RANGE_ALPHABET_DEFAULT: RangeAlphabet = RangeAlphabet {
    range_start: char::MAX,
    range_end: char::MAX,
    alphabet: Alphabet::Latin,
};

/* #[const_trait]
trait ConstDefault {
    fn default<const RUNTIME: bool>() -> Self;
}
impl const ConstDefault for RangeAlphabet {
    fn default() -> Self {
        RangeAlphabet {
            range_start: char::MAX,
            range_end: char::MAX,
            alphabet: Alphabet::Latin,
        }
    }
} */

const fn char_ranges_array_sorted() -> [RangeAlphabet; LEN] {
    let mut res: [RangeAlphabet; LEN] = [RANGE_ALPHABET_DEFAULT; LEN];

    // foreach BY_ALPHABET
    let mut i = 0;
    while i < crate::script::BY_ALPHABET.len() {
        let (alphabet, ranges) = crate::script::BY_ALPHABET[i];
        // foreach charset in BY_ALPHABET
        let mut j = 0;
        while j < ranges.len() {
            let range = ranges[j];
            // looking for insertion
            let mut ins = 0;
            while ins < LEN {
                let mut prev = res[ins];
                if range.0 < prev.range_start {
                    res[ins] = RangeAlphabet {
                        range_start: range.0,
                        range_end: range.1,
                        alphabet,
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
const CHAR_RANGES_SORTED: [RangeAlphabet; LEN] = char_ranges_array_sorted();

/* #[test]
fn testing() {
    panic!("{:?}", CHAR_RANGES_SORTED);
} */

#[inline(always)]
fn compare(ra: &RangeAlphabet, ch: char) -> Ordering {
    if ch < ra.range_start {
        Ordering::Greater
    } else if ch > ra.range_end {
        Ordering::Less
    } else {
        Ordering::Equal
    }
}

pub(crate) fn find_alphabet(ch: char) -> Option<Alphabet> {
    CHAR_RANGES_SORTED
        .binary_search_by(|ra| compare(ra, ch))
        .ok()
        .map(|i| unsafe { CHAR_RANGES_SORTED.get(i).unwrap() }.alphabet) // todo: unchecked
}

/* pub(crate) fn find_alphabet(ch: char) -> Option<Alphabet> {
    crate::script::BY_ALPHABET
        .iter()
        .find(|(_, chars)| chars.iter().any(|cs| ch > cs.0 || ch <= cs.1))
        .map(|v| v.0)
}

pub(crate) fn alphabet_same(alphabet: Alphabet, ch: char) -> bool {
    crate::script::BY_ALPHABET
        .iter()
        .find(|(a, _)| a == &alphabet)
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

const ALPHABET_LANGS: &[(Alphabet, &[(Language, &[char])])] = &[(
    Alphabet::Cyrillic,
    &[
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
    ],
)];

fn alphabet_langs_chars(a: Alphabet) -> &'static [(Language, &'static [char])] {
    match a {
        Alphabet::Adlam => &[(Language::Fulani, &[]), (Language::Pular, &[])],
        Alphabet::Ahom => &[(Language::Ahom, &[])],
        Alphabet::Anatolian_Hieroglyphs => &[(Language::Luwian, &[])],
        Alphabet::Arabic => &[
            (Language::Arabic, &[]),
            (Language::Persian, &[]),
            (Language::Urdu, &[]),
            (Language::Pashto, &[]),
            (Language::Kurdish, &[]),
            (Language::Sindhi, &[]),
            (Language::Uyghur, &[]),
        ],
        Alphabet::Armenian => &[(Language::Armenian, &[])],
        Alphabet::Avestan => &[(Language::Avestan, &[])],
        Alphabet::Balinese => &[(Language::Balinese, &[])],
        Alphabet::Bamum => &[(Language::Bamum, &[])],
        Alphabet::Bassa_Vah => &[(Language::Bassa, &[])],
        // Alphabet::Batak => &[Language::Batak Toba, Language::Batak Karo, Language::Batak Mandailing, Language::Batak Pakpak, Language::Batak Simalungun, Language::Batak Angkola],
        Alphabet::Bengali => &[
            (Language::Bengali, &[]),
            (Language::Assamese, &[]),
            (Language::BishnupriyaManipuri, &[]),
        ],
        Alphabet::Bhaiksuki => &[(Language::Bhaiksuki, &[])],
        Alphabet::Bopomofo => &[(Language::MandarinChinese, &[])],
        Alphabet::Brahmi => &[(Language::Sanskrit, &[]), (Language::Prakrit, &[])],
        // Alphabet::Braille => &[(Language::Any language adapted to Braille,&[])],
        Alphabet::Buginese => &[(Language::Buginese, &[]), (Language::Makassarese, &[])],
        Alphabet::Buhid => &[(Language::Buhid, &[])],
        // Alphabet::Canadian_Aboriginal => &[(Language::Cree,&[]),(Language::Inuktitut,&[]),(Language::Ojibwe,&[])],
        Alphabet::Carian => &[(Language::Carian, &[])],
        Alphabet::Caucasian_Albanian => &[(Language::CaucasianAlbanian, &[])],
        Alphabet::Chakma => &[(Language::Chakma, &[])],
        Alphabet::Cham => &[(Language::Cham, &[])],
        Alphabet::Cherokee => &[(Language::Cherokee, &[])],
        Alphabet::Chorasmian => &[(Language::Chorasmian, &[])],
        Alphabet::Coptic => &[(Language::Coptic, &[])],
        Alphabet::Cuneiform => &[
            (Language::Sumerian, &[]),
            (Language::Akkadian, &[]),
            (Language::Hittite, &[]),
        ],
        Alphabet::Cypriot => &[(Language::AncientGreek, &[])],
        // Alphabet::Cypro_Minoan => &[(Language::Unknown (used in ancient Cyprus),&[])],
        Alphabet::Cyrillic => &[
            (Language::Russian, &[]),
            (Language::Bulgarian, &[]),
            (Language::Serbian, &[]),
            (Language::Ukrainian, &[]),
            (Language::Belarusian, &[]),
            (Language::Macedonian, &[]),
        ],
        Alphabet::Deseret => &[(Language::EnglishMormon, &[])],
        Alphabet::Devanagari => &[
            (Language::Hindi, &[]),
            (Language::Marathi, &[]),
            (Language::Nepali, &[]),
            (Language::Sanskrit, &[]),
        ],
        Alphabet::Dives_Akuru => &[(Language::MaldivianDhivehi, &[])],
        Alphabet::Dogra => &[(Language::Dogri, &[])],
        // Alphabet::Duployan => &[Language::Shorthand systems for English, Language::French],
        Alphabet::Egyptian_Hieroglyphs => &[(Language::EgyptianAncient, &[])],
        Alphabet::Elbasan => &[(Language::AlbanianHistorical, &[])],
        Alphabet::Elymaic => &[(Language::Elymaic, &[])],
        Alphabet::Ethiopic => &[
            (Language::Amharic, &[]),
            (Language::Tigrinya, &[]),
            (Language::Geez, &[]),
            (Language::Oromo, &[]),
        ],
        Alphabet::Garay => &[(Language::Wolof, &[])],
        Alphabet::Georgian => &[(Language::Georgian, &[])],
        Alphabet::Glagolitic => &[(Language::OldChurchSlavonic, &[])],
        Alphabet::Gothic => &[(Language::Gothic, &[])],
        Alphabet::Grantha => &[(Language::Sanskrit, &[]), (Language::Tamil, &[])],
        Alphabet::Greek => &[(Language::Greek, &[])],
        Alphabet::Gujarati => &[(Language::Gujarati, &[])],
        Alphabet::Gunjala_Gondi => &[(Language::Gondi, &[])],
        Alphabet::Gurmukhi => &[(Language::Punjabi, &[])],
        Alphabet::Gurung_Khema => &[(Language::Gurung, &[])],
        Alphabet::Han => &[
            (Language::Chinese, &[]),
            (Language::JapaneseKanji, &[]),
            (Language::KoreanHanja, &[]),
        ],
        Alphabet::Hangul => &[(Language::Korean, &[])],
        Alphabet::Hanifi_Rohingya => &[(Language::Rohingya, &[])],
        Alphabet::Hanunoo => &[(Language::Hanunoo, &[])],
        Alphabet::Hatran => &[(Language::HatranAramaic, &[])],
        Alphabet::Hebrew => &[(Language::Hebrew, &[]), (Language::Yiddish, &[])],
        Alphabet::Hiragana => &[(Language::Japanese, &[])],
        Alphabet::Imperial_Aramaic => &[(Language::Aramaic, &[])],
        Alphabet::Inscriptional_Pahlavi => &[(Language::MiddlePersian, &[])],
        Alphabet::Inscriptional_Parthian => &[(Language::Parthian, &[])],
        Alphabet::Javanese => &[(Language::Javanese, &[])],
        Alphabet::Kaithi => &[
            (Language::Bhojpuri, &[]),
            (Language::Magahi, &[]),
            (Language::Maithili, &[]),
        ],
        Alphabet::Kannada => &[(Language::Kannada, &[])],
        Alphabet::Katakana => &[(Language::Japanese, &[])],
        Alphabet::Kawi => &[(Language::OldJavanese, &[]), (Language::Sanskrit, &[])],
        Alphabet::Kayah_Li => &[(Language::KayahLi, &[])],
        Alphabet::Kharoshthi => &[(Language::Gandhari, &[])],
        Alphabet::Khitan_Small_Script => &[(Language::Khitan, &[])],
        Alphabet::Khmer => &[(Language::Khmer, &[])],
        Alphabet::Khojki => &[(Language::Sindhi, &[]), (Language::Khoja, &[])],
        Alphabet::Khudawadi => &[(Language::Sindhi, &[])],
        Alphabet::Kirat_Rai => &[(Language::KiratRai, &[])],
        Alphabet::Lao => &[(Language::Lao, &[])],
        Alphabet::Latin => &[
            (Language::English, &[]),
            (Language::Spanish, &[]),
            (Language::French, &[]),
            (Language::German, &[]),
            (Language::Italian, &[]),
        ],
        Alphabet::Lepcha => &[(Language::Lepcha, &[])],
        Alphabet::Limbu => &[(Language::Limbu, &[])],
        // Alphabet::Linear_A => &[(Language::Unknown (Minoan civilization),&[])],
        Alphabet::Linear_B => &[(Language::MycenaeanGreek, &[])],
        Alphabet::Lisu => &[(Language::Lisu, &[])],
        Alphabet::Lycian => &[(Language::Lycian, &[])],
        Alphabet::Lydian => &[(Language::Lydian, &[])],
        Alphabet::Mahajani => &[
            (Language::Hindi, &[]),
            (Language::Punjabi, &[]),
            (Language::Marwari, &[]),
        ],
        Alphabet::Makasar => &[(Language::Makasar, &[])],
        Alphabet::Malayalam => &[(Language::Malayalam, &[])],
        Alphabet::Mandaic => &[(Language::Mandaic, &[]), (Language::Aramaic, &[])],
        Alphabet::Manichaean => &[(Language::MiddlePersian, &[]), (Language::Sogdian, &[])],
        Alphabet::Marchen => &[(Language::MarchenBuddhist, &[])],
        Alphabet::Masaram_Gondi => &[(Language::Gondi, &[])],
        Alphabet::Medefaidrin => &[(Language::Medefaidrin, &[])],
        Alphabet::Meetei_Mayek => &[(Language::ManipuriMeetei, &[])],
        Alphabet::Mende_Kikakui => &[(Language::Mende, &[])],
        Alphabet::Meroitic_Cursive => &[(Language::Meroitic, &[])],
        Alphabet::Meroitic_Hieroglyphs => &[(Language::Meroitic, &[])],
        Alphabet::Miao => &[(Language::HmongMiao, &[])],
        Alphabet::Modi => &[(Language::Marathi, &[])],
        Alphabet::Mongolian => &[(Language::Mongolian, &[])],
        Alphabet::Mro => &[(Language::Mro, &[])],
        Alphabet::Multani => &[(Language::Saraiki, &[])],
        Alphabet::Myanmar => &[(Language::Burmese, &[]), (Language::Shan, &[])],
        Alphabet::Nabataean => &[(Language::NabataeanAramaic, &[])],
        Alphabet::Nag_Mundari => &[(Language::Mundari, &[])],
        Alphabet::Nandinagari => &[(Language::Sanskrit, &[])],
        Alphabet::New_Tai_Lue => &[(Language::TaiLue, &[])],
        Alphabet::Newa => &[(Language::Newari, &[])],
        Alphabet::Nko => &[(Language::NKoMandé, &[])],
        Alphabet::Nushu => &[(Language::NushuChina, &[])],
        Alphabet::Nyiakeng_Puachue_Hmong => &[(Language::Hmong, &[])],
        Alphabet::Ogham => &[(Language::OldIrish, &[])],
        Alphabet::Ol_Chiki => &[(Language::Santali, &[])],
        Alphabet::Ol_Onal => &[(Language::Ho, &[])],
        Alphabet::Old_Hungarian => &[(Language::HungarianOld, &[])],
        Alphabet::Old_Italic => &[
            (Language::Etruscan, &[]),
            (Language::Oscan, &[]),
            (Language::Umbrian, &[]),
        ],
        Alphabet::Old_North_Arabian => &[(Language::OldNorthArabian, &[])],
        Alphabet::Old_Permic => &[(Language::Komi, &[])],
        Alphabet::Old_Persian => &[(Language::OldPersian, &[])],
        Alphabet::Old_Sogdian => &[(Language::Sogdian, &[])],
        Alphabet::Old_South_Arabian => &[(Language::OldSouthArabian, &[])],
        Alphabet::Old_Turkic => &[(Language::OldTurkic, &[])],
        Alphabet::Old_Uyghur => &[(Language::OldUyghur, &[])],
        Alphabet::Oriya => &[(Language::OriyaOdia, &[])],
        Alphabet::Osage => &[(Language::Osage, &[])],
        Alphabet::Osmanya => &[(Language::Somali, &[])],
        Alphabet::Pahawh_Hmong => &[(Language::Hmong, &[])],
        Alphabet::Palmyrene => &[(Language::PalmyreneAramaic, &[])],
        Alphabet::Pau_Cin_Hau => &[(Language::PauCinHauChin, &[])],
        Alphabet::Phags_Pa => &[(Language::Mongolian, &[]), (Language::Tibetan, &[])],
        Alphabet::Phoenician => &[(Language::Phoenician, &[])],
        Alphabet::Psalter_Pahlavi => &[(Language::MiddlePersian, &[])],
        Alphabet::Rejang => &[(Language::Rejang, &[])],
        Alphabet::Runic => &[(Language::OldNorse, &[]), (Language::OldEnglish, &[])],
        Alphabet::Samaritan => &[(Language::SamaritanHebrew, &[])],
        Alphabet::Saurashtra => &[(Language::Saurashtra, &[])],
        Alphabet::Sharada => &[(Language::Sanskrit, &[]), (Language::Kashmiri, &[])],
        Alphabet::Shavian => &[(Language::EnglishPhonetic, &[])],
        Alphabet::Siddham => &[(Language::Sanskrit, &[])],
        Alphabet::SignWriting => &[(Language::Signlanguages, &[])],
        Alphabet::Sinhala => &[(Language::Sinhala, &[])],
        Alphabet::Sogdian => &[(Language::Sogdian, &[])],
        Alphabet::Sora_Sompeng => &[(Language::Sora, &[])],
        Alphabet::Soyombo => &[
            (Language::Mongolian, &[]),
            (Language::Tibetan, &[]),
            (Language::Sanskrit, &[]),
        ],
        Alphabet::Sundanese => &[(Language::Sundanese, &[])],
        Alphabet::Sunuwar => &[(Language::Sunuwar, &[])],
        Alphabet::Syloti_Nagri => &[(Language::Sylheti, &[])],
        Alphabet::Syriac => &[(Language::Syriac, &[]), (Language::Aramaic, &[])],
        Alphabet::Tagalog => &[(Language::Tagalog, &[])],
        Alphabet::Tagbanwa => &[(Language::Tagbanwa, &[])],
        Alphabet::Tai_Le => &[(Language::TaiLe, &[])],
        Alphabet::Tai_Tham => &[
            (Language::NorthernThai, &[]),
            (Language::TaiLue, &[]),
            (Language::Lao, &[]),
        ],
        Alphabet::Tai_Viet => &[(Language::TaiDam, &[]), (Language::TaiDón, &[])],
        Alphabet::Takri => &[(Language::Dogri, &[]), (Language::Kashmiri, &[])],
        Alphabet::Tamil => &[(Language::Tamil, &[])],
        Alphabet::Tangsa => &[(Language::Tangsa, &[])],
        Alphabet::Tangut => &[(Language::Tangut, &[])],
        Alphabet::Telugu => &[(Language::Telugu, &[])],
        Alphabet::Thaana => &[(Language::MaldivianDhivehi, &[])],
        Alphabet::Thai => &[(Language::Thai, &[])],
        Alphabet::Tibetan => &[(Language::Tibetan, &[])],
        Alphabet::Tifinagh => &[(Language::Berber, &[])],
        Alphabet::Tirhuta => &[(Language::Maithili, &[])],
        Alphabet::Todhri => &[(Language::AlbanianHistorical, &[])],
        Alphabet::Toto => &[(Language::Toto, &[])],
        Alphabet::Tulu_Tigalari => &[(Language::Tulu, &[]), (Language::Sanskrit, &[])],
        Alphabet::Ugaritic => &[(Language::Ugaritic, &[])],
        Alphabet::Vai => &[(Language::Vai, &[])],
        Alphabet::Vithkuqi => &[(Language::Albanian, &[])],
        Alphabet::Wancho => &[(Language::Wancho, &[])],
        Alphabet::Warang_Citi => &[(Language::Ho, &[])],
        Alphabet::Yezidi => &[(Language::KurdishYazidi, &[])],
        Alphabet::Yi => &[(Language::Yi, &[])],
        Alphabet::Zanabazar_Square => &[
            (Language::Mongolian, &[]),
            (Language::Tibetan, &[]),
            (Language::Sanskrit, &[]),
        ],
    }
}
