// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::Deserialize;
use serde::Serialize;

use crate::models::Character;
use crate::models::Date;
use crate::models::Gender;
use crate::models::Image;
use crate::models::Language;
use crate::models::Name;

use crate::Client;
use crate::Result;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Person {
    pub id: i64,
    pub name: Name,
    #[serde(rename = "languageV2")]
    pub language: Language,
    pub image: Option<Image>,
    pub description: Option<String>,
    pub primary_occupations: Option<Vec<String>>,
    pub gender: Gender,
    pub date_of_birth: Option<Date>,
    pub date_of_death: Option<Date>,
    pub age: Option<i64>,
    // pub years_active: Option<(u64, u64)>,
    pub home_town: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    #[serde(rename = "siteUrl")]
    pub url: String,
    #[serde(skip)]
    pub characters: Option<Vec<Character>>,
    pub favourites: i64,
    pub mod_notes: Option<String>,
    #[serde(skip)]
    pub(crate) is_full_loaded: bool,
}

impl Person {
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            let mut person = Client::default().get_person(self.id).await.unwrap();
            person.is_full_loaded = true;
            Ok(person)
        } else {
            panic!("This person is already full loaded")
        }
    }

    pub async fn get_medias<T>() -> Result<T> {
        unimplemented!()
    }

    pub async fn get_character_medias<T>(_character_id: i64) -> Result<T> {
        unimplemented!()
    }
}
