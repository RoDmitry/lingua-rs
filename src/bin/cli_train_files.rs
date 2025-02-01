use ::std::fs;
use ::std::io::Write;
use ::std::path::Path;
use ::std::str::FromStr;
use ::std::sync::Arc;
use ::std::sync::Mutex;
use ::std::{thread, time::Duration};
use cap::Cap;
use clap::Parser;
// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;
use lingua::{lang_arr_default, script_char_to_langs, Language, LanguageModelFilesWriter, Script};
// use rayon::prelude::*;

// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static ALLOCATOR: Cap<Jemalloc> = Cap::new(Jemalloc, usize::MAX);
// static ALLOCATOR: Jemalloc = Jemalloc;

// #[cfg(target_env = "msvc")]
#[global_allocator]
static ALLOCATOR: Cap<::std::alloc::System> = Cap::new(::std::alloc::System, usize::MAX);

// #[cfg(not(target_env = "msvc"))]
// #[global_allocator]
// static GLOBAL: Jemalloc = Jemalloc;

fn str_to_langs(s: &str) -> &[Language] {
    match s {
        "Arab" => script_char_to_langs(Script::Arabic, char::default()),
        "Armn" => script_char_to_langs(Script::Armenian, char::default()),
        "Beng" => script_char_to_langs(Script::Bengali, char::default()),
        "Cyrl" => script_char_to_langs(Script::Cyrillic, char::default()),
        "Deva" => script_char_to_langs(Script::Devanagari, char::default()),
        "Ethi" => script_char_to_langs(Script::Ethiopic, char::default()),
        "Geor" => script_char_to_langs(Script::Georgian, char::default()),
        "Grek" => script_char_to_langs(Script::Greek, char::default()),
        "Gujr" => script_char_to_langs(Script::Gujarati, char::default()),
        "Guru" => script_char_to_langs(Script::Gurmukhi, char::default()),
        "Hang" => script_char_to_langs(Script::Hangul, char::default()),
        "Hans" => &[Language::ChineseSimplified],
        "Hant" => &[
            Language::ChineseTraditional,
            Language::ChineseCantoneseTraditional,
        ],
        "Hebr" => script_char_to_langs(Script::Hebrew, char::default()),
        "Jpan" => &[Language::Japanese],
        "Khmr" => script_char_to_langs(Script::Khmer, char::default()),
        "Knda" => script_char_to_langs(Script::Kannada, char::default()),
        "Laoo" => script_char_to_langs(Script::Lao, char::default()),
        "Latn" => script_char_to_langs(Script::Latin, char::default()),
        "Mlym" => script_char_to_langs(Script::Malayalam, char::default()),
        "Mymr" => script_char_to_langs(Script::Myanmar, char::default()),
        "Olck" => script_char_to_langs(Script::OlChiki, char::default()),
        "Orya" => script_char_to_langs(Script::Oriya, char::default()),
        "Sinh" => script_char_to_langs(Script::Sinhala, char::default()),
        "Taml" => script_char_to_langs(Script::Tamil, char::default()),
        "Telu" => script_char_to_langs(Script::Telugu, char::default()),
        "Tfng" => script_char_to_langs(Script::Tifinagh, char::default()),
        "Thai" => script_char_to_langs(Script::Thai, char::default()),
        "Tibt" => script_char_to_langs(Script::Tibetan, char::default()),
        _ => unreachable!(),
    }
}

#[derive(Parser)]
#[command(version, about)]
struct Args {
    #[arg(short = 'i', required = true)]
    inp: String,

    #[arg(short = 'o', required = true)]
    out: String,

    /// List of iso-639-1 language codes
    #[arg(
        short,
        long,
        help = "comma seperated list of iso-639-1 codes of languages to detect, if not specified, all supported language will be used. Setting this improves accuracy and resource usage.",
        num_args = 1,
        value_delimiter = ',',
        required = false
    )]
    languages: Vec<String>,

    #[arg(
        short = 'n',
        long,
        help = "Classify language per line, this only works if text is not supplied directly as an argument"
    )]
    per_line: bool,

    #[arg(short = 'L', long, help = "List supported languages")]
    list: bool,

    #[arg(
        short,
        long,
        help = "Show all confidence values (entire probability distribution), rather than just the winning score. Does not work with --multi"
    )]
    all: bool,

    #[arg(short = 'q', long, help = "Quick/low accuracy mode")]
    quick: bool,

    #[arg(
        short = 'm',
        long,
        help = "Classify multiple languages in mixed texts, will return matches along with UTF-8 byte offsets. Can not be combined with line mode."
    )]
    multi: bool,

    #[arg(
        short = 'c',
        long,
        help = "Confidence threshold, only output results with at least this confidence value (0.0-1.0)"
    )]
    confidence: Option<f64>,

    #[arg(
        short = 'M',
        long,
        help = "Minimum text length (without regard for whitespace, punctuation or numerals!). Shorter fragments will be classified as 'unknown'"
    )]
    minlength: Option<u8>,

    #[arg(short = 'p', help = "preload models")]
    preload: bool,

    #[arg(short, long, default_value = "\t")]
    delimiter: String,

    #[arg(required = false)]
    text: Vec<String>,
}

const THREADS: usize = 8;
const MEM_LIMIT_SLEEP: usize = 6 * 1024 * 1024 * 1024;
// use std::sync::atomic::AtomicBool;
// use std::sync::atomic::Ordering;
// use std::sync::Arc;
fn main() {
    let args = Args::parse();
    let paths = fs::read_dir(&args.inp).unwrap();
    /* let files: Vec<_> = paths
    .into_iter()
    .map(|p| p.unwrap())
    .map(|path| (path.file_name().into_string().unwrap(), path.path()))
    .collect(); */
    let pool = threadpool::ThreadPool::new(THREADS);
    let langs_seen = Arc::new(Mutex::new(lang_arr_default::<bool>()));

    // let point = Arc::new(AtomicBool::new(false));
    for path in paths {
        // files.into_par_iter().for_each(|(file_name, path)| {
        let out_path = args.out.clone();
        // let point = point.clone();
        let langs_seen = langs_seen.clone();
        pool.execute(move || {
            let path = path.unwrap();
            let file_name = path.file_name().into_string().unwrap();
            println!("New: {}", file_name);
            while ALLOCATOR.allocated() > MEM_LIMIT_SLEEP {
                println!(
                    "*{}* Mem allocated: {}MB Sleeping...",
                    file_name,
                    ALLOCATOR.allocated() / (1024 * 1024)
                );
                let time = Duration::from_secs(30);
                thread::sleep(time);
            }
            println!(
                "*{}* Mem allocated: {}MB",
                file_name,
                ALLOCATOR.allocated() / (1024 * 1024)
            );
            {
                let [lang, alph] = file_name.split('_').collect::<Vec<_>>()[..] else {
                    unreachable!()
                };
                let lang = match Language::from_str(&file_name)  {
                    Ok(l) => l,
                    _ => {
                        let Ok(l) = Language::from_str(lang) else {
                            panic!("*{}* Not found lang: {}", file_name, lang);
                        };
                        l
                    }
                };
                {
                    let mut guard = langs_seen.lock().unwrap();
                    let lang_seen = guard.get_mut(lang as usize).unwrap();
                    if *lang_seen {
                        panic!("*{}* Have already seen lang: {}", file_name, lang);
                    }
                    *lang_seen = true;
                }
                // skip in order
                /* if point.load(Ordering::SeqCst) {
                } else if lang == Language::UzbekNorthern {
                    point.store(true, Ordering::SeqCst);
                } else {
                    return;
                } */

                let langs = str_to_langs(alph);
                if !langs.contains(&lang) {
                    panic!(
                        "*{}* Not found lang: {lang} in {:?}",
                        file_name, langs
                    );
                };
                // TODO: rm this filter
                /* if !matches!(lang, Language::English) {
                    return;
                } */
                // TODO: rm this filter
                if alph != "Latn" {
                    return;
                }

                let model_name = lang.to_string();
                let mod_dir = stringcase::snake_case(&model_name);

                let out_path = Path::new(&out_path);
                let out_mod_path = out_path.join(&mod_dir);
                if out_mod_path.join("unigrams.rs").exists() {
                    println!("EXISTS {} {:?}", file_name, lang);
                    return;
                }
                println!("*{}* started {:?}", file_name, lang);

                /* let lines = io::stdin()
                .lines()
                .map(|r| r.unwrap())
                .filter(|line| !line.trim().is_empty()); */
                let text = fs::read_to_string(path.path()).unwrap();
                let result = LanguageModelFilesWriter::create_and_write_language_model(
                    &out_mod_path,
                    text,
                    lang,
                );
                println!("*{}* {:?}", file_name, result);

                {
                    let file_path = out_mod_path.join("mod.rs");
                    let mut file = fs::File::create(file_path).unwrap();
                    file.write_all(b"mod bigrams;\nmod trigrams;\nmod unigrams;\n\n")
                        .unwrap();
                    file.write_all(b"pub struct ").unwrap();
                    file.write_all(model_name.as_bytes()).unwrap();
                    file.write_all(b"Model;\n\nimpl crate::Model for ").unwrap();
                    file.write_all(model_name.as_bytes()).unwrap();
                    file.write_all(b"Model {\n").unwrap();
                    file.write_all(
                        b"    fn check_unigram(c: char) -> f64 {\n        unigrams::prob(c)\n    }\n",
                    )
                    .unwrap();
                    file.write_all(
                        b"    fn check_bigram(g: &[char; 2]) -> f64 {\n        bigrams::prob(g)\n    }\n",
                    )
                    .unwrap();
                    file.write_all(
                        b"    fn check_trigram(g: &[char; 3]) -> f64 {\n        trigrams::prob(g)\n    }\n",
                    )
                    .unwrap();
                    file.write_all(b"}\n").unwrap();
                }

                {
                    let file_path = out_path.join("lib.rs");
                    let mut file = fs::File::options().append(true).open(file_path).unwrap();
                    file.write_all(b"mod ").unwrap();
                    file.write_all(mod_dir.as_bytes()).unwrap();
                    file.write_all(b";\n").unwrap();
                    file.write_all(b"pub use ").unwrap();
                    file.write_all(mod_dir.as_bytes()).unwrap();
                    file.write_all(b"::*;\n").unwrap();
                }

                {
                    let file_path = out_path.join("macros.rs");
                    let mut file = fs::File::options().append(true).open(file_path).unwrap();
                    // file.write_all(b"(Alphabet::").unwrap();
                    // file.write_all(alphabet.to_full_dbg().as_bytes()).unwrap();
                    file.write_all(b"Language::").unwrap();
                    file.write_all(lang.to_string().as_bytes()).unwrap();
                    file.write_all(b" => Some(Box::new(parselang_models::").unwrap();
                    file.write_all(model_name.as_bytes()).unwrap();
                    file.write_all(b"Model)),\n").unwrap();
                }
            }
            println!(
                "*{}* malloc_trim {:?} {:?}MB",
                file_name,
                unsafe { libc::malloc_trim(0) },
                ALLOCATOR.allocated() / (1024 * 1024)
            );
        });
    }

    pool.join();
}
