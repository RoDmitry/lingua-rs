use clap::Parser;
use lingua::{
    DetectionResult, IsoCode639_1, Language, LanguageDetector, LanguageDetectorBuilder,
    LanguageModelFilesWriter,
};
use std::io::{self, BufRead, Read};
use std::path::Path;
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about)]
struct Args {
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
    /* if args.list {
        let mut languages: Vec<Language> = Language::all().into_iter().collect();
        languages.sort();
        for language in languages {
            println!("{} - {}", language.iso_code_639_1(), language);
        }
        std::process::exit(0);
    } */

    // Select languages by ISO 639-1 code.
    /* let languages: Vec<_> = args
    .languages
    .iter()
    .map(|lang| {
        IsoCode639_1::from_str(lang.as_str())
            .expect("Supported iso639-1 language code expected")
    })
    .collect(); */

    let mut buf: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("expected input via stdin");
    let text = String::from_utf8(buf).expect("Input should be valid utf-8");

    let words: Vec<_> = text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|text| LanguageDetector::filter_text_to_words(text).into_iter())
        .flatten()
        .filter(|(_, wd)| wd.alphabets_count.contains_key(&Language::English))
        .map(|(w, _)| w)
        .collect();

    let output_directory_path = Path::new(&args.out);

    let result = LanguageModelFilesWriter::create_and_write_language_model(
        output_directory_path,
        &words,
        &Language::English,
        "\\p{L}",
    );
    println!("{:?}", result)
}

#[inline]
fn long_enough(line: &str, minlength: u8) -> bool {
    line.chars().filter(|c| c.is_alphabetic()).count() >= minlength as usize
}

fn print_confidence_values(
    results: &Vec<(Language, f64)>,
    delimiter: &str,
    confidence_threshold: Option<f64>,
    all: bool,
) {
    let mut found = false;
    for result in results {
        if confidence_threshold.is_none()
            || (confidence_threshold.is_some() && result.1 >= confidence_threshold.unwrap())
        {
            found = true;
            print!("{}{}{}\n", result.0.iso_code_639_1(), delimiter, result.1);
        }
        if !all {
            break;
        }
    }
    if !found {
        print!("unknown{}\n", delimiter);
    }
}

fn print_line_with_confidence_values(
    line: &str,
    results: &Vec<(Language, f64)>,
    delimiter: &str,
    confidence_threshold: Option<f64>,
    all: bool,
) {
    for result in results {
        if confidence_threshold.is_none()
            || (confidence_threshold.is_some() && result.1 >= confidence_threshold.unwrap())
        {
            print!(
                "{}{}{}{}{}\n",
                result.0.iso_code_639_1(),
                delimiter,
                result.1,
                delimiter,
                line
            );
        } else {
            print!("unknown{}{}{}\n", delimiter, delimiter, line);
        }
        if !all {
            break;
        }
    }
}

/* fn print_with_offset(results: &Vec<DetectionResult>, text: &str, delimiter: &str) {
    for result in results {
        print!(
            "{}{}{}{}{}{}{}\n",
            result.start_index(),
            delimiter,
            result.end_index(),
            delimiter,
            result.language().iso_code_639_1(),
            delimiter,
            &text[result.start_index()..result.end_index()],
        );
    }
} */
