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

use once_cell::sync::Lazy;
use regex::Regex;

/* pub(crate) static JAPANESE_CHARACTER_SET: Lazy<CharSet> =
Lazy::new(|| CharSet::from_char_classes(&["Hiragana", "Katakana", "Han"])); */
pub(crate) static MULTIPLE_WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new("\\s+").unwrap());
pub(crate) static NUMBERS: Lazy<Regex> = Lazy::new(|| Regex::new("\\p{N}").unwrap());
pub(crate) static PUNCTUATION: Lazy<Regex> = Lazy::new(|| Regex::new("\\p{P}").unwrap());
pub(crate) static TOKENS_WITH_OPTIONAL_WHITESPACE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        "\\s*(?:\\p{Han}|\\p{Hangul}|\\p{Hiragana}|\\p{Katakana}|[\\p{L}'-]+)[\\p{N}\\p{P}]*\\s*",
    )
    .unwrap()
});
pub(crate) static TOKENS_WITHOUT_WHITESPACE: Lazy<Regex> =
    Lazy::new(|| Regex::new("\\p{Han}|\\p{Hangul}|\\p{Hiragana}|\\p{Katakana}|\\p{L}+").unwrap());
