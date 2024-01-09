use strum::IntoEnumIterator;
use whisper::token::Language;

pub fn get_available_languages() -> Vec<String> {
    let mut langs: Vec<String> = Language::iter()
        .map(|lang| lang.as_alpha2().to_string())
        .collect();
    langs.sort();
    langs
}
