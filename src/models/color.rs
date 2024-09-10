// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum Color {
    Blue,
    #[default]
    Purple,
    Pink,
    Orange,
    Red,
    Green,
    Gray,
    #[serde(untagged)]
    Hex(String),
}
