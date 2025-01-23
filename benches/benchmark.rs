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

use ::std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use lingua::{langs_count_max, Language, LanguageDetectorBuilder};
use rayon::prelude::*;

// This is the common subset of languages that is supported by all
// language detection libraries in this benchmark.
static COMMON_LANGUAGES: [Language; 15] = [
    Language::Arabic,
    // Language::ChineseSimplified,
    Language::Dutch,
    Language::English,
    Language::French,
    Language::German,
    Language::Hindi,
    Language::Italian,
    Language::Japanese,
    Language::Korean,
    Language::Portuguese,
    Language::Russian,
    Language::Spanish,
    Language::Swedish,
    Language::Turkish,
    Language::Vietnamese,
];

static SENTENCES: [&str; 16] = [
    "و في نفس الوقت أقول بأن الشيخ صالح لم يشر إلى مسؤولية الدولة التي تسمح لمواطنيها بملكية قنوات تبث ما تبث بل إنه حصر المسؤولية على ملاك هذه القنوات.",
    "102年度彰化县劳工运动会暨园游会于12月1日(星期日)上午在县立体育场盛大登场，来自全县共61个事业单位及职业工会超过3,000多位选手参加，运动会场将展开一系列的竞技对战。",
    "Aan de fysieke gesteldheid van de aspirant-beoefenaar worden geen bijzondere eisen gesteld anders dan een goede gezondheid.",
    "Here, in a region abundant with natural beauty, golfers will surely be rewarded with an exceptional golf experience.",
    "Les affranchissements étaient très rares et s'ils accordaient la liberté à l'ancien esclave, ils ne lui conféraient pas le titre de citoyen.",
    "Natürlich war sie kein Pferd, dachte sie, aber warum wurde sie dann geritten, hatte einen Reiter zu tragen, war gesattelt, bekam Sporen und Lederpeitsche?",
    "अब इन्हें एक अलग प्लेट में निकाल कर गरमा-गरम आलू की सब्जी, हरे धनिये की चटनी या मीठी चटनी के साथ परोस कर खाइये और सबको खिलाइये।",
    "Alla fine del secolo cambiarono nome, divenendo uno Capitano e l’altro Difensore, ma mantenendo le stesse caratteristiche degli anni precedenti.",
    "・京都大学施設に電離圏における電子数などの状況を取得可能なイオノゾンデ受信機（斜入射観測装置）を設置することで、新たな観測手法が地震先行現象検出に資するかを検証する。",
    "아울러 가장 많은 수가 일하고 있는 직업은 곡식작물 재배자(109만6천명)로 조사됐고, 상점판매 및 관리인(97만8천명), 상점판매원(87만3천명), 일반 영업원(59만명) 등이 뒤를 이었다.",
    "Dizer que não estou, significaria explicar porquê e não me apetece nada desfiar o rosário das minhas lamentações.",
    "То есть присяжные не сочли возможным осудить на соучастие в убийстве и убийство людей, доказательства вины которых не были предъявлены.",
    "Con frecuencia creo que Francia es malinterpretada, seala, aludiendo a la imagen que tiene el pas internacionalmente en materia de tica de trabajo.",
    "Med dagens stadshusmajoritet är det övervikt för ett enplanstorg med bostäder, alltså för en ombyggnad i linje med alternativ maxi.",
    "Mezuniyet hediyesi olarak yerleşkenin kuzey batı bölümüne dikilmiş vişnelerin meyvesini, tohumunu almışlardır.",
    "Cuốn sách là cẩm nang hữu ích để tham khảo và học hỏi, giúp các bà mẹ Việt tự tin hơn trong cách dạy con.",
];

/* fn benchmark_preloading_all_language_models(c: &mut Criterion) {
    let mut group = c.benchmark_group("language models");
    group.sample_size(10);
    group.bench_function("preload all language models", |bencher| {
        bencher.iter(|| {
            let detector = LanguageDetectorBuilder::from_all_languages()
                .with_preloaded_language_models()
                .build();
            detector.unload_language_models();
        })
    });
} */

fn benchmark_filter(c: &mut Criterion) {
    let sentences = SENTENCES.repeat(125);

    let mut group1 = c.benchmark_group("Filter");
    group1.bench_function("run", |bencher| {
        bencher.iter(|| {
            sentences.iter().for_each(|sentence| {
                let found_words = lingua::from_ch_iter(sentence.char_indices());
                for wd in found_words {
                    let langs = langs_count_max(wd.langs_cnt).0;
                    if langs.is_empty() {
                        panic!("empty langs");
                    }
                }
            });
        });
    });
    group1.finish();
}

fn benchmark_detector(c: &mut Criterion) {
    let low_accuracy_detector_for_all_languages = LanguageDetectorBuilder::from_all_languages()
        .with_low_accuracy_mode()
        .with_preloaded_language_models()
        .build();

    let high_accuracy_detector_for_all_languages = LanguageDetectorBuilder::from_all_languages()
        .with_preloaded_language_models()
        .build();

    let low_accuracy_detector_for_common_languages =
        LanguageDetectorBuilder::from_languages(&COMMON_LANGUAGES)
            .with_low_accuracy_mode()
            .with_preloaded_language_models()
            .build();

    let high_accuracy_detector_for_common_languages =
        LanguageDetectorBuilder::from_languages(&COMMON_LANGUAGES)
            .with_preloaded_language_models()
            .build();

    let sentences = SENTENCES.repeat(125);

    let mut group1 = c.benchmark_group("Detector with all languages in single thread");
    group1.bench_function("low accuracy mode", |bencher| {
        bencher.iter(|| {
            sentences.iter().for_each(|sentence| {
                black_box(low_accuracy_detector_for_all_languages.detect_language_of(*sentence));
            });
        });
    });
    group1.bench_function("high accuracy mode", |bencher| {
        bencher.iter(|| {
            sentences.iter().for_each(|sentence| {
                black_box(high_accuracy_detector_for_all_languages.detect_language_of(*sentence));
            });
        });
    });
    group1.finish();

    /* let mut group2 = c.benchmark_group("Detector with all languages in multiple threads");
    group2.bench_function("low accuracy mode", |bencher| {
        bencher.iter(|| {
            sentences.par_iter().for_each(|sentence| {
                black_box(low_accuracy_detector_for_all_languages.detect_language_of(*sentence));
            });
        });
    });
    group2.bench_function("high accuracy mode", |bencher| {
        bencher.iter(|| {
            sentences.par_iter().for_each(|sentence| {
                black_box(high_accuracy_detector_for_all_languages.detect_language_of(*sentence));
            });
        });
    });
    group2.finish(); */

    let mut group3 = c.benchmark_group("Detector with common languages in single thread");
    group3.bench_function("low accuracy mode", |bencher| {
        bencher.iter(|| {
            sentences.iter().for_each(|sentence| {
                black_box(low_accuracy_detector_for_common_languages.detect_language_of(*sentence));
            });
        });
    });
    group3.bench_function("high accuracy mode", |bencher| {
        bencher.iter(|| {
            sentences.iter().for_each(|sentence| {
                black_box(
                    high_accuracy_detector_for_common_languages.detect_language_of(*sentence),
                );
            });
        });
    });
    group3.finish();

    /* let mut group4 = c.benchmark_group("Detector with common languages in multiple threads");
    group4.bench_function("low accuracy mode", |bencher| {
        bencher.iter(|| {
            sentences.par_iter().for_each(|sentence| {
                black_box(low_accuracy_detector_for_common_languages.detect_language_of(*sentence));
            });
        });
    });
    group4.bench_function("high accuracy mode", |bencher| {
        bencher.iter(|| {
            sentences.par_iter().for_each(|sentence| {
                black_box(
                    high_accuracy_detector_for_common_languages.detect_language_of(*sentence),
                );
            });
        });
    });
    group4.finish(); */
}

criterion_group!(
    benches,
    // benchmark_preloading_all_language_models,
    benchmark_filter,
    benchmark_detector,
);

criterion_main!(benches);
