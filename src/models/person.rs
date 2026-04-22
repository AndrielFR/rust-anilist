// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

use super::{Character, Date, Gender, Image, Language, Name};
use crate::{Client, Result};

/// Represents a person.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Person {
    /// The ID of the person.
    pub id: i64,
    /// The name of the person.
    pub name: Name,
    /// The language of the person.
    #[serde(rename = "languageV2")]
    pub language: Language,
    /// The image of the person, if any.
    pub image: Option<Image>,
    /// The description of the person, if any.
    pub description: Option<String>,
    /// The primary occupations of the person, if any.
    pub primary_occupations: Option<Vec<String>>,
    /// The gender of the person.
    pub gender: Gender,
    /// The date of birth of the person, if any.
    pub date_of_birth: Option<Date>,
    /// The date of death of the person, if any.
    pub date_of_death: Option<Date>,
    /// The age of the person, if any.
    pub age: Option<i64>,
    // The years the person was active, if any.
    // pub years_active: Option<(u64, u64)>,
    /// The hometown of the person, if any.
    pub home_town: Option<String>,
    /// The blood type of the person, if any.
    pub blood_type: Option<String>,
    /// Whether the person is a favorite, if any.
    pub is_favourite: Option<bool>,
    /// Whether the person is blocked from being a favorite, if any.
    pub is_favourite_blocked: Option<bool>,
    /// The URL of the person's site.
    #[serde(rename = "siteUrl")]
    pub url: String,
    /// The characters associated with the person, if any.
    #[serde(skip)]
    pub characters: Option<Vec<Character>>,
    /// The number of favorites the person has.
    pub favourites: Option<i64>,
    /// The moderator notes for the person, if any.
    pub mod_notes: Option<String>,

    /// The client used to fetch additional data.
    #[serde(skip)]
    pub(crate) client: Client,
    /// Whether the person's data is fully loaded.
    #[serde(default)]
    pub(crate) is_full_loaded: bool,
}

impl Person {
    /// Loads the full details of the person.
    ///
    /// # Errors
    ///
    /// Returns an error if the person details cannot be loaded.
    ///
    /// # Panics
    ///
    /// Panics if the person is already fully loaded.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::{models::Person, Result};
    /// #
    /// # async fn f(person: Person) -> Result<()> {
    /// let person = person.load_full().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            self.client.get_person(self.id).await
        } else {
            panic!("This person is already full loaded")
        }
    }

    /// Retrieves the media associated with the person.
    ///
    /// # Errors
    ///
    /// Returns an error if the media cannot be retrieved.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the media to be returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::{models::{Anime, Person}, Result};
    /// #
    /// # async fn f(person: Person) -> Result<()> {
    /// let animes = person.get_medias::<Anime>().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_medias<T>(&self) -> Result<Vec<T>> {
        unimplemented!()
    }

    /// Retrieves the media associated with a character.
    ///
    /// # Arguments
    ///
    /// * `character_id` - The ID of the character whose media is to be retrieved.
    ///
    /// # Errors
    ///
    /// Returns an error if the media cannot be retrieved.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the media to be returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::{models::{Manga, Person}, Result};
    /// #
    /// # async fn f(person: Person) -> Result<()> {
    /// let char_mangas = person.get_character_medias::<Manga>(1).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_character_medias<T>(&self, _character_id: i64) -> Result<Vec<T>> {
        unimplemented!()
    }
}
