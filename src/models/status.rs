// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Clone, PartialEq)]
pub enum Status {
    Finished,
    Releasing,
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

impl Default for Status {
    fn default() -> Self {
        Status::NotYetReleased
    }
}
