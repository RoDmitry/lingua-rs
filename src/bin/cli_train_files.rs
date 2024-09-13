use ::std::fs;
use ::std::io::{self, BufRead, Read};
use ::std::path::Path;
use ::std::str::FromStr;
use ::std::{thread, time::Duration};
use cap::Cap;
use clap::Parser;
// #[cfg(not(target_env = "msvc"))]
// use jemallocator::Jemalloc;
use lingua::{
    str_to_alphabets, Alphabet, DetectionResult, IsoCode639_1, Language, LanguageDetector,
    LanguageDetectorBuilder, LanguageModelFilesWriter,
};
use rayon::prelude::*;

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
const MEM_MIN_USAGE: usize = 6 * 1024 * 1024 * 1024;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
fn main() {
    let args = Args::parse();
    let paths = fs::read_dir(&args.inp).unwrap();
    /* let files: Vec<_> = paths
    .into_iter()
    .map(|p| p.unwrap())
    .map(|path| (path.file_name().into_string().unwrap(), path.path()))
    .collect(); */
    let pool = threadpool::ThreadPool::new(THREADS);

    let point = Arc::new(AtomicBool::new(false));
    for path in paths {
        // files.into_par_iter().for_each(|(file_name, path)| {
        let out_path = args.out.clone();
        let point = point.clone();
        pool.execute(move || {
            let path = path.unwrap();
            let file_name = path.file_name().into_string().unwrap();
            while ALLOCATOR.allocated() > MEM_MIN_USAGE {
                println!("*{}* Mem allocated: {}MB Sleeping...", file_name, ALLOCATOR.allocated() / (1024 * 1024));
                let time = Duration::from_secs(30);
                thread::sleep(time);
            }
            println!(
                "*{}* Mem allocated: {}MB",
                file_name,
                ALLOCATOR.allocated() / (1024 * 1024)
            );
            {
                println!("Name: {}", file_name);
                let [lang, alph] = file_name.split('_').collect::<Vec<_>>()[..] else {
                    unreachable!()
                };

                let Ok(lang) = Language::from_str(lang) else {
                    panic!("*{}* Not found lang: {}", file_name, lang);
                };
                // skip in order
                if point.load(Ordering::SeqCst) {
                } else if lang == Language::UzbekNorthern {
                    point.store(true, Ordering::SeqCst);
                } else {
                    return;
                }

                let alphabets = str_to_alphabets(alph);
                let Some(alphabet) = alphabets
                    .iter()
                    .find(|a| <&[Language]>::from(**a).contains(&lang))
                else {
                    panic!(
                        "*{}* Not found alphabet for lang: {lang} in {:?}",
                        file_name, alphabets
                    );
                };
                if !matches!(alphabet, Alphabet::Latin(_)) {
                    return;
                }
                println!("*{}* lang: {:?}", file_name, lang);
                println!("*{}* alphabet: {:?}", file_name, alphabet);

                /* let lines = io::stdin()
                .lines()
                .map(|r| r.unwrap())
                .filter(|line| !line.trim().is_empty()); */
                let text = fs::read_to_string(path.path()).unwrap();

                let out_path = out_path + "/" + &lang.to_string() + "/" + &alphabet.to_string();
                let output_directory_path = Path::new(&out_path);
                let result = LanguageModelFilesWriter::create_and_write_language_model(
                    output_directory_path,
                    text,
                    &lang,
                );
                println!("*{}* {:?}", file_name, result);
            }
            println!(
                "*{}* malloc_trim {:?} {:?}MB",
                file_name,
                unsafe { libc::malloc_trim(0) }, ALLOCATOR.allocated() / (1024 * 1024)
            );
        });
    }

    pool.join();
}
