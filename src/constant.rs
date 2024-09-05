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

use ahash::{AHashMap, AHashSet};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::language::Language;

/* pub(crate) static JAPANESE_CHARACTER_SET: Lazy<CharSet> =
Lazy::new(|| CharSet::from_char_classes(&["Hiragana", "Katakana", "Han"])); */
pub(crate) static MULTIPLE_WHITESPACE: Lazy<Regex> = Lazy::new(|| Regex::new("\\s+").unwrap());
pub(crate) static NUMBERS: Lazy<Regex> = Lazy::new(|| Regex::new("\\p{N}").unwrap());
pub(crate) static PUNCTUATION: Lazy<Regex> = Lazy::new(|| Regex::new("\\p{P}").unwrap());
pub(crate) static LETTERS: Lazy<Regex> =
    Lazy::new(|| Regex::new("\\p{Han}|\\p{Hangul}|\\p{Hiragana}|\\p{Katakana}|\\p{L}+").unwrap());
pub(crate) static TOKENS_WITH_OPTIONAL_WHITESPACE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        "\\s*(?:\\p{Han}|\\p{Hangul}|\\p{Hiragana}|\\p{Katakana}|[\\p{L}'-]+)[\\p{N}\\p{P}]*\\s*",
    )
    .unwrap()
});
pub(crate) static TOKENS_WITHOUT_WHITESPACE: Lazy<Regex> =
    Lazy::new(|| Regex::new("\\p{Han}|\\p{Hangul}|\\p{Hiragana}|\\p{Katakana}|\\p{L}+").unwrap());

pub(crate) static CHARS_TO_LANGUAGES_MAPPING: Lazy<AHashMap<&'static str, AHashSet<Language>>> =
    Lazy::new(|| {
        let mut mapping = AHashMap::new();

        if cfg!(feature = "portuguese") || cfg!(feature = "vietnamese") {
            mapping.insert("Ãã", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "portuguese") {
                    languages.insert(Language::Portuguese);
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::Vietnamese);
                }
                languages
            });
        }

        if cfg!(feature = "lithuanian") || cfg!(feature = "polish") {
            mapping.insert("ĄąĘę", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "lithuanian") {
                    languages.insert(Language::Lithuanian);
                }
                if cfg!(feature = "polish") {
                    languages.insert(Language::Polish);
                }
                languages
            });
        }

        if cfg!(feature = "polish") || cfg!(feature = "romanian") {
            mapping.insert("Żż", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "polish") {
                    languages.insert(Language::Polish);
                }
                if cfg!(feature = "romanian") {
                    languages.insert(Language::Romanian);
                }
                languages
            });
        }

        if cfg!(feature = "french") || cfg!(feature = "romanian") {
            mapping.insert("Îî", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "french") {
                    languages.insert(Language::French);
                }
                if cfg!(feature = "romanian") {
                    languages.insert(Language::Romanian);
                }
                languages
            });
        }

        if cfg!(feature = "basque") || cfg!(feature = "spanish") {
            mapping.insert("Ññ", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "basque") {
                    languages.insert(Language::Basque);
                }
                if cfg!(feature = "spanish") {
                    languages.insert(Language::Spanish);
                }
                languages
            });
        }

        if cfg!(feature = "czech") || cfg!(feature = "slovak") {
            mapping.insert("ŇňŤť", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "czech") {
                    languages.insert(Language::Czech);
                }
                if cfg!(feature = "slovak") {
                    languages.insert(Language::Slovak);
                }
                languages
            });
        }

        if cfg!(feature = "romanian") || cfg!(feature = "vietnamese") {
            mapping.insert("Ăă", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "romanian") {
                    languages.insert(Language::Romanian);
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::Vietnamese);
                }
                languages
            });
        }

        if cfg!(feature = "azerbaijani") || cfg!(feature = "turkish") {
            mapping.insert("İıĞğ", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "azerbaijani") {
                    languages.insert(Language::Azerbaijani);
                }
                if cfg!(feature = "turkish") {
                    languages.insert(Language::Turkish);
                }
                languages
            });
        }

        if cfg!(feature = "azerbaijani") || cfg!(feature = "kazakh") {
            mapping.insert("Әә", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "azerbaijani") {
                    languages.insert(Language::Azerbaijani);
                }
                if cfg!(feature = "kazakh") {
                    languages.insert(Language::Kazakh);
                }
                languages
            });
        }

        if cfg!(feature = "macedonian") || cfg!(feature = "serbian") {
            mapping.insert("ЈјЉљЊњ", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "macedonian") {
                    languages.insert(Language::Macedonian);
                }
                if cfg!(feature = "serbian") {
                    languages.insert(Language::Serbian);
                }
                languages
            });
        }

        if cfg!(feature = "vietnamese") || cfg!(feature = "yoruba") {
            mapping.insert("ẸẹỌọ", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::Vietnamese);
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::Yoruba);
                }
                languages
            });
        }

        if cfg!(feature = "icelandic") || cfg!(feature = "turkish") {
            mapping.insert("ÐðÞþ", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "icelandic") {
                    languages.insert(Language::Icelandic);
                }
                if cfg!(feature = "turkish") {
                    languages.insert(Language::Turkish);
                }
                languages
            });
        }

        if cfg!(feature = "french") || cfg!(feature = "hungarian") {
            mapping.insert("Ûû", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "french") {
                    languages.insert(Language::French);
                }
                if cfg!(feature = "hungarian") {
                    languages.insert(Language::Hungarian);
                }
                languages
            });
        }

        if cfg!(feature = "maori") || cfg!(feature = "yoruba") {
            mapping.insert("Ōō", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "maori") {
                    languages.insert(Language::Maori);
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::Yoruba);
                }
                languages
            });
        }

        if cfg!(feature = "kazakh") || cfg!(feature = "mongolian") {
            mapping.insert("ӨөҮү", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "kazakh") {
                    languages.insert(Language::Kazakh);
                }
                if cfg!(feature = "mongolian") {
                    languages.insert(Language::Mongolian);
                }
                languages
            });
        }

        if cfg!(feature = "latvian") || cfg!(feature = "maori") || cfg!(feature = "yoruba") {
            mapping.insert("ĀāĒēĪī", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "latvian") {
                    languages.insert(Language::Latvian);
                }
                if cfg!(feature = "maori") {
                    languages.insert(Language::Maori);
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::Yoruba);
                }
                languages
            });
        }

        if cfg!(feature = "azerbaijani") || cfg!(feature = "romanian") || cfg!(feature = "turkish")
        {
            mapping.insert("Şş", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "azerbaijani") {
                    languages.insert(Language::Azerbaijani);
                }
                if cfg!(feature = "romanian") {
                    languages.insert(Language::Romanian);
                }
                if cfg!(feature = "turkish") {
                    languages.insert(Language::Turkish);
                }
                languages
            });
        }

        if cfg!(feature = "czech") || cfg!(feature = "romanian") || cfg!(feature = "slovak") {
            mapping.insert("Ďď", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "czech") {
                    languages.insert(Language::Czech);
                }
                if cfg!(feature = "romanian") {
                    languages.insert(Language::Romanian);
                }
                if cfg!(feature = "slovak") {
                    languages.insert(Language::Slovak);
                }
                languages
            });
        }

        if cfg!(feature = "bosnian") || cfg!(feature = "croatian") || cfg!(feature = "polish") {
            mapping.insert("Ćć", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "bosnian") {
                    languages.insert(Language::Bosnian);
                }
                if cfg!(feature = "croatian") {
                    languages.insert(Language::Croatian);
                }
                if cfg!(feature = "polish") {
                    languages.insert(Language::Polish);
                }
                languages
            });
        }

        if cfg!(feature = "bosnian") || cfg!(feature = "croatian") || cfg!(feature = "vietnamese") {
            mapping.insert("Đđ", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "bosnian") {
                    languages.insert(Language::Bosnian);
                }
                if cfg!(feature = "croatian") {
                    languages.insert(Language::Croatian);
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::Vietnamese);
                }
                languages
            });
        }

        if cfg!(feature = "belarusian") || cfg!(feature = "kazakh") || cfg!(feature = "ukrainian") {
            mapping.insert("Іі", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "belarusian") {
                    languages.insert(Language::Belarusian);
                }
                if cfg!(feature = "kazakh") {
                    languages.insert(Language::Kazakh);
                }
                if cfg!(feature = "ukrainian") {
                    languages.insert(Language::Ukrainian);
                }
                languages
            });
        }

        if cfg!(feature = "italian") || cfg!(feature = "vietnamese") || cfg!(feature = "yoruba") {
            mapping.insert("Ìì", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "italian") {
                    languages.insert(Language::Italian);
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::Vietnamese);
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::Yoruba);
                }
                languages
            });
        }

        if cfg!(feature = "bokmal") || cfg!(feature = "danish") || cfg!(feature = "nynorsk") {
            mapping.insert("Øø", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "bokmal") {
                    languages.insert(Language::Bokmal);
                }
                if cfg!(feature = "danish") {
                    languages.insert(Language::Danish);
                }
                if cfg!(feature = "nynorsk") {
                    languages.insert(Language::Nynorsk);
                }
                languages
            });
        }

        if cfg!(feature = "latvian")
            || cfg!(feature = "lithuanian")
            || cfg!(feature = "maori")
            || cfg!(feature = "yoruba")
        {
            mapping.insert("Ūū", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "latvian") {
                    languages.insert(Language::Latvian);
                }
                if cfg!(feature = "lithuanian") {
                    languages.insert(Language::Lithuanian);
                }
                if cfg!(feature = "maori") {
                    languages.insert(Language::Maori);
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::Yoruba);
                }
                languages
            });
        }

        if cfg!(feature = "afrikaans")
            || cfg!(feature = "albanian")
            || cfg!(feature = "dutch")
            || cfg!(feature = "french")
        {
            mapping.insert("Ëë", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "afrikaans") {
                    languages.insert(Language::Afrikaans);
                }
                if cfg!(feature = "albanian") {
                    languages.insert(Language::Albanian);
                }
                if cfg!(feature = "dutch") {
                    languages.insert(Language::Dutch);
                }
                if cfg!(feature = "french") {
                    languages.insert(Language::French);
                }
                languages
            });
        }

        if cfg!(feature = "french")
            || cfg!(feature = "italian")
            || cfg!(feature = "vietnamese")
            || cfg!(feature = "yoruba")
        {
            mapping.insert("ÈèÙù", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "french") {
                    languages.insert(Language::French);
                }
                if cfg!(feature = "italian") {
                    languages.insert(Language::Italian);
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::Vietnamese);
                }
                if cfg!(feature = "yoruba") {
                    languages.insert(Language::Yoruba);
                }
                languages
            });
        }

        if cfg!(feature = "afrikaans")
            || cfg!(feature = "french")
            || cfg!(feature = "portuguese")
            || cfg!(feature = "vietnamese")
        {
            mapping.insert("Êê", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "afrikaans") {
                    languages.insert(Language::Afrikaans);
                }
                if cfg!(feature = "french") {
                    languages.insert(Language::French);
                }
                if cfg!(feature = "portuguese") {
                    languages.insert(Language::Portuguese);
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::Vietnamese);
                }
                languages
            });
        }

        if cfg!(feature = "estonian")
            || cfg!(feature = "hungarian")
            || cfg!(feature = "portuguese")
            || cfg!(feature = "vietnamese")
        {
            mapping.insert("Õõ", {
                let mut languages = AHashSet::new();
                if cfg!(feature = "estonian") {
                    languages.insert(Language::Estonian);
                }
                if cfg!(feature = "hungarian") {
                    languages.insert(Language::Hungarian);
                }
                if cfg!(feature = "portuguese") {
                    languages.insert(Language::Portuguese);
                }
                if cfg!(feature = "vietnamese") {
                    languages.insert(Language::Vietnamese);
                }
                languages
            });

            if cfg!(feature = "french")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "slovak")
                || cfg!(feature = "vietnamese")
            {
                mapping.insert("Ôô", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "french") {
                        languages.insert(Language::French);
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::Portuguese);
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::Slovak);
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::Vietnamese);
                    }
                    languages
                });
            }

            if cfg!(feature = "belarusian")
                || cfg!(feature = "kazakh")
                || cfg!(feature = "mongolian")
                || cfg!(feature = "russian")
            {
                mapping.insert("ЁёЫыЭэ", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "belarusian") {
                        languages.insert(Language::Belarusian);
                    }
                    if cfg!(feature = "kazakh") {
                        languages.insert(Language::Kazakh);
                    }
                    if cfg!(feature = "mongolian") {
                        languages.insert(Language::Mongolian);
                    }
                    if cfg!(feature = "russian") {
                        languages.insert(Language::Russian);
                    }
                    languages
                });
            }

            if cfg!(feature = "bulgarian")
                || cfg!(feature = "kazakh")
                || cfg!(feature = "mongolian")
                || cfg!(feature = "russian")
            {
                mapping.insert("ЩщЪъ", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "bulgarian") {
                        languages.insert(Language::Bulgarian);
                    }
                    if cfg!(feature = "kazakh") {
                        languages.insert(Language::Kazakh);
                    }
                    if cfg!(feature = "mongolian") {
                        languages.insert(Language::Mongolian);
                    }
                    if cfg!(feature = "russian") {
                        languages.insert(Language::Russian);
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "italian")
                || cfg!(feature = "vietnamese")
                || cfg!(feature = "yoruba")
            {
                mapping.insert("Òò", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::Catalan);
                    }
                    if cfg!(feature = "italian") {
                        languages.insert(Language::Italian);
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::Vietnamese);
                    }
                    if cfg!(feature = "yoruba") {
                        languages.insert(Language::Yoruba);
                    }
                    languages
                });
            }

            if cfg!(feature = "french")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "romanian")
                || cfg!(feature = "turkish")
                || cfg!(feature = "vietnamese")
            {
                mapping.insert("Ââ", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "french") {
                        languages.insert(Language::French);
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::Portuguese);
                    }
                    if cfg!(feature = "romanian") {
                        languages.insert(Language::Romanian);
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::Turkish);
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::Vietnamese);
                    }
                    languages
                });
            }

            if cfg!(feature = "bokmal")
                || cfg!(feature = "danish")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "nynorsk")
            {
                mapping.insert("Ææ", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "bokmal") {
                        languages.insert(Language::Bokmal);
                    }
                    if cfg!(feature = "danish") {
                        languages.insert(Language::Danish);
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::Icelandic);
                    }
                    if cfg!(feature = "nynorsk") {
                        languages.insert(Language::Nynorsk);
                    }
                    languages
                });
            }

            if cfg!(feature = "bokmal")
                || cfg!(feature = "danish")
                || cfg!(feature = "nynorsk")
                || cfg!(feature = "swedish")
            {
                mapping.insert("Åå", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "bokmal") {
                        languages.insert(Language::Bokmal);
                    }
                    if cfg!(feature = "danish") {
                        languages.insert(Language::Danish);
                    }
                    if cfg!(feature = "nynorsk") {
                        languages.insert(Language::Nynorsk);
                    }
                    if cfg!(feature = "swedish") {
                        languages.insert(Language::Swedish);
                    }
                    languages
                });
            }

            if cfg!(feature = "czech")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "slovak")
                || cfg!(feature = "turkish")
                || cfg!(feature = "vietnamese")
            {
                mapping.insert("Ýý", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "czech") {
                        languages.insert(Language::Czech);
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::Icelandic);
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::Slovak);
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::Turkish);
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::Vietnamese);
                    }
                    languages
                });
            }

            if cfg!(feature = "estonian")
                || cfg!(feature = "finnish")
                || cfg!(feature = "german")
                || cfg!(feature = "slovak")
                || cfg!(feature = "swedish")
            {
                mapping.insert("Ää", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "estonian") {
                        languages.insert(Language::Estonian);
                    }
                    if cfg!(feature = "finnish") {
                        languages.insert(Language::Finnish);
                    }
                    if cfg!(feature = "german") {
                        languages.insert(Language::German);
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::Slovak);
                    }
                    if cfg!(feature = "swedish") {
                        languages.insert(Language::Swedish);
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "french")
                || cfg!(feature = "italian")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "vietnamese")
            {
                mapping.insert("Àà", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::Catalan);
                    }
                    if cfg!(feature = "french") {
                        languages.insert(Language::French);
                    }
                    if cfg!(feature = "italian") {
                        languages.insert(Language::Italian);
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::Portuguese);
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::Vietnamese);
                    }
                    languages
                });
            }

            if cfg!(feature = "azerbaijani")
                || cfg!(feature = "catalan")
                || cfg!(feature = "estonian")
                || cfg!(feature = "german")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "spanish")
                || cfg!(feature = "turkish")
            {
                mapping.insert("Üü", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "azerbaijani") {
                        languages.insert(Language::Azerbaijani);
                    }
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::Catalan);
                    }
                    if cfg!(feature = "estonian") {
                        languages.insert(Language::Estonian);
                    }
                    if cfg!(feature = "german") {
                        languages.insert(Language::German);
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::Hungarian);
                    }
                    if cfg!(feature = "spanish") {
                        languages.insert(Language::Spanish);
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::Turkish);
                    }
                    languages
                });
            }

            if cfg!(feature = "bosnian")
                || cfg!(feature = "czech")
                || cfg!(feature = "croatian")
                || cfg!(feature = "latvian")
                || cfg!(feature = "lithuanian")
                || cfg!(feature = "slovak")
                || cfg!(feature = "slovene")
            {
                mapping.insert("ČčŠšŽž", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "bosnian") {
                        languages.insert(Language::Bosnian);
                    }
                    if cfg!(feature = "czech") {
                        languages.insert(Language::Czech);
                    }
                    if cfg!(feature = "croatian") {
                        languages.insert(Language::Croatian);
                    }
                    if cfg!(feature = "latvian") {
                        languages.insert(Language::Latvian);
                    }
                    if cfg!(feature = "lithuanian") {
                        languages.insert(Language::Lithuanian);
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::Slovak);
                    }
                    if cfg!(feature = "slovene") {
                        languages.insert(Language::Slovene);
                    }
                    languages
                });
            }

            if cfg!(feature = "albanian")
                || cfg!(feature = "azerbaijani")
                || cfg!(feature = "basque")
                || cfg!(feature = "catalan")
                || cfg!(feature = "french")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "turkish")
            {
                mapping.insert("Çç", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "albanian") {
                        languages.insert(Language::Albanian);
                    }
                    if cfg!(feature = "azerbaijani") {
                        languages.insert(Language::Azerbaijani);
                    }
                    if cfg!(feature = "basque") {
                        languages.insert(Language::Basque);
                    }
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::Catalan);
                    }
                    if cfg!(feature = "french") {
                        languages.insert(Language::French);
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::Portuguese);
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::Turkish);
                    }
                    languages
                });
            }

            if cfg!(feature = "azerbaijani")
                || cfg!(feature = "estonian")
                || cfg!(feature = "finnish")
                || cfg!(feature = "german")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "swedish")
                || cfg!(feature = "turkish")
            {
                mapping.insert("Öö", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "azerbaijani") {
                        languages.insert(Language::Azerbaijani);
                    }
                    if cfg!(feature = "estonian") {
                        languages.insert(Language::Estonian);
                    }
                    if cfg!(feature = "finnish") {
                        languages.insert(Language::Finnish);
                    }
                    if cfg!(feature = "german") {
                        languages.insert(Language::German);
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::Hungarian);
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::Icelandic);
                    }
                    if cfg!(feature = "swedish") {
                        languages.insert(Language::Swedish);
                    }
                    if cfg!(feature = "turkish") {
                        languages.insert(Language::Turkish);
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "irish")
                || cfg!(feature = "polish")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "slovak")
                || cfg!(feature = "spanish")
                || cfg!(feature = "vietnamese")
                || cfg!(feature = "yoruba")
            {
                mapping.insert("Óó", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::Catalan);
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::Hungarian);
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::Icelandic);
                    }
                    if cfg!(feature = "irish") {
                        languages.insert(Language::Irish);
                    }
                    if cfg!(feature = "polish") {
                        languages.insert(Language::Polish);
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::Portuguese);
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::Slovak);
                    }
                    if cfg!(feature = "spanish") {
                        languages.insert(Language::Spanish);
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::Vietnamese);
                    }
                    if cfg!(feature = "yoruba") {
                        languages.insert(Language::Yoruba);
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "czech")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "irish")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "slovak")
                || cfg!(feature = "spanish")
                || cfg!(feature = "vietnamese")
                || cfg!(feature = "yoruba")
            {
                mapping.insert("ÁáÍíÚú", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::Catalan);
                    }
                    if cfg!(feature = "czech") {
                        languages.insert(Language::Czech);
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::Icelandic);
                    }
                    if cfg!(feature = "irish") {
                        languages.insert(Language::Irish);
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::Hungarian);
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::Portuguese);
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::Slovak);
                    }
                    if cfg!(feature = "spanish") {
                        languages.insert(Language::Spanish);
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::Vietnamese);
                    }
                    if cfg!(feature = "yoruba") {
                        languages.insert(Language::Yoruba);
                    }
                    languages
                });
            }

            if cfg!(feature = "catalan")
                || cfg!(feature = "czech")
                || cfg!(feature = "french")
                || cfg!(feature = "hungarian")
                || cfg!(feature = "icelandic")
                || cfg!(feature = "irish")
                || cfg!(feature = "italian")
                || cfg!(feature = "portuguese")
                || cfg!(feature = "slovak")
                || cfg!(feature = "spanish")
                || cfg!(feature = "vietnamese")
                || cfg!(feature = "yoruba")
            {
                mapping.insert("Éé", {
                    let mut languages = AHashSet::new();
                    if cfg!(feature = "catalan") {
                        languages.insert(Language::Catalan);
                    }
                    if cfg!(feature = "czech") {
                        languages.insert(Language::Czech);
                    }
                    if cfg!(feature = "french") {
                        languages.insert(Language::French);
                    }
                    if cfg!(feature = "hungarian") {
                        languages.insert(Language::Hungarian);
                    }
                    if cfg!(feature = "icelandic") {
                        languages.insert(Language::Icelandic);
                    }
                    if cfg!(feature = "irish") {
                        languages.insert(Language::Irish);
                    }
                    if cfg!(feature = "italian") {
                        languages.insert(Language::Italian);
                    }
                    if cfg!(feature = "portuguese") {
                        languages.insert(Language::Portuguese);
                    }
                    if cfg!(feature = "slovak") {
                        languages.insert(Language::Slovak);
                    }
                    if cfg!(feature = "spanish") {
                        languages.insert(Language::Spanish);
                    }
                    if cfg!(feature = "vietnamese") {
                        languages.insert(Language::Vietnamese);
                    }
                    if cfg!(feature = "yoruba") {
                        languages.insert(Language::Yoruba);
                    }
                    languages
                });
            }
        }

        mapping
    });
