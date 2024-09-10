// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "lowercase"))]
pub struct Title {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: String,
    pub user_preferred: Option<String>,
}
