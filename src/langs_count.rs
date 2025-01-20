use crate::{lang::Script, Language};
use ahash::AHashMap;
// use ::std::collections::hash_map::Entry;
use fixed_map::Map;

pub(crate) fn process_langs_count(
    script_langs: AHashMap<Script, Map<Language, usize>>,
) -> Map<Language, usize> {
    if script_langs.is_empty() {
        return Default::default();
    }

    let mut script_langs_iter = script_langs.into_values();
    let mut langs_count = script_langs_iter.next().unwrap();

    for langs_count_more in script_langs_iter {
        for (lang, cnt) in langs_count_more {
            let v = langs_count.entry(lang).or_default();
            *v = v.wrapping_add(cnt);
        }
    }

    let lang_count_max = langs_count
        .iter()
        .map(|(_, cnt)| cnt)
        .fold(1, |acc, cnt| acc.max(*cnt));

    langs_count.retain(|_, cnt| *cnt == lang_count_max);

    langs_count
}

/* pub(crate) fn process_langs_count(
    script_langs: AHashMap<Script, Map<Language, usize>>,
) -> Map<Language, usize> {
    if script_langs.is_empty() {
        return Default::default();
    }
    let mut script_langs_iter = script_langs.into_values();
    let mut langs_count = script_langs_iter.next().unwrap();

    for mut langs_cnt in script_langs_iter {
        langs_count.retain(|l, count| {
            let Some(cnt) = langs_cnt.remove(l) else {
                return false;
            };

            *count = count.wrapping_add(cnt);

            true
        });
    }

    let lang_count_max = langs_count
        .iter()
        .map(|(_, cnt)| cnt)
        .fold(1, |acc, cnt| acc.max(*cnt));

    langs_count.retain(|_, cnt| {
        *cnt == lang_count_max
    });

    langs_count
} */

/* pub(crate) fn process_alphabets_count<'t>(
    // word_alphabets_count: AHashMap<(Script, Alphabet), usize>,
    word_alphabets: Vec<(Script, ScriptAlphabets)>,
    // word_alphabets_count: Map<Alphabet, usize>,
) -> AHashMap<Language, Vec<Alphabet>> {
    let mut word_alphabets_count: AHashMap<(Script, Alphabet), usize> = AHashMap::new();
    for (script, alphabets) in word_alphabets {
        for alphabet in alphabets.iter() {
            word_alphabets_count
                .entry((script, alphabet))
                .and_modify(|c| *c = c.wrapping_add(1)) // todo rm and_modify
                .or_insert(1);
            /* match cnt_entry {
                Entry::Occupied(cnt_o) => {
                    let cnt = cnt_o.into_mut();
                    *cnt = cnt.wrapping_add(1);
                }
                Entry::Vacant(cnt) => {
                    cnt.insert(1);
                }
            } */
        }
    }

    let mut script_alphabets: AHashMap<Script, AHashMap<Language, Vec<(Alphabet, usize)>>> =
        AHashMap::new();
    for ((s, a), c) in word_alphabets_count {
        let sa_entry = script_alphabets.entry(s).or_default();
        for &l in <&[Language]>::from(a) {
            sa_entry.entry(l).or_default().push((a, c));
        }
    }
    if script_alphabets.is_empty() {
        return Default::default();
    }
    // let script_alphabets_len = script_alphabets.len();

    let mut script_alphabets_iter = script_alphabets.into_values();
    let mut langs_alphabets_count = script_alphabets_iter.next().unwrap();

    for mut langs_alphs_cnt in script_alphabets_iter {
        langs_alphabets_count.retain(|l, alphabets_count| {
            let Some(asc) = langs_alphs_cnt.remove(l) else {
                return false;
            };

            for (alphabet, cnt) in asc {
                if alphabets_count
                    .iter_mut()
                    .find(|(a, _)| *a == alphabet)
                    .map(|(_, c)| *c = c.wrapping_add(cnt))
                    .is_none()
                {
                    alphabets_count.push((alphabet, cnt));
                }
            }

            true
        });
    }

    // if script_alphabets_len == 1 {
    let lang_alphabets_count_max = langs_alphabets_count
        .iter()
        .map(|(_, asc)| asc)
        .flatten()
        .fold(1, |acc, (_, cnt)| acc.max(*cnt));
    // let lang_alphabets_count_half = lang_alphabets_count_max >> 1;

    langs_alphabets_count.retain(|_, asc| {
        // acs.retain(|(cnt, _)| *cnt > lang_alphabets_count_half);
        asc.retain(|(_, cnt)| *cnt == lang_alphabets_count_max);
        !asc.is_empty()
    });

    // langs_alphabets_count.sort_unstable_by(|(_, cnt1), (_, cnt2)| cnt2.cmp(cnt1));
    // }

    langs_alphabets_count
        .into_iter()
        .map(|(l, a)| (l, a.into_iter().map(|a| a.0).collect()))
        .collect()
} */

#[cfg(test)]
mod test {
    use super::*;
    use crate::word_iter;
    use crate::Language::*;
    use ahash::AHashSet;
    use rstest::*;

    #[rstest(
        word,
        expected_language,
        // words with unique characters
        case::czech("subjektů", Some(Czech)),
        case::esperanto("nesufiĉecon", Some(Esperanto)),
        case::esperanto("intermiksiĝis", Some(Esperanto)),
        case::esperanto("monaĥinoj", Some(Esperanto)),
        case::esperanto("kreitaĵoj", Some(Esperanto)),
        case::esperanto("ŝpinante", Some(Esperanto)),
        case::esperanto("apenaŭ", Some(Esperanto)),
        case::german("groß", Some(German)),
        case::greek("σχέδια", Some(Greek)),
        case::hungarian("fekvő", Some(Hungarian)),
        case::hungarian("meggyűrűzni", Some(Hungarian)),
        case::japanese("ヴェダイヤモンド", Some(Japanese)),
        case::kazakh("әлем", Some(Kazakh)),
        case::kazakh("шаруашылығы", Some(Kazakh)),
        case::kazakh("ақын", Some(Kazakh)),
        case::kazakh("оның", Some(Kazakh)),
        case::kazakh("шұрайлы", Some(Kazakh)),
        case::lithuanian("mergelės", Some(Lithuanian)),
        case::lithuanian("įrengus", Some(Lithuanian)),
        case::lithuanian("slegiamų", Some(Lithuanian)),
        case::macedonian("припаѓа", Some(Macedonian)),
        case::macedonian("ќерка", Some(Macedonian)),
        case::polish("państwowych", Some(Polish)),
        case::polish("mniejszości", Some(Polish)),
        case::polish("groźne", Some(Polish)),
        case::romanian("ialomiţa", Some(Romanian)),
        case::serbian("наслеђивања", Some(Serbian)),
        case::serbian("неисквареношћу", Some(Serbian)),
        case::slovak("podĺa", Some(Slovak)),
        case::slovak("pohľade", Some(Slovak)),
        case::slovak("mŕtvych", Some(Slovak)),
        case::ukrainian("ґрунтовому", Some(Ukrainian)),
        case::ukrainian("пропонує", Some(Ukrainian)),
        case::vietnamese("cằm", Some(Vietnamese)),
        case::vietnamese("thần", Some(Vietnamese)),
        case::vietnamese("chẳng", Some(Vietnamese)),
        case::vietnamese("quẩy", Some(Vietnamese)),
        case::vietnamese("sẵn", Some(Vietnamese)),
        case::vietnamese("nhẫn", Some(Vietnamese)),
        case::vietnamese("dắt", Some(Vietnamese)),
        case::vietnamese("chất", Some(Vietnamese)),
        case::vietnamese("đạp", Some(Vietnamese)),
        case::vietnamese("mặn", Some(Vietnamese)),
        case::vietnamese("hậu", Some(Vietnamese)),
        case::vietnamese("hiền", Some(Vietnamese)),
        case::vietnamese("lẻn", Some(Vietnamese)),
        case::vietnamese("biểu", Some(Vietnamese)),
        case::vietnamese("diễm", Some(Vietnamese)),
        case::vietnamese("phế", Some(Vietnamese)),
        case::vietnamese("việc", Some(Vietnamese)),
        case::vietnamese("chỉnh", Some(Vietnamese)),
        case::vietnamese("thơ", Some(Vietnamese)),
        case::vietnamese("nguồn", Some(Vietnamese)),
        case::vietnamese("thờ", Some(Vietnamese)),
        case::vietnamese("sỏi", Some(Vietnamese)),
        case::vietnamese("tổng", Some(Vietnamese)),
        case::vietnamese("nhở", Some(Vietnamese)),
        case::vietnamese("mỗi", Some(Vietnamese)),
        case::vietnamese("bỡi", Some(Vietnamese)),
        case::vietnamese("tốt", Some(Vietnamese)),
        case::vietnamese("giới", Some(Vietnamese)),
        case::vietnamese("một", Some(Vietnamese)),
        case::vietnamese("hợp", Some(Vietnamese)),
        case::vietnamese("hưng", Some(Vietnamese)),
        case::vietnamese("từng", Some(Vietnamese)),
        case::vietnamese("của", Some(Vietnamese)),
        case::vietnamese("sử", Some(Vietnamese)),
        case::vietnamese("những", Some(Vietnamese)),
        case::vietnamese("chức", Some(Vietnamese)),
        case::vietnamese("thực", Some(Vietnamese)),
        case::vietnamese("kỳ", Some(Vietnamese)),
        case::vietnamese("kỷ", Some(Vietnamese)),
        case::vietnamese("mỵ", Some(Vietnamese)),
        case::yoruba("aṣiwèrè", Some(Yoruba)),
        case::yoruba("ṣaaju", Some(Yoruba)),
        case("والموضوع", None),
        case("сопротивление", None),
        case("house", None),

        // words with unique script
        case::armenian("ունենա", Some(Armenian)),
        case::georgian("გარეუბან", Some(Georgian)),
        case::greek("σταμάτησε", Some(Greek)),
        case::gujarati("ઉપકરણોની", Some(Gujarati)),
        case::japanese("びさ", Some(Japanese)),
        case::korean("대결구도가", Some(Korean)),
        case::punjabi_eastern("ਮੋਟਰਸਾਈਕਲਾਂ", Some(PunjabiEastern)),
        case::tamil("துன்பங்களை", Some(Tamil)),
        case::telugu("కృష్ణదేవరాయలు", Some(Telugu)),
        case::thai("ในทางหลวงหมายเลข", Some(Thai)),
    )]
    fn assert_language_detection_with_rules_works_correctly(
        word: &str,
        expected_language: Option<Language>,
    ) {
        let found_words = word_iter::from_ch_iter(word.char_indices());
        let languages: AHashSet<_> = found_words
            .into_iter()
            .map(|w| process_langs_count(w.script_langs))
            .flatten()
            .map(|(l, _)| l)
            .collect();

        let language = if languages.len() > 1 {
            None
        } else {
            languages.iter().next().copied()
        };

        assert_eq!(
            language, expected_language,
            "expected {:?} for word '{}', got {:?}",
            expected_language, word, languages
        );
    }

    #[rstest(word, expected_languages,
        case::czech("rozdělit", ahashset!(Fon, Czech)),
        case::azerbaijani_north("məhərrəm", ahashset!(TatarCrimean, AzerbaijaniNorth)),
        case::czech("tvořen", ahashset!(Silesian, Czech)),
        case::latvian("teoloģiska", ahashset!(Latvian, Latgalian)),
        case::latvian("blaķene", ahashset!(Latvian, Latgalian)),
        case::latvian("ceļojumiem", ahashset!(Latgalian, Latvian)),
        case::latvian("numuriņu", ahashset!(Latgalian, Latvian)),
        case::polish("zmieniły", ahashset!(Silesian, Polish)),
        case::vietnamese("kẽm", ahashset!(Vietnamese, Guarani, Ewe)),
        case::vietnamese("trĩ", ahashset!(Vietnamese, Ewe, Guarani)),
        case::vietnamese("ravị", ahashset!(Vietnamese, Igbo)),
        case::vietnamese("cũng", ahashset!(Guarani, Vietnamese)),
        case::vietnamese("dụng", ahashset!(Vietnamese, Igbo)),
        case::vietnamese("mỹ", ahashset!(Vietnamese, Guarani)),
        case::arab(
            "والموضوع", 
            ahashset!(
                Uyghur, PersianWestern, ArabicSouthLevantine, Arabic, ArabicSouthernYemeni, Persian,
                AzerbaijaniSouth, Kashmiri, KurdishCentral, Dari, KanuriCentral, Kurdish, AcehneseJawi,
                ArabicEgyptian, ArabicMesopotamian, Pashto, Urdu, ArabicMoroccan, BanjarJawi,
                ArabicNorthLevantine, ArabicNajdi, ArabicTunisian, PastoSouthern, Sindhi
            )),
        case::ru1(
            "сопротивление",
            ahashset!(Serbian, Kazakh, Russian, Ukrainian, Bulgarian, Macedonian, MongolianHalh)
        ),
        case::ru2("этот", ahashset!(Belarusian, Kazakh, MongolianHalh, Russian)),
        case::ru3("огнём", ahashset!(Belarusian, Kazakh, MongolianHalh, Russian)),
        case::ukr1("пристрої", ahashset!(Ukrainian, OldChurchSlavonic)),
        case::bel1("раскрывае", ahashset!(Belarusian, Kazakh, MongolianHalh, Russian)),
        case::bel2("павінен", ahashset!(Belarusian, Kazakh, Ukrainian)),
        case::bul1(
            "плаваща",
            ahashset!(Kazakh, Bulgarian, OldChurchSlavonic, Russian, MongolianHalh, Ukrainian)
        ),
        case::bul2("довършат", ahashset!(Russian, Kazakh, Bulgarian, OldChurchSlavonic, MongolianHalh)),
        case::mon1("үндсэн", ahashset!(Kazakh, MongolianHalh)),
        case::mon2("дөхөж", ahashset!(Kazakh, MongolianHalh)),
        case::mac1("затоплување", ahashset!(Macedonian, Serbian)),
        case::mac2("ректасцензија", ahashset!(Macedonian, Serbian)),
        case::mac3("набљудувач", ahashset!(Macedonian, Serbian)),
        case::mac4("џамиите", ahashset!(Macedonian, Serbian)),
        case::mac5("ѕидови", ahashset!(OldChurchSlavonic, Macedonian)),
        case::latv1("aizklātā", ahashset!(Latgalian, Latin, Latvian)),
        case::latv2("sistēmas", ahashset!(Latgalian, Latvian, Latin)),
        case::latv3("palīdzi", ahashset!(Latin, Latvian, Latgalian)),
        case::viet1("nhẹn", ahashset!(Vietnamese, Yoruba)),
        case::viet2("chọn", ahashset!(Vietnamese, Igbo)),
        case::croat1("prihvaćanju", ahashset!(Croatian, Bosnian)),
        case::croat2("nađete", ahashset!(Bosnian, Croatian, Vietnamese)),
        case::port1("visão", ahashset!(Guarani, Portuguese, Ewe, Vietnamese)),
        case::port2(
            "catedráticos",
            ahashset!(
                Spanish, AymaraCentral, Slovak, Afrikaans, Hungarian, Limburgish, Kabuverdianu,
                Asturian, Galician, Occitan, Shona, Fon, Portuguese, Guarani, Vietnamese, Czech,
                Papiamento, Lombard, Igbo, Dutch, Irish
            )
        ),
        case::port3(
            "política",
            ahashset!(
                Spanish, Lombard, Portuguese, Vietnamese, Guarani, Catalan, Afrikaans, Asturian,
                Fon, Igbo, Shona, Irish, Kikuyu, Kabuverdianu, Slovak, Sardinian, Limburgish,
                AymaraCentral, Galician, Kinyarwanda, Hungarian, Czech, Occitan
            )
        ),
        case::port4(
            "música",
            ahashset!(
                Igbo, Shona, Lombard, Czech, Kikuyu, AymaraCentral, Afrikaans, Portuguese,
                Slovak, Hungarian, Kinyarwanda, Spanish, Papiamento, Galician, Kabuverdianu,
                Irish, Limburgish, Guarani, Vietnamese, Catalan, Fon, Sardinian, Asturian, Occitan
            )
        ),
        case::pol1("wystąpią", ahashset!(Polish)),
        case::pol2("budowę", ahashset!(Polish)),
        case::pol3("kradzieżami", ahashset!(Maltese, Polish)),
        case::lith1("nebūsime", ahashset!(Lithuanian, Latgalian, Latin, Latvian)),
        case::rom1("afişate", ahashset!(TatarCrimean, Turkish, KurdishNorthern, AzerbaijaniNorth)),
        case::rom2(
            "înviat",
            ahashset!(Guarani, French, Friulian, Romanian, KurdishNorthern, Limburgish, Afrikaans)
        ),
        case::rom3("pregătire", ahashset!(Romanian, Vietnamese)),
        case::it1(
            "venerdì",
            ahashset!(
                Friulian, Czech, Sicilian, Fon, Limburgish, Venetian, Lombard, Sardinian, Vietnamese, Italian
            )
        ),
        case::es1(
            "años", 
            ahashset!(
                Ilocano, Basque, Waray, Asturian, Spanish, Papiamento, AymaraCentral, Guarani, Wolof, Galician
            )
        ),
        case::slov1("rozohňuje", ahashset!(Silesian, Czech, Slovak)),
        case::cz1("rtuť", ahashset!(Czech, Slovak)),
        case::cz2("jeďte", ahashset!(Slovak, Czech)),
        case::cz3("vývoj", ahashset!(Faroese, Slovak, Afrikaans, Icelandic, Czech, Guarani)),
        case::cz4(
            "zaručen",
            ahashset!(Latgalian, Czech, Slovene, Slovak, Silesian, Latvian, Bosnian, Kabyle, Croatian, Lithuanian)
        ),
        case::cz5(
            "zkouškou", 
            ahashset!(
                Sepedi, Latvian, Estonian, Slovene, Slovak, Bosnian, Latgalian, Silesian, Czech,
                Croatian, Lithuanian
            )
        ),
        case::cz6(
            "navržen",
            ahashset!(
                Croatian, Latgalian, Latvian, Silesian, Slovak, Lithuanian, Slovene, Bosnian, Czech, Estonian
            )
        ),
        case::ic1("minjaverðir", ahashset!(Icelandic, Faroese)),
        case::ic2("þagnarskyldu", ahashset!(Icelandic)),
        case::alb1(
            "hashemidëve",
            ahashset!(Afrikaans, Dutch, Catalan, French, Limburgish, AymaraCentral, Luxembourgish, AlbanianTosk)
        ),
        case::fr1(
            "forêt",
            ahashset!(Sepedi, KurdishNorthern, Kabuverdianu, Welsh, Afrikaans, Portuguese, French, Friulian)
        ),
        case::fr2(
            "succèdent",
            ahashset!(
                GaelicScottish, Acehnese, Papiamento, Czech, Italian, Vietnamese, Sicilian, Sardinian,
                Lombard, Friulian, Dutch, CreoleHaitian, Catalan, Afrikaans, Occitan, Venetian, Fon,
                Limburgish, French
            )
        ),
        case::fr3(
            "où",
            ahashset!(
                Fon, Sardinian, Venetian, Yoruba, Sicilian, GaelicScottish, French, Vietnamese,
                Friulian, Czech, Limburgish, Lombard, Italian
            )
        ),
        case::fr4(
            "contrôle",
            ahashset!(
                Friulian, Slovak, Afrikaans, French, Portuguese, Welsh, Sepedi, Bambara, Limburgish,
                Vietnamese, Acehnese
            )
        ),
        case::fr5(
            "façonnage",
            ahashset!(
                Friulian, KurdishNorthern, Catalan, Bokmal, AlbanianTosk, Portuguese, Venetian,
                Ligurian, AzerbaijaniNorth, Turkmen, Turkish, French, Occitan, Bosnian, Kabuverdianu,
                TatarCrimean
            )
        ),
        case::est1(
            "tõeliseks",
            ahashset!(Vietnamese, Portuguese, Kabuverdianu, Guarani, Estonian)
        ),
        case::est2(
            "päralt",
            ahashset!(
                DinkaSouthwestern, Finnish, Swedish, German, Afrikaans, Turkmen, Luxembourgish,
                AymaraCentral, Estonian, Dutch, Slovak, Limburgish
            )
        ),
        case::dan1("direktør", ahashset!(Faroese, Danish, Nynorsk, Czech, Bokmal)),
        case::dan2("indebærer", ahashset!(Latin, Faroese, Nynorsk, Danish, Bokmal, Icelandic, French)),
        case::dan3("måned", ahashset!(Bokmal, Danish, Nynorsk, Swedish)),
        case::cat1(
            "pràctiques",
            ahashset!(Sardinian, Italian, Portuguese, French, Catalan, Venetian, Vietnamese, Sicilian)
        ),
        case::cat2(
            "contradicció",
            ahashset!(
                Irish, Sicilian, Vietnamese, Luxembourgish, AymaraCentral, Venetian, Sardinian,
                Limburgish, Asturian, Slovak, Galician, Shona, Lombard, Portuguese, Catalan,
                Fon, Spanish, Papiamento, Hungarian, Czech, Igbo, Dutch, Kabuverdianu, Guarani,
                Occitan, Afrikaans, Polish
            )
        ),
        case::cat3(
            "només",
            ahashset!(
                Igbo, Catalan, Bambara, Czech, Sardinian, Danish, Guarani, Venetian, Hungarian,
                Bokmal, Limburgish, Vietnamese, Balinese, Ligurian, Luxembourgish, Shona, Yoruba,
                Acehnese, Portuguese, Lombard, Icelandic, Afrikaans, Ilocano, French, Ewe, Asturian,
                Slovak, Waray, Papiamento, Dutch, Italian, CreoleHaitian, Kabuverdianu, Galician,
                Fon, Occitan, Banjar, Sicilian, AymaraCentral, Spanish, Irish
            )
        ),
        case::cat4("substituïts", ahashset!(Catalan, French, Afrikaans, AymaraCentral, Dutch)),
        case::ge1(
            "überrascht",
            ahashset!(
                Afrikaans, Catalan, French, Luxembourgish, Spanish, Asturian, Portuguese,
                Dutch, Hungarian, Turkish, German, Limburgish, TatarCrimean, Papiamento,
                AzerbaijaniNorth, AymaraCentral
            )
        ),
        case::ge2(
            "höher",
            ahashset!(
                AzerbaijaniNorth, Hungarian, Dutch, Estonian, Swedish, Turkmen, TatarCrimean,
                Limburgish, Luxembourgish, Icelandic, AymaraCentral, Finnish, Afrikaans,
                DinkaSouthwestern, Acehnese, German, Turkish
            )
        ),
        case::en1(
            "house",
            ahashset!(
                TatarCrimean, Latin, Ganda, Kimbundu, Italian, Romanian, Kinyarwanda, Shona, Estonian,
                Tagalog, Tamasheq, UzbekNorthern, Jingpho, Spanish, Luo, Papiamento, Waray, Bambara,
                German, Icelandic, Lithuanian, Mossi, Friulian, Sicilian, Venetian, Kabiye, Indonesian,
                Turkmen, Occitan, Danish, Faroese, Cebuano, Vietnamese, Silesian, CreoleHaitian, Mizo,
                Basque, Luxembourgish, French, Sundanese, Kikongo, OromoWestCentral, Xhosa, Nynorsk,
                Sepedi, AlbanianTosk, Czech, Nyanja, Swati, Zulu, Sesotho, KurdishNorthern, Ewe,
                Hungarian, English, Latvian, Rundi, Acehnese, Minangkabau, Turkish, Irish, Lombard,
                Dyula, Balinese, Lingala, Maltese, KanuriCentral, Slovene, Tsonga,
                Sango, Tumbuka, Igbo, Afrikaans, Catalan, Buginese, Polish, Welsh, Croatian, Tswana,
                FulfuldeNigerian, Wolof, AymaraCentral, Limburgish, AzerbaijaniNorth, Portuguese,
                Finnish, Twi, Kikuyu, Banjar, Bosnian, Kabyle, Guarani, Yoruba, Sardinian, Javanese,
                Fon, Somali, Bokmal, Ligurian, Dutch, GaelicScottish, Nuer, Swahili, Galician, Swedish,
                Latgalian, Umbundu, Slovak, Malay, Kamba, LubaKasai, Asturian, Esperanto, Bemba, Hausa,
                TokPisin, Chokwe, Kabuverdianu
            )
        ),
        case::marat1(
            "मिळते", 
            ahashset!(Maithili, Awadhi, Magahi, Chhattisgarhi, Hindi, Sanskrit, Bhojpuri, Marathi, Nepali, Kashmiri)
        ),
        case::ben1("জানাতে", ahashset!(Assamese, BishnupriyaManipuri, Bengali, Meitei)),
        case::heb1("בתחרויות", ahashset!(YiddishEastern, Yiddish, Hebrew)),
        case("nebûtu", ahashset!(French, Limburgish, Afrikaans, KurdishNorthern, Friulian, Welsh)),
        case(
            "viòiem",
            ahashset!(Fon, Lombard, Friulian, Occitan, Catalan, Vietnamese, CreoleHaitian, Italian, Ewe, Limburgish, Sardinian)
        ),
        case("labâk", ahashset!(Romanian, French, Portuguese, Vietnamese)),
        case("šefčovič's",
            ahashset!(Slovak, Lithuanian, Silesian, Croatian, Latgalian, Czech, Bosnian, Latvian, Slovene)
        ),
    )]
    fn assert_language_filtering_with_rules_works_correctly(
        word: &str,
        expected_languages: AHashSet<Language>,
    ) {
        let found_words = word_iter::from_ch_iter(word.char_indices());
        let languages: AHashSet<_> = found_words
            .into_iter()
            .map(|w| process_langs_count(w.script_langs))
            .flatten()
            .map(|(l, _)| l)
            .collect();

        assert_eq!(
            languages, expected_languages,
            "expected {:?} for word '{}', got {:?}",
            expected_languages, word, languages
        );
    }

    #[rstest(
        text,
        expected_language,
        case::kanji("昨日、東京で大切な友達に会いました。", Japanese), // Kanji (Han) + Hiragana
        case::chinese("也有越來越多的人開始飼養寵物", ChineseSimplified),
    )]
    fn assert_language_detection_with_rules_text_works_correctly(
        text: &str,
        expected_language: Language,
    ) {
        let found_words = word_iter::from_ch_iter(text.char_indices());
        let languages: AHashSet<_> = found_words
            .into_iter()
            .map(|w| process_langs_count(w.script_langs))
            .flatten()
            .map(|(l, _)| l)
            .collect();
        /* let language = if languages.len() > 1 {
            None
        } else {
            languages.into_iter().next()
        }; */

        assert!(
            languages.contains(&expected_language),
            "expected {:?} for text '{}', got {:?}",
            expected_language,
            text,
            languages
        );
    }
}
