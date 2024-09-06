use super::{Language, Script};
use alphabet_match_macro::alphabet_match;

/// add all the leters of all the alphabets in the script group
/// or not all, only if it does not require to exclude letters
/// if the script group has only one language, then leave it empty
pub(crate) fn script_lang_alphabets(a: Script, ch: char) -> &'static [Language] {
    match a {
        Script::Adlam => alphabet_match!([(Language::Fulani, []), (Language::Pular, [])]),
        Script::Ahom => alphabet_match!([(Language::Ahom, [])]),
        Script::Anatolian_Hieroglyphs => alphabet_match!([(Language::Luwian, [])]),
        Script::Arabic => alphabet_match!([
            (Language::Arabic, []),
            (Language::Kurdish, []),
            (Language::Pashto, []),
            (Language::Persian, []),
            (Language::Sindhi, []),
            (Language::Urdu, []),
            (Language::Uyghur, []),
        ]),
        Script::Armenian => alphabet_match!([(Language::Armenian, [])]),
        Script::Avestan => alphabet_match!([(Language::Avestan, [])]),
        Script::Balinese => alphabet_match!([(Language::Balinese, [])]),
        Script::Bamum => alphabet_match!([(Language::Bamum, [])]),
        Script::Bassa_Vah => alphabet_match!([(Language::Bassa, [])]),
        // Script::Batak => [Language::Batak Toba, Language::Batak Karo, Language::Batak Mandailing, Language::Batak Pakpak, Language::Batak Simalungun, Language::Batak Angkola],
        Script::Bengali => alphabet_match!([
            (Language::Assamese, []),
            (Language::Bengali, []),
            (Language::BishnupriyaManipuri, []),
        ]),
        Script::Bhaiksuki => alphabet_match!([(Language::Bhaiksuki, [])]),
        Script::Bopomofo => alphabet_match!([(Language::MandarinChinese, [])]),
        Script::Brahmi => alphabet_match!([(Language::Sanskrit, []), (Language::Prakrit, [])]),
        // Script::Braille => [(Language::Any language adapted to Braille,[])],
        Script::Buginese => {
            alphabet_match!([(Language::Buginese, []), (Language::Makassarese, [])])
        }
        Script::Buhid => alphabet_match!([(Language::Buhid, [])]),
        // Script::Canadian_Aboriginal => [(Language::Cree,[]),(Language::Inuktitut,[]),(Language::Ojibwe,[])],
        Script::Carian => alphabet_match!([(Language::Carian, [])]),
        Script::Caucasian_Albanian => alphabet_match!([(Language::CaucasianAlbanian, [])]),
        Script::Chakma => alphabet_match!([(Language::Chakma, [])]),
        Script::Cham => alphabet_match!([(Language::Cham, [])]),
        Script::Cherokee => alphabet_match!([(Language::Cherokee, [])]),
        Script::Chorasmian => alphabet_match!([(Language::Chorasmian, [])]),
        // if you want to add something here, validate that char's range is active in `ucd.rs`
        // during parsing these considered as connectors, not chars of the word
        // example1: `can't` for english is one word, for other lang it is two words: `can, t`
        //   because if Languages of all 3 chars do not intersect, it will be two words
        // example2: `word1' word2` for all langs will be parsed as two words without `'`,
        //   because next char after `'` is space, which is not a char of any language
        Script::Common => match ch {
            '\'' => &[Language::Belarusian, Language::English],
            '¡' => &[Language::Spanish],
            '¿' => &[Language::Spanish],
            _ => &[], // must be always empty
        },
        Script::Coptic => alphabet_match!([(Language::Coptic, [])]),
        Script::Cuneiform => alphabet_match!([
            (Language::Akkadian, []),
            (Language::Hittite, []),
            (Language::Sumerian, []),
        ]),
        Script::Cypriot => alphabet_match!([(Language::AncientGreek, [])]),
        // Script::Cypro_Minoan => [(Language::Unknown (used in ancient Cyprus),[])],
        Script::Cyrillic => alphabet_match!([
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
                Language::Mongolian,
                [
                    'А', 'а', 'Б', 'б', 'В', 'в', 'Г', 'г', 'Д', 'д', 'Е', 'е', 'Ё', 'ё', 'Ж', 'ж',
                    'З', 'з', 'И', 'и', 'Й', 'й', 'К', 'к', 'Л', 'л', 'М', 'м', 'Н', 'н', 'О', 'о',
                    'Ө', 'ө', 'П', 'п', 'Р', 'р', 'С', 'с', 'Т', 'т', 'У', 'у', 'Ү', 'ү', 'Ф', 'ф',
                    'Х', 'х', 'Ц', 'ц', 'Ч', 'ч', 'Ш', 'ш', 'Щ', 'щ', 'Ъ', 'ъ', 'Ы', 'ы', 'Ь', 'ь',
                    'Э', 'э', 'Ю', 'ю', 'Я', 'я',
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
        Script::Deseret => alphabet_match!([(Language::EnglishMormon, [])]),
        Script::Devanagari => alphabet_match!([
            (Language::Hindi, []),
            (Language::Marathi, []),
            (Language::Nepali, []),
            (Language::Sanskrit, []),
        ]),
        Script::Dives_Akuru => alphabet_match!([(Language::MaldivianDhivehi, [])]),
        Script::Dogra => alphabet_match!([(Language::Dogri, [])]),
        // Script::Duployan => [Language::Shorthand systems for English, Language::French],
        Script::Egyptian_Hieroglyphs => alphabet_match!([(Language::EgyptianAncient, [])]),
        Script::Elbasan => alphabet_match!([(Language::AlbanianHistorical, [])]),
        Script::Elymaic => alphabet_match!([(Language::Elymaic, [])]),
        Script::Ethiopic => alphabet_match!([
            (Language::Amharic, []),
            (Language::Geez, []),
            (Language::Oromo, []),
            (Language::Tigrinya, []),
        ]),
        Script::Garay => alphabet_match!([(Language::Wolof, [])]),
        Script::Georgian => alphabet_match!([(Language::Georgian, [])]),
        Script::Glagolitic => alphabet_match!([(Language::OldChurchSlavonic, [])]),
        Script::Gothic => alphabet_match!([(Language::Gothic, [])]),
        Script::Grantha => alphabet_match!([(Language::Sanskrit, []), (Language::Tamil, [])]),
        Script::Greek => alphabet_match!([(Language::Greek, [])]),
        Script::Gujarati => alphabet_match!([(Language::Gujarati, [])]),
        Script::Gunjala_Gondi => alphabet_match!([(Language::Gondi, [])]),
        Script::Gurmukhi => alphabet_match!([(Language::Punjabi, [])]),
        Script::Gurung_Khema => alphabet_match!([(Language::Gurung, [])]),
        Script::Han => alphabet_match!([
            (Language::Chinese, []),
            (
                Language::Japanese, // Kanji also uses all Traditional Chinese characters
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
            (Language::Korean, []), // Hanja
        ]),
        Script::Hangul => alphabet_match!([(Language::Korean, [])]),
        Script::Hanifi_Rohingya => alphabet_match!([(Language::Rohingya, [])]),
        Script::Hanunoo => alphabet_match!([(Language::Hanunoo, [])]),
        Script::Hatran => alphabet_match!([(Language::HatranAramaic, [])]),
        Script::Hebrew => alphabet_match!([(Language::Hebrew, []), (Language::Yiddish, [])]),
        Script::Hiragana => alphabet_match!([(Language::Japanese, [])]),
        Script::Imperial_Aramaic => alphabet_match!([(Language::Aramaic, [])]),
        Script::Inscriptional_Pahlavi => alphabet_match!([(Language::MiddlePersian, [])]),
        Script::Inscriptional_Parthian => alphabet_match!([(Language::Parthian, [])]),
        Script::Javanese => alphabet_match!([(Language::Javanese, [])]),
        Script::Kaithi => alphabet_match!([
            (Language::Bhojpuri, []),
            (Language::Magahi, []),
            (Language::Maithili, []),
        ]),
        Script::Kannada => alphabet_match!([(Language::Kannada, [])]),
        Script::Katakana => alphabet_match!([(Language::Japanese, [])]),
        Script::Kawi => alphabet_match!([(Language::OldJavanese, []), (Language::Sanskrit, [])]),
        Script::Kayah_Li => alphabet_match!([(Language::KayahLi, [])]),
        Script::Kharoshthi => alphabet_match!([(Language::Gandhari, [])]),
        Script::Khitan_Small_Script => alphabet_match!([(Language::Khitan, [])]),
        Script::Khmer => alphabet_match!([(Language::Khmer, [])]),
        Script::Khojki => alphabet_match!([(Language::Sindhi, []), (Language::Khoja, [])]),
        Script::Khudawadi => alphabet_match!([(Language::Sindhi, [])]),
        Script::Kirat_Rai => alphabet_match!([(Language::KiratRai, [])]),
        Script::Lao => alphabet_match!([(Language::Lao, [])]),
        Script::Latin => alphabet_match!([
            (
                Language::Afrikaans,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ç', 'ç', 'É', 'é', 'Ë', 'ë', 'Ï', 'ï', 'Ô', 'ô'
                ]
            ),
            (
                Language::Albanian,
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
                Language::Azerbaijani,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'Ə', 'ə', 'F', 'f',
                    'G', 'g', 'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'Q', 'q',
                    'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's',
                    'Ş', 'ş', 'T', 't', 'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
            ),
            (
                Language::Basque,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'T', 't', 'U', 'u', 'X', 'x', 'Z', 'z', 'Ç', 'ç', 'Ñ', 'ñ'
                ]
            ),
            (
                Language::Bokmal,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å', 'Ç', 'ç', 'É', 'é'
                ]
            ),
            (
                Language::Bosnian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž', 'Ç', 'ç',
                ]
            ),
            (
                Language::Catalan,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'X', 'x', 'Y', 'y',
                    'Z', 'z', 'Ç', 'ç', 'Ï', 'ï', 'Ë', 'ë', 'Ü', 'ü', 'À', 'à', 'È', 'è', 'Ò', 'ò'
                ]
            ),
            (
                Language::Croatian,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Č', 'č', 'Ć', 'ć', 'D', 'd', 'Đ', 'đ', 'E', 'e',
                    'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l',
                    /* 'Lj', 'lj', */ 'M', 'm', 'N', 'n', /* 'Nj', 'nj', */ 'O', 'o',
                    'P', 'p', 'R', 'r', 'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Z', 'z',
                    'Ž', 'ž', 'Ç', 'ç',
                ]
            ),
            (
                Language::Czech,
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
                Language::Danish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Æ', 'æ', 'Ø', 'ø', 'Å', 'å', 'Ç', 'ç', 'É', 'é', 'Ê', 'ê',
                    'Ô', 'ô'
                ]
            ),
            (
                Language::Dutch,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z', 'Ç', 'ç', 'É', 'é', 'Ï', 'ï', 'Ü', 'ü', 'Ë', 'ë'
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
                Language::Esperanto,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ĉ', 'ĉ', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ĝ', 'ĝ', 'H', 'h', 'Ĥ', 'ĥ', 'I', 'i', 'J', 'j', 'Ĵ', 'ĵ', 'K', 'k', 'L', 'l',
                    'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's', 'Ŝ', 'ŝ', 'T', 't',
                    'U', 'u', 'Ŭ', 'ŭ', 'V', 'v', 'Z', 'z',
                ]
            ),
            (
                Language::Estonian,
                [
                    'A', 'a', 'B', 'b', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i',
                    'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r',
                    'S', 's', 'Š', 'š', 'T', 't', 'U', 'u', 'V', 'v', 'Õ', 'õ', 'Ä', 'ä', 'Ö', 'ö',
                    'Ü', 'ü', 'Z', 'z', 'Ž', 'ž',
                ]
            ),
            (
                Language::Finnish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y',
                    'Z', 'z', 'Ä', 'ä', 'Ö', 'ö', 'Š', 'š', 'Ž', 'ž'
                ]
            ),
            (
                Language::French,
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
                Language::Ganda,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'W', 'w', 'Y', 'y', 'É', 'é', 'È', 'è'
                ]
            ),
            (
                Language::German,
                [
                    'A', 'a', 'Ä', 'ä', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'H', 'h', 'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o',
                    'Ö', 'ö', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'Ü', 'ü',
                    'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z', 'ẞ', 'ß'
                ]
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
                Language::Lithuanian,
                [
                    'A', 'a', 'Ą', 'ą', 'B', 'b', 'C', 'c', 'Č', 'č', 'D', 'd', 'E', 'e', 'Ę', 'ę',
                    'Ė', 'ė', 'F', 'f', 'G', 'g', 'H', 'h', 'I', 'i', 'Į', 'į', 'Y', 'y', 'J', 'j',
                    'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p', 'R', 'r', 'S', 's',
                    'Š', 'š', 'T', 't', 'U', 'u', 'Ų', 'ų', 'Ū', 'ū', 'V', 'v', 'Z', 'z', 'Ž', 'ž',
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
                Language::Maori,
                [
                    'A', 'a', 'E', 'e', 'H', 'h', 'I', 'i', 'K', 'k', 'M', 'm', 'N', 'n', 'O', 'o',
                    'P', 'p', 'R', 'r', 'T', 't', 'U', 'u', 'W', 'w', /* 'Ng', 'ng', */ 'G',
                    'g', /* 'Wh', 'wh' */ 'Ā', 'ā', 'Ē', 'ē', 'Ī', 'ī', 'Ō', 'ō', 'Ū', 'ū'
                ]
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
                Language::Romanian,
                [
                    'A', 'a', 'Ă', 'ă', 'Â', 'â', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f',
                    'G', 'g', 'H', 'h', 'I', 'i', 'Î', 'î', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'P', 'p', 'Q', 'q', 'R', 'r', 'S', 's', 'Ș', 'ș', 'T', 't',
                    'Ț', 'ț', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x', 'Y', 'y', 'Z', 'z',
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
                Language::Sotho,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'Q', 'q', 'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'X', 'x',
                    'Y', 'y', 'Z', 'z'
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
                Language::Swahili,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g', 'H', 'h',
                    'I', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm', 'N', 'n', 'O', 'o', 'P', 'p',
                    'R', 'r', 'S', 's', 'T', 't', 'U', 'u', 'V', 'v', 'W', 'w', 'Y', 'y', 'Z', 'z'
                ]
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
                Language::Turkish,
                [
                    'A', 'a', 'B', 'b', 'C', 'c', 'Ç', 'ç', 'D', 'd', 'E', 'e', 'F', 'f', 'G', 'g',
                    'Ğ', 'ğ', 'H', 'h', 'I', 'ı', 'İ', 'i', 'J', 'j', 'K', 'k', 'L', 'l', 'M', 'm',
                    'N', 'n', 'O', 'o', 'Ö', 'ö', 'P', 'p', 'R', 'r', 'S', 's', 'Ş', 'ş', 'T', 't',
                    'U', 'u', 'Ü', 'ü', 'V', 'v', 'Y', 'y', 'Z', 'z',
                ]
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
        Script::Lepcha => alphabet_match!([(Language::Lepcha, [])]),
        Script::Limbu => alphabet_match!([(Language::Limbu, [])]),
        // Script::Linear_A => [(Language::Unknown (Minoan civilization),[])],
        Script::Linear_B => alphabet_match!([(Language::MycenaeanGreek, [])]),
        Script::Lisu => alphabet_match!([(Language::Lisu, [])]),
        Script::Lycian => alphabet_match!([(Language::Lycian, [])]),
        Script::Lydian => alphabet_match!([(Language::Lydian, [])]),
        Script::Mahajani => alphabet_match!([
            (Language::Hindi, []),
            (Language::Marwari, []),
            (Language::Punjabi, []),
        ]),
        Script::Makasar => alphabet_match!([(Language::Makasar, [])]),
        Script::Malayalam => alphabet_match!([(Language::Malayalam, [])]),
        Script::Mandaic => alphabet_match!([(Language::Mandaic, []), (Language::Aramaic, [])]),
        Script::Manichaean => {
            alphabet_match!([(Language::MiddlePersian, []), (Language::Sogdian, [])])
        }
        Script::Marchen => alphabet_match!([(Language::MarchenBuddhist, [])]),
        Script::Masaram_Gondi => alphabet_match!([(Language::Gondi, [])]),
        Script::Medefaidrin => alphabet_match!([(Language::Medefaidrin, [])]),
        Script::Meetei_Mayek => alphabet_match!([(Language::ManipuriMeetei, [])]),
        Script::Mende_Kikakui => alphabet_match!([(Language::Mende, [])]),
        Script::Meroitic_Cursive => alphabet_match!([(Language::Meroitic, [])]),
        Script::Meroitic_Hieroglyphs => alphabet_match!([(Language::Meroitic, [])]),
        Script::Miao => alphabet_match!([(Language::HmongMiao, [])]),
        Script::Modi => alphabet_match!([(Language::Marathi, [])]),
        Script::Mongolian => alphabet_match!([(Language::Mongolian, [])]),
        Script::Mro => alphabet_match!([(Language::Mro, [])]),
        Script::Multani => alphabet_match!([(Language::Saraiki, [])]),
        Script::Myanmar => alphabet_match!([(Language::Burmese, []), (Language::Shan, [])]),
        Script::Nabataean => alphabet_match!([(Language::NabataeanAramaic, [])]),
        Script::Nag_Mundari => alphabet_match!([(Language::Mundari, [])]),
        Script::Nandinagari => alphabet_match!([(Language::Sanskrit, [])]),
        Script::New_Tai_Lue => alphabet_match!([(Language::TaiLue, [])]),
        Script::Newa => alphabet_match!([(Language::Newari, [])]),
        Script::Nko => alphabet_match!([(Language::NKoMandé, [])]),
        Script::Nushu => alphabet_match!([(Language::NushuChina, [])]),
        Script::Nyiakeng_Puachue_Hmong => alphabet_match!([(Language::Hmong, [])]),
        Script::Ogham => alphabet_match!([(Language::OldIrish, [])]),
        Script::Ol_Chiki => alphabet_match!([(Language::Santali, [])]),
        Script::Ol_Onal => alphabet_match!([(Language::Ho, [])]),
        Script::Old_Hungarian => alphabet_match!([(Language::HungarianOld, [])]),
        Script::Old_Italic => alphabet_match!([
            (Language::Etruscan, []),
            (Language::Oscan, []),
            (Language::Umbrian, []),
        ]),
        Script::Old_North_Arabian => alphabet_match!([(Language::OldNorthArabian, [])]),
        Script::Old_Permic => alphabet_match!([(Language::Komi, [])]),
        Script::Old_Persian => alphabet_match!([(Language::OldPersian, [])]),
        Script::Old_Sogdian => alphabet_match!([(Language::Sogdian, [])]),
        Script::Old_South_Arabian => alphabet_match!([(Language::OldSouthArabian, [])]),
        Script::Old_Turkic => alphabet_match!([(Language::OldTurkic, [])]),
        Script::Old_Uyghur => alphabet_match!([(Language::OldUyghur, [])]),
        Script::Oriya => alphabet_match!([(Language::OriyaOdia, [])]),
        Script::Osage => alphabet_match!([(Language::Osage, [])]),
        Script::Osmanya => alphabet_match!([(Language::Somali, [])]),
        Script::Pahawh_Hmong => alphabet_match!([(Language::Hmong, [])]),
        Script::Palmyrene => alphabet_match!([(Language::PalmyreneAramaic, [])]),
        Script::Pau_Cin_Hau => alphabet_match!([(Language::PauCinHauChin, [])]),
        Script::Phags_Pa => alphabet_match!([(Language::Mongolian, []), (Language::Tibetan, [])]),
        Script::Phoenician => alphabet_match!([(Language::Phoenician, [])]),
        Script::Psalter_Pahlavi => alphabet_match!([(Language::MiddlePersian, [])]),
        Script::Rejang => alphabet_match!([(Language::Rejang, [])]),
        Script::Runic => alphabet_match!([(Language::OldNorse, []), (Language::OldEnglish, [])]),
        Script::Samaritan => alphabet_match!([(Language::SamaritanHebrew, [])]),
        Script::Saurashtra => alphabet_match!([(Language::Saurashtra, [])]),
        Script::Sharada => alphabet_match!([(Language::Sanskrit, []), (Language::Kashmiri, [])]),
        Script::Shavian => alphabet_match!([(Language::EnglishPhonetic, [])]),
        Script::Siddham => alphabet_match!([(Language::Sanskrit, [])]),
        Script::SignWriting => alphabet_match!([(Language::Signlanguages, [])]),
        Script::Sinhala => alphabet_match!([(Language::Sinhala, [])]),
        Script::Sogdian => alphabet_match!([(Language::Sogdian, [])]),
        Script::Sora_Sompeng => alphabet_match!([(Language::Sora, [])]),
        Script::Soyombo => alphabet_match!([
            (Language::Mongolian, []),
            (Language::Sanskrit, []),
            (Language::Tibetan, []),
        ]),
        Script::Sundanese => alphabet_match!([(Language::Sundanese, [])]),
        Script::Sunuwar => alphabet_match!([(Language::Sunuwar, [])]),
        Script::Syloti_Nagri => alphabet_match!([(Language::Sylheti, [])]),
        Script::Syriac => alphabet_match!([(Language::Syriac, []), (Language::Aramaic, [])]),
        Script::Tagalog => alphabet_match!([(Language::Tagalog, [])]),
        Script::Tagbanwa => alphabet_match!([(Language::Tagbanwa, [])]),
        Script::Tai_Le => alphabet_match!([(Language::TaiLe, [])]),
        Script::Tai_Tham => alphabet_match!([
            (Language::Lao, []),
            (Language::NorthernThai, []),
            (Language::TaiLue, []),
        ]),
        Script::Tai_Viet => alphabet_match!([(Language::TaiDam, []), (Language::TaiDón, [])]),
        Script::Takri => alphabet_match!([(Language::Dogri, []), (Language::Kashmiri, [])]),
        Script::Tamil => alphabet_match!([(Language::Tamil, [])]),
        Script::Tangsa => alphabet_match!([(Language::Tangsa, [])]),
        Script::Tangut => alphabet_match!([(Language::Tangut, [])]),
        Script::Telugu => alphabet_match!([(Language::Telugu, [])]),
        Script::Thaana => alphabet_match!([(Language::MaldivianDhivehi, [])]),
        Script::Thai => alphabet_match!([(Language::Thai, [])]),
        Script::Tibetan => alphabet_match!([(Language::Tibetan, [])]),
        Script::Tifinagh => alphabet_match!([(Language::Berber, [])]),
        Script::Tirhuta => alphabet_match!([(Language::Maithili, [])]),
        Script::Todhri => alphabet_match!([(Language::AlbanianHistorical, [])]),
        Script::Toto => alphabet_match!([(Language::Toto, [])]),
        Script::Tulu_Tigalari => alphabet_match!([(Language::Sanskrit, []), (Language::Tulu, [])]),
        Script::Ugaritic => alphabet_match!([(Language::Ugaritic, [])]),
        Script::Vai => alphabet_match!([(Language::Vai, [])]),
        Script::Vithkuqi => alphabet_match!([(Language::Albanian, [])]),
        Script::Wancho => alphabet_match!([(Language::Wancho, [])]),
        Script::Warang_Citi => alphabet_match!([(Language::Ho, [])]),
        Script::Yezidi => alphabet_match!([(Language::KurdishYazidi, [])]),
        Script::Yi => alphabet_match!([(Language::Yi, [])]),
        Script::Zanabazar_Square => alphabet_match!([
            (Language::Mongolian, []),
            (Language::Sanskrit, []),
            (Language::Tibetan, []),
        ]),
    }
}
