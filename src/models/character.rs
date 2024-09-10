// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::Deserialize;
use serde::Serialize;

use crate::models::Date;
use crate::models::Gender;
use crate::models::Image;
use crate::models::Name;
use crate::models::Person;

use crate::{Client, Result};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Character {
    pub id: i64,
    pub name: Name,
    pub role: Option<Role>,
    pub image: Image,
    pub description: String,
    pub gender: Option<Gender>,
    pub date_of_birth: Option<Date>,
    pub age: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub url: String,
    pub favourites: Option<i64>,
    pub voice_actors: Option<Vec<Person>>,
    pub mod_notes: Option<String>,
    #[serde(skip)]
    pub(crate) is_full_loaded: bool,
}

impl Character {
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            let mut character = Client::default()
                .get_character(serde_json::json!({"id": self.id}))
                .await?;
            character.is_full_loaded = true;

            Ok(character)
        } else {
            panic!("This character is already full loaded")
        }
    }

    pub async fn get_medias<T>(&self) -> Result<T> {
        unimplemented!()
    }

    pub fn is_full_loaded(&self) -> bool {
        self.is_full_loaded
    }
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub enum Role {
    #[default]
    Background,
    Main,
    Supporting,
}
