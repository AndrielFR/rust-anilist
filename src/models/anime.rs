// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::Deserialize;
use serde::Serialize;

use crate::models::Character;
use crate::models::Cover;
use crate::models::Date;
use crate::models::Format;
use crate::models::Link;
use crate::models::Person;
use crate::models::Relation;
use crate::models::Season;
use crate::models::Source;
use crate::models::Status;
use crate::models::Studio;
use crate::models::Tag;
use crate::models::Title;
use crate::Client;
use crate::Result;

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Anime {
    pub id: i64,
    pub id_mal: Option<i64>,
    pub title: Title,
    pub format: Format,
    pub status: Status,
    pub description: String,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub season: Option<Season>,
    pub season_year: Option<i64>,
    pub season_int: Option<i64>,
    pub episodes: Option<i64>,
    pub duration: Option<i64>,
    pub country_of_origin: Option<String>,
    pub is_licensed: Option<bool>,
    pub source: Option<Source>,
    pub hashtag: Option<String>,
    pub updated_at: Option<i64>,
    #[serde(rename = "coverImage")]
    pub cover: Cover,
    #[serde(rename = "bannerImage")]
    pub banner: Option<String>,
    pub genres: Option<Vec<String>>,
    pub synonyms: Option<Vec<String>>,
    pub average_score: Option<i64>,
    pub mean_score: Option<i64>,
    pub popularity: Option<i64>,
    pub is_locked: Option<bool>,
    pub trending: Option<i64>,
    pub favourites: Option<i64>,
    pub tags: Option<Vec<Tag>>,
    #[serde(skip)]
    pub relations: Option<Vec<Relation>>,
    #[serde(skip)]
    pub characters: Option<Vec<Character>>,
    #[serde(skip)]
    pub staff: Option<Vec<Person>>,
    #[serde(skip)]
    pub studios: Option<Vec<Studio>>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub is_adult: Option<bool>,
    pub next_airing_episode: Option<AiringSchedule>,
    pub external_links: Option<Vec<Link>>,
    pub streaming_episodes: Option<Vec<Link>>,
    #[serde(rename = "siteUrl")]
    pub url: String,
    #[serde(skip)]
    pub(crate) is_full_loaded: bool,
}

impl Anime {
    pub async fn load_full(self) -> Result<Self> {
        if !self.is_full_loaded {
            let mut anime = Client::default()
                .get_anime(serde_json::json!({"id": self.id}))
                .await
                .unwrap();
            anime.is_full_loaded = true;
            Ok(anime)
        } else {
            panic!("This anime is already full loaded")
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AiringSchedule {
    id: i64,
    #[serde(rename = "airingAt")]
    at: i64,
    #[serde(rename = "timeUntilAiring")]
    time_until: i64,
    episode: i64,
}
