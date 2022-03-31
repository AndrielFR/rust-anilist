// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Default, Clone)]
pub struct Score {
    pub average: f64,
    pub mean: f64,
}

#[derive(Debug, Clone)]
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
