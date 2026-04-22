// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Studio` struct.

use serde::{Deserialize, Serialize};

use crate::Result;

/// Represents a studio with various attributes.
///
/// The `Studio` struct contains detailed information about a studio,
/// including its ID, name, whether it is an animation studio, URL,
/// whether it is a favorite, and the number of favorites.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Studio {
    /// The ID of the studio.
    pub id: i64,
    /// The name of the studio.
    pub name: String,
    /// Whether the studio is an animation studio.
    pub is_animation_studio: bool,
    /// The URL of the studio.
    #[serde(rename = "siteUrl")]
    pub url: Option<String>,
    /// Whether the studio is a favorite.
    pub is_favourite: Option<bool>,
    /// The number of favorites the studio has.
    pub favourites: Option<i64>,
}

impl Studio {
    /// Retrieves media associated with the studio.
    ///
    /// This function fetches media related to the studio and returns a
    /// result containing the media data of type `T`.
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type of the media to be returned.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use rust_anilist::{models::{Anime, Studio}, Result};
    /// #
    /// # async fn f(studio: Studio) -> Result<()> {
    /// let animes = studio.get_medias::<Anime>().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_medias<T>(&self) -> Result<Vec<T>> {
        unimplemented!()
    }
}
