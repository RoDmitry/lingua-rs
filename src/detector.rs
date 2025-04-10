use crate::{
    constant::{TOKENS_WITHOUT_WHITESPACE, TOKENS_WITH_OPTIONAL_WHITESPACE},
    json::{load_model, parse_model},
    model::{prepare_ngrams, NgramString},
    result::DetectionResult,
};
use ::core::{
    cmp::Ordering,
    hash::{BuildHasher, Hash},
    ops::Range,
};
use ::std::{
    collections::{HashMap, HashSet},
    sync::RwLock,
};
use ahash::{AHashMap, AHashSet};
use alphabet_detector::{
    fulltext_filter_with_margin, slang_arr_default_nc, Script, ScriptLanguage, ScriptLanguageArr,
};
use debug_unsafe::slice::SliceGetter;
use fraction::Zero;
use itertools::Itertools;
#[cfg(not(target_family = "wasm"))]
use rayon::prelude::*;

pub(crate) const NGRAM_MAX_SIZE: usize = 5;
type LanguageModelNgram = AHashMap<NgramString, f64>;
type LanguageModelNgrams = [LanguageModelNgram; NGRAM_MAX_SIZE];

struct LanguageModel {
    ngrams: LanguageModelNgrams,
    min_probability: f64,
}

impl Default for LanguageModel {
    #[inline]
    fn default() -> Self {
        Self {
            ngrams: Default::default(),
            min_probability: f64::NEG_INFINITY,
        }
    }
}

impl LanguageModel {
    #[inline]
    fn update_ngram(&mut self, ngram_model: LanguageModelNgram, index: usize) {
        if index == 0 {
            self.min_probability = if !ngram_model.is_empty() {
                (1.0 / ngram_model.len() as f64).ln()
            } else {
                f64::NEG_INFINITY
            }
        }
        if let Some(n) = self.ngrams.get_mut(index) {
            *n = ngram_model
        }
    }
}

impl From<LanguageModelNgrams> for LanguageModel {
    #[inline]
    fn from(ngrams: LanguageModelNgrams) -> Self {
        let min_probability = if !ngrams.get_safe_unchecked(0).is_empty() {
            (1.0 / ngrams.get_safe_unchecked(0).len() as f64).ln()
        } else {
            f64::NEG_INFINITY
        };

        Self {
            ngrams,
            min_probability,
        }
    }
}

type LanguagesModels = ScriptLanguageArr<RwLock<LanguageModel>>;
// type LanguagesModelsRef = &'static LanguagesModels;

// static LANGUAGES_MODELS: LazyLock<LanguagesModels> =
// LazyLock::new(|| ::core::array::from_fn(|_| Default::default()));

/// final result
pub struct Word {
    pub chars: Vec<char>,
    pub range: Range<usize>,
    pub most_frequent_script: Script,
    /// Languages listed here are always from the most frequent Alphabet
    pub checked_languages: Vec<(ScriptLanguage, Script, f64)>,
    /// can include most frequent Alphabet, but for different languages
    pub unverified_alphabet_languages: AHashMap<ScriptLanguage, Vec<Script>>,
}

/// This struct detects the language of given input text.
#[cfg_attr(feature = "python", pyo3::prelude::pyclass)]
pub struct LanguageDetector {
    languages_preloaded: AHashSet<ScriptLanguage>,
    is_low_accuracy_mode_enabled: bool,
    languages_models: LanguagesModels,
}

impl LanguageDetector {
    pub(crate) fn new(
        languages_preload: AHashSet<ScriptLanguage>,
        is_low_accuracy_mode_enabled: bool,
    ) -> Self {
        let detector = Self {
            languages_preloaded: languages_preload,
            is_low_accuracy_mode_enabled,
            languages_models: ::core::array::from_fn(|_| Default::default()),
        };

        detector.load_languages_models(&detector.languages_preloaded);

        detector
    }

    fn load_languages_models(&self, languages: &AHashSet<ScriptLanguage>) {
        #[cfg(not(target_family = "wasm"))]
        let languages_iter = languages.par_iter();
        #[cfg(target_family = "wasm")]
        let languages_iter = languages.iter();

        languages_iter.for_each(|&language| {
            self.load_language_model(language, 1);
            self.load_language_model(language, 2);
            self.load_language_model(language, 3);

            if !self.is_low_accuracy_mode_enabled {
                self.load_language_model(language, 4);
                self.load_language_model(language, 5);
            }
        });
    }

    /// Clears all language models loaded by this [`LanguageDetector`] instance
    /// and frees allocated memory previously consumed by the models.
    pub fn unload_language_models(&self) {
        #[cfg(not(target_family = "wasm"))]
        let languages_iter = self.languages_preloaded.par_iter();
        #[cfg(target_family = "wasm")]
        let languages_iter = self.languages.iter();

        languages_iter.for_each(|&language| {
            *self
                .languages_models
                .get_safe_unchecked(language as usize)
                .write()
                .unwrap() = Default::default();
        });
    }

    /// Detects the language of given input text.
    /// If the language cannot be reliably detected, [`None`] is returned.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// [`detect_in_parallel`](#method.detect_in_parallel)
    /// instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let detected_language = detector.detect("languages are awesome");
    ///
    /// assert_eq!(detected_language, Some(English));
    /// ```
    pub fn detect(&self, text: &str, minimum_distance: f64) -> Option<ScriptLanguage> {
        self.detect_with_languages(text, &self.languages_preloaded, minimum_distance)
    }

    /// Detects the languages of all given input texts.
    /// If the language cannot be reliably detected for a text,
    /// [`None`] is put into the result vector.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// If you do not want or need parallel execution, use method
    /// [`detect`](#method.detect) instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let detected_languages = detector.detect_in_parallel(&[
    ///     "languages are awesome",
    ///     "Sprachen sind großartig",
    ///     "des langues sont géniales",
    ///     "los idiomas son geniales"
    /// ]);
    ///
    /// assert_eq!(
    ///     detected_languages,
    ///     vec![
    ///         Some(English),
    ///         Some(German),
    ///         Some(French),
    ///         Some(Spanish)
    ///     ]
    /// );
    /// ```
    #[cfg(not(target_family = "wasm"))]
    pub fn detect_in_parallel(
        &self,
        texts: &[&str],
        minimum_distance: f64,
    ) -> Vec<Option<ScriptLanguage>> {
        texts
            .into_par_iter()
            .map(|text| self.detect(text, minimum_distance))
            .collect()
    }

    pub fn detect_with_languages<S: BuildHasher + Default>(
        &self,
        text: &str,
        languages: &HashSet<ScriptLanguage, S>,
        minimum_distance: f64,
    ) -> Option<ScriptLanguage> {
        debug_assert!(
            (0.0..1.0).contains(&minimum_distance),
            "Minimum relative distance must lie in between 0.0 and 0.99"
        );
        let mut confidence = self
            .compute_confidence_for_languages(text, languages)
            .into_iter();

        let (most_likely_language, most_likely_language_probability) = confidence.next()?;

        let Some((_, second_most_likely_language_probability)) = confidence.next() else {
            return Some(most_likely_language);
        };

        let language_probability_diff =
            (most_likely_language_probability - second_most_likely_language_probability).abs();

        if language_probability_diff < f64::EPSILON || language_probability_diff < minimum_distance
        {
            return None;
        }

        Some(most_likely_language)
    }

    /// Attempts to detect multiple languages in mixed-language text.
    ///
    /// This feature is experimental and under continuous development.
    ///
    /// A vector of [`DetectionResult`] is returned containing an entry for each contiguous
    /// single-language text section as identified by the library. Each entry consists
    /// of the identified language, a start index and an end index. The indices denote
    /// the substring that has been identified as a contiguous single-language text section.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// [`detect_multiple_in_parallel`](#method.detect_multiple_in_parallel)
    /// instead.
    /// ```
    /// use lingua::Language::{English, French, German};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German
    /// ])
    /// .build();
    ///
    /// let sentence = "Parlez-vous français? \
    ///     Ich spreche Französisch nur ein bisschen. \
    ///     A little bit is better than nothing.";
    ///
    /// let results = detector.detect_multiple(sentence);
    ///
    /// if let [first, second, third] = &results[..] {
    ///     assert_eq!(first.language(), French);
    ///     assert_eq!(
    ///         &sentence[first.start_index()..first.end_index()],
    ///         "Parlez-vous français? "
    ///     );
    ///
    ///     assert_eq!(second.language(), German);
    ///     assert_eq!(
    ///         &sentence[second.start_index()..second.end_index()],
    ///         "Ich spreche Französisch nur ein bisschen. "
    ///     );
    ///
    ///     assert_eq!(third.language(), English);
    ///     assert_eq!(
    ///         &sentence[third.start_index()..third.end_index()],
    ///         "A little bit is better than nothing."
    ///     );
    /// }
    /// ```
    pub fn detect_multiple(&self, text_str: &str) -> Vec<DetectionResult> {
        if text_str.is_empty() {
            return vec![];
        }

        let tokens_without_whitespace = TOKENS_WITHOUT_WHITESPACE
            .find_iter(text_str)
            .map(|mat| mat.as_str())
            .collect_vec();

        if tokens_without_whitespace.is_empty() {
            return vec![];
        }

        let mut results = vec![];
        let mut language_counts = AHashMap::new();

        let language = self.detect(text_str, 0.0);
        if let Some(lang) = language {
            Self::increment_counter(&mut language_counts, lang, 1);
        }

        for word in tokens_without_whitespace.iter() {
            if word.chars().count() < 5 {
                continue;
            }
            let language = self.detect(word, 0.0);
            if let Some(lang) = language {
                Self::increment_counter(&mut language_counts, lang, 1);
            }
        }

        let languages = language_counts
            .keys()
            .cloned()
            .collect::<AHashSet<ScriptLanguage>>();

        if languages.len() == 1 {
            let result = DetectionResult {
                start_index: 0,
                end_index: text_str.len(),
                word_count: tokens_without_whitespace.len(),
                language: *languages.iter().next().unwrap(),
            };
            results.push(result);
        } else {
            let mut current_start_index = 0;
            let mut current_end_index = 0;
            let mut word_count = 0;
            let mut current_language = None;

            let last_index = TOKENS_WITH_OPTIONAL_WHITESPACE.find_iter(text_str).count() - 1;
            let token_matches = TOKENS_WITH_OPTIONAL_WHITESPACE.find_iter(text_str);

            for (i, token_match) in token_matches.enumerate() {
                let word = token_match.as_str();
                let language = self.detect_with_languages(word, &languages, 0.0);

                if i == 0 || (current_language.is_none() && language.is_some()) {
                    current_language = language;
                }

                if let Some(lang) = language {
                    if let Some(current_lang) = current_language {
                        if lang != current_lang {
                            let result = DetectionResult {
                                start_index: current_start_index,
                                end_index: current_end_index,
                                word_count,
                                language: current_lang,
                            };
                            results.push(result);
                            current_start_index = current_end_index;
                            current_language = Some(lang);
                            word_count = 0;
                        }
                    }
                }

                current_end_index = token_match.end();
                word_count += 1;

                if i == last_index {
                    if let Some(current_lang) = current_language {
                        let result = DetectionResult {
                            start_index: current_start_index,
                            end_index: current_end_index,
                            word_count,
                            language: current_lang,
                        };
                        results.push(result);
                    }
                }
            }

            if results.len() > 1 {
                let mut mergeable_result_indices = vec![];

                for (i, result) in results.iter().enumerate() {
                    if result.word_count == 1 {
                        mergeable_result_indices.push(i);
                    }
                }

                merge_adjacent_results(&mut results, &mut mergeable_result_indices);

                if results.len() > 1 {
                    mergeable_result_indices.clear();

                    for i in 0..results.len() - 1 {
                        if results[i].language == results[i + 1].language {
                            mergeable_result_indices.push(i + 1);
                        }
                    }

                    merge_adjacent_results(&mut results, &mut mergeable_result_indices);
                }
            }
        }

        results
    }

    /// Attempts to detect multiple languages in mixed-language text.
    ///
    /// This feature is experimental and under continuous development.
    ///
    /// A vector of [`DetectionResult`] is returned for each text containing an
    /// entry for each contiguous single-language text section as identified by
    /// the library. Each entry consists of the identified language, a start index
    /// and an end index. The indices denote the substring that has been identified
    /// as a contiguous single-language text section.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// If you do not want or need parallel execution, use method
    /// [`detect_multiple`](#method.detect_multiple)
    /// instead.
    #[cfg(not(target_family = "wasm"))]
    pub fn detect_multiple_in_parallel(&self, texts: &[&str]) -> Vec<Vec<DetectionResult>> {
        texts
            .into_par_iter()
            .map(|text| self.detect_multiple(text))
            .collect()
    }

    /// Computes confidence values for each language supported by this detector for the given
    /// input text. These values denote how likely it is that the given text has been written
    /// in any of the languages supported by this detector.
    ///
    // TODO: VERIFY
    // A vector of two-element tuples is returned containing those languages which the
    // calling instance of [`LanguageDetector`] has been built from, together with their
    // confidence values. The entries are sorted by their confidence value in descending order.
    // Each value is a probability between 0.0 and 1.0. The probabilities of all languages will
    // sum to 1.0. If the language is unambiguously identified by the rule engine, the value
    // 1.0 will always be returned for this language. The other languages will receive a value
    // of 0.0.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// [`compute_confidence_in_parallel`](#method.compute_confidence_in_parallel)
    /// instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let result = detector
    ///     .compute_confidence("languages are awesome")
    ///     .into_iter()
    ///     .map(|(language, confidence)| (language, (confidence * 100.0).round() / 100.0))
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     result,
    ///     vec![
    ///         (English, 0.93),
    ///         (French, 0.04),
    ///         (German, 0.02),
    ///         (Spanish, 0.01)
    ///     ]
    /// );
    /// ```
    pub fn compute_confidence(&self, text: &str) -> Vec<(ScriptLanguage, f64)> {
        self.compute_confidence_for_languages(text, &self.languages_preloaded)
    }

    /// Computes confidence values for each language supported by this detector for all the given
    /// input texts. The confidence values denote how likely it is that the given text has been written
    /// in any of the languages supported by this detector.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let result = detector
    ///     .compute_confidence_in_parallel(&[
    ///         "languages are awesome",
    ///         "Sprachen sind großartig"
    ///     ])
    ///     .into_iter()
    ///     .map(|vector| {
    ///         vector
    ///             .into_iter()
    ///             .map(|(language, confidence)| {
    ///                 (language, (confidence * 100.0).round() / 100.0)
    ///             })
    ///             .collect::<Vec<_>>()
    ///     })
    ///     .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     result,
    ///     vec![
    ///         vec![
    ///             (English, 0.93),
    ///             (French, 0.04),
    ///             (German, 0.02),
    ///             (Spanish, 0.01)
    ///         ],
    ///         vec![
    ///             (German, 1.0),
    ///             (English, 0.0),
    ///             (French, 0.0),
    ///             (Spanish, 0.0)
    ///         ]
    ///     ]
    /// );
    #[cfg(not(target_family = "wasm"))]
    pub fn compute_confidence_in_parallel(
        &self,
        texts: &[&str],
    ) -> Vec<Vec<(ScriptLanguage, f64)>> {
        texts
            .into_par_iter()
            .map(|&text| self.compute_confidence(text))
            .collect()
    }

    pub fn compute_confidence_for_languages<S: BuildHasher + Default>(
        &self,
        text_str: &str,
        search_languages: &HashSet<ScriptLanguage, S>,
    ) -> Vec<(ScriptLanguage, f64)> {
        if text_str.is_empty() {
            return Default::default();
        }

        // let mut values = Vec::with_capacity(search_languages.len());

        /* for &language in search_languages {
            values.push((language, 0.0));
        } */

        /* let words = split_text_into_words(text_str);
        if words.is_empty() {
            return values;
        } */

        // let filtered_languages = Self::process_words(&words, search_languages);

        /* let found_words = alphabet_detector::from_ch_iter(text_str.char_indices());

        let mut words = Vec::new();
        let mut languages: AHashMap<Language, usize> = Default::default();
        for wd in found_words {
            let len = wd.chars.len();
            words.push(wd.chars);
            let langs = langs_count_max(wd.langs_cnt).0;
            for search_lang in search_languages {
                if langs.contains(search_lang) {
                    let cnt = languages.entry(*search_lang).or_default();
                    *cnt += len;
                }
            }
        } */
        let (words, langs) = fulltext_filter_with_margin::<Vec<char>, 95>(text_str.char_indices());
        let filtered_languages: AHashSet<_> = langs
            .filter(|(l, _)| search_languages.contains(l))
            .map(|(l, _)| l) // todo: maybe use count?
            .collect();

        if words.is_empty() || filtered_languages.is_empty() {
            return Default::default();
        }

        /* let lang_alphabets_count_max = languages.iter().fold(1, |acc, (_, &cnt)| acc.max(cnt));
        languages.retain(|_, cnt| {
            *cnt == lang_alphabets_count_max
        }); */

        // let language_detected_by_rules =
        // Self::find_most_frequent_opt(&mut total_language_counts);

        /* if let Some(language) = language_detected_by_rules {
            update_confidence_values(&mut values, language, 1.0);
            values.sort_by(confidence_values_comparator);
            return values;
        } */

        // let words_count_half = (words.len() as f64) * 0.5;
        /* let filtered_languages = self.filter_languages_by_rules(
            &words,
            // search_languages,
            words_count_half,
            // total_script_counts,
            filtered_languages,
        ); */

        if filtered_languages.len() == 1 {
            let lang = filtered_languages.into_iter().next().unwrap();
            // update_confidence_values(&mut values, lang, 0.0);
            // values.sort_by(order_by_probability);
            return vec![(lang, 0.0)];
        }

        let character_count: usize = words.iter().map(|wd| wd.buf.len()).sum();

        /* if self.is_low_accuracy_mode_enabled && character_count < 3 {
            values.sort_by(order_by_probability);
            return values;
        } */

        let ngram_length_range = if self.is_low_accuracy_mode_enabled {
            if character_count >= 120 {
                3..4usize
            } else {
                1..4usize
            }
        } else if character_count >= 120 {
            3..NGRAM_MAX_SIZE + 1
        } else {
            1..NGRAM_MAX_SIZE + 1
        };

        self.load_language_models_by_ngram_len(ngram_length_range.end - 1, &filtered_languages);

        let probabilities: Vec<_> = ngram_length_range
            .into_iter()
            .filter(|i| *i <= character_count)
            .map(|ngram_length| {
                self.compute(
                    words.iter().map(|wd| wd.buf.as_ref()),
                    ngram_length,
                    &filtered_languages,
                )
            })
            .collect();

        let mut probabilities_sums = self.sum_up_probabilities(probabilities, filtered_languages);

        if probabilities_sums.is_empty() {
            return Default::default();
        }

        probabilities_sums.sort_unstable_by(order_by_probability_and_lang);
        /* println!(
            "res {:?}",
            &probabilities_sums[..probabilities_sums.len().min(5)]
        ); */

        probabilities_sums
    }

    /// Computes the confidence value for the given language and input text. This value denotes
    /// how likely it is that the given text has been written in the given language.
    ///
    /// The value that this method computes is a number between 0.0 and 1.0. If the language is
    /// unambiguously identified by the rule engine, the value 1.0 will always be returned.
    /// If the given language is not supported by this detector instance, the value 0.0 will
    /// always be returned.
    ///
    /// This method operates in a single thread. If you want to classify
    /// a very large set of texts, you will probably want to use method
    /// [`compute_relative_probability_in_parallel`](#method.compute_relative_probability_in_parallel)
    /// instead.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let confidence = detector.compute_relative_probability("languages are awesome", French);
    /// let rounded_confidence = (confidence * 100.0).round() / 100.0;
    ///
    /// assert_eq!(rounded_confidence, 0.04);
    /// ```
    pub fn compute_relative_probability(&self, text: &str, language: ScriptLanguage) -> f64 {
        let mut confidence = self.compute_confidence(text);
        LanguageDetector::transform_to_relative_probabilities(&mut confidence);
        confidence
            .into_iter()
            .find(|(l, _)| *l == language)
            .map(|(_, p)| p)
            .unwrap_or(0.0)
    }

    /// Computes the confidence values of all input texts for the given language.
    /// A confidence value denotes how likely it is that a given text has been
    /// written in a given language.
    ///
    /// The values that this method computes are numbers between 0.0 and 1.0. If the language is
    /// unambiguously identified by the rule engine, the value 1.0 will always be returned.
    /// If the given language is not supported by this detector instance, the value 0.0 will
    /// always be returned.
    ///
    /// This method is a good fit if you want to classify a very large set of texts.
    /// It potentially operates in multiple threads, depending on how many idle CPU
    /// cores are available and how many texts are passed to this method.
    ///
    /// ```
    /// use lingua::Language::{English, French, German, Spanish};
    /// use lingua::LanguageDetectorBuilder;
    ///
    /// let detector = LanguageDetectorBuilder::from_languages(&[
    ///     English,
    ///     French,
    ///     German,
    ///     Spanish
    /// ])
    /// .build();
    ///
    /// let result = detector.compute_relative_probability_in_parallel(
    ///     &[
    ///         "languages are awesome",
    ///         "Sprachen sind großartig",
    ///         "des langues sont géniales",
    ///         "los idiomas son geniales"
    ///     ],
    ///     French
    /// )
    /// .into_iter()
    /// .map(|confidence| (confidence * 100.0).round() / 100.0)
    /// .collect::<Vec<_>>();
    ///
    /// assert_eq!(
    ///     result,
    ///     vec![
    ///         0.04,
    ///         0.0,
    ///         0.94,
    ///         0.07
    ///     ]
    /// );
    /// ```
    #[cfg(not(target_family = "wasm"))]
    pub fn compute_relative_probability_in_parallel(
        &self,
        texts: &[&str],
        language: ScriptLanguage,
    ) -> Vec<f64> {
        texts
            .into_par_iter()
            .map(|text| self.compute_relative_probability(text, language))
            .collect()
    }

    fn load_language_models_by_ngram_len(
        &self,
        ngram_length: usize,
        filtered_languages: &AHashSet<ScriptLanguage>,
    ) {
        match ngram_length {
            1 => {
                for &language in filtered_languages {
                    self.load_language_model(language, 1);
                }
            }
            2 => {
                for &language in filtered_languages {
                    self.load_language_model(language, 1);
                    self.load_language_model(language, 2);
                }
            }
            3 => {
                for &language in filtered_languages {
                    self.load_language_model(language, 1);
                    self.load_language_model(language, 2);
                    self.load_language_model(language, 3);
                }
            }
            4 => {
                for &language in filtered_languages {
                    self.load_language_model(language, 1);
                    self.load_language_model(language, 2);
                    self.load_language_model(language, 3);
                    self.load_language_model(language, 4);
                }
            }
            5 => {
                for &language in filtered_languages {
                    self.load_language_model(language, 1);
                    self.load_language_model(language, 2);
                    self.load_language_model(language, 3);
                    self.load_language_model(language, 4);
                    self.load_language_model(language, 5);
                }
            }
            _ => unreachable!(),
        }
    }

    fn compute<'a>(
        &'a self,
        words_iter: impl Iterator<Item = &'a [char]>,
        ngram_length: usize,
        filtered_languages: &AHashSet<ScriptLanguage>,
    ) -> ScriptLanguageArr<(f64, usize)> {
        // todo: move prepare_ngrams out of here
        let ngrams = prepare_ngrams(words_iter, ngram_length);

        let probabilities = self.compute_languages_ngrams_confidence(
            ngrams.iter().map(NgramString::as_str),
            filtered_languages,
            ngram_length,
        );

        probabilities
    }

    fn compute_languages_ngrams_confidence<'a>(
        &'a self,
        ngrams_iter: impl Iterator<Item = &'a str> + Clone,
        filtered_languages: &AHashSet<ScriptLanguage>,
        ngram_length: usize,
    ) -> ScriptLanguageArr<(f64, usize)> {
        let mut probabilities = slang_arr_default_nc();
        for &language in filtered_languages.iter() {
            let sum = self.compute_ngrams_confidence(language, ngrams_iter.clone(), ngram_length);
            *probabilities.get_safe_unchecked_mut(language as usize) = sum;
        }
        probabilities
    }

    /// `probabilities` must be ordered
    fn transform_to_relative_probabilities(
        // values: &mut Vec<(ScriptLanguage, f64)>,
        // probability_map: Option<&AHashMap<ScriptLanguage, f64>>,
        probabilities: &mut Vec<(ScriptLanguage, f64)>,
    ) {
        if probabilities.is_empty() {
            return;
        }

        debug_assert!(!probabilities.iter().any(|(_, p)| p.is_nan()));

        let first_probability = probabilities.first().unwrap().1;
        if first_probability.is_zero() {
            let zeroes = probabilities
                .iter()
                .position(|(_, p)| !p.is_zero())
                .unwrap_or(probabilities.len());
            probabilities.truncate(zeroes);
            let len = zeroes as f64;
            probabilities.iter_mut().for_each(|(_, p)| *p = 1.0 / len);

            return;
        }

        if first_probability == f64::NEG_INFINITY {
            let len = probabilities.len() as f64;
            probabilities.iter_mut().for_each(|(_, p)| *p = 1.0 / len);

            return;
        }

        probabilities.iter_mut().for_each(|(_, p)| *p = p.exp());
        let denominator: f64 = probabilities.iter().map(|(_, p)| *p).sum();

        // If the denominator is still zero, the exponent of the summed
        // log probabilities is too large to be computed for very long input strings.
        // So we simply set the probability of the most likely language to 1.0 and
        // leave the other languages at 0.0.
        if denominator.is_zero() {
            if let Some((_, p)) = probabilities.first_mut() {
                *p = 1.0
            }
            probabilities.truncate(1);
            // For very long inputs, only trigrams are used, so we safely access first.
            /* let probability_map = probability_map.unwrap();
            let most_likely_language = *probability_map
                .iter()
                .max_by(|(_, first_probability), (_, second_probability)| {
                    first_probability.total_cmp(second_probability)
                })
                .unwrap()
                .0;

            probabilities[0] = (most_likely_language, 1.0);
            probabilities.truncate(1); */
        } else {
            probabilities
                .iter_mut()
                .for_each(|(_, p)| *p /= denominator);
            /* for (language, probability) in probabilities {
                for value in values.iter_mut() {
                    if value.0 == language {
                        // Apply softmax function
                        let normalized_probability = probability / denominator;
                        value.1 = normalized_probability;
                        break;
                    }
                }
            } */
        }
    }

    fn compute_ngrams_confidence<'a>(
        &'a self,
        language: ScriptLanguage,
        ngrams_iter: impl Iterator<Item = &'a str>,
        ngram_length: usize,
    ) -> (f64, usize) {
        debug_assert!(
            (1..=NGRAM_MAX_SIZE).contains(&ngram_length),
            "ngram length {} is not in range 1..={NGRAM_MAX_SIZE}",
            ngram_length
        );

        let language_model_lock = self
            .languages_models
            .get_safe_unchecked(language as usize)
            .read()
            .unwrap();

        let Some(language_model) = language_model_lock
            .ngrams
            .get(ngram_length - 1)
            .filter(|m| !m.is_empty())
        else {
            // todo: it must somehow request the lower ngram model to be used,
            // if it has not been used already (example: long sentences of Han)
            return (language_model_lock.min_probability, 1);
        };

        let mut cnt = 0;
        let mut sum = 0.0;
        for ngram in ngrams_iter {
            let probability = language_model
                .get(ngram)
                .copied()
                .inspect(|_| cnt += 1)
                .unwrap_or(language_model_lock.min_probability);

            sum += probability;
        }
        (sum, cnt)
    }

    fn sum_up_probabilities(
        &self,
        probabilities: Vec<ScriptLanguageArr<(f64, usize)>>,
        filtered_languages: AHashSet<ScriptLanguage>,
    ) -> Vec<(ScriptLanguage, f64)> {
        let mut summed_up_probabilities = Vec::with_capacity(filtered_languages.len());
        for language in filtered_languages.into_iter() {
            let lang_data = probabilities
                .iter()
                .map(|it| *it.get_safe_unchecked(language as usize));
            let sum: f64 = lang_data.clone().map(|(p, _)| p).sum();
            let sum_cnt: usize = lang_data.clone().map(|(_, cnt)| cnt).sum();

            summed_up_probabilities.push((language, sum / sum_cnt as f64));
        }

        summed_up_probabilities
    }

    fn load_language_model(&self, language: ScriptLanguage, ngram_length: usize) {
        debug_assert!(
            (1..=NGRAM_MAX_SIZE).contains(&ngram_length),
            "ngram length {ngram_length} is not in range 1..={NGRAM_MAX_SIZE}"
        );

        let ngram_models = self.languages_models.get_safe_unchecked(language as usize);
        let index = ngram_length - 1;
        let ngram_models_guard = ngram_models.read().unwrap();
        if ngram_models_guard
            .ngrams
            .get_safe_unchecked(index)
            .capacity()
            > 0
        {
            return;
        }
        drop(ngram_models_guard);
        let mut ngram_models_guard = ngram_models.write().unwrap();
        if ngram_models_guard
            .ngrams
            .get_safe_unchecked(index)
            .capacity()
            > 0
        {
            return;
        }
        let ngram_model_raw = load_model(language, ngram_length);
        let ngram_model = match ngram_model_raw {
            Ok(m) => parse_model(m, ngram_length),
            _ => AHashMap::with_capacity(1),
        };
        ngram_models_guard.update_ngram(ngram_model, index);
    }

    fn increment_counter<T: Eq + Hash, S: BuildHasher>(
        counts: &mut HashMap<T, usize, S>,
        key: T,
        value: usize,
    ) {
        let counter = counts.entry(key).or_insert(0);
        *counter += value;
    }
}

fn order_by_probability_and_lang(
    first: &(ScriptLanguage, f64),
    second: &(ScriptLanguage, f64),
) -> Ordering {
    second
        .1
        .total_cmp(&first.1)
        .then_with(|| first.0.cmp(&second.0))
}

/* fn update_confidence_values(
    values: &mut Vec<(ScriptLanguage, f64)>,
    language: ScriptLanguage,
    probability: f64,
) {
    for value in values {
        if value.0 == language {
            value.1 = probability;
            break;
        }
    }
} */

fn merge_adjacent_results(
    results: &mut Vec<DetectionResult>,
    mergeable_result_indices: &mut Vec<usize>,
) {
    mergeable_result_indices.sort();
    mergeable_result_indices.reverse();

    for idx in mergeable_result_indices {
        let i = *idx;

        if i == 0 {
            results[i + 1] = DetectionResult {
                start_index: results[i].start_index,
                end_index: results[i + 1].end_index,
                word_count: results[i].word_count + results[i + 1].word_count,
                language: results[i + 1].language,
            };
        } else {
            results[i - 1] = DetectionResult {
                start_index: results[i - 1].start_index,
                end_index: results[i].end_index,
                word_count: results[i - 1].word_count + results[i].word_count,
                language: results[i - 1].language,
            };
        }

        results.remove(i);

        if results.len() == 1 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{builder::LanguageDetectorBuilder, ScriptLanguage::*};
    use ::std::sync::LazyLock;
    use float_cmp::approx_eq;
    use rstest::*;

    fn create_mock_language_models(
        ngrams_model: [AHashMap<&'static str, f64>; NGRAM_MAX_SIZE],
    ) -> LanguageModel {
        let ngrams = ngrams_model.map(|model| {
            model
                .into_iter()
                .map(|(k, v)| (NgramString::try_from_str(k).unwrap(), v.ln()))
                .collect()
        });
        LanguageModel::from(ngrams)
    }

    fn round_to_two_decimal_places(value: f64) -> f64 {
        (value * 100.0).round() / 100.0
    }

    const ENGLISH_UNIGRAMS_COUNT: f64 = 7.0;
    fn language_model_for_english() -> LanguageModel {
        create_mock_language_models([
            ahashmap!(
                "a" => 0.01,
                "l" => 0.02,
                "t" => 0.03,
                "e" => 0.04,
                "r" => 0.05,
                "o" => 1.0,
                "k" => 1.0,
            ),
            ahashmap!(
                "al" => 0.11,
                "lt" => 0.12,
                "te" => 0.13,
                "er" => 0.14,
            ),
            ahashmap!(
                "alt" => 0.19,
                "lte" => 0.2,
                "ter" => 0.21,
            ),
            ahashmap!(
                "alte" => 0.25,
                "lter" => 0.26,
            ),
            ahashmap!(
                "alter" => 0.29,
            ),
        ])
    }

    const GERMAN_UNIGRAMS_COUNT: f64 = 6.0;
    fn language_model_for_german() -> LanguageModel {
        create_mock_language_models([
            ahashmap!(
                "a" => 0.06,
                "l" => 0.07,
                "t" => 0.08,
                "e" => 0.09,
                "r" => 0.1,
                "o" => 1.0,
            ),
            ahashmap!(
                "al" => 0.15,
                "lt" => 0.16,
                "te" => 0.17,
                "er" => 0.18,
            ),
            ahashmap!(
                "alt" => 0.22,
                "lte" => 0.23,
                "ter" => 0.24,
            ),
            ahashmap!(
                "alte" => 0.27,
                "lter" => 0.28,
            ),
            ahashmap!("alter" => 0.3),
        ])
    }

    // ##############################
    // DETECTORS
    // ##############################

    static MOCK_DETECTOR_ENGLISH_AND_GERMAN: LazyLock<LanguageDetector> = LazyLock::new(|| {
        let languages = ahashset!(English, German);

        let languages_models: LanguagesModels =
            ::core::array::from_fn(|_| RwLock::new(Default::default()));
        *languages_models
            .get_safe_unchecked(English as usize)
            .write()
            .unwrap() = language_model_for_english();
        *languages_models
            .get_safe_unchecked(German as usize)
            .write()
            .unwrap() = language_model_for_german();

        LanguageDetector {
            languages_preloaded: languages,
            is_low_accuracy_mode_enabled: false,
            languages_models,
        }
    });

    static DETECTOR_ALL_LANGUAGES: LazyLock<LanguageDetector> =
        LazyLock::new(|| LanguageDetector::new(ScriptLanguage::all().collect(), false));

    // ##############################
    // TESTS
    // ##############################

    #[rstest(
        language,
        ngram,
        expected_probability,
        case(English, "a", 0.01),
        case(English, "lt", 0.12),
        case(English, "ter", 0.21),
        case(English, "alte", 0.25),
        case(English, "alter", 0.29),
        case(German, "t", 0.08),
        case(German, "er", 0.18),
        case(German, "alt", 0.22),
        case(German, "lter", 0.28),
        case(German, "alter", 0.3)
    )]
    fn test_model_ngram_lookup(language: ScriptLanguage, ngram: &str, expected_probability: f64) {
        let ngram_length = ngram.chars().count();
        // mock_detector_for_english_and_german
        // .load_language_models_by_ngram_len(ngram_length, &ahashset!(language));

        let language_model_lock = MOCK_DETECTOR_ENGLISH_AND_GERMAN
            .languages_models
            .get_safe_unchecked(language as usize)
            .read()
            .unwrap();

        let probability = language_model_lock.ngrams[ngram_length - 1]
            .get(ngram)
            .copied()
            .unwrap_or(f64::NEG_INFINITY);

        let expected_probability = expected_probability.ln();

        assert_eq!(
            probability, expected_probability,
            "expected probability {} for language '{:?}' and ngram '{}', got {}",
            expected_probability, language, ngram, probability
        );
    }

    #[rstest(
        ngrams,
        expected_ngrams_confidence,
        case(
            vec!["a", "l", "t", "e", "r"],
            0.01_f64.ln() + 0.02_f64.ln() + 0.03_f64.ln() + 0.04_f64.ln() + 0.05_f64.ln()
        ),
        case(
            // last one is unknown trigram
            vec!["alt", "lte", "tez"],
            0.19_f64.ln() + 0.2_f64.ln() + (1_f64 / ENGLISH_UNIGRAMS_COUNT).ln()
        ),
        case(
            // unknown fivegram
            vec!["aquas"],
            (1_f64 / ENGLISH_UNIGRAMS_COUNT).ln()
        ),
        case(
            // English only unigram
            vec!["k"],
            1.0_f64.ln()
        )
    )]
    fn test_compute_ngrams_confidence(ngrams: Vec<&'static str>, expected_ngrams_confidence: f64) {
        // mock_detector_for_english_and_german.load_languages_models(&ahashset!(English));
        let (ngrams_confidence, _cnt) = MOCK_DETECTOR_ENGLISH_AND_GERMAN.compute_ngrams_confidence(
            English,
            ngrams.iter().copied(),
            ngrams[0].chars().count(),
        );

        assert!(
            approx_eq!(f64, ngrams_confidence, expected_ngrams_confidence, ulps = 1),
            "expected sum {} for language '{:?}' and ngrams {:?}, got {}",
            expected_ngrams_confidence,
            English,
            ngrams,
            ngrams_confidence
        );
    }

    #[rstest(
        ngrams,
        expected_probabilities,
        case::unigram_model(
            vec!["a", "l", "t", "e", "r"],
            ahashmap!(
                English => 0.01_f64.ln() + 0.02_f64.ln() + 0.03_f64.ln() + 0.04_f64.ln() + 0.05_f64.ln(),
                German => 0.06_f64.ln() + 0.07_f64.ln() + 0.08_f64.ln() + 0.09_f64.ln() + 0.1_f64.ln()
            )
        ),
        case::trigram_model(
            vec!["alt", "lte", "ter", "wxy"],
            ahashmap!(
                English => 0.19_f64.ln() + 0.2_f64.ln() + 0.21_f64.ln() + (1_f64 / ENGLISH_UNIGRAMS_COUNT).ln(),
                German => 0.22_f64.ln() + 0.23_f64.ln() + 0.24_f64.ln() + (1_f64 / GERMAN_UNIGRAMS_COUNT).ln()
            )
        ),
        case::quadrigram_model(
            vec!["alte", "lter", "wxyz"],
            ahashmap!(
                English => 0.25_f64.ln() + 0.26_f64.ln() + (1_f64 / ENGLISH_UNIGRAMS_COUNT).ln(),
                German => 0.27_f64.ln() + 0.28_f64.ln() + (1_f64 / GERMAN_UNIGRAMS_COUNT).ln()
            )
        )
    )]
    fn test_compute_languages_ngrams_confidence(
        ngrams: Vec<&'static str>,
        expected_probabilities: AHashMap<ScriptLanguage, f64>,
    ) {
        let languages = ahashset!(English, German);
        let probabilities = MOCK_DETECTOR_ENGLISH_AND_GERMAN.compute_languages_ngrams_confidence(
            ngrams.iter().copied(),
            &languages,
            ngrams[0].chars().count(),
        );

        for (language, (probability, _cnt)) in probabilities.into_iter().enumerate() {
            if probability.is_zero() {
                continue;
            }
            let language = ScriptLanguage::from_usize_unchecked(language);
            let expected_probability = expected_probabilities[&language];

            assert!(
                approx_eq!(f64, probability, expected_probability, ulps = 1),
                "expected probability {} for language '{:?}', got {}",
                expected_probability,
                language,
                probability
            );
        }
    }

    #[rstest(
        text,
        expected_confidence,
        case::language_detected_by_rules("groß", vec![(German, 1.0)]),
        case::known_ngrams("Alter", vec![(German, 0.62), (English, 0.38)]),
        case::english_only_ngrams("k", vec![(English, 1.0)]),
        case::unique_ngrams("o", vec![(English, 0.5), (German, 0.5)]),
        case::unknown_ngrams("проарплап", vec![]),
    )]
    fn test_compute_confidence(text: &str, expected_confidence: Vec<(ScriptLanguage, f64)>) {
        let mut confidence = MOCK_DETECTOR_ENGLISH_AND_GERMAN.compute_confidence(text);

        LanguageDetector::transform_to_relative_probabilities(&mut confidence);
        confidence
            .iter_mut()
            .for_each(|(_, p)| *p = round_to_two_decimal_places(*p));

        assert_eq!(confidence, expected_confidence);
    }

    #[rstest(
        text,
        expected_confidence,
        case::script_no_models("ꨕ", vec![(ChamEastern, 0.5), (ChamWestern, 0.5)]),
    )]
    fn test_compute_confidence_no_filter(
        text: &str,
        expected_confidence: Vec<(ScriptLanguage, f64)>,
    ) {
        let mut confidence = MOCK_DETECTOR_ENGLISH_AND_GERMAN.compute_confidence_for_languages(
            text,
            &ScriptLanguage::all().collect::<AHashSet<_>>(),
        );

        LanguageDetector::transform_to_relative_probabilities(&mut confidence);
        confidence
            .iter_mut()
            .for_each(|(_, p)| *p = round_to_two_decimal_places(*p));

        assert_eq!(confidence, expected_confidence);
    }

    #[rstest(
        text,
        language,
        expected_confidence,
        case::german_detected_by_rules("groß", German, 1.0),
        case::english_detected_by_rules("groß", English, 0.0),
        case::german_known_ngrams("Alter", German, 0.62),
        case::english_known_ngrams("Alter", English, 0.38),
        case::english_only_ngrams("k", English, 1.0),
        case::german_unknown_ngrams("проарплап", German, 0.0),
        case::english_unknown_ngrams("проарплап", English, 0.0),
        case::unknown_language("groß", French, 0.0)
    )]
    fn test_compute_relative_probability(
        text: &str,
        language: ScriptLanguage,
        expected_confidence: f64,
    ) {
        let confidence =
            MOCK_DETECTOR_ENGLISH_AND_GERMAN.compute_relative_probability(text, language);

        assert_eq!(round_to_two_decimal_places(confidence), expected_confidence);
    }

    #[rstest(
        word,
        expected_language,
        case("Alter", Some(German)),
        case("проарплап", None)
    )]
    fn test_detect(word: &str, expected_language: Option<ScriptLanguage>) {
        let detected_language = MOCK_DETECTOR_ENGLISH_AND_GERMAN.detect(word, 0.0);
        assert_eq!(detected_language, expected_language);
    }

    #[rstest]
    fn test_detect_multiple_for_empty_string() {
        assert!(DETECTOR_ALL_LANGUAGES.detect_multiple("").is_empty());
    }

    #[rstest(
        sentence,
        expected_word_count,
        expected_language,
        case::english_1(
            "I'm really not sure whether multi-language detection is a good idea.",
            11,
            English
        ),
        case::english_2("I'm frightened! 🙈", 3, English),
        case::kazakh("V төзімділік спорт", 3, Kazakh)
    )]
    fn test_detect_multiple_with_one_language(
        sentence: &str,
        expected_word_count: usize,
        expected_language: ScriptLanguage,
    ) {
        let results = DETECTOR_ALL_LANGUAGES.detect_multiple(sentence);
        assert_eq!(results.len(), 1);

        let result = &results[0];
        let substring = &sentence[result.start_index()..result.end_index()];
        assert_eq!(substring, sentence);
        assert_eq!(result.word_count, expected_word_count);
        assert_eq!(result.language(), expected_language);
    }

    #[rstest(
        sentence,
        expected_first_substring,
        expected_first_word_count,
        expected_first_language,
        expected_second_substring,
        expected_second_word_count,
        expected_second_language,
        case::english_german(
            "  He   turned around and asked: \"Entschuldigen Sie, sprechen Sie Deutsch?\"",
            "  He   turned around and asked: ",
            5,
            English,
            "\"Entschuldigen Sie, sprechen Sie Deutsch?\"",
            5,
            German
        ),
        case::chinese_english(
            "上海大学是一个好大学. It is such a great university.",
            "上海大学是一个好大学. ",
            10,
            ChineseSimplified,
            "It is such a great university.",
            6,
            English
        ),
        case::english_russian(
            "English German French - Английский язык",
            "English German French - ",
            4,
            English,
            "Английский язык",
            2,
            Russian
        )
    )]
    fn test_detect_multiple_with_two_languages(
        sentence: &str,
        expected_first_substring: &str,
        expected_first_word_count: usize,
        expected_first_language: ScriptLanguage,
        expected_second_substring: &str,
        expected_second_word_count: usize,
        expected_second_language: ScriptLanguage,
    ) {
        let results = DETECTOR_ALL_LANGUAGES.detect_multiple(sentence);
        assert_eq!(results.len(), 2);

        let first_result = &results[0];
        let first_substring = &sentence[first_result.start_index()..first_result.end_index()];
        assert_eq!(first_substring, expected_first_substring);
        assert_eq!(first_result.word_count, expected_first_word_count);
        assert_eq!(first_result.language(), expected_first_language);

        let second_result = &results[1];
        let second_substring = &sentence[second_result.start_index()..second_result.end_index()];
        assert_eq!(second_substring, expected_second_substring);
        assert_eq!(second_result.word_count, expected_second_word_count);
        assert_eq!(second_result.language(), expected_second_language);
    }

    #[rstest(
        sentence,
        expected_first_substring,
        expected_first_word_count,
        expected_first_language,
        expected_second_substring,
        expected_second_word_count,
        expected_second_language,
        expected_third_substring,
        expected_third_word_count,
        expected_third_language,
        case::french_german_english(
            "Parlez-vous français? Ich spreche Französisch nur ein bisschen. A little bit is better than nothing.",
            "Parlez-vous français? ",
            2,
            French,
            "Ich spreche Französisch nur ein bisschen. ",
            6,
            German,
            "A little bit is better than nothing.",
            7,
            English
        ),
        /* case::polish_german_english(
            "Płaszczowo-rurowe wymienniki ciepła Uszczelkowe der blaue himmel über berlin 中文 the quick brown fox jumps over the lazy dog",
            "Płaszczowo-rurowe wymienniki ciepła Uszczelkowe ",
            4,
            Polish,
            "der blaue himmel über berlin 中文 ",
            7,
            German,
            "the quick brown fox jumps over the lazy dog",
            9,
            English
        ), */
    )]
    fn test_detect_multiple_with_three_languages(
        sentence: &str,
        expected_first_substring: &str,
        expected_first_word_count: usize,
        expected_first_language: ScriptLanguage,
        expected_second_substring: &str,
        expected_second_word_count: usize,
        expected_second_language: ScriptLanguage,
        expected_third_substring: &str,
        expected_third_word_count: usize,
        expected_third_language: ScriptLanguage,
    ) {
        let results = DETECTOR_ALL_LANGUAGES.detect_multiple(sentence);
        assert_eq!(results.len(), 3, "{} {:?}", sentence, results);

        let first_result = &results[0];
        let first_substring = &sentence[first_result.start_index()..first_result.end_index()];
        assert_eq!(first_substring, expected_first_substring);
        assert_eq!(first_result.word_count, expected_first_word_count);
        assert_eq!(first_result.language(), expected_first_language);

        let second_result = &results[1];
        let second_substring = &sentence[second_result.start_index()..second_result.end_index()];
        assert_eq!(second_substring, expected_second_substring);
        assert_eq!(second_result.word_count, expected_second_word_count);
        assert_eq!(second_result.language(), expected_second_language);

        let third_result = &results[2];
        let third_substring = &sentence[third_result.start_index()..third_result.end_index()];
        assert_eq!(third_substring, expected_third_substring);
        assert_eq!(third_result.word_count, expected_third_word_count);
        assert_eq!(third_result.language(), expected_third_language);
    }

    /* #[rstest(
        sentence,
        expected_first_substring,
        expected_first_word_count,
        expected_first_language,
        expected_second_substring,
        expected_second_word_count,
        expected_second_language,
        expected_third_substring,
        expected_third_word_count,
        expected_third_language,
        expected_fourth_substring,
        expected_fourth_word_count,
        expected_fourth_language,
        case::polish_german_chinese_english(
            "Płaszczowo-rurowe wymienniki ciepła Uszczelkowe der blaue himmel über berlin 中文 the quick brown fox jumps over the lazy dog",
            "Płaszczowo-rurowe wymienniki ciepła Uszczelkowe ",
            4,
            Polish,
            "der blaue himmel über berlin ",
            5,
            German,
            "中文 ",
            2,
            Chinese,
            "the quick brown fox jumps over the lazy dog",
            9,
            English
        )
    )]
    fn test_detect_multiple_with_four_languages(
        sentence: &str,
        expected_first_substring: &str,
        expected_first_word_count: usize,
        expected_first_language: Language,
        expected_second_substring: &str,
        expected_second_word_count: usize,
        expected_second_language: Language,
        expected_third_substring: &str,
        expected_third_word_count: usize,
        expected_third_language: Language,
        expected_fourth_substring: &str,
        expected_fourth_word_count: usize,
        expected_fourth_language: Language,
    ) {
        let results = DETECTOR_ALL_LANGUAGES.detect_multiple(sentence);
        assert_eq!(results.len(), 4, "{:?}", results);

        let first_result = &results[0];
        let first_substring = &sentence[first_result.start_index()..first_result.end_index()];
        assert_eq!(first_substring, expected_first_substring);
        assert_eq!(first_result.word_count, expected_first_word_count);
        assert_eq!(first_result.language(), expected_first_language);

        let second_result = &results[1];
        let second_substring = &sentence[second_result.start_index()..second_result.end_index()];
        assert_eq!(second_substring, expected_second_substring);
        assert_eq!(second_result.word_count, expected_second_word_count);
        assert_eq!(second_result.language(), expected_second_language);

        let third_result = &results[2];
        let third_substring = &sentence[third_result.start_index()..third_result.end_index()];
        assert_eq!(third_substring, expected_third_substring);
        assert_eq!(third_result.word_count, expected_third_word_count);
        assert_eq!(third_result.language(), expected_third_language);

        let fourth_result = &results[3];
        let fourth_substring = &sentence[fourth_result.start_index()..fourth_result.end_index()];
        assert_eq!(fourth_substring, expected_fourth_substring);
        assert_eq!(fourth_result.word_count, expected_fourth_word_count);
        assert_eq!(fourth_result.language(), expected_fourth_language);
    } */

    #[rstest(
        builder_languages,
        text,
        expected_language,
        case(vec![English, Kazakh], "нормаланбайды", Some(Kazakh)),
        case(vec![English, Kazakh], "нормаланбайды I", Some(Kazakh)),
        case(vec![Kazakh, MongolianHalh], "Балаларды жүзуге үй-рету бассейнінің үй-жайы", Some(Kazakh)),
        // case::simplified_chinese(vec![ChineseSimplified, ChineseTraditional, ChineseCantoneseTraditional, Japanese], "经济", Some(ChineseSimplified)),
        // case::traditional_chinese(vec![ChineseSimplified, Japanese], "經濟", Some(ChineseSimplified)),
        case::kanji(vec![ChineseSimplified, Japanese], "経済", Some(Japanese)),
        case::kanji2(vec![ChineseSimplified, Japanese], "自動販売機", Some(Japanese)),
        // case::arab(vec![Acehnese, Arabic], "والموضوع", Some(Arabic)),
    )]
    fn test_specific_language_detection_problems(
        builder_languages: Vec<ScriptLanguage>,
        text: &str,
        expected_language: Option<ScriptLanguage>,
    ) {
        let detector = LanguageDetectorBuilder::from_languages(&builder_languages).build();

        let language = detector.detect(text, 0.0);
        assert_eq!(language, expected_language);
    }

    /* #[should_panic]
    #[rstest(text,
        case("kejurnas iii пїѕ aa boxer cup iii пїѕ bertempat di bandung jumlah peserta petarung dari daerah provinsi jawa barat dki jakarta jawa timur sumatera utara sumatera barat nusa tenggara barat bali kalimantan barat"),
    )]
    fn assert_language_filtering_with_rules_text_panics(
        text: &str,
    ) {
        let words = split_text_into_words(text);

        let filtered_languages =
            LanguageDetector::process_words(&words, &DETECTOR_ALL_LANGUAGES.languages);

        /* let words_count_half = (words.len() as f64) * 0.5;
        let filtered_languages = DETECTOR_ALL_LANGUAGES.filter_languages_by_rules(
            &words,
            // &DETECTOR_ALL_LANGUAGES.languages,
            words_count_half,
            // alps,
            filtered_languages,
        ); */
    } */

    #[rstest(invalid_str, case(""), case(" \n  \t;"), case("3<856%)§"))]
    fn assert_strings_without_letters_return_no_language(invalid_str: &str) {
        assert_eq!(DETECTOR_ALL_LANGUAGES.detect(invalid_str, 0.0), None);
    }

    #[rstest(text, expected_language, case("I know you әлем", Some(English)))]
    fn assert_language_detection_correct(text: &str, expected_language: Option<ScriptLanguage>) {
        assert_eq!(DETECTOR_ALL_LANGUAGES.detect(text, 0.0), expected_language);
    }

    #[rstest(text, languages,
        case(
            "ام وی با نیکی میناج تیزر داشت؟؟؟؟؟؟ i vote for bts ( _ ) as the _ via ( _ )",
            vec!(English, Urdu)
        ),
        case(
            "Az elmúlt hétvégén 12-re emelkedett az elhunyt koronavírus-fertőzöttek száma Szlovákiában. Mindegyik szociális otthon dolgozóját letesztelik, Matovič szerint az ingázóknak még várniuk kellene a teszteléssel",
            vec!(Hungarian, Slovak)
        )
    )]
    fn assert_language_detection_is_deterministic(text: &str, languages: Vec<ScriptLanguage>) {
        let detector = LanguageDetector::new(languages.iter().cloned().collect(), false);
        let mut detected_languages = AHashSet::new();
        for _ in 0..100 {
            let language = detector.detect(text, 0.0);
            detected_languages.insert(language.unwrap());
        }
        assert_eq!(
            detected_languages.len(),
            1,
            "language detector is non-deterministic for languages {:?}",
            languages
        );
    }

    #[rstest]
    fn test_low_accuracy_mode() {
        let detector = LanguageDetector::new(ahashset!(English, German), true);

        assert_ne!(detector.detect("bed", 0.0), None);
        assert_ne!(detector.detect("be", 0.0), None);
        assert_ne!(detector.detect("b", 0.0), None);

        assert_eq!(detector.detect("", 0.0), None);
    }
}
