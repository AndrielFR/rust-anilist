//·SPDX-License-Identifier:·MIT
//·Copyright·(c)·2022·Andriel·Ferreira·<https://github.com/AndrielFR>

use crate::models::Language;
use crate::models::Color;

#[derive(Debug, Default, Clone)]
pub struct Link {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub url: String,
    pub site: String,
    pub site_id: Option<i64>,
    pub link_type: Option<Type>,
    pub language: Option<Language>,
    pub color: Option<Color>,
    pub icon: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Type {
    Info,
    Streaming,
    Social,
}

impl Default for Type {
    fn default() -> Self {
        Type::Info
    }
}
