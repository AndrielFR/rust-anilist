// SPDX-License-Identifier: MIT↴↴
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>↴↴

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum Season {
    Winter,
    Spring,
    Summer,
    Fall,
}
