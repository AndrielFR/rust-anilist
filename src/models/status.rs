// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Status {
    Finished,
    Releasing,
    #[default]
    NotYetReleased,
    Cancelled,
    Hiatus,
    Current,
    Planning,
    Completed,
    Dropped,
    Paused,
    Repeating,
}
