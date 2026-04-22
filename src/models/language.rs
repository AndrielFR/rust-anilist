// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Language` enum.

use serde::{Deserialize, Serialize};

/// Represents a language with various options.
///
/// The `Language` enum defines a list of supported languages, each with
/// an associated variant. The default language is Japanese.
#[derive(Debug, Default, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum Language {
    /// The Japanese language.
    #[default]
    Japanese,
    /// The English language.
    English,
    /// The Korean language.
    Korean,
    /// The Italian language.
    Italian,
    /// The Spanish language.
    Spanish,
    /// The Portuguese language.
    Portuguese,
    /// The French language.
    French,
    /// The German language.
    German,
    /// The Hebrew language.
    Hebrew,
    /// The Hungarian language.
    Hungarian,
    /// The Chinese language.
    Chinese,
    /// The Arabic language.
    Arabic,
    /// The Filipino language.
    Filipino,
    /// The Catalan language.
    Catalan,
    /// The Finnish language.
    Finnish,
    /// The Turkish language.
    Turkish,
    /// The Dutch language.
    Dutch,
    /// The Swedish language.
    Swedish,
    /// The Thai language.
    Thai,
    /// The Tagalog language.
    Tagalog,
    /// The Malaysian language.
    Malaysian,
    /// The Indonesian language.
    Indonesian,
    /// The Vietnamese language.
    Vietnamese,
    /// The Nepali language.
    Nepali,
    /// The Hindi language.
    Hindi,
    /// The Urdu language.
    Urdu,
    /// The Polish language.
    Polish,
}

impl Language {
    /// Returns the ISO 639-1 code of the language.
    pub fn code(&self) -> &str {
        match self {
            Language::Japanese => "ja",
            Language::English => "en",
            Language::Korean => "ko",
            Language::Italian => "it",
            Language::Spanish => "es",
            Language::Portuguese => "pt",
            Language::French => "fr",
            Language::German => "de",
            Language::Hebrew => "he",
            Language::Hungarian => "hu",
            Language::Chinese => "zh",
            Language::Arabic => "ar",
            Language::Filipino => "fil",
            Language::Catalan => "ca",
            Language::Finnish => "fi",
            Language::Turkish => "tr",
            Language::Dutch => "nl",
            Language::Swedish => "sv",
            Language::Thai => "th",
            Language::Tagalog => "tl",
            Language::Malaysian => "ms",
            Language::Indonesian => "id",
            Language::Vietnamese => "vi",
            Language::Nepali => "ne",
            Language::Hindi => "hi",
            Language::Urdu => "ur",
            Language::Polish => "pl",
        }
    }

    /// Returns the ISO 639-1 code of the language.
    ///
    /// Alias of `code`.
    pub fn iso(&self) -> &str {
        self.code()
    }

    /// Returns the name of the language in the native language.
    pub fn native(&self) -> &str {
        match self {
            Language::Japanese => "日本語",
            Language::English => "English",
            Language::Korean => "한국어",
            Language::Italian => "Italiano",
            Language::Spanish => "Español",
            Language::Portuguese => "Português",
            Language::French => "Français",
            Language::German => "Deutsch",
            Language::Hebrew => "עברית",
            Language::Hungarian => "Magyar",
            Language::Chinese => "中文",
            Language::Arabic => "العربية",
            Language::Filipino => "Filipino",
            Language::Catalan => "Català",
            Language::Finnish => "Suomi",
            Language::Turkish => "Türkçe",
            Language::Dutch => "Nederlands",
            Language::Swedish => "Svenska",
            Language::Thai => "ไทย",
            Language::Tagalog => "Tagalog",
            Language::Malaysian => "Bahasa Melayu",
            Language::Indonesian => "Bahasa Indonesia",
            Language::Vietnamese => "Tiếng Việt",
            Language::Nepali => "नेपाली",
            Language::Hindi => "हिंदी",
            Language::Urdu => "اردو",
            Language::Polish => "Polski",
        }
    }
}

impl From<&str> for Language {
    fn from(value: &str) -> Self {
        match value.trim().to_uppercase().as_str() {
            "JA" | "JP" | "JAPANESE" => Language::Japanese,
            "EN" | "UK" | "ENGLISH" => Language::English,
            "KO" | "KOREAN" => Language::Korean,
            "IT" | "ITALIAN" => Language::Italian,
            "ES" | "SPANISH" => Language::Spanish,
            "PT" | "PORTUGUESE" => Language::Portuguese,
            "FR" | "FRENCH" => Language::French,
            "DE" | "GERMAN" => Language::German,
            "HE" | "HEBREW" => Language::Hebrew,
            "HU" | "HUNGARIAN" => Language::Hungarian,
            "ZH" | "CHINESE" => Language::Chinese,
            "AR" | "ARABIC" => Language::Arabic,
            "FIL" | "PHILIPPINE" => Language::Filipino,
            "CA" | "CATALAN" => Language::Catalan,
            "FI" | "FINNISH" => Language::Finnish,
            "TR" | "TURKISH" => Language::Turkish,
            "NL" | "DUTCH" => Language::Dutch,
            "SV" | "SWEDISH" => Language::Swedish,
            "TH" | "THAI" => Language::Thai,
            "TL" | "TAGALOG" => Language::Tagalog,
            "MS" | "MALAYSIAN" => Language::Malaysian,
            "ID" | "INDONESIAN" => Language::Indonesian,
            "VI" | "VIETNAMESE" => Language::Vietnamese,
            "NE" | "NEPALI" => Language::Nepali,
            "HI" | "HINDI" => Language::Hindi,
            "UR" | "URDU" => Language::Urdu,
            "PL" | "POLISH" => Language::Polish,
            _ => Language::default(),
        }
    }
}

impl From<String> for Language {
    fn from(value: String) -> Self {
        Language::from(value.as_str())
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Japanese => write!(f, "Japanese"),
            Language::English => write!(f, "English"),
            Language::Korean => write!(f, "Korean"),
            Language::Italian => write!(f, "Italian"),
            Language::Spanish => write!(f, "Spanish"),
            Language::Portuguese => write!(f, "Portuguese"),
            Language::French => write!(f, "French"),
            Language::German => write!(f, "German"),
            Language::Hebrew => write!(f, "Hebrew"),
            Language::Hungarian => write!(f, "Hungarian"),
            Language::Chinese => write!(f, "Chinese"),
            Language::Arabic => write!(f, "Arabic"),
            Language::Filipino => write!(f, "Filipino"),
            Language::Catalan => write!(f, "Catalan"),
            Language::Finnish => write!(f, "Finnish"),
            Language::Turkish => write!(f, "Turkish"),
            Language::Dutch => write!(f, "Dutch"),
            Language::Swedish => write!(f, "Swedish"),
            Language::Thai => write!(f, "Thai"),
            Language::Tagalog => write!(f, "Tagalog"),
            Language::Malaysian => write!(f, "Malaysian"),
            Language::Indonesian => write!(f, "Indonesian"),
            Language::Vietnamese => write!(f, "Vietnamese"),
            Language::Nepali => write!(f, "Nepali"),
            Language::Hindi => write!(f, "Hindi"),
            Language::Urdu => write!(f, "Urdu"),
            Language::Polish => write!(f, "Polish"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_code() {
        assert_eq!(Language::Japanese.code(), "ja");
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::Korean.code(), "ko");
        assert_eq!(Language::Italian.code(), "it");
        assert_eq!(Language::Spanish.code(), "es");
        assert_eq!(Language::Portuguese.code(), "pt");
        assert_eq!(Language::French.code(), "fr");
        assert_eq!(Language::German.code(), "de");
        assert_eq!(Language::Hebrew.code(), "he");
        assert_eq!(Language::Hungarian.code(), "hu");
        assert_eq!(Language::Chinese.code(), "zh");
        assert_eq!(Language::Arabic.code(), "ar");
        assert_eq!(Language::Filipino.code(), "fil");
        assert_eq!(Language::Catalan.code(), "ca");
        assert_eq!(Language::Finnish.code(), "fi");
        assert_eq!(Language::Turkish.code(), "tr");
        assert_eq!(Language::Dutch.code(), "nl");
        assert_eq!(Language::Swedish.code(), "sv");
        assert_eq!(Language::Thai.code(), "th");
        assert_eq!(Language::Tagalog.code(), "tl");
        assert_eq!(Language::Malaysian.code(), "ms");
        assert_eq!(Language::Indonesian.code(), "id");
        assert_eq!(Language::Vietnamese.code(), "vi");
        assert_eq!(Language::Nepali.code(), "ne");
        assert_eq!(Language::Hindi.code(), "hi");
        assert_eq!(Language::Urdu.code(), "ur");
        assert_eq!(Language::Polish.code(), "pl");
    }

    #[test]
    fn test_iso() {
        assert_eq!(Language::Japanese.iso(), "ja");
        assert_eq!(Language::English.iso(), "en");
        assert_eq!(Language::Korean.iso(), "ko");
        assert_eq!(Language::Italian.iso(), "it");
        assert_eq!(Language::Spanish.iso(), "es");
        assert_eq!(Language::Portuguese.iso(), "pt");
        assert_eq!(Language::French.iso(), "fr");
        assert_eq!(Language::German.iso(), "de");
        assert_eq!(Language::Hebrew.iso(), "he");
        assert_eq!(Language::Hungarian.iso(), "hu");
        assert_eq!(Language::Chinese.iso(), "zh");
        assert_eq!(Language::Arabic.iso(), "ar");
        assert_eq!(Language::Filipino.iso(), "fil");
        assert_eq!(Language::Catalan.iso(), "ca");
        assert_eq!(Language::Finnish.iso(), "fi");
        assert_eq!(Language::Turkish.iso(), "tr");
        assert_eq!(Language::Dutch.iso(), "nl");
        assert_eq!(Language::Swedish.iso(), "sv");
        assert_eq!(Language::Thai.iso(), "th");
        assert_eq!(Language::Tagalog.iso(), "tl");
        assert_eq!(Language::Malaysian.iso(), "ms");
        assert_eq!(Language::Indonesian.iso(), "id");
        assert_eq!(Language::Vietnamese.iso(), "vi");
        assert_eq!(Language::Nepali.iso(), "ne");
        assert_eq!(Language::Hindi.iso(), "hi");
        assert_eq!(Language::Urdu.iso(), "ur");
        assert_eq!(Language::Polish.iso(), "pl");
    }

    #[test]
    fn test_native() {
        assert_eq!(Language::Japanese.native(), "日本語");
        assert_eq!(Language::English.native(), "English");
        assert_eq!(Language::Korean.native(), "한국어");
        assert_eq!(Language::Italian.native(), "Italiano");
        assert_eq!(Language::Spanish.native(), "Español");
        assert_eq!(Language::Portuguese.native(), "Português");
        assert_eq!(Language::French.native(), "Français");
        assert_eq!(Language::German.native(), "Deutsch");
        assert_eq!(Language::Hebrew.native(), "עברית");
        assert_eq!(Language::Hungarian.native(), "Magyar");
        assert_eq!(Language::Chinese.native(), "中文");
        assert_eq!(Language::Arabic.native(), "العربية");
        assert_eq!(Language::Filipino.native(), "Filipino");
        assert_eq!(Language::Catalan.native(), "Català");
        assert_eq!(Language::Finnish.native(), "Suomi");
        assert_eq!(Language::Turkish.native(), "Türkçe");
        assert_eq!(Language::Dutch.native(), "Nederlands");
        assert_eq!(Language::Swedish.native(), "Svenska");
        assert_eq!(Language::Thai.native(), "ไทย");
        assert_eq!(Language::Tagalog.native(), "Tagalog");
        assert_eq!(Language::Malaysian.native(), "Bahasa Melayu");
        assert_eq!(Language::Indonesian.native(), "Bahasa Indonesia");
        assert_eq!(Language::Vietnamese.native(), "Tiếng Việt");
        assert_eq!(Language::Nepali.native(), "नेपाली");
        assert_eq!(Language::Hindi.native(), "हिंदी");
        assert_eq!(Language::Urdu.native(), "اردو");
        assert_eq!(Language::Polish.native(), "Polski");
    }

    #[test]
    fn test_from_str() {
        assert_eq!(Language::from("ja"), Language::Japanese);
        assert_eq!(Language::from("EN"), Language::English);
        assert_eq!(Language::from("ko"), Language::Korean);
        assert_eq!(Language::from("it"), Language::Italian);
        assert_eq!(Language::from("es"), Language::Spanish);
        assert_eq!(Language::from("pt"), Language::Portuguese);
        assert_eq!(Language::from("fr"), Language::French);
        assert_eq!(Language::from("de"), Language::German);
        assert_eq!(Language::from("he"), Language::Hebrew);
        assert_eq!(Language::from("hu"), Language::Hungarian);
        assert_eq!(Language::from("zh"), Language::Chinese);
        assert_eq!(Language::from("ar"), Language::Arabic);
        assert_eq!(Language::from("fil"), Language::Filipino);
        assert_eq!(Language::from("ca"), Language::Catalan);
        assert_eq!(Language::from("fi"), Language::Finnish);
        assert_eq!(Language::from("tr"), Language::Turkish);
        assert_eq!(Language::from("nl"), Language::Dutch);
        assert_eq!(Language::from("sv"), Language::Swedish);
        assert_eq!(Language::from("th"), Language::Thai);
        assert_eq!(Language::from("tl"), Language::Tagalog);
        assert_eq!(Language::from("ms"), Language::Malaysian);
        assert_eq!(Language::from("id"), Language::Indonesian);
        assert_eq!(Language::from("vi"), Language::Vietnamese);
        assert_eq!(Language::from("ne"), Language::Nepali);
        assert_eq!(Language::from("hi"), Language::Hindi);
        assert_eq!(Language::from("ur"), Language::Urdu);
        assert_eq!(Language::from("pl"), Language::Polish);
        assert_eq!(Language::from("unknown"), Language::Japanese); // Default case
    }

    #[test]
    fn test_from_string() {
        assert_eq!(Language::from("ja".to_string()), Language::Japanese);
        assert_eq!(Language::from("EN".to_string()), Language::English);
        assert_eq!(Language::from("ko".to_string()), Language::Korean);
        assert_eq!(Language::from("it".to_string()), Language::Italian);
        assert_eq!(Language::from("es".to_string()), Language::Spanish);
        assert_eq!(Language::from("pt".to_string()), Language::Portuguese);
        assert_eq!(Language::from("fr".to_string()), Language::French);
        assert_eq!(Language::from("de".to_string()), Language::German);
        assert_eq!(Language::from("he".to_string()), Language::Hebrew);
        assert_eq!(Language::from("hu".to_string()), Language::Hungarian);
        assert_eq!(Language::from("zh".to_string()), Language::Chinese);
        assert_eq!(Language::from("ar".to_string()), Language::Arabic);
        assert_eq!(Language::from("fil".to_string()), Language::Filipino);
        assert_eq!(Language::from("ca".to_string()), Language::Catalan);
        assert_eq!(Language::from("fi".to_string()), Language::Finnish);
        assert_eq!(Language::from("tr".to_string()), Language::Turkish);
        assert_eq!(Language::from("nl".to_string()), Language::Dutch);
        assert_eq!(Language::from("sv".to_string()), Language::Swedish);
        assert_eq!(Language::from("th".to_string()), Language::Thai);
        assert_eq!(Language::from("tl".to_string()), Language::Tagalog);
        assert_eq!(Language::from("ms".to_string()), Language::Malaysian);
        assert_eq!(Language::from("id".to_string()), Language::Indonesian);
        assert_eq!(Language::from("vi".to_string()), Language::Vietnamese);
        assert_eq!(Language::from("ne".to_string()), Language::Nepali);
        assert_eq!(Language::from("hi".to_string()), Language::Hindi);
        assert_eq!(Language::from("ur".to_string()), Language::Urdu);
        assert_eq!(Language::from("pl".to_string()), Language::Polish);
        assert_eq!(Language::from("unknown".to_string()), Language::Japanese); // Default case
    }
}
