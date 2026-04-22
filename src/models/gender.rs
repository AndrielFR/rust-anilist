// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Gender` enum.

use serde::{Deserialize, Serialize};

/// Represents the gender of a person.
///
/// The `Gender` enum defines various gender identities, including male,
/// female, non-binary, and other custom genders.
#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub enum Gender {
    /// Represents the male gender.
    Male,
    /// Represents the female gender.
    Female,
    /// Represents the non-binary gender.
    #[serde(rename = "Non-binary")]
    NonBinary,
    /// Represents a custom gender specified by a string.
    #[serde(untagged)]
    Other(Option<String>),
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Other(Some(String::from("Neutral")))
    }
}
