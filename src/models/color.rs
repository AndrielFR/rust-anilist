// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Blue,
    Purple,
    Pink,
    Orange,
    Red,
    Green,
    Gray,
    Hex(String),
}

impl Default for Color {
    fn default() -> Self {
        Color::Purple
    }
}
