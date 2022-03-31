// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Model;

#[derive(Debug, Clone)]
pub struct Relation {
    pub media: Model,
    pub id: i64,
    pub relation_type: Type,
    pub is_main_studio: bool,
}

#[derive(Debug, Clone)]
pub enum Type {
    Adaptation,
    Prequel,
    Sequel,
    Parent,
    SideStory,
    Character,
    Summary,
    Alternative,
    SpinOff,
    Other,
    Source,
    Compilation,
    Contains,
}
