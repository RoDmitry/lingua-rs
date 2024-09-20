pub(crate) mod alphabet;
mod language;
mod script;
mod ucd;

pub use alphabet::*;
pub use language::Language;
pub use script::Script;

/* pub(crate) fn alphabet_lang_to_model(
    a: Alphabet,
    l: Language,
) -> Option<Box<dyn parselang_models::Model>> {
    parselang_models::match_alphabet_lang_to_model!(a, l)
} */
