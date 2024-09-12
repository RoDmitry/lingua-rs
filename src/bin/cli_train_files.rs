use clap::Parser;
use lingua::{
    str_to_alphabets, DetectionResult, IsoCode639_1, Language, LanguageDetector,
    LanguageDetectorBuilder, LanguageModelFilesWriter,
};
use rayon::prelude::*;
use std::fs;
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::str::FromStr;
#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

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

fn main() {
    let args = Args::parse();
    let paths = fs::read_dir(&args.inp).unwrap();
    /* let files: Vec<_> = paths
    .into_iter()
    .map(|p| p.unwrap())
    .map(|path| (path.file_name().into_string().unwrap(), path.path()))
    .collect(); */
    let pool = threadpool::ThreadPool::new(4);

    for path in paths {
        // files.into_par_iter().for_each(|(file_name, path)| {
        let out_path = args.out.clone();
        pool.execute(move || {
            let path = path.unwrap();
            let file_name = path.file_name().into_string().unwrap();
            let thread_id: u8 = rand::random();
            println!("*{}* Name: {}", thread_id, file_name);
            let [lang, alph] = file_name.split('_').collect::<Vec<_>>()[..] else {
                unreachable!()
            };

            let Ok(lang) = Language::from_str(lang) else {
                panic!("*{}* Not found lang: {}", thread_id, lang);
            };
            if lang == Language::Japanese {
                return;
            }

            let alphabets = str_to_alphabets(alph);
            let Some(alphabet) = alphabets
                .iter()
                .find(|a| <&[Language]>::from(**a).contains(&lang))
            else {
                panic!(
                    "*{}* Not found alphabet for lang: {lang} in {:?}",
                    thread_id, alphabets
                );
            };
            println!("*{}* lang: {:?}", thread_id, lang);
            println!("*{}* alphabet: {:?}", thread_id, alphabet);

            /* let lines = io::stdin()
            .lines()
            .map(|r| r.unwrap())
            .filter(|line| !line.trim().is_empty()); */
            let text = fs::read_to_string(path.path()).unwrap();
            // let text = fs::read_to_string(path).unwrap();

            let out_path = out_path + "/" + &lang.to_string() + "/" + &alphabet.to_string();
            let output_directory_path = Path::new(&out_path);
            let result = LanguageModelFilesWriter::create_and_write_language_model(
                output_directory_path,
                text,
                &lang,
            );
            println!("*{}* {:?}", thread_id, result);
        });
    }

    pool.join();
}
