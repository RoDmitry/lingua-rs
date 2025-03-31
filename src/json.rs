/*
 * Copyright © 2020-present Peter M. Stahl pemistahl@gmail.com
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either expressed or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use crate::fraction::Fraction;
use ::std::{
    io::{Cursor, ErrorKind, Read},
    path::PathBuf,
};
use ahash::AHashMap;
use alphabet_detector::Language;
use brotli::Decompressor;
use compact_str::CompactString;
use include_dir::{include_dir, Dir};
use serde_map::SerdeMap;

pub(crate) fn file_name_by_length(ngram_length: usize) -> &'static str {
    match ngram_length {
        1 => "unigram.encom.br",
        2 => "bigram.encom.br",
        3 => "trigram.encom.br",
        4 => "quadrigram.encom.br",
        5 => "fivegram.encom.br",
        _ => panic!("ngram length {ngram_length} is not in range 1..6"),
    }
}

pub type LanguageModel = SerdeMap<Fraction, String>;

pub(crate) fn to_relative_frequencies(
    fraction_ngrams: SerdeMap<Fraction, String>,
) -> AHashMap<CompactString, f64> {
    let mut res = AHashMap::new();
    for (fraction, ngrams) in fraction_ngrams {
        let floating_point_value = fraction.to_f64();
        for ngram in ngrams.split(' ') {
            res.insert(CompactString::new(ngram), floating_point_value);
        }
    }
    res
}

pub const MODELS_DIRECTORY: Dir = include_dir!("$CARGO_MANIFEST_DIR/lang_models");

pub(crate) fn load_model(
    language: Language,
    ngram_length: usize,
) -> std::io::Result<LanguageModel> {
    let file_name = file_name_by_length(ngram_length);
    let file_path = PathBuf::from(&language.to_string()).join(file_name);
    let compressed_file = MODELS_DIRECTORY
        .get_file(file_path)
        .ok_or(ErrorKind::NotFound)?;
    let compressed_file_reader = Cursor::new(compressed_file.contents());
    let mut uncompressed_file = Decompressor::new(compressed_file_reader, 4096);
    let mut uncompressed_file_content = String::new();
    uncompressed_file.read_to_string(&mut uncompressed_file_content)?;

    Ok(serde_encom::from_str(&uncompressed_file_content).unwrap())
}

/*fn get_language_models_directory(language: Language) -> Option<Dir<'static>> {
    match language {
        #[cfg(feature = "afrikaans")]
        Language::Afrikaans => Some(AFRIKAANS_MODELS_DIRECTORY),

        /*  #[cfg(feature = "albanian")]
        Language::AlbanianTosk => Some(ALBANIAN_MODELS_DIRECTORY), */
        #[cfg(feature = "arabic")]
        Language::Arabic => Some(ARABIC_MODELS_DIRECTORY),

        #[cfg(feature = "armenian")]
        Language::Armenian => Some(ARMENIAN_MODELS_DIRECTORY),

        /* #[cfg(feature = "azerbaijani")]
        Language::AzerbaijaniNorth => Some(AZERBAIJANI_MODELS_DIRECTORY), */
        #[cfg(feature = "basque")]
        Language::Basque => Some(BASQUE_MODELS_DIRECTORY),

        #[cfg(feature = "belarusian")]
        Language::Belarusian => Some(BELARUSIAN_MODELS_DIRECTORY),

        #[cfg(feature = "bengali")]
        Language::Bengali => Some(BENGALI_MODELS_DIRECTORY),

        #[cfg(feature = "bokmal")]
        Language::Bokmal => Some(BOKMAL_MODELS_DIRECTORY),

        #[cfg(feature = "bosnian")]
        Language::Bosnian => Some(BOSNIAN_MODELS_DIRECTORY),

        #[cfg(feature = "bulgarian")]
        Language::Bulgarian => Some(BULGARIAN_MODELS_DIRECTORY),

        #[cfg(feature = "catalan")]
        Language::Catalan => Some(CATALAN_MODELS_DIRECTORY),

        /* #[cfg(feature = "chinese")]
        Language::ChineseSimplified => Some(CHINESE_MODELS_DIRECTORY), */
        #[cfg(feature = "croatian")]
        Language::Croatian => Some(CROATIAN_MODELS_DIRECTORY),

        #[cfg(feature = "czech")]
        Language::Czech => Some(CZECH_MODELS_DIRECTORY),

        #[cfg(feature = "danish")]
        Language::Danish => Some(DANISH_MODELS_DIRECTORY),

        #[cfg(feature = "dutch")]
        Language::Dutch => Some(DUTCH_MODELS_DIRECTORY),

        #[cfg(feature = "english")]
        Language::English => Some(ENGLISH_MODELS_DIRECTORY),

        #[cfg(feature = "esperanto")]
        Language::Esperanto => Some(ESPERANTO_MODELS_DIRECTORY),

        #[cfg(feature = "estonian")]
        Language::Estonian => Some(ESTONIAN_MODELS_DIRECTORY),

        #[cfg(feature = "finnish")]
        Language::Finnish => Some(FINNISH_MODELS_DIRECTORY),

        #[cfg(feature = "french")]
        Language::French => Some(FRENCH_MODELS_DIRECTORY),

        #[cfg(feature = "ganda")]
        Language::Ganda => Some(GANDA_MODELS_DIRECTORY),

        #[cfg(feature = "georgian")]
        Language::Georgian => Some(GEORGIAN_MODELS_DIRECTORY),

        #[cfg(feature = "german")]
        Language::German => Some(GERMAN_MODELS_DIRECTORY),

        #[cfg(feature = "greek")]
        Language::Greek => Some(GREEK_MODELS_DIRECTORY),

        #[cfg(feature = "gujarati")]
        Language::Gujarati => Some(GUJARATI_MODELS_DIRECTORY),

        #[cfg(feature = "hebrew")]
        Language::Hebrew => Some(HEBREW_MODELS_DIRECTORY),

        #[cfg(feature = "hindi")]
        Language::Hindi => Some(HINDI_MODELS_DIRECTORY),

        #[cfg(feature = "hungarian")]
        Language::Hungarian => Some(HUNGARIAN_MODELS_DIRECTORY),

        #[cfg(feature = "icelandic")]
        Language::Icelandic => Some(ICELANDIC_MODELS_DIRECTORY),

        #[cfg(feature = "indonesian")]
        Language::Indonesian => Some(INDONESIAN_MODELS_DIRECTORY),

        #[cfg(feature = "irish")]
        Language::Irish => Some(IRISH_MODELS_DIRECTORY),

        #[cfg(feature = "italian")]
        Language::Italian => Some(ITALIAN_MODELS_DIRECTORY),

        #[cfg(feature = "japanese")]
        Language::Japanese => Some(JAPANESE_MODELS_DIRECTORY),

        #[cfg(feature = "kazakh")]
        Language::Kazakh => Some(KAZAKH_MODELS_DIRECTORY),

        #[cfg(feature = "korean")]
        Language::Korean => Some(KOREAN_MODELS_DIRECTORY),

        #[cfg(feature = "latin")]
        Language::Latin => Some(LATIN_MODELS_DIRECTORY),

        #[cfg(feature = "latvian")]
        Language::Latvian => Some(LATVIAN_MODELS_DIRECTORY),

        #[cfg(feature = "lithuanian")]
        Language::Lithuanian => Some(LITHUANIAN_MODELS_DIRECTORY),

        #[cfg(feature = "macedonian")]
        Language::Macedonian => Some(MACEDONIAN_MODELS_DIRECTORY),

        #[cfg(feature = "malay")]
        Language::Malay => Some(MALAY_MODELS_DIRECTORY),

        #[cfg(feature = "maori")]
        Language::Maori => Some(MAORI_MODELS_DIRECTORY),

        #[cfg(feature = "marathi")]
        Language::Marathi => Some(MARATHI_MODELS_DIRECTORY),

        /* #[cfg(feature = "mongolian")]
        Language::MongolianHalh => Some(MONGOLIAN_MODELS_DIRECTORY), */
        #[cfg(feature = "nynorsk")]
        Language::Nynorsk => Some(NYNORSK_MODELS_DIRECTORY),

        #[cfg(feature = "persian")]
        Language::Persian => Some(PERSIAN_MODELS_DIRECTORY),

        #[cfg(feature = "polish")]
        Language::Polish => Some(POLISH_MODELS_DIRECTORY),

        #[cfg(feature = "portuguese")]
        Language::Portuguese => Some(PORTUGUESE_MODELS_DIRECTORY),

        /* #[cfg(feature = "punjabi")]
        Language::PunjabiEastern => Some(PUNJABI_MODELS_DIRECTORY), */
        #[cfg(feature = "romanian")]
        Language::Romanian => Some(ROMANIAN_MODELS_DIRECTORY),

        #[cfg(feature = "russian")]
        Language::Russian => Some(RUSSIAN_MODELS_DIRECTORY),

        #[cfg(feature = "serbian")]
        Language::Serbian => Some(SERBIAN_MODELS_DIRECTORY),

        #[cfg(feature = "shona")]
        Language::Shona => Some(SHONA_MODELS_DIRECTORY),

        #[cfg(feature = "slovak")]
        Language::Slovak => Some(SLOVAK_MODELS_DIRECTORY),

        #[cfg(feature = "slovene")]
        Language::Slovene => Some(SLOVENE_MODELS_DIRECTORY),

        #[cfg(feature = "somali")]
        Language::Somali => Some(SOMALI_MODELS_DIRECTORY),

        #[cfg(feature = "sesotho")]
        Language::Sesotho => Some(SESOTHO_MODELS_DIRECTORY),

        #[cfg(feature = "spanish")]
        Language::Spanish => Some(SPANISH_MODELS_DIRECTORY),

        #[cfg(feature = "swahili")]
        Language::Swahili => Some(SWAHILI_MODELS_DIRECTORY),

        #[cfg(feature = "swedish")]
        Language::Swedish => Some(SWEDISH_MODELS_DIRECTORY),

        #[cfg(feature = "tagalog")]
        Language::Tagalog => Some(TAGALOG_MODELS_DIRECTORY),

        #[cfg(feature = "tamil")]
        Language::Tamil => Some(TAMIL_MODELS_DIRECTORY),

        #[cfg(feature = "telugu")]
        Language::Telugu => Some(TELUGU_MODELS_DIRECTORY),

        #[cfg(feature = "thai")]
        Language::Thai => Some(THAI_MODELS_DIRECTORY),

        #[cfg(feature = "tsonga")]
        Language::Tsonga => Some(TSONGA_MODELS_DIRECTORY),

        #[cfg(feature = "tswana")]
        Language::Tswana => Some(TSWANA_MODELS_DIRECTORY),

        #[cfg(feature = "turkish")]
        Language::Turkish => Some(TURKISH_MODELS_DIRECTORY),

        #[cfg(feature = "ukrainian")]
        Language::Ukrainian => Some(UKRAINIAN_MODELS_DIRECTORY),

        #[cfg(feature = "urdu")]
        Language::Urdu => Some(URDU_MODELS_DIRECTORY),

        #[cfg(feature = "vietnamese")]
        Language::Vietnamese => Some(VIETNAMESE_MODELS_DIRECTORY),

        #[cfg(feature = "welsh")]
        Language::Welsh => Some(WELSH_MODELS_DIRECTORY),

        #[cfg(feature = "xhosa")]
        Language::Xhosa => Some(XHOSA_MODELS_DIRECTORY),

        #[cfg(feature = "yoruba")]
        Language::Yoruba => Some(YORUBA_MODELS_DIRECTORY),

        #[cfg(feature = "zulu")]
        Language::Zulu => Some(ZULU_MODELS_DIRECTORY),

        _ => None,
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_model() {
        load_model(Language::English, 1).unwrap();
    }
}
