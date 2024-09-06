use crate::ExtraCheck;
use ::std::cmp::Ordering;
use strum_macros::EnumIter;

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

use super::ucd::BY_NAME;

const fn char_ranges_count() -> usize {
    let mut i = 0;
    let mut cnt = 0;
    while i < BY_NAME.len() {
        cnt += BY_NAME[i].1.len();
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
    while i < BY_NAME.len() {
        let (script, ranges) = BY_NAME[i];
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

impl Script {
    pub(crate) fn find(ch: char) -> Option<Self> {
        CHAR_RANGES_SORTED
            .binary_search_by(|ra| compare(ra, ch))
            .ok()
            .map(|i| unsafe { CHAR_RANGES_SORTED.get(i).unwrap() }.script) // todo: unchecked
    }
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
