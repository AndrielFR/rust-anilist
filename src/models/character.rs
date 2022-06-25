// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Date;
use crate::models::Gender;
use crate::models::Image;
use crate::models::Name;
use crate::models::Person;

use crate::{Client, Result};

#[derive(Debug, Default, Clone, PartialEq)]
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
    pub url: String,
    pub favourites: Option<i64>,
    pub voice_actors: Option<Vec<Person>>,
    pub mod_notes: Option<String>,
    pub(crate) is_full_loaded: bool,
}

impl Character {
    pub(crate) fn parse(data: &serde_json::Value) -> Self {
        Self {
            id: data["id"].as_i64().unwrap(),
            name: data["name"].as_object().map(|object| {
                Name {
                    first: object["first"].as_str().unwrap().to_string(),
                    middle: object["middle"].as_str().map(String::from),
                    last: object["last"].as_str().map(String::from),
                    full: object["full"].as_str().unwrap().to_string(),
                    native: object["native"].as_str().map(String::from),
                    alternative: object["alternative"].as_array().unwrap().into_iter().map(|item| item.as_str().unwrap().to_string()).collect::<Vec<String>>(),
                    alternative_spoiler: object["alternativeSpoiler"].as_array().unwrap().into_iter().map(|item| item.as_str().unwrap().to_string()).collect::<Vec<String>>(),
                    user_preferred: object["userPreferred"].as_str().map(String::from),
                }
            }).unwrap(),
            role: data["role"].as_str().map(|role| {
                match role.to_ascii_lowercase().as_str() {
                    "main" => Role::Main,
                    "supporting" => Role::Supporting,
                    _ => Role::default(),
                }
            }),
            image: data["image"].as_object().map(|object| {
                Image {
                    large: object["large"].as_str().unwrap().to_string(),
                    medium: object["medium"].as_str().unwrap().to_string(),
                }
            }).unwrap(),
            description: data["description"].as_str().unwrap().to_string(),
            gender: data["gender"].as_str().map(|gender| {
                match gender.to_ascii_lowercase().as_str() {
                    "male" => Gender::Male,
                    "female" => Gender::Female,
                    "nonbinary" => Gender::NonBinary,
                    _ => Gender::Other(gender.to_string()),
                }
            }),
            date_of_birth: data["dateOfBirth"].as_object().map(|object| {
                Date {
                    year: object["year"].as_i64(), // TODO: Use u64
                    month: object["month"].as_i64(), // Same as above
                    day: object["day"].as_i64(), // Same as above
                }
            }),
            age: data["age"].as_str().map(String::from),
            blood_type: data["bloodType"].as_str().map(String::from),
            is_favourite: data["isFavourite"].as_bool(),
            is_favourite_blocked: data["isFavouriteBlocked"].as_bool(),
            url: data["siteUrl"].as_str().unwrap().to_string(),
            favourites: data["favourites"].as_i64(),
            mod_notes: data["modNotes"].as_str().map(String::from),
            ..Default::default()
        }
    }

    pub async fn load_full(self, client: &Client) -> Result<Self> {
        if !self.is_full_loaded {
            let mut character = client
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

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Background,
    Main,
    Supporting,
}

impl Default for Role {
    fn default() -> Self {
        Role::Background
    }
}
