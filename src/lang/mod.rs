pub(crate) mod alphabet;
mod language;
mod script;
mod ucd;

pub(crate) use alphabet::LatinAlphabet;
pub(crate) use script::Script;

pub use alphabet::str_to_alphabets;
pub use alphabet::Alphabet;
pub use language::Language;

pub(crate) fn alphabet_lang_to_model(
    a: Alphabet,
    l: Language,
) -> Option<Box<dyn parselang_models::Model>> {
    parselang_models::match_alphabet_lang_to_model!(a, l)
}
