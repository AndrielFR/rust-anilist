// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Gender;
use crate::models::Image;
use crate::models::Model;
use crate::models::Name;
use crate::models::Person;

#[derive(Debug, Default, Clone)]
pub struct Character {
    pub id: i64,
    pub name: Name,
    pub role: Option<Role>,
    pub image: Image,
    pub description: String,
    pub gender: Option<Gender>,
    pub date_of_birth: Option<i64>,
    pub age: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub url: String,
    pub medias: Option<Vec<Model>>,
    pub favourites: Option<i64>,
    pub voice_actors: Option<Vec<Person>>,
    pub mod_notes: Option<String>,
}

impl Character {
    pub(crate) fn parse(data: &serde_json::Value, character: Option<Character>) -> Self {
        let mut character = match character {
            Some(character) => character,
            None => Character::default(),
        };

        character.id = data["id"].as_i64().unwrap();

        if let Some(name_object) = data["name"].as_object() {
            let mut name = Name::default();

            name.first = name_object["first"].as_str().unwrap().to_string();

            if let Some(middle) = name_object["middle"].as_str() {
                name.middle = Some(middle.to_string());
            }

            if let Some(last) = name_object["last"].as_str() {
                name.last = Some(last.to_string());
            }

            name.full = name_object["full"].as_str().unwrap().to_string();

            if let Some(native) = name_object["native"].as_str() {
                name.native = Some(native.to_string());
            }

            if let Some(alternative_array) = name_object["alternative"].as_array() {
                let mut alternative = Vec::with_capacity(alternative_array.len());

                for alternative_name in alternative_array {
                    alternative.push(alternative_name.as_str().unwrap().to_string());
                }

                name.alternative = alternative;
            }

            if let Some(alternative_spoiler_array) = name_object["alternativeSpoiler"].as_array() {
                let mut alternative_spoiler = Vec::with_capacity(alternative_spoiler_array.len());

                for alternative_spoiler_name in alternative_spoiler_array {
                    alternative_spoiler
                        .push(alternative_spoiler_name.as_str().unwrap().to_string());
                }

                name.alternative_spoiler = alternative_spoiler;
            }

            if let Some(user_preferred) = name_object["userPreferred"].as_str() {
                name.user_preferred = Some(user_preferred.to_string());
            }

            character.name = name;
        }

        character
    }
}

#[derive(Debug, Clone)]
pub enum Role {
    Main,
    Supporting,
    Background,
}

impl Default for Role {
    fn default() -> Self {
        Role::Background
    }
}
