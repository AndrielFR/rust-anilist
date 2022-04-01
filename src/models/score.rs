// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Score {
    pub average: i64,
    pub mean: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Format {
    Point100,
    Point10Decimal,
    Point10,
    Point5,
    Point3,
}

impl Default for Format {
    fn default() -> Self {
        Format::Point10Decimal
    }
}
