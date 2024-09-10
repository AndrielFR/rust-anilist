// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum Gender {
    Male,
    Female,
    #[serde(rename = "Non-binary")]
    NonBinary,
    Other(String),
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Other(String::from("Neutral"))
    }
}
