use super::{Language, Script};
use alphabet_match_macro::alphabet_match;

pub enum Alphabet {
    AfrikaansLatin,
    Ahom,
    AkkadianCuneiform,
    AlbanianHistoricalElbasan,
    AlbanianHistoricalTodhri,
    AlbanianLatin,
    AlbanianVithkuqi,
    AmharicEthiopic,
    AncientGreekCypriot,
    AngkolaBatak,
    Arabic,
    AramaicHatran,
    AramaicImperial,
    AramaicMandaic,
    AramaicNabataean,
    AramaicPalmyrene,
    AramaicSyriac,
    Armenian,
    AssameseBengali,
    Avestan,
    AzerbaijaniLatin,
    Balinese,
    Bamum,
    BantawaKiratRai,
    BasqueLatin,
    BassaVah,
    BelarusianCyrillic,
    Bengali,
    BerberTifinagh,
    Bhaiksuki,
    BhojpuriKaithi,
    BishnupriyaManipuriBengali,
    BokmalLatin,
    BosnianLatin,
    Braille, // Any language adapted to Braille
    BuddhistMarchen,
    Buginese,
    Buhid,
    BulgarianCyrillic,
    BurmeseMyanmar,
    Carian,
    CatalanLatin,
    CaucasianAlbanian,
    Chakma,
    Cham,
    Cherokee,
    ChineseMandarinBopomofo,
    ChineseNushu,
    ChineseSimplified,
    ChineseTraditional,
    ZoPauCinHau,
    Chorasmian,
    Coptic,
    CreeCanadianAboriginal,
    CroatianLatin,
    CyproMinoan, // used in ancient Cyprus
    CzechLatin,
    DanishLatin,
    DogriDogra,
    DogriTakri,
    DutchLatin,
    EgyptianHieroglyphs,
    Elymaic,
    EnglishDuployan, //Shorthand systems for English
    EnglishLatin,
    EnglishMormonDeseret,
    EnglishPhoneticShavian,
    EsperantoLatin,
    EstonianLatin,
    EtruscanOldItalic,
    FinnishLatin,
    FrenchDuployan,
    FrenchLatin,
    FulaniAdlam,
    GandaLatin,
    GandhariKharoshthi,
    GeezEthiopic,
    Georgian,
    GermanLatin,
    GondiGunjala,
    GondiMasaram,
    Gothic,
    Greek,
    Gujarati,
    GurungKhema,
    Hanunoo,
    Hebrew,
    HebrewSamaritan,
    HindiDevanagari,
    HindiMahajani,
    HittiteCuneiform,
    HmongNyiakengPuachue,
    HmongPahawh,
    HoOlOnal,
    HoWarangCiti,
    HungarianLatin,
    IcelandicLatin,
    IndonesianLatin,
    InuktitutCanadianAboriginal,
    IrishLatin,
    ItalianLatin,
    JapaneseHiragana,
    JapaneseKanji,
    JapaneseKatakana,
    Javanese,
    Kannada,
    KannadaTuluTigalari,
    KaroBatak,
    KashmiriSharada,
    KashmiriTakri,
    KazakhCyrillic,
    KhitanSmallScript,
    Khmer,
    KhojaKhojki,
    KoreanHangul,
    KoreanHanja,
    KurdishArabic,
    KurdishYezidi,
    LaoTaiTham,
    Latin,
    LatvianLatin,
    Lepcha,
    KayahLi,
    Limbu,
    Lisu,
    LithuanianLatin,
    LuwianAnatolianHieroglyphs,
    Lycian,
    Lydian,
    MacedonianCyrillic,
    MagahiKaithi,
    MaithiliKaithi,
    MaithiliTirhuta,
    Makasar,
    MakassareseBuginese,
    Malayalam,
    MalayLatin,
    MaldivianDhivehiDivesAkuru,
    MaldivianDhivehiThaana,
    Mandaic,
    MandailingBatak,
    MandeNKo,
    ManipuriMeeteiMayek,
    MaoriLatin,
    MarathiDevanagari,
    MarathiModi,
    MarwariMahajani,
    Medefaidrin,
    MendeKikakui,
    MeroiticCursive,
    MeroiticHieroglyphs,
    HmongMiao,
    MiddlePersianInscriptionalPahlavi,
    MiddlePersianManichaean,
    MiddlePersianPsalterPahlavi,
    MinoanLinearA,
    Mongolian,
    MongolianCyrillic,
    MongolianPhagsPa,
    MongolianSoyombo,
    MongolianZanabazarSquare,
    Mro,
    MundariNag,
    MycenaeanGreekLinearB,
    NabataeanAramaic,
    NepaliDevanagari,
    NewariNewa,
    NorthernThaiTaiTham,
    NynorskLatin,
    OdiaOriya,
    OjibweCanadianAboriginal,
    OldChurchSlavonicGlagolitic,
    OldEnglish,
    OldHungarian,
    OldIrishOgham,
    OldJavaneseKawi,
    OldKomiPermic,
    OldNorseRunic,
    OldNorthArabian,
    OldPersian,
    OldSogdian,
    OldSouthArabian,
    OldTurkic,
    OldUyghur,
    OromoEthiopic,
    Osage,
    OscanOldItalic,
    PakpakBatak,
    ParthianInscriptional,
    PashtoArabic,
    PersianArabic,
    Phoenician,
    PolishLatin,
    PortugueseLatin,
    PrakritBrahmi,
    PularAdlam,
    PunjabiGurmukhi,
    PunjabiMahajani,
    Rejang,
    RohingyaHanifi,
    RomanianLatin,
    RussianCyrillic,
    SanskritBrahmi,
    SanskritDevanagari,
    SanskritGrantha,
    SanskritKawi,
    SanskritNandinagari,
    SanskritSharada,
    SanskritSiddham,
    SanskritSoyombo,
    SanskritTuluTigalari,
    SanskritZanabazarSquare,
    SantaliOlChiki,
    SaraikiMultani,
    Saurashtra,
    SerbianCyrillic,
    ShanMyanmar,
    ShonaLatin,
    SignlanguageWriting,
    SimalungunBatak,
    SindhiArabic,
    SindhiKhojki,
    SindhiKhudawadi,
    Sinhala,
    SlovakLatin,
    SloveneLatin,
    Sogdian,
    SogdianManichaean,
    SomaliLatin,
    SomaliOsmanya,
    Sora,
    SothoLatin,
    SpanishLatin,
    SumerianCuneiform,
    Sundanese,
    Sunuwar,
    SwahiliLatin,
    SwedishLatin,
    SylhetiSylotiNagri,
    Syriac,
    Tagalog,
    TagalogLatin,
    Tagbanwa,
    TaiDamTaiViet,
    TaiDonTaiViet,
    TaiLe,
    TaiLueNew,
    TaiLueTaiTham,
    Tamil,
    TamilGrantha,
    Tangsa,
    Tangut,
    Telugu,
    Thai,
    Tibetan,
    TibetanPhagsPa,
    TibetanSoyombo,
    TigrinyaEthiopic,
    TobaBatak,
    Toto,
    TsongaLatin,
    TswanaLatin,
    TuluTigalari,
    TurkishLatin,
    Ugaritic,
    UkrainianCyrillic,
    UmbrianOldItalic,
    UrduArabic,
    UyghurArabic,
    Vai,
    VietnameseLatin,
    Wancho,
    WelshLatin,
    WolofGaray,
    XhosaLatin,
    Yi,
    YiddishHebrew,
    YorubaLatin,
    ZuluLatin,
}

impl From<Alphabet> for &[Language] {
    fn from(a: Alphabet) -> Self {
        use Alphabet::*;
        match a {
            AfrikaansLatin => &[Language::Afrikaans],
            Ahom => &[Language::Ahom],
            AkkadianCuneiform => &[Language::Akkadian],
            AlbanianHistoricalElbasan => &[Language::AlbanianHistorical],
            AlbanianHistoricalTodhri => &[Language::AlbanianHistorical],
            AlbanianLatin => &[Language::Albanian],
            AlbanianVithkuqi => &[Language::Albanian],
            AmharicEthiopic => &[Language::Amharic],
            AncientGreekCypriot => &[Language::AncientGreek],
            AngkolaBatak => &[Language::Angkola],
            Arabic => &[Language::Arabic],
            AramaicHatran => &[Language::Aramaic],
            AramaicImperial => &[Language::Aramaic],
            AramaicMandaic => &[Language::Aramaic],
            AramaicNabataean => &[Language::Aramaic],
            AramaicPalmyrene => &[Language::Aramaic],
            AramaicSyriac => &[Language::Aramaic],
            Armenian => &[Language::Armenian],
            AssameseBengali => &[Language::Assamese],
            Avestan => &[Language::Avestan],
            AzerbaijaniLatin => &[Language::Azerbaijani],
            Balinese => &[Language::Balinese],
            Bamum => &[Language::Bamum],
            BantawaKiratRai => &[Language::Bantawa],
            BasqueLatin => &[Language::Basque],
            BassaVah => &[Language::Bassa],
            BelarusianCyrillic => &[Language::Belarusian],
            Bengali => &[Language::Bengali],
            BerberTifinagh => &[Language::Berber],
            Bhaiksuki => &[Language::Bhaiksuki],
            BhojpuriKaithi => &[Language::Bhojpuri],
            BishnupriyaManipuriBengali => &[Language::BishnupriyaManipuri],
            BokmalLatin => &[Language::Bokmal],
            BosnianLatin => &[Language::Bosnian],
            Braille => &[Language::Braille],
            BuddhistMarchen => &[Language::BuddhistMarchen],
            Buginese => &[Language::Buginese],
            Buhid => &[Language::Buhid],
            BulgarianCyrillic => &[Language::Bulgarian],
            BurmeseMyanmar => &[Language::Burmese],
            Carian => &[Language::Carian],
            CatalanLatin => &[Language::Catalan],
            CaucasianAlbanian => &[Language::CaucasianAlbanian],
            Chakma => &[Language::Chakma],
            Cham => &[Language::Cham],
            Cherokee => &[Language::Cherokee],
            ChineseMandarinBopomofo => &[Language::ChineseMandarin],
            ChineseNushu => &[Language::ChineseTuhua],
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
            Chorasmian => &[Language::Chorasmian],
            Coptic => &[Language::Coptic],
            CreeCanadianAboriginal => &[Language::Cree],
            CroatianLatin => &[Language::Croatian],
            CyproMinoan => &[Language::CyproMinoan],
            CzechLatin => &[Language::Czech],
            DanishLatin => &[Language::Danish],
            DogriDogra => &[Language::Dogri],
            DogriTakri => &[Language::Dogri],
            DutchLatin => &[Language::Dutch],
            EgyptianHieroglyphs => &[Language::EgyptianHieroglyphs],
            Elymaic => &[Language::Elymaic],
            EnglishDuployan => &[Language::EnglishDuployan],
            EnglishLatin => &[Language::English],
            EnglishMormonDeseret => &[Language::EnglishMormon],
            EnglishPhoneticShavian => &[Language::EnglishPhonetic],
            EsperantoLatin => &[Language::Esperanto],
            EstonianLatin => &[Language::Estonian],
            EtruscanOldItalic => &[Language::Etruscan],
            FinnishLatin => &[Language::Finnish],
            FrenchDuployan => &[Language::FrenchDuployan],
            FrenchLatin => &[Language::French],
            FulaniAdlam => &[Language::Fulani],
            GandaLatin => &[Language::Ganda],
            GandhariKharoshthi => &[Language::Gandhari],
            GeezEthiopic => &[Language::Geez],
            Georgian => &[Language::Georgian],
            GermanLatin => &[Language::German],
            GondiGunjala => &[Language::Gondi],
            GondiMasaram => &[Language::Gondi],
            Gothic => &[Language::Gothic],
            Greek => &[Language::Greek],
            Gujarati => &[Language::Gujarati],
            GurungKhema => &[Language::Gurung],
            Hanunoo => &[Language::Hanunoo],
            Hebrew => &[Language::Hebrew],
            HebrewSamaritan => &[Language::Hebrew],
            HindiDevanagari => &[Language::Hindi],
            HindiMahajani => &[Language::Hindi],
            HittiteCuneiform => &[Language::Hittite],
            HmongMiao => &[Language::Hmong],
            HmongNyiakengPuachue => &[Language::Hmong],
            HmongPahawh => &[Language::Hmong],
            HoOlOnal => &[Language::Ho],
            HoWarangCiti => &[Language::Ho],
            HungarianLatin => &[Language::Hungarian],
            IcelandicLatin => &[Language::Icelandic],
            IndonesianLatin => &[Language::Indonesian],
            InuktitutCanadianAboriginal => &[Language::Inuktitut],
            IrishLatin => &[Language::Irish],
            ItalianLatin => &[Language::Italian],
            JapaneseHiragana => &[Language::Japanese],
            JapaneseKanji => &[Language::Japanese],
            JapaneseKatakana => &[Language::Japanese],
            Javanese => &[Language::Javanese],
            Kannada => &[Language::Kannada],
            KannadaTuluTigalari => &[Language::Kannada],
            KaroBatak => &[Language::Karo],
            KashmiriSharada => &[Language::Kashmiri],
            KashmiriTakri => &[Language::Kashmiri],
            KayahLi => &[Language::KayahLi],
            KazakhCyrillic => &[Language::Kazakh],
            KhitanSmallScript => &[Language::Khitan],
            Khmer => &[Language::Khmer],
            KhojaKhojki => &[Language::Khoja],
            KoreanHangul => &[Language::Korean],
            KoreanHanja => &[Language::Korean],
            KurdishArabic => &[Language::Kurdish],
            KurdishYezidi => &[Language::Kurdish],
            LaoTaiTham => &[Language::Lao],
            Latin => &[Language::Latin],
            LatvianLatin => &[Language::Latvian],
            Lepcha => &[Language::Lepcha],
            Limbu => &[Language::Limbu],
            Lisu => &[Language::Lisu],
            LithuanianLatin => &[Language::Lithuanian],
            LuwianAnatolianHieroglyphs => &[Language::Luwian],
            Lycian => &[Language::Lycian],
            Lydian => &[Language::Lydian],
            MacedonianCyrillic => &[Language::Macedonian],
            MagahiKaithi => &[Language::Magahi],
            MaithiliKaithi => &[Language::Maithili],
            MaithiliTirhuta => &[Language::Maithili],
            Makasar => &[Language::Makasar],
            MakassareseBuginese => &[Language::Makassarese],
            Malayalam => &[Language::Malayalam],
            MalayLatin => &[Language::Malay],
            MaldivianDhivehiDivesAkuru => &[Language::MaldivianDhivehi],
            MaldivianDhivehiThaana => &[Language::MaldivianDhivehi],
            Mandaic => &[Language::Mandaic],
            MandailingBatak => &[Language::Mandailing],
            MandeNKo => &[Language::Mande],
            ManipuriMeeteiMayek => &[Language::ManipuriMeetei],
            MaoriLatin => &[Language::Maori],
            MarathiDevanagari => &[Language::Marathi],
            MarathiModi => &[Language::Marathi],
            MarwariMahajani => &[Language::Marwari],
            Medefaidrin => &[Language::Medefaidrin],
            MendeKikakui => &[Language::Mende],
            MeroiticCursive => &[Language::Meroitic],
            MeroiticHieroglyphs => &[Language::Meroitic],
            MiddlePersianInscriptionalPahlavi => &[Language::MiddlePersian],
            MiddlePersianManichaean => &[Language::MiddlePersian],
            MiddlePersianPsalterPahlavi => &[Language::MiddlePersian],
            MinoanLinearA => &[Language::Minoan],
            Mongolian => &[Language::Mongolian],
            MongolianCyrillic => &[Language::Mongolian],
            MongolianPhagsPa => &[Language::Mongolian],
            MongolianSoyombo => &[Language::Mongolian],
            MongolianZanabazarSquare => &[Language::Mongolian],
            Mro => &[Language::Mro],
            MundariNag => &[Language::Mundari],
            MycenaeanGreekLinearB => &[Language::MycenaeanGreek],
            NabataeanAramaic => &[Language::NabataeanAramaic],
            NepaliDevanagari => &[Language::Nepali],
            NewariNewa => &[Language::Newari],
            NorthernThaiTaiTham => &[Language::NorthernThai],
            NynorskLatin => &[Language::Nynorsk],
            OdiaOriya => &[Language::Odia],
            OjibweCanadianAboriginal => &[Language::Ojibwe],
            OldChurchSlavonicGlagolitic => &[Language::OldChurchSlavonic],
            OldEnglish => &[Language::OldEnglish],
            OldHungarian => &[Language::OldHungarian],
            OldIrishOgham => &[Language::OldIrish],
            OldJavaneseKawi => &[Language::OldJavanese],
            OldKomiPermic => &[Language::OldKomi],
            OldNorseRunic => &[Language::OldNorse],
            OldNorthArabian => &[Language::OldNorthArabian],
            OldPersian => &[Language::OldPersian],
            OldSogdian => &[Language::OldSogdian],
            OldSouthArabian => &[Language::OldSouthArabian],
            OldTurkic => &[Language::OldTurkic],
            OldUyghur => &[Language::OldUyghur],
            OromoEthiopic => &[Language::Oromo],
            Osage => &[Language::Osage],
            OscanOldItalic => &[Language::Oscan],
            PakpakBatak => &[Language::Pakpak],
            ParthianInscriptional => &[Language::Parthian],
            PashtoArabic => &[Language::Pashto],
            PersianArabic => &[Language::Persian],
            Phoenician => &[Language::Phoenician],
            PolishLatin => &[Language::Polish],
            PortugueseLatin => &[Language::Portuguese],
            PrakritBrahmi => &[Language::Prakrit],
            PularAdlam => &[Language::Pular],
            PunjabiGurmukhi => &[Language::Punjabi],
            PunjabiMahajani => &[Language::Punjabi],
            Rejang => &[Language::Rejang],
            RohingyaHanifi => &[Language::Rohingya],
            RomanianLatin => &[Language::Romanian],
            RussianCyrillic => &[Language::Russian],
            SanskritBrahmi => &[Language::Sanskrit],
            SanskritDevanagari => &[Language::Sanskrit],
            SanskritGrantha => &[Language::Sanskrit],
            SanskritKawi => &[Language::Sanskrit],
            SanskritNandinagari => &[Language::Sanskrit],
            SanskritSharada => &[Language::Sanskrit],
            SanskritSiddham => &[Language::Sanskrit],
            SanskritSoyombo => &[Language::Sanskrit],
            SanskritTuluTigalari => &[Language::Sanskrit],
            SanskritZanabazarSquare => &[Language::Sanskrit],
            SantaliOlChiki => &[Language::Santali],
            SaraikiMultani => &[Language::Saraiki],
            Saurashtra => &[Language::Saurashtra],
            SerbianCyrillic => &[Language::Serbian],
            ShanMyanmar => &[Language::Shan],
            ShonaLatin => &[Language::Shona],
            SignlanguageWriting => &[Language::Signlanguages],
            SimalungunBatak => &[Language::Simalungun],
            SindhiArabic => &[Language::Sindhi],
            SindhiKhojki => &[Language::Sindhi],
            SindhiKhudawadi => &[Language::Sindhi],
            Sinhala => &[Language::Sinhala],
            SlovakLatin => &[Language::Slovak],
            SloveneLatin => &[Language::Slovene],
            Sogdian => &[Language::Sogdian],
            SogdianManichaean => &[Language::Sogdian],
            SomaliLatin => &[Language::Somali],
            SomaliOsmanya => &[Language::Somali],
            Sora => &[Language::Sora],
            SothoLatin => &[Language::Sotho],
            SpanishLatin => &[Language::Spanish],
            SumerianCuneiform => &[Language::Sumerian],
            Sundanese => &[Language::Sundanese],
            Sunuwar => &[Language::Sunuwar],
            SwahiliLatin => &[Language::Swahili],
            SwedishLatin => &[Language::Swedish],
            SylhetiSylotiNagri => &[Language::Sylheti],
            Syriac => &[Language::Syriac],
            Tagalog => &[Language::Tagalog],
            TagalogLatin => &[Language::Tagalog],
            Tagbanwa => &[Language::Tagbanwa],
            TaiDamTaiViet => &[Language::TaiDam],
            TaiDonTaiViet => &[Language::TaiDon],
            TaiLe => &[Language::TaiLe],
            TaiLueNew => &[Language::TaiLue],
            TaiLueTaiTham => &[Language::TaiLue],
            Tamil => &[Language::Tamil],
            TamilGrantha => &[Language::Tamil],
            Tangsa => &[Language::Tangsa],
            Tangut => &[Language::Tangut],
            Telugu => &[Language::Telugu],
            Thai => &[Language::Thai],
            Tibetan => &[Language::Tibetan],
            TibetanPhagsPa => &[Language::Tibetan],
            TibetanSoyombo => &[Language::Tibetan],
            TigrinyaEthiopic => &[Language::Tigrinya],
            TobaBatak => &[Language::Toba],
            Toto => &[Language::Toto],
            TsongaLatin => &[Language::Tsonga],
            TswanaLatin => &[Language::Tswana],
            TuluTigalari => &[Language::Tulu],
            TurkishLatin => &[Language::Turkish],
            Ugaritic => &[Language::Ugaritic],
            UkrainianCyrillic => &[Language::Ukrainian],
            UmbrianOldItalic => &[Language::Umbrian],
            UrduArabic => &[Language::Urdu],
            UyghurArabic => &[Language::Uyghur],
            Vai => &[Language::Vai],
            VietnameseLatin => &[Language::Vietnamese],
            Wancho => &[Language::Wancho],
            WelshLatin => &[Language::Welsh],
            WolofGaray => &[Language::Wolof],
            XhosaLatin => &[Language::Xhosa],
            Yi => &[Language::Yi],
            YiddishHebrew => &[Language::Yiddish],
            YorubaLatin => &[Language::Yoruba],
            ZoPauCinHau => &[Language::ZoLanguages],
            ZuluLatin => &[Language::Zulu],
        }
    }
}

/// add all the leters of all the alphabets in the script group
/// or not all, only if it does not require to exclude letters
/// if the script group has only one language, then leave it empty
pub(crate) fn script_alphabets(a: Script, ch: char) -> &'static [Alphabet] {
    match a {
        Script::Adlam => alphabet_match!([(Alphabet::FulaniAdlam, []), (Alphabet::PularAdlam, [])]),
        Script::Ahom => alphabet_match!([(Alphabet::Ahom, [])]),
        Script::AnatolianHieroglyphs => {
            alphabet_match!([(Alphabet::LuwianAnatolianHieroglyphs, [])])
        }
        Script::Arabic => alphabet_match!([
            (Alphabet::Arabic, []),
            (Alphabet::KurdishArabic, []),
            (Alphabet::PashtoArabic, []),
            (Alphabet::PersianArabic, []),
            (Alphabet::SindhiArabic, []),
            (Alphabet::UrduArabic, []),
            (Alphabet::UyghurArabic, []),
        ]),
        Script::Armenian => alphabet_match!([(Alphabet::Armenian, [])]),
        Script::Avestan => alphabet_match!([(Alphabet::Avestan, [])]),
        Script::Balinese => alphabet_match!([(Alphabet::Balinese, [])]),
        Script::Bamum => alphabet_match!([(Alphabet::Bamum, [])]),
        Script::BassaVah => alphabet_match!([(Alphabet::BassaVah, [])]),
        Script::Batak => {
            alphabet_match!([
                (Alphabet::TobaBatak, []),
                (Alphabet::KaroBatak, []),
                (Alphabet::MandailingBatak, []),
                (Alphabet::PakpakBatak, []),
                (Alphabet::SimalungunBatak, []),
                (Alphabet::AngkolaBatak, []),
            ])
        }
        Script::Bengali => alphabet_match!([
            (Alphabet::AssameseBengali, []),
            (Alphabet::Bengali, []),
            (Alphabet::BishnupriyaManipuriBengali, []),
        ]),
        Script::Bhaiksuki => alphabet_match!([(Alphabet::Bhaiksuki, [])]),
        Script::Bopomofo => alphabet_match!([(Alphabet::ChineseMandarinBopomofo, [])]),
        Script::Brahmi => alphabet_match!([
            (Alphabet::SanskritBrahmi, []),
            (Alphabet::PrakritBrahmi, [])
        ]),
        Script::Braille => alphabet_match!([(Alphabet::Braille, [])]),
        Script::Buginese => {
            alphabet_match!([
                (Alphabet::Buginese, []),
                (Alphabet::MakassareseBuginese, [])
            ])
        }
        Script::Buhid => alphabet_match!([(Alphabet::Buhid, [])]),
        Script::CanadianAboriginal => alphabet_match!([
            (Alphabet::CreeCanadianAboriginal, []),
            (Alphabet::InuktitutCanadianAboriginal, []),
            (Alphabet::OjibweCanadianAboriginal, [])
        ]),
        Script::Carian => alphabet_match!([(Alphabet::Carian, [])]),
        Script::CaucasianAlbanian => alphabet_match!([(Alphabet::CaucasianAlbanian, [])]),
        Script::Chakma => alphabet_match!([(Alphabet::Chakma, [])]),
        Script::Cham => alphabet_match!([(Alphabet::Cham, [])]),
        Script::Cherokee => alphabet_match!([(Alphabet::Cherokee, [])]),
        Script::Chorasmian => alphabet_match!([(Alphabet::Chorasmian, [])]),
        // if you want to add something here, validate that char's range is active in `ucd.rs`
        // during parsing these considered as connectors, not chars of the word
        // example1: `can't` for english is one word, for other lang it is two words: `can, t`
        //   because if Alphabets of all 3 chars do not intersect, it will be two words
        // example2: `word1' word2` for all langs will be parsed as two words without `'`,
        //   because next char after `'` is space, which is not a char of any language
        Script::Common => match ch {
            '\'' => &[Alphabet::BelarusianCyrillic, Alphabet::EnglishLatin],
            '¡' => &[Alphabet::SpanishLatin],
            '¿' => &[Alphabet::SpanishLatin],
            _ => &[], // must be always empty
        },
        Script::Coptic => alphabet_match!([(Alphabet::Coptic, [])]),
        Script::Cuneiform => alphabet_match!([
            (Alphabet::AkkadianCuneiform, []),
            (Alphabet::HittiteCuneiform, []),
            (Alphabet::SumerianCuneiform, []),
        ]),
        Script::Cypriot => alphabet_match!([(Alphabet::AncientGreekCypriot, [])]),
        Script::CyproMinoan => alphabet_match!([(Alphabet::CyproMinoan, [])]),
        Script::Cyrillic => alphabet_match!([
            (
                Alphabet::BelarusianCyrillic,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'І', 'і', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ў', 'ў', 'Ф', 'ф', 'Х', 'х',
                    'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Alphabet::BulgarianCyrillic,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ж', 'ж', 'З', 'з',
                    'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о', 'П', 'п',
                    'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч',
                    'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ь', 'ь', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Alphabet::KazakhCyrillic,
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
                Alphabet::MacedonianCyrillic,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Ѓ', 'ѓ', 'Е', 'е', 'Ж', 'ж',
                    'З', 'з', 'Ѕ', 'ѕ', 'И', 'и', 'Ј', 'ј', 'К', 'к', 'Л', 'л', 'Љ', 'љ', 'М', 'м',
                    'Н', 'н', 'Њ', 'њ', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ќ', 'ќ',
                    'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Џ', 'џ', 'Ш', 'ш',
                ]
            ),
            (
                Alphabet::MongolianCyrillic,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ү', 'ү', 'Ф', 'ф',
                    'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь',
                    'Э', 'э', 'Ю', 'ю', 'Я', 'я',
                ]
            ),
            (
                Alphabet::RussianCyrillic,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ф', 'ф', 'Х', 'х', 'Ц', 'ц',
                    'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь', 'Э', 'э', 'Ю', 'ю',
                    'Я', 'я',
                ],
            ),
            (
                Alphabet::SerbianCyrillic,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Ђ', 'ђ', 'Е', 'е', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Ј', 'ј', 'К', 'к', 'Л', 'л', 'Љ', 'љ', 'М', 'м', 'Н', 'н',
                    'Њ', 'њ', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'Ћ', 'ћ', 'У', 'у',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Џ', 'џ', 'Ш', 'ш',
                ]
            ),
            (
                Alphabet::UkrainianCyrillic,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Ґ', 'ґ', 'Д', 'д', 'Е', 'е', 'Є', 'є',
                    'Ж', 'ж', 'З', 'з', 'И', 'и', 'І', 'і', 'Ї', 'ї', 'Й', 'й', 'К', 'к', 'Л', 'л',
                    'М', 'м', 'Н', 'н', 'О', 'о', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у',
                    'Ф', 'ф', 'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ь', 'ь', 'Ю', 'ю',
                    'Я', 'я',
                ],
            ),
        ]),
        Script::Deseret => alphabet_match!([(Alphabet::EnglishMormonDeseret, [])]),
        Script::Devanagari => alphabet_match!([
            (Alphabet::HindiDevanagari, []),
            (Alphabet::MarathiDevanagari, []),
            (Alphabet::NepaliDevanagari, []),
            (Alphabet::SanskritDevanagari, []),
        ]),
        Script::DivesAkuru => alphabet_match!([(Alphabet::MaldivianDhivehiDivesAkuru, [])]),
        Script::Dogra => alphabet_match!([(Alphabet::DogriDogra, [])]),
        Script::Duployan => alphabet_match!([
            (Alphabet::EnglishDuployan, []),
            (Alphabet::FrenchDuployan, [])
        ]),
        Script::EgyptianHieroglyphs => alphabet_match!([(Alphabet::EgyptianHieroglyphs, [])]),
        Script::Elbasan => alphabet_match!([(Alphabet::AlbanianHistoricalElbasan, [])]),
        Script::Elymaic => alphabet_match!([(Alphabet::Elymaic, [])]),
        Script::Ethiopic => alphabet_match!([
            (Alphabet::AmharicEthiopic, []),
            (Alphabet::GeezEthiopic, []),
            (Alphabet::OromoEthiopic, []),
            (Alphabet::TigrinyaEthiopic, []),
        ]),
        Script::Garay => alphabet_match!([(Alphabet::WolofGaray, [])]),
        Script::Georgian => alphabet_match!([(Alphabet::Georgian, [])]),
        Script::Glagolitic => alphabet_match!([(Alphabet::OldChurchSlavonicGlagolitic, [])]),
        Script::Gothic => alphabet_match!([(Alphabet::Gothic, [])]),
        Script::Grantha => alphabet_match!([
            (Alphabet::SanskritGrantha, []),
            (Alphabet::TamilGrantha, [])
        ]),
        Script::Greek => alphabet_match!([(Alphabet::Greek, [])]),
        Script::Gujarati => alphabet_match!([(Alphabet::Gujarati, [])]),
        Script::GunjalaGondi => alphabet_match!([(Alphabet::GondiGunjala, [])]),
        Script::Gurmukhi => alphabet_match!([(Alphabet::PunjabiGurmukhi, [])]),
        Script::GurungKhema => alphabet_match!([(Alphabet::GurungKhema, [])]),
        Script::Han => alphabet_match!([
            (Alphabet::ChineseSimplified, []),
            (Alphabet::ChineseTraditional, []),
            (
                // https://en.wikipedia.org/wiki/List_of_j%C5%8Dy%C5%8D_kanji
                // it also uses all Traditional Chinese characters
                Alphabet::JapaneseKanji,
                [
                    '亜', '悪', '圧', '囲', '医', '為', '壱', '逸', '隠', '栄', '営', '衛', '駅',
                    '謁', '円', '塩', '縁', '艶', '応', '欧', '殴', '桜', '奥', '横', '温', '穏',
                    '仮', '価', '禍', '画', '会', '悔', '海', '絵', '壊', '懐', '慨', '概', '拡',
                    '殻', '覚', '学', '岳', '楽', '喝', '渇', '褐', '缶', '巻', '陥', '勧', '寛',
                    '漢', '関', '歓', '観', '気', '祈', '既', '帰', '器', '亀', '偽', '戯', '犠',
                    '旧', '拠', '挙', '虚', '峡', '挟', '狭', '郷', '響', '暁', '勤', '謹', '区',
                    '駆', '勲', '薫', '径', '茎', '恵', '掲', '渓', '経', '蛍', '軽', '継', '鶏',
                    '芸', '撃', '欠', '研', '県', '倹', '剣', '険', '圏', '検', '献', '権', '顕',
                    '験', '厳', '広', '効', '恒', '黄', '鉱', '号', '国', '黒', '穀', '砕', '済',
                    '斎', '剤', '殺', '雑', '参', '桟', '蚕', '惨', '賛', '残', '糸', '祉', '視',
                    '歯', '児', '辞', '湿', '実', '写', '社', '者', '煮', '釈', '寿', '収', '臭',
                    '従', '渋', '獣', '縦', '祝', '粛', '処', '暑', '署', '緒', '諸', '叙', '将',
                    '祥', '称', '渉', '焼', '証', '奨', '条', '状', '乗', '浄', '剰', '畳', '縄',
                    '壌', '嬢', '譲', '醸', '触', '嘱', '神', '真', '寝', '慎', '尽', '図', '粋',
                    '酔', '穂', '随', '髄', '枢', '数', '瀬', '声', '斉', '静', '窃', '摂', '節',
                    '専', '浅', '戦', '践', '銭', '潜', '繊', '禅', '祖', '双', '壮', '争', '荘',
                    '捜', '挿', '巣', '曽', '痩', '装', '僧', '層', '総', '騒', '増', '憎', '蔵',
                    '贈', '臓', '即', '属', '続', '堕', '対', '体', '帯', '滞', '台', '滝', '択',
                    '沢', '担', '単', '胆', '嘆', '団', '断', '弾', '遅', '痴', '虫', '昼', '鋳',
                    '著', '庁', '徴', '聴', '懲', '勅', '鎮', '塚', '逓', '鉄', '点', '転', '伝',
                    '都', '灯', '当', '党', '盗', '稲', '闘', '徳', '独', '読', '突', '届', '難',
                    '弐', '悩', '脳', '覇', '拝', '廃', '売', '梅', '麦', '発', '髪', '抜', '繁',
                    '晩', '蛮', '卑', '秘', '碑', '浜', '賓', '頻', '敏', '瓶', '侮', '福', '払',
                    '仏', '併', '並', '塀', '餅', '辺', '変', '勉', '歩', '宝', '豊', '褒', '墨',
                    '翻', '毎', '万', '満', '免', '麺', '黙', '弥', '訳', '薬', '与', '予', '余',
                    '誉', '揺', '様', '謡', '来', '頼', '乱', '覧', '欄', '竜', '隆', '虜', '両',
                    '猟', '緑', '涙', '塁', '類', '礼', '励', '戻', '霊', '齢', '暦', '歴', '恋',
                    '練', '錬', '炉', '労', '郎', '朗', '廊', '楼', '録', '湾', '弁'
                ]
            ),
            (Alphabet::KoreanHanja, []),
        ]),
        Script::Hangul => alphabet_match!([(Alphabet::KoreanHangul, [])]),
        Script::HanifiRohingya => alphabet_match!([(Alphabet::RohingyaHanifi, [])]),
        Script::Hanunoo => alphabet_match!([(Alphabet::Hanunoo, [])]),
        Script::Hatran => alphabet_match!([(Alphabet::AramaicHatran, [])]),
        Script::Hebrew => alphabet_match!([(Alphabet::Hebrew, []), (Alphabet::YiddishHebrew, [])]),
        Script::Hiragana => alphabet_match!([(Alphabet::JapaneseHiragana, [])]),
        Script::ImperialAramaic => alphabet_match!([(Alphabet::AramaicImperial, [])]),
        Script::InscriptionalPahlavi => {
            alphabet_match!([(Alphabet::MiddlePersianInscriptionalPahlavi, [])])
        }
        Script::InscriptionalParthian => alphabet_match!([(Alphabet::ParthianInscriptional, [])]),
        Script::Javanese => alphabet_match!([(Alphabet::Javanese, [])]),
        Script::Kaithi => alphabet_match!([
            (Alphabet::BhojpuriKaithi, []),
            (Alphabet::MagahiKaithi, []),
            (Alphabet::MaithiliKaithi, []),
        ]),
        Script::Kannada => alphabet_match!([(Alphabet::Kannada, [])]),
        Script::Katakana => alphabet_match!([(Alphabet::JapaneseKatakana, [])]),
        Script::Kawi => alphabet_match!([
            (Alphabet::OldJavaneseKawi, []),
            (Alphabet::SanskritKawi, [])
        ]),
        Script::KayahLi => alphabet_match!([(Alphabet::KayahLi, [])]),
        Script::Kharoshthi => alphabet_match!([(Alphabet::GandhariKharoshthi, [])]),
        Script::KhitanSmallScript => alphabet_match!([(Alphabet::KhitanSmallScript, [])]),
        Script::Khmer => alphabet_match!([(Alphabet::Khmer, [])]),
        Script::Khojki => {
            alphabet_match!([(Alphabet::SindhiKhojki, []), (Alphabet::KhojaKhojki, [])])
        }
        Script::Khudawadi => alphabet_match!([(Alphabet::SindhiKhudawadi, [])]),
        Script::KiratRai => alphabet_match!([(Alphabet::BantawaKiratRai, [])]),
        Script::Lao => alphabet_match!([(Alphabet::LaoTaiTham, [])]),
        Script::Latin => alphabet_match!([
            (
                Alphabet::AfrikaansLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ç', 'ç', 'É', 'é', 'Ë', 'ë', 'Ï', 'ï', 'Ô', 'ô'
                ]
            ),
            (
                Alphabet::AlbanianLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', /* 'Dh', 'dh', */ 'E',
                    'e', 'Ë', 'ë', 'F', 'f', 'G', 'g', /* 'Gj', 'gj', */ 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', /* 'Ll', 'll', */ 'M', 'm', 'N', 'n',
                    /* 'Nj', 'nj', */ 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r',
                    /* 'Rr', 'rr', */ 'S', 's', /* 'Sh', 'sh', */ 'T', 't',
                    /* 'Th', 'th', */ 'U', 'u', 'V', 'v', 'X', 'x',
                    /* 'Xh', 'xh', */ 'Y', 'y', 'Z', 'z', /* 'Zh', 'zh', */
                    'Â', 'â', 'Ê', 'ê'
                ]
            ),
            (
                Alphabet::AzerbaijaniLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'Ə', 'ə', 'F', 'f',
                    'G', 'g', 'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'Q', 'q',
                    'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's',
                    'Ş', 'ş', 'T', 't', 'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::BasqueLatin,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'X', 'x', 'Z', 'z', 'Ç', 'ç', 'Ñ', 'ñ'
                ]
            ),
            (
                Alphabet::BokmalLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å', 'Ç', 'ç', 'É', 'é'
                ]
            ),
            (
                Alphabet::BosnianLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž', 'Ç', 'ç',
                ]
            ),
            (
                Alphabet::CatalanLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y',
                    'Z', 'z', 'Ç', 'ç', 'Ï', 'ï', 'Ë', 'ë', 'Ü', 'ü', 'À', 'à', 'È', 'è', 'Ò', 'ò'
                ]
            ),
            (
                Alphabet::CroatianLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž', 'Ç', 'ç',
                ]
            ),
            (
                Alphabet::CzechLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'Ď', 'ď', 'E', 'e', 'É', 'é',
                    'Ě', 'ě', 'F', 'f', 'G', 'g', 'H', 'h', /* 'Ch', 'ch', */ 'I', 'i', 'Í',
                    'í', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ň', 'ň', 'O', 'o', 'Ó',
                    'ó', 'P', 'p', 'Q', 'q', 'R', 'r', 'Ř', 'ř', 'S', 's', 'Š', 'š', 'T', 't', 'Ť',
                    'ť', 'U', 'u', 'Ú', 'ú', 'Ů', 'ů', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Ý',
                    'ý', 'Z', 'z', 'Ž', 'ž',
                ]
            ),
            (
                Alphabet::DanishLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å', 'Ç', 'ç', 'É', 'é', 'Ê', 'ê',
                    'Ô', 'ô'
                ]
            ),
            (
                Alphabet::DutchLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ç', 'ç', 'É', 'é', 'Ï', 'ï', 'Ü', 'ü', 'Ë', 'ë'
                ]
            ),
            (
                Alphabet::EnglishLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::EsperantoLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ĉ', 'ĉ', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ĝ', 'ĝ', 'H', 'h', 'Ĥ', 'ĥ', 'I', 'i', 'J', 'j', 'Ĵ', 'ĵ', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'Ŝ', 'ŝ', 'T', 't',
                    'U', 'u', 'Ŭ', 'ŭ', 'V', 'v', 'Z', 'z',
                ]
            ),
            (
                Alphabet::EstonianLatin,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Õ', 'õ', 'Ä', 'ä', 'Ö', 'ö',
                    'Ü', 'ü', 'Z', 'z', 'Ž', 'ž',
                ]
            ),
            (
                Alphabet::FinnishLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z', 'Ä', 'ä', 'Ö', 'ö', 'Š', 'š', 'Ž', 'ž'
                ]
            ),
            (
                Alphabet::FrenchLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'À', 'à', 'Â', 'â', 'Æ', 'æ', 'Ç', 'ç', 'É', 'é', 'È', 'è',
                    'Ê', 'ê', 'Ë', 'ë', 'Î', 'î', 'Ï', 'ï', 'Ô', 'ô', 'Œ', 'œ', 'Ù', 'ù', 'Û', 'û',
                    'Ü', 'ü', 'Ÿ', 'ÿ'
                ]
            ),
            (
                Alphabet::GandaLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'É', 'é', 'È', 'è'
                ]
            ),
            (
                Alphabet::GermanLatin,
                [
                    'A', 'a', 'Ä', 'ä', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'Ü', 'ü',
                    'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'ẞ', 'ß'
                ]
            ),
            (
                Alphabet::HungarianLatin,
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
                Alphabet::IcelandicLatin,
                [
                    'A', 'a', 'Á', 'á', 'B', 'b', 'D', 'd', 'Ð', 'ð', 'E', 'e', 'É', 'é', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Í', 'í', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u',
                    'Ú', 'ú', 'V', 'v', 'X', 'x', 'Y', 'y', 'Ý', 'ý', 'Þ', 'þ', 'Æ', 'æ', 'Ö', 'ö',
                ]
            ),
            (
                Alphabet::IndonesianLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::IrishLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'T', 't', 'U', 'u', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú'
                ]
            ),
            (
                Alphabet::ItalianLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z', 'À', 'à', 'È', 'è', 'É', 'é',
                    'Ì', 'ì', 'Ò', 'ò', 'Ù', 'ù'
                ]
            ),
            (
                Alphabet::Latin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y', 'Z', 'z',
                    'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū', 'Æ', 'æ', 'Œ', 'œ'
                ]
            ),
            (
                Alphabet::LatvianLatin,
                [
                    'A', 'a', 'Ā', 'ā', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ē', 'ē',
                    'F', 'f', 'G', 'g', 'Ģ', 'ģ', 'H', 'h', 'I', 'i', 'Ī', 'ī', 'J', 'j', 'K', 'k',
                    'Ķ', 'ķ', 'L', 'l', 'Ļ', 'ļ', 'M', 'm', 'N', 'n', 'Ņ', 'ņ', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'Ū', 'ū', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                Alphabet::LithuanianLatin,
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'Ė', 'ė', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Į', 'į', 'Y', 'y', 'J', 'j',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'Š', 'š', 'T', 't', 'U', 'u', 'Ų', 'ų', 'Ū', 'ū', 'V', 'v', 'Z', 'z', 'Ž', 'ž',
                ]
            ),
            (
                Alphabet::MalayLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::MaoriLatin,
                [
                    'A', 'a', 'E', 'e', 'H', 'h', 'I', 'i', 'K', 'k', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'T', 't', 'U', 'u', 'W', 'w', /* 'Ng', 'ng', */ 'G',
                    'g', /* 'Wh', 'wh' */ 'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū'
                ]
            ),
            (
                Alphabet::NynorskLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å',
                ]
            ),
            (
                Alphabet::PolishLatin,
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Ć', 'ć', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'Ł', 'ł',
                    'M', 'm', 'N', 'n', 'Ń', 'ń', 'O', 'o', 'Ó', 'ó', 'P', 'p', 'R', 'r', 'S', 's',
                    'Ś', 'ś', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'Z', 'z', 'Ź', 'ź', 'Ż', 'ż',
                ]
            ),
            (
                Alphabet::PortugueseLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Á', 'á', 'Â', 'â', 'Ã', 'ã', 'À', 'à', 'Ç', 'ç', 'É', 'é',
                    'Ê', 'ê', 'Í', 'í', 'Ó', 'ó', 'Ô', 'ô', 'Õ', 'õ', 'Ú', 'ú', 'Ü', 'ü'
                ]
            ),
            (
                Alphabet::RomanianLatin,
                [
                    'A', 'a', 'Ă', 'ă', 'Â', 'â', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Î', 'î', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ș', 'ș', 'T', 't',
                    'Ț', 'ț', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::ShonaLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z',
                    'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú'
                ]
            ),
            (
                Alphabet::SlovakLatin,
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
                Alphabet::SloveneLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž',
                ]
            ),
            (
                Alphabet::SomaliLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Q', 'q',
                    'R', 'r', 'S', 's', /* 'Sh', 'sh', */ 'T', 't', 'U', 'u', 'W', 'w', 'X',
                    'x', 'Y', 'y'
                ]
            ),
            (
                Alphabet::SothoLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::SpanishLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'Ñ', 'ñ', 'O', 'o',
                    'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w',
                    'X', 'x', 'Y', 'y', 'Z', 'z', 'Á', 'á', 'É', 'é', 'Í', 'í', 'Ó', 'ó', 'Ú', 'ú',
                    'Ü', 'ü'
                ]
            ),
            (
                Alphabet::SwahiliLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::SwedishLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Å', 'å', 'Ä', 'ä', 'Ö', 'ö',
                ]
            ),
            (
                Alphabet::TagalogLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n',
                    /* 'Ng', 'ng', */ 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T',
                    't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z'
                ]
            ),
            (
                Alphabet::TsongaLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z'
                ]
            ),
            (
                Alphabet::TswanaLatin,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y'
                ]
            ),
            (
                Alphabet::TurkishLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Alphabet::VietnameseLatin,
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
                Alphabet::WelshLatin,
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
                Alphabet::XhosaLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z'
                ]
            ),
            (
                Alphabet::YorubaLatin,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'Ẹ', 'ẹ', 'F', 'f', 'G', 'g',
                    /* 'Gb', 'gb', */ 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M',
                    'm', 'N', 'n', 'O', 'o', 'Ọ', 'ọ', 'P', 'p', 'R', 'r', 'S', 's', 'Ṣ', 'ṣ', 'T',
                    't', 'U', 'u', 'W', 'w', 'Y', 'y', 'À', 'à', 'Á', 'á', 'È', 'è', 'É', 'é', 'Ì',
                    'ì', 'Í', 'í', 'Ò', 'ò', 'Ó', 'ó', 'Ù', 'ù', 'Ú', 'ú',
                ]
            ),
            (
                Alphabet::ZuluLatin,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
                ]
            ),
        ]),
        Script::Lepcha => alphabet_match!([(Alphabet::Lepcha, [])]),
        Script::Limbu => alphabet_match!([(Alphabet::Limbu, [])]),
        Script::LinearA => alphabet_match!([(Alphabet::MinoanLinearA, [])]),
        Script::LinearB => alphabet_match!([(Alphabet::MycenaeanGreekLinearB, [])]),
        Script::Lisu => alphabet_match!([(Alphabet::Lisu, [])]),
        Script::Lycian => alphabet_match!([(Alphabet::Lycian, [])]),
        Script::Lydian => alphabet_match!([(Alphabet::Lydian, [])]),
        Script::Mahajani => alphabet_match!([
            (Alphabet::HindiMahajani, []),
            (Alphabet::MarwariMahajani, []),
            (Alphabet::PunjabiMahajani, []),
        ]),
        Script::Makasar => alphabet_match!([(Alphabet::Makasar, [])]),
        Script::Malayalam => alphabet_match!([(Alphabet::Malayalam, [])]),
        Script::Mandaic => {
            alphabet_match!([(Alphabet::Mandaic, []), (Alphabet::AramaicMandaic, [])])
        }
        Script::Manichaean => {
            alphabet_match!([
                (Alphabet::MiddlePersianManichaean, []),
                (Alphabet::SogdianManichaean, [])
            ])
        }
        Script::Marchen => alphabet_match!([(Alphabet::BuddhistMarchen, [])]),
        Script::MasaramGondi => alphabet_match!([(Alphabet::GondiMasaram, [])]),
        Script::Medefaidrin => alphabet_match!([(Alphabet::Medefaidrin, [])]),
        Script::MeeteiMayek => alphabet_match!([(Alphabet::ManipuriMeeteiMayek, [])]),
        Script::MendeKikakui => alphabet_match!([(Alphabet::MendeKikakui, [])]),
        Script::MeroiticCursive => alphabet_match!([(Alphabet::MeroiticCursive, [])]),
        Script::MeroiticHieroglyphs => alphabet_match!([(Alphabet::MeroiticHieroglyphs, [])]),
        Script::Miao => alphabet_match!([(Alphabet::HmongMiao, [])]),
        Script::Modi => alphabet_match!([(Alphabet::MarathiModi, [])]),
        Script::Mongolian => alphabet_match!([(Alphabet::Mongolian, [])]),
        Script::Mro => alphabet_match!([(Alphabet::Mro, [])]),
        Script::Multani => alphabet_match!([(Alphabet::SaraikiMultani, [])]),
        Script::Myanmar => {
            alphabet_match!([(Alphabet::BurmeseMyanmar, []), (Alphabet::ShanMyanmar, [])])
        }
        Script::Nabataean => alphabet_match!([(Alphabet::AramaicNabataean, [])]),
        Script::NagMundari => alphabet_match!([(Alphabet::MundariNag, [])]),
        Script::Nandinagari => alphabet_match!([(Alphabet::SanskritNandinagari, [])]),
        Script::NewTaiLue => alphabet_match!([(Alphabet::TaiLueNew, [])]),
        Script::Newa => alphabet_match!([(Alphabet::NewariNewa, [])]),
        Script::Nko => alphabet_match!([(Alphabet::MandeNKo, [])]),
        Script::Nushu => alphabet_match!([(Alphabet::ChineseNushu, [])]),
        Script::NyiakengPuachueHmong => alphabet_match!([(Alphabet::HmongNyiakengPuachue, [])]),
        Script::Ogham => alphabet_match!([(Alphabet::OldIrishOgham, [])]),
        Script::OlChiki => alphabet_match!([(Alphabet::SantaliOlChiki, [])]),
        Script::OlOnal => alphabet_match!([(Alphabet::HoOlOnal, [])]),
        Script::OldHungarian => alphabet_match!([(Alphabet::OldHungarian, [])]),
        Script::OldItalic => alphabet_match!([
            (Alphabet::EtruscanOldItalic, []),
            (Alphabet::OscanOldItalic, []),
            (Alphabet::UmbrianOldItalic, []),
        ]),
        Script::OldNorthArabian => alphabet_match!([(Alphabet::OldNorthArabian, [])]),
        Script::OldPermic => alphabet_match!([(Alphabet::OldKomiPermic, [])]),
        Script::OldPersian => alphabet_match!([(Alphabet::OldPersian, [])]),
        Script::OldSogdian => alphabet_match!([(Alphabet::OldSogdian, [])]),
        Script::OldSouthArabian => alphabet_match!([(Alphabet::OldSouthArabian, [])]),
        Script::OldTurkic => alphabet_match!([(Alphabet::OldTurkic, [])]),
        Script::OldUyghur => alphabet_match!([(Alphabet::OldUyghur, [])]),
        Script::Oriya => alphabet_match!([(Alphabet::OdiaOriya, [])]),
        Script::Osage => alphabet_match!([(Alphabet::Osage, [])]),
        Script::Osmanya => alphabet_match!([(Alphabet::SomaliOsmanya, [])]),
        Script::PahawhHmong => alphabet_match!([(Alphabet::HmongPahawh, [])]),
        Script::Palmyrene => alphabet_match!([(Alphabet::AramaicPalmyrene, [])]),
        Script::PauCinHau => alphabet_match!([(Alphabet::ZoPauCinHau, [])]),
        Script::PhagsPa => alphabet_match!([
            (Alphabet::MongolianPhagsPa, []),
            (Alphabet::TibetanPhagsPa, [])
        ]),
        Script::Phoenician => alphabet_match!([(Alphabet::Phoenician, [])]),
        Script::PsalterPahlavi => alphabet_match!([(Alphabet::MiddlePersianPsalterPahlavi, [])]),
        Script::Rejang => alphabet_match!([(Alphabet::Rejang, [])]),
        Script::Runic => {
            alphabet_match!([(Alphabet::OldNorseRunic, []), (Alphabet::OldEnglish, [])])
        }
        Script::Samaritan => alphabet_match!([(Alphabet::HebrewSamaritan, [])]),
        Script::Saurashtra => alphabet_match!([(Alphabet::Saurashtra, [])]),
        Script::Sharada => alphabet_match!([
            (Alphabet::SanskritSharada, []),
            (Alphabet::KashmiriSharada, [])
        ]),
        Script::Shavian => alphabet_match!([(Alphabet::EnglishPhoneticShavian, [])]),
        Script::Siddham => alphabet_match!([(Alphabet::SanskritSiddham, [])]),
        Script::SignWriting => alphabet_match!([(Alphabet::SignlanguageWriting, [])]),
        Script::Sinhala => alphabet_match!([(Alphabet::Sinhala, [])]),
        Script::Sogdian => alphabet_match!([(Alphabet::Sogdian, [])]),
        Script::SoraSompeng => alphabet_match!([(Alphabet::Sora, [])]),
        Script::Soyombo => alphabet_match!([
            (Alphabet::MongolianSoyombo, []),
            (Alphabet::SanskritSoyombo, []),
            (Alphabet::TibetanSoyombo, []),
        ]),
        Script::Sundanese => alphabet_match!([(Alphabet::Sundanese, [])]),
        Script::Sunuwar => alphabet_match!([(Alphabet::Sunuwar, [])]),
        Script::SylotiNagri => alphabet_match!([(Alphabet::SylhetiSylotiNagri, [])]),
        Script::Syriac => alphabet_match!([(Alphabet::Syriac, []), (Alphabet::AramaicSyriac, [])]),
        Script::Tagalog => alphabet_match!([(Alphabet::Tagalog, [])]),
        Script::Tagbanwa => alphabet_match!([(Alphabet::Tagbanwa, [])]),
        Script::TaiLe => alphabet_match!([(Alphabet::TaiLe, [])]),
        Script::TaiTham => alphabet_match!([
            (Alphabet::LaoTaiTham, []),
            (Alphabet::NorthernThaiTaiTham, []),
            (Alphabet::TaiLueTaiTham, []),
        ]),
        Script::TaiViet => {
            alphabet_match!([(Alphabet::TaiDamTaiViet, []), (Alphabet::TaiDonTaiViet, [])])
        }
        Script::Takri => {
            alphabet_match!([(Alphabet::DogriTakri, []), (Alphabet::KashmiriTakri, [])])
        }
        Script::Tamil => alphabet_match!([(Alphabet::Tamil, [])]),
        Script::Tangsa => alphabet_match!([(Alphabet::Tangsa, [])]),
        Script::Tangut => alphabet_match!([(Alphabet::Tangut, [])]),
        Script::Telugu => alphabet_match!([(Alphabet::Telugu, [])]),
        Script::Thaana => alphabet_match!([(Alphabet::MaldivianDhivehiThaana, [])]),
        Script::Thai => alphabet_match!([(Alphabet::Thai, [])]),
        Script::Tibetan => alphabet_match!([(Alphabet::Tibetan, [])]),
        Script::Tifinagh => alphabet_match!([(Alphabet::BerberTifinagh, [])]),
        Script::Tirhuta => alphabet_match!([(Alphabet::MaithiliTirhuta, [])]),
        Script::Todhri => alphabet_match!([(Alphabet::AlbanianHistoricalTodhri, [])]),
        Script::Toto => alphabet_match!([(Alphabet::Toto, [])]),
        Script::TuluTigalari => alphabet_match!([
            (Alphabet::SanskritTuluTigalari, []),
            (Alphabet::TuluTigalari, []),
            (Alphabet::KannadaTuluTigalari, [])
        ]),
        Script::Ugaritic => alphabet_match!([(Alphabet::Ugaritic, [])]),
        Script::Vai => alphabet_match!([(Alphabet::Vai, [])]),
        Script::Vithkuqi => alphabet_match!([(Alphabet::AlbanianVithkuqi, [])]),
        Script::Wancho => alphabet_match!([(Alphabet::Wancho, [])]),
        Script::WarangCiti => alphabet_match!([(Alphabet::HoWarangCiti, [])]),
        Script::Yezidi => alphabet_match!([(Alphabet::KurdishYezidi, [])]),
        Script::Yi => alphabet_match!([(Alphabet::Yi, [])]),
        Script::ZanabazarSquare => alphabet_match!([
            (Alphabet::MongolianZanabazarSquare, []),
            (Alphabet::SanskritZanabazarSquare, []),
            (Alphabet::TibetanPhagsPa, []),
        ]),
    }
}
