use fixed_map::Map;
use std::ops::Range;

use crate::{
    lang::{
        alphabet::{char_combine, script_char_to_langs},
        Script,
    },
    Language,
};

pub struct WordIterator<I: Iterator<Item = (Option<Script>, usize, char)>> {
    iter: I,
    next_char: Option<(Option<Script>, usize, char)>,
    word_buf: Vec<char>,
    word_start_index: usize,
    not_saved_word_end_index: usize,
    prev_char_script: Script,
    // prev_char_langs: Set<Language>,
    word_langs_cnt: Map<Language, usize>,
    word_common_langs_cnt: Map<Language, usize>,
    res: Option<WordData>,
}

/* impl<CT: Iterator<Item = (usize, char)>, I: Iterator<Item = (Option<Script>, usize, char)>> From<T>
    for WordIterator<I>
{ */
pub fn from_ch_iter(
    ch_iter: impl Iterator<Item = (usize, char)>,
) -> WordIterator<impl Iterator<Item = (Option<Script>, usize, char)>> {
    let mut iter = ch_iter
        .map(|(ch_idx, ch)| (Script::find(ch), ch_idx, ch))
        // .filter(|(s, _, _)| s != &Some(Script::Inherited))
        /* .map(|(scr, ch_idx, ch)| {
            (
                scr.map(|s| script_char_to_alphabets(s, ch))
                    .unwrap_or_default(),
                scr.unwrap_or(Script::Common),
                ch_idx,
                ch,
            )
        }) */
        .chain([(None, usize::MAX - 1, '\0')]);
    let next_char: Option<(Option<Script>, usize, char)> = iter.next();

    WordIterator {
        iter,
        next_char,
        word_buf: Default::default(),
        word_start_index: Default::default(),
        not_saved_word_end_index: Default::default(),
        prev_char_script: Script::Common,
        // prev_char_langs: Default::default(),
        word_langs_cnt: Default::default(),
        word_common_langs_cnt: Default::default(),
        res: None,
    }
}

#[derive(Debug)]
pub struct WordData {
    pub chars: Vec<char>,
    pub langs_cnt: Map<Language, usize>,
    pub range: Range<usize>,
}

impl<I: Iterator<Item = (Option<Script>, usize, char)>> Iterator for WordIterator<I> {
    type Item = WordData;

    fn next(&mut self) -> Option<Self::Item> {
        while self.res.is_none() {
            let Some((script, mut ch_idx, mut ch)) = self.next_char.take() else {
                break;
            };
            self.next_char = self.iter.next();

            if script == Some(Script::Inherited) {
                continue;
            }
            if let Some((Some(Script::Inherited), i, c)) = self.next_char {
                ch = char_combine(script, ch, c);
                ch_idx = i;
                self.next_char = self.iter.next();
            }
            if ch == '’' {
                ch = '\'';
            }

            let langs = script
                .map(|s| script_char_to_langs(s, ch))
                .unwrap_or_default();

            /* let langs: Set<Language> = script_alphabets
            .iter()
            .map(|a| <&[Language]>::from(a))
            .flatten()
            .copied()
            .collect(); */

            let script = script.unwrap_or(Script::Common); // why Common, maybe skip?

            let langs_not_intersect = if self.prev_char_script != script {
                !langs.iter().any(|&l| self.word_langs_cnt.contains_key(l))
            } else {
                false
            };
            // && !self.prev_char_langs.is_empty()
            // && self.prev_char_langs.intersection(&langs).next().is_none();

            let ch_skip = if langs.is_empty() {
                true
            } else if script == Script::Common {
                if self.prev_char_script == Script::Common || langs_not_intersect {
                    true
                } else if let Some((next_char_script, _, _)) = self.next_char {
                    next_char_script.is_none() || next_char_script == Some(Script::Common)
                } else {
                    true
                }
            } else {
                false
            };

            if ch_skip || langs_not_intersect {
                if !self.word_buf.is_empty() {
                    // saves word
                    /* if let Some(w) = words.get_mut(&self.word_buf) {
                        w.text_indexes
                            .push((self.word_start_index, self.not_saved_word_end_index));
                    } else {
                        let alphabets_count = if process_langs {
                            let ac = process_alphabets_count(self.word_alphabets);
                            if ac.is_empty() {
                                println!("empty: {}", word_buf);
                            }
                            ac
                        } else {
                            Default::default()
                        };
                        let word_data = WordData {
                            alphabets_count,
                            text_indexes: vec![(
                                self.word_start_index,
                                self.not_saved_word_end_index,
                            )],
                        };
                        words.insert(self.word_buf, word_data);
                    } */
                    for (lang, cnt) in std::mem::take(&mut self.word_common_langs_cnt) {
                        let v = self.word_langs_cnt.entry(lang).or_default();
                        *v = v.wrapping_add(cnt);
                    }

                    self.res = Some(WordData {
                        chars: std::mem::take(&mut self.word_buf),
                        langs_cnt: std::mem::take(&mut self.word_langs_cnt),
                        range: self.word_start_index..self.not_saved_word_end_index,
                    });

                    // reset temp variables
                    // self.word_buf = Default::default();
                    // word_alphabets_count = Default::default();
                    // self.word_alphabets = Default::default();
                }
                self.word_start_index = ch_idx + ch.len_utf8();
            }

            if !ch_skip {
                self.not_saved_word_end_index = ch_idx + ch.len_utf8();
                self.word_buf.push(ch.to_lowercase().next().unwrap()); // maybe check each char?
                let langs_cnt = if script == Script::Common {
                    &mut self.word_common_langs_cnt
                } else {
                    &mut self.word_langs_cnt
                };
                for &lang in langs {
                    let v = langs_cnt.entry(lang).or_default();
                    *v = v.wrapping_add(1);
                }
            }
            self.prev_char_script = script;
            // self.prev_char_langs = langs;
        }

        self.res.take()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ahash::AHashSet;
    use rstest::*;

    #[rstest(text, expected_words,
        case("word", ahashset!("word")),
        case("worda wordb", ahashset!("worda", "wordb")),
        case("worda'", ahashset!("worda")),
        case("'worda", ahashset!("worda")),
        case("'worda'", ahashset!("worda")),
        case("''worda''", ahashset!("worda")),
        case("can't", ahashset!("can't")),
        case("worda' wordb", ahashset!("worda", "wordb")),
        case("worda 'wordb", ahashset!("worda", "wordb")),
        case("'worda', 'wordb'", ahashset!("worda", "wordb")),
        case("ПроSto", ahashset!("про", "sto")),
        case::chinese("中文", ahashset!("中文")),
        case("worda 🙈", ahashset!("worda")),
        case::kanji("昨日、東京で大切な友達に会いました。", ahashset!("昨日", "東京で大切な友達に会いました")),
        case("this is a sentence", ahashset!("this", "is", "a", "sentence")),
        case("I can't do this", ahashset!("i", "can't", "do", "this")),
        case(
            "上海大学是一个好大学 this is a sentence",
            ahashset!("上海大学是一个好大学", "this", "is", "a", "sentence")
        ),
        case(
            "Weltweit    gibt es ungefähr 6.000 Sprachen.",
            ahashset!("weltweit", "gibt", "es", "ungefähr", "sprachen")
        ),
        case("This,is ok", ahashset!("this", "is", "ok")),
        case::chinese2("中,文", ahashset!("中", "文")),
        case::chinese3("和little", ahashset!("和", "little")),
        case(
            "Thi̇s is one word", // This = THİS with lowered İ
            ahashset!("this", "is", "one", "word")
        ),
        case("Spanish Ñ two chars", ahashset!("spanish", "ñ", "two", "chars")),
        case("Spanish lowered ñ two chars", ahashset!("spanish", "lowered", "ñ", "two", "chars")),
        // case::japanese("ㄹ語幹に付く態転換接尾辞に", ahashset!("ㄹ", "語幹に付く態転換接尾辞に")),
        // case::japanese2("ㅈ語幹用言に付く場合には", ahashset!("ㅈ", "語幹用言に付く場合には")),
        // case::japanese3("現代朝鮮語にも存在する上昇二重母音ㅑ", ahashset!("現代朝鮮語にも存在する上昇二重母音", "ㅑ")),
    )]
    fn test_filter_text_to_words(text: &str, expected_words: AHashSet<&str>) {
        let found_words: Vec<_> = from_ch_iter(text.char_indices())
            .map(|wd| wd.chars.into_iter().collect::<String>())
            .collect();
        let words: AHashSet<&str> = found_words.iter().map(|w| w.as_str()).collect();

        assert_eq!(
            words, expected_words,
            "text: {}\nwords: {:?}\nexpected: {:?}",
            text, words, expected_words
        );
    }
}
