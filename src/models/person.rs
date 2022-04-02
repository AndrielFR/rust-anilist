// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Character;
use crate::models::Date;
use crate::models::Gender;
use crate::models::Image;
use crate::models::Language;
use crate::models::Name;

use crate::Result;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Person {
    pub id: i64,
    pub name: Name,
    pub language: Language,
    pub image: Option<Image>,
    pub description: Option<String>,
    pub primary_occupations: Option<Vec<String>>,
    pub gender: Gender,
    pub date_of_birth: Option<Date>,
    pub date_of_death: Option<Date>,
    pub age: Option<i64>,
    pub years_active: Option<(u64, u64)>,
    pub home_town: Option<String>,
    pub blood_type: Option<String>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub url: String,
    pub characters: Option<Vec<Character>>,
    pub favourites: i64,
    pub mod_notes: Option<String>,
    pub(crate) is_full_loaded: bool,
}

impl Person {
    pub(crate) fn parse(data: &serde_json::Value) -> Self {
        let mut person = Person::default();

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

        if let Some(language) = data["languageV2"].as_str() {
            person.language = match language.to_uppercase().as_str() {
                "ENGLISH" => Language::English,
                "KOREAN" => Language::Korean,
                "ITALIAN" => Language::Italian,
                "SPANISH" => Language::Spanish,
                "PORTUGUESE" => Language::Portuguese,
                "FRENCH" => Language::French,
                "GERMAN" => Language::German,
                "HEBREW" => Language::Hebrew,
                "HUNGARIAN" => Language::Hungarian,
                "CHINESE" => Language::Chinese,
                "ARABIC" => Language::Arabic,
                "FILIPINO" => Language::Filipino,
                "CATALAN" => Language::Catalan,
                "FINNISH" => Language::Finnish,
                "TURKISH" => Language::Turkish,
                "DUTCH" => Language::Dutch,
                "SWEDISH" => Language::Swedish,
                "THAI" => Language::Thai,
                "TAGALOG" => Language::Tagalog,
                "MALAYSIAN" => Language::Malaysian,
                "INDONESIAN" => Language::Indonesian,
                "VIETNAMESE" => Language::Vietnamese,
                "NEPALI" => Language::Nepali,
                "HINDI" => Language::Hindi,
                "URDU" => Language::Urdu,
                _ => Language::default(),
            };
        }

        if let Some(image_object) = data["image"].as_object() {
            person.image = Some(Image {
                large: image_object["large"].as_str().unwrap().to_string(),
                medium: image_object["medium"].as_str().unwrap().to_string(),
            });
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

            person.date_of_birth = Some(date);
        }

        if let Some(date_of_death) = data["dateOfDeath"].as_object() {
            let mut date = Date::default();

            if let Some(year) = date_of_death["year"].as_i64() {
                date.year = Some(year);
            }
            if let Some(month) = date_of_death["month"].as_i64() {
                date.month = Some(month);
            }
            if let Some(day) = date_of_death["day"].as_i64() {
                date.day = Some(day);
            }

            person.date_of_death = Some(date);
        }

        if let Some(age) = data["age"].as_i64() {
            person.age = Some(age);
        }

        if let Some(years_active) = data["yearsActive"].as_array() {
            person.years_active = match years_active.len() {
                2 => Some((
                    years_active[0].as_u64().unwrap(),
                    years_active[1].as_u64().unwrap(),
                )),
                1 => Some((years_active[0].as_u64().unwrap(), 0)),
                _ => None,
            };
        }

        if let Some(home_town) = data["homeTown"].as_str() {
            person.home_town = Some(home_town.to_string());
        }

        if let Some(blood_type) = data["bloodType"].as_str() {
            person.blood_type = Some(blood_type.to_string());
        }

        if let Some(is_favourite) = data["isFavourite"].as_bool() {
            person.is_favourite = Some(is_favourite);
        }

        if let Some(is_favourite_blocked) = data["isFavouriteBlocked"].as_bool() {
            person.is_favourite_blocked = Some(is_favourite_blocked);
        }

        person.url = data["siteUrl"].as_str().unwrap().to_string();

        if let Some(characters) = data["characters"].as_object() {
            if let Some(nodes) = characters["nodes"].as_array() {
                let mut characters: Vec<Character> = Vec::with_capacity(nodes.len());

                for node in nodes {
                    characters.push(Character::parse(node));
                }

                person.characters = Some(characters);
            }
        }

        person.favourites = data["favourites"].as_i64().unwrap();

        if let Some(mod_notes) = data["modNotes"].as_str() {
            person.mod_notes = Some(mod_notes.to_string());
        }

        person
    }

    pub async fn load_full(self) -> crate::Result<Self> {
        if !self.is_full_loaded {
            let mut person = crate::Client::new().get_person(self.id).await.unwrap();
            person.is_full_loaded = true;
            Ok(person)
        } else {
            panic!("This person is already full loaded")
        }
    }

    pub async fn get_medias<T>() -> Result<T> {
        todo!()
    }

    pub async fn get_character_medias<T>(_character_id: i64) -> Result<T> {
        todo!()
    }
}
