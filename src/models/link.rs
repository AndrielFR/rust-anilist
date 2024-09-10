// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::Deserialize;
use serde::Serialize;

use crate::models::Color;
use crate::models::Language;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Link {
    pub id: Option<i64>,
    pub title: Option<String>,
    pub thumbnail: Option<String>,
    pub url: Option<String>,
    pub site: Option<String>,
    pub site_id: Option<i64>,
    pub link_type: Option<Type>,
    pub language: Option<Language>,
    pub color: Option<Color>,
    pub icon: Option<String>,
    pub notes: Option<String>,
    pub is_disabled: Option<bool>,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "UPPERCASE"))]
pub enum Type {
    #[default]
    Info,
    Streaming,
    Social,
}
