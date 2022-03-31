// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Character;
use crate::models::Gender;
use crate::models::Image;
use crate::models::Language;
use crate::models::Model;
use crate::models::Name;
use crate::models::User;

#[derive(Debug, Default, Clone)]
pub struct Person {
    id: i64,
    name: Name,
    language: Language,
    image: Option<Image>,
    description: Option<String>,
    primary_occupations: Option<Vec<String>>,
    gender: Gender,
    date_of_birth: Option<i64>,
    date_of_death: Option<i64>,
    age: Option<i64>,
    years_active: Option<(i64, i64)>,
    home_town: Option<String>,
    blood_type: Option<String>,
    is_favourite: Option<bool>,
    is_favourite_blocked: Option<bool>,
    url: String,
    person_medias: Option<Vec<Model>>,
    characters: Option<Vec<Character>>,
    character_medias: Option<Vec<Model>>,
    submitter: Option<User>,
    submitter_status: Option<i64>,
    submitter_notes: Option<String>,
    favourites: i64,
    mod_notes: Option<String>,
}

impl Person {
    pub(crate) fn parse(data: &serde_json::Value, person: Option<Person>) -> Self {
        let mut person = match person {
            Some(person) => person,
            None => Person::default(),
        };

        person.id = data["id"].as_i64().unwrap();

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

            if let Some(user_preferred) = name_object["userPreferred"].as_str() {
                name.user_preferred = Some(user_preferred.to_string());
            }

            person.name = name;
        }

        person
    }
}
