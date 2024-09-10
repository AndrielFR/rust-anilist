// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum Language {
    #[default]
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
