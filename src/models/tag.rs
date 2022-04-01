// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub category: String,
    pub rank: i64,
    pub is_general_spoiler: bool,
    pub is_media_spoiler: bool,
    pub is_adult: bool,
    pub user_id: Option<i64>,
}
