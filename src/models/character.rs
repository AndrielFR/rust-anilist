// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Date;
use crate::models::Gender;
use crate::models::Image;
use crate::models::Name;
use crate::models::Person;

use crate::Result;

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
        let mut character = Character::default();

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

        if let Some(image_object) = data["image"].as_object() {
            let mut image = Image::default();

            if let Some(large) = image_object["large"].as_str() {
                image.large = large.to_string();
            }

            if let Some(medium) = image_object["medium"].as_str() {
                image.medium = medium.to_string();
            }

            character.image = image;
        }

        character.description = data["description"].as_str().unwrap().to_string();

        if let Some(gender) = data["gender"].as_str() {
            character.gender = match gender {
                "Male" => Some(Gender::Male),
                "Femalm" => Some(Gender::Female),
                "NonBinary" => Some(Gender::NonBinary),
                other => Some(Gender::Other(other.to_string())),
            };
        }

        if let Some(date_of_birth) = data["dateOfBirth"].as_object() {
            let mut date = Date::default();

            if let Some(year) = date_of_birth["year"].as_i64() {
                date.year = Some(year);
            }

            if let Some(month) = date_of_birth["month"].as_i64() {
                date.month = Some(month);
            }

            if let Some(day) = date_of_birth["day"].as_i64() {
                date.day = Some(day);
            }

            character.date_of_birth = Some(date);
        }

        if let Some(age) = data["age"].as_str() {
            character.age = Some(age.to_string());
        }

        if let Some(blood_type) = data["bloodType"].as_str() {
            character.blood_type = Some(blood_type.to_string());
        }

        if let Some(is_favourite) = data["isFavourite"].as_bool() {
            character.is_favourite = Some(is_favourite);
        }

        if let Some(is_favourite_blocked) = data["isFavouriteBlocked"].as_bool() {
            character.is_favourite_blocked = Some(is_favourite_blocked);
        }

        character.url = data["siteUrl"].as_str().unwrap().to_string();

        if let Some(favourites) = data["favourites"].as_i64() {
            character.favourites = Some(favourites);
        }

        if let Some(mod_notes) = data["modNotes"].as_str() {
            character.mod_notes = Some(mod_notes.to_string());
        }

        character
    }

    pub async fn load_full(self) -> crate::Result<Self> {
        if !self.is_full_loaded {
            let mut character = crate::Client::new()
                .get_character(serde_json::json!({"id": self.id}))
                .await
                .unwrap();
            character.is_full_loaded = true;
            Ok(character)
        } else {
            panic!("This character is already full loaded")
        }
    }

    pub async fn get_medias<T>() -> Result<T> {
        todo!()
    }
}

#[derive(Debug, Clone, PartialEq)]
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
