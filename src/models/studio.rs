// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Studio {
    pub id: i64,
    pub name: String,
    pub is_animation_studio: bool,
    pub url: String,
    pub is_favourite: Option<bool>,
    pub favourites: i64,
}

impl Studio {
    pub(crate) fn parse(data: &serde_json::Value, studio: Option<Studio>) -> Self {
        let mut studio = match studio {
            Some(studio) => studio,
            None => Studio::default(),
        };

        studio.id = data["id"].as_i64().unwrap();
        studio.name = data["name"].as_str().unwrap().to_string();
        studio.is_animation_studio = data["isAnimationStudio"].as_bool().unwrap();

        studio.url = data["siteUrl"].as_str().unwrap().to_string();

        if let Some(is_favourite) = data["isFavourite"].as_bool() {
            studio.is_favourite = Some(is_favourite);
        }

        studio.favourites = data["favourites"].as_i64().unwrap();

        studio
    }

    pub async fn get_medias<T>() -> Result<T> {
        todo!()
    }
}
