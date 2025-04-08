use ::std::{
    fs,
    fs::File,
    io::BufReader,
    path::Path,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};
use alphabet_detector::{slang_arr_default, Script, ScriptLanguage};
use cap::Cap;
use clap::Parser;
use lingua::{read_iter::ReadCharsChunks, LanguageModelFilesWriter};
// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;
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
    let langs_seen = Arc::new(Mutex::new(slang_arr_default::<bool>()));

    // let point = Arc::new(AtomicBool::new(false));
    for path in paths {
        // files.into_par_iter().for_each(|(file_name, path)| {
        let out_path = args.out.clone();
        // let point = point.clone();
        let langs_seen = langs_seen.clone();
        pool.execute(move || {
            let path = path.unwrap();
            let file_name = path.file_name().into_string().unwrap();
            println!("*{}* New", file_name);
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
                let Some(lang) = ScriptLanguage::from_str(&file_name) else {
                    panic!("*{}* Not found lang", file_name);
                };
                {
                    let mut guard = langs_seen.lock().unwrap();
                    let lang_seen = guard.get_mut(lang as usize).unwrap();
                    if *lang_seen {
                        drop(guard);
                        panic!("*{}* Have already seen lang: {:?}", file_name, lang);
                    }
                    *lang_seen = true;
                }
                // skip in order
                /* if point.load(Ordering::SeqCst) {
                } else if lang == ScriptLanguage::UzbekNorthern {
                    point.store(true, Ordering::SeqCst);
                } else {
                    return;
                } */

                let script = <Option<Script>>::from(lang);
                let langs = script
                    .map(ScriptLanguage::all_with_script)
                    .unwrap_or_default();
                if langs.len() == 1 {
                    println!("*{}* SKIP single lang {:?} in script", file_name, lang);
                    return;
                }
                // TODO: rm this filter
                /* if !matches!(lang, ScriptLanguage::English) {
                    return;
                } */
                // TODO: rm this filter
                /* if alph != "Latn" {
                    return;
                } */

                let out_path = Path::new(&out_path);
                let out_mod_path = out_path.join(lang.into_str());
                if out_mod_path.join("unigrams.encom.br").exists() {
                    println!("*{}* EXISTS {:?}", file_name, lang);
                    return;
                }
                println!("*{}* started {:?}", file_name, lang);

                /* let lines = io::stdin()
                .lines()
                .map(|r| r.unwrap())
                .filter(|line| !line.trim().is_empty()); */

                // let text = fs::read_to_string(path.path()).unwrap();

                let file = BufReader::new(File::open(path.path()).expect("open failed"));
                let ch_iter = file.chars_chunks(b'\n').map(|v| (0, v.unwrap()));
                let result = LanguageModelFilesWriter::create_and_write_language_model(
                    &out_mod_path,
                    ch_iter,
                    lang,
                );
                println!("*{}* done model {:?}", file_name, result);

                /* {
                    let file_path = out_mod_path.join("mod.rs");
                    let mut file = fs::File::create(file_path).unwrap();
                    file.write_all(b"mod unigrams;\nmod bigrams;\nmod trigrams;\nmod quadrigrams;\nmod fivegrams;\n\n")
                        .unwrap();
                    file.write_all(b"pub struct ").unwrap();
                    file.write_all(model_name.as_bytes()).unwrap();
                    file.write_all(b"Model;\n\nimpl crate::Model for ").unwrap();
                    file.write_all(model_name.as_bytes()).unwrap();
                    file.write_all(b"Model {\n").unwrap();
                    file.write_all(
                        b"    #[inline(always)]\n    fn check_unigram(c: char) -> f64 {\n        unigrams::prob(c)\n    }\n",
                    )
                    .unwrap();
                    file.write_all(
                        b"    #[inline(always)]\n    fn check_bigram(g: &[char; 2]) -> f64 {\n        bigrams::prob(g)\n    }\n",
                    )
                    .unwrap();
                    file.write_all(
                        b"    #[inline(always)]\n    fn check_trigram(g: &[char; 3]) -> f64 {\n        trigrams::prob(g)\n    }\n",
                    )
                    .unwrap();
                    file.write_all(
                        b"    #[inline(always)]\n    fn check_quadrigram(g: &[char; 4]) -> f64 {\n        quadrigrams::prob(g)\n    }\n",
                    )
                    .unwrap();
                    file.write_all(
                        b"    #[inline(always)]\n    fn check_fivegram(g: &[char; 5]) -> f64 {\n        fivegrams::prob(g)\n    }\n",
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
                    file.write_all(b"ScriptLanguage::").unwrap();
                    // file.write_all(lang.to_full_dbg().as_bytes()).unwrap();
                    file.write_all(lang.to_string().as_bytes()).unwrap();
                    file.write_all(b" => Some(Box::new(lang_models::").unwrap();
                    file.write_all(model_name.as_bytes()).unwrap();
                    file.write_all(b"Model)),\n").unwrap();
                } */
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
