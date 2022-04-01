// SPDX-License-Identifier: MIT
//
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Clone, PartialEq)]
pub enum Language {
    Japanese,
    English,
    Korean,
    Italian,
    Spanish,
    Portuguese,
    French,
    German,
    Hebrew,
    Hungarian,
    Chinese,
    Arabic,
    Filipino,
    Catalan,
    Finnish,
    Turkish,
    Dutch,
    Swedish,
    Thai,
    Tagalog,
    Malaysian,
    Indonesian,
    Vietnamese,
    Nepali,
    Hindi,
    Urdu,
}

impl Default for Language {
    fn default() -> Self {
        Language::Japanese
    }
}
