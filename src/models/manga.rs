// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Manga` struct and its related types.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    Character, Cover, Date, Format, Link, Person, Relation, Source, Status, Studio, Tag, Title,
};
use crate::{Client, Result};

/// Represents a manga with various attributes.
///
/// The `Manga` struct contains detailed information about a manga,
/// including its ID, title, format, status, description, dates,
/// chapters, volumes, country of origin, licensing status, source,
/// hashtags, images, genres, synonyms, scores, popularity, tags,
/// relations, characters, staff, studios, and other metadata.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default, rename_all(deserialize = "camelCase"))]
pub struct Manga {
    /// The ID of the manga.
    pub id: i64,
    /// The ID of the manga on MAL.
    pub id_mal: Option<i64>,
    /// The title of the manga.
    pub title: Title,
    /// The format of the manga.
    pub format: Format,
    /// The status of the manga.
    pub status: Status,
    /// The description of the manga.
    pub description: String,
    /// The start date of the manga.
    pub start_date: Option<Date>,
    /// The end date of the manga.
    pub end_date: Option<Date>,
    /// The number of chapters of the manga.
    pub chapters: Option<u16>,
    /// The number of volumes of the manga.
    pub volumes: Option<u16>,
    /// The country of origin of the manga.
    pub country_of_origin: Option<String>,
    /// Whether the manga is licensed or not.
    pub is_licensed: Option<bool>,
    /// The source of the manga.
    pub source: Option<Source>,
    /// The hashtag of the manga.
    pub hashtag: Option<String>,
    /// The updated date of the manga.
    pub updated_at: Option<u64>,
    /// The cover image of the manga.
    #[serde(rename = "coverImage")]
    pub cover: Cover,
    /// The banner image of the manga.
    #[serde(rename = "bannerImage")]
    pub banner: Option<String>,
    /// The genres of the manga.
    pub genres: Option<Vec<String>>,
    /// The synonyms of the manga.
    pub synonyms: Option<Vec<String>>,
    /// The average score of the manga.
    pub average_score: Option<u8>,
    /// The mean score of the manga.
    pub mean_score: Option<u8>,
    /// The popularity of the manga.
    pub popularity: Option<u32>,
    /// Whether the manga is locked or not.
    pub is_locked: Option<bool>,
    /// The trending of the manga.
    pub trending: Option<u32>,
    /// The number of favourites of the manga.
    pub favourites: Option<u32>,
    /// The tags of the manga.
    pub tags: Option<Vec<Tag>>,
    /// The relations of the manga.
    pub(crate) relations: Value,
    /// The characters of the manga.
    pub(crate) characters: Value,
    /// The staff of the manga.
    #[serde(skip)]
    pub staff: Option<Vec<Person>>,
    /// The studios of the manga.
    #[serde(skip)]
    pub studios: Option<Vec<Studio>>,
    /// Whether the manga is favourite or not.
    pub is_favourite: Option<bool>,
    /// Whether the manga is blocked or not.
    pub is_favourite_blocked: Option<bool>,
    /// Whether the manga is adult or not.
    pub is_adult: bool,
    /// The external links of the manga.
    pub external_links: Option<Vec<Link>>,
    /// The site URL of the manga.
    #[serde(rename = "siteUrl")]
    pub url: String,

    /// The client used to fetch additional data.
    #[serde(skip)]
    pub(crate) client: Client,
    /// Whether the person's data is fully loaded.
    #[serde(default)]
    pub(crate) is_full_loaded: bool,
}

impl Manga {
    /// Loads the full details of the manga.
    ///
    /// # Errors
    ///
    /// Returns an error if the manga details cannot be loaded.
    ///
    /// # Panics
    ///
    /// Panics if the manga is already fully loaded.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::{models::Manga, Result};
    /// #
    /// # async fn f(manga: Manga) -> Result<()> {
    /// let manga = manga.load_full().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            self.client.get_manga(self.id).await
        } else {
            panic!("This manga is already full loaded")
        }
    }

    /// Returns the characters of the manga.
    pub fn characters(&self) -> Result<Vec<Character>> {
        let binding = Vec::new();
        let edges = self
            .characters
            .as_object()
            .and_then(|obj| obj.get("edges"))
            .and_then(|edges| edges.as_array())
            .unwrap_or(&binding);

        let mut characters = Vec::with_capacity(edges.len());

        for edge in edges {
            let binding = serde_json::Map::new();
            let obj = edge.as_object().unwrap_or(&binding);
            let node = obj.get("node").unwrap_or(&Value::Null);
            let role = obj.get("role").and_then(|role| role.as_str()).unwrap_or("");

            let mut character: Character = serde_json::from_value(node.clone()).unwrap_or_default();
            character.role = Some(role.into());
            characters.push(character);
        }

        Ok(characters)
    }

    /// Returns the relations of the manga.
    pub fn relations(&self) -> Result<Vec<Relation>> {
        let binding = Vec::new();
        let edges = self
            .relations
            .as_object()
            .and_then(|obj| obj.get("edges"))
            .and_then(|edges| edges.as_array())
            .unwrap_or(&binding);

        let relations = edges
            .iter()
            .map(|r| serde_json::from_value(r.clone()).unwrap_or_default())
            .collect();

        Ok(relations)
    }
}
