use clap::Parser;
use lingua::{DetectionResult, IsoCode639_1, Language, LanguageDetectorBuilder};
use std::io::{self, Read};
use std::str::FromStr;

#[derive(Parser)]
#[command(version, about)]
struct Args {
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
    if args.list {
        let mut languages: Vec<Language> = Language::all().into_iter().collect();
        languages.sort();
        for language in languages {
            println!("{} - {}", language.iso_code_639_1(), language);
        }
        std::process::exit(0);
    }

    // Select languages by ISO 639-1 code.
    let languages: Vec<_> = args
        .languages
        .iter()
        .map(|lang| {
            IsoCode639_1::from_str(lang.as_str())
                .expect("Supported iso639-1 language code expected")
        })
        .collect();
    let mut builder: LanguageDetectorBuilder = if languages.is_empty() {
        LanguageDetectorBuilder::from_all_languages()
    } else {
        LanguageDetectorBuilder::from_iso_codes_639_1(&languages)
    };
    if args.quick {
        builder.with_low_accuracy_mode();
    }
    if args.preload {
        builder.with_preloaded_language_models();
    }
    let detector = builder.build();

    let mut buf: Vec<u8> = Vec::new();
    io::stdin()
        .read_to_end(&mut buf)
        .expect("expected input via stdin");
    let text = String::from_utf8(buf).expect("Input should be valid utf-8");

    let results = detector.detect_multiple_languages_of(&text);
    print_with_offset(&results, &text, &args.delimiter)
}

fn print_with_offset(results: &Vec<DetectionResult>, text: &str, delimiter: &str) {
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
}
