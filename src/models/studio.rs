// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

use crate::Result;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Studio {
    pub id: i64,
    pub name: String,
    pub is_animation_studio: bool,
    pub url: String,
    pub is_favourite: Option<bool>,
    pub favourites: i64,
}

impl Studio {
    pub async fn get_medias<T>() -> Result<T> {
        todo!()
    }
}
