// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Clone, PartialEq)]
pub enum Gender {
    Male,
    Female,
    NonBinary,
    Other(String),
}

impl Default for Gender {
    fn default() -> Self {
        Gender::Other(String::from("Neutral"))
    }
}
