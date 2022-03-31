// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

#[derive(Debug, Default, Clone)]
pub struct Studio {
    pub id: i64,
    pub name: String,
    pub is_animation_studio: bool,
    pub medias: Option<Vec<crate::models::Model>>,
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

        // TODO: Uncomment this
        // if let Some(medias_object) = data["media"].as_object() {
        //     if let Some(nodes) = medias_object["nodes"].as_array() {
        //         let mut medias: Vec<crate::models::Model> = Vec::with_capacity(nodes.len());
        //
        //         for node in nodes {
        //             medias.push(crate::models::Model::new(&node["type"].as_str().unwrap().to_lowercase(), node).unwrap());
        //         }
        //
        //         studio.medias = Some(medias);
        //     }
        // }

        studio.url = data["siteUrl"].as_str().unwrap().to_string();

        if let Some(is_favourite) = data["isFavourite"].as_bool() {
            studio.is_favourite = Some(is_favourite);
        }

        studio.favourites = data["favourites"].as_i64().unwrap();

        studio
    }
}
