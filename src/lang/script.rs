use ::std::cmp::Ordering;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, EnumIter, fixed_map::Key)]
pub enum Script {
    Adlam,
    Ahom,
    AnatolianHieroglyphs,
    Arabic,
    Armenian,
    Avestan,
    Balinese,
    Bamum,
    BassaVah,
    Batak,
    Bengali,
    Bhaiksuki,
    Bopomofo,
    Brahmi,
    Braille,
    Buginese,
    Buhid,
    CanadianAboriginal,
    Carian,
    CaucasianAlbanian,
    Chakma,
    Cham,
    Cherokee,
    Chorasmian,
    Common,
    Coptic,
    Cuneiform,
    Cypriot,
    CyproMinoan,
    Cyrillic,
    Deseret,
    Devanagari,
    DivesAkuru,
    Dogra,
    Duployan,
    EgyptianHieroglyphs,
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
    GunjalaGondi,
    Gurmukhi,
    GurungKhema,
    Han,
    Hangul,
    HanifiRohingya,
    Hanunoo,
    Hatran,
    Hebrew,
    Hiragana,
    ImperialAramaic,
    Inherited,
    InscriptionalPahlavi,
    InscriptionalParthian,
    Javanese,
    Kaithi,
    Kannada,
    Katakana,
    Kawi,
    KayahLi,
    Kharoshthi,
    KhitanSmallScript,
    Khmer,
    Khojki,
    Khudawadi,
    KiratRai,
    Lao,
    Latin,
    Lepcha,
    Limbu,
    LinearA,
    LinearB,
    Lisu,
    Lycian,
    Lydian,
    Mahajani,
    Makasar,
    Malayalam,
    Mandaic,
    Manichaean,
    Marchen,
    MasaramGondi,
    Medefaidrin,
    MeeteiMayek,
    MendeKikakui,
    MeroiticCursive,
    MeroiticHieroglyphs,
    Miao,
    Modi,
    Mongolian,
    Mro,
    Multani,
    Myanmar,
    Nabataean,
    NagMundari,
    Nandinagari,
    NewTaiLue,
    Newa,
    Nko,
    Nushu,
    NyiakengPuachueHmong,
    Ogham,
    OlChiki,
    OlOnal,
    OldHungarian,
    OldItalic,
    OldNorthArabian,
    OldPermic,
    OldPersian,
    OldSogdian,
    OldSouthArabian,
    OldTurkic,
    OldUyghur,
    Oriya,
    Osage,
    Osmanya,
    PahawhHmong,
    Palmyrene,
    PauCinHau,
    PhagsPa,
    Phoenician,
    PsalterPahlavi,
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
    SoraSompeng,
    Soyombo,
    Sundanese,
    Sunuwar,
    SylotiNagri,
    Syriac,
    Tagalog,
    Tagbanwa,
    TaiLe,
    TaiTham,
    TaiViet,
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
    TuluTigalari,
    Ugaritic,
    Vai,
    Vithkuqi,
    Wancho,
    WarangCiti,
    Yezidi,
    Yi,
    ZanabazarSquare,
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

// impl ExtraCheck for Script {}

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
