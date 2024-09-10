// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use std::time::Duration;

use crate::models::{Anime, Character, Manga, Person};
use crate::Result;

#[derive(Clone)]
pub struct Client {
    api_token: Option<String>,
    timeout: u64,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            api_token: None,
            timeout: 20,
        }
    }
}

impl Client {
    pub fn api_token(mut self, token: &str) -> Self {
        self.api_token = Some(token.to_string());
        self
    }

    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout = seconds;
        self
    }

    pub async fn get_anime(&self, variables: serde_json::Value) -> Result<crate::models::Anime> {
        let data = self.request("anime", "get", variables).await.unwrap();
        let mut anime = serde_json::from_str::<Anime>(&data["data"]["Media"].to_string()).unwrap();
        anime.is_full_loaded = true;

        Ok(anime)
    }

    pub async fn get_manga(&self, variables: serde_json::Value) -> Result<crate::models::Manga> {
        let data = self.request("manga", "get", variables).await.unwrap();
        let mut manga = serde_json::from_str::<Manga>(&data["data"]["Media"].to_string()).unwrap();
        manga.is_full_loaded = true;

        Ok(manga)
    }

    pub async fn get_character(
        &self,
        variables: serde_json::Value,
    ) -> Result<crate::models::Character> {
        let data = self.request("character", "get", variables).await.unwrap();
        let mut character =
            serde_json::from_str::<Character>(&data["data"]["Character"].to_string()).unwrap();
        character.is_full_loaded = true;

        Ok(character)
    }

    pub async fn get_char(&self, variables: serde_json::Value) -> Result<crate::models::Character> {
        self.get_character(variables).await
    }

    pub async fn get_person(&self, id: i64) -> Result<crate::models::Person> {
        let data = self
            .request("person", "get", serde_json::json!({ "id": id }))
            .await
            .unwrap();
        let mut person =
            serde_json::from_str::<Person>(&data["data"]["Staff"].to_string()).unwrap();
        person.is_full_loaded = true;

        Ok(person)
    }

    pub async fn search_anime(
        &self,
        variables: serde_json::Value,
    ) -> Option<Vec<crate::models::Anime>> {
        let result = self.request("anime", "search", variables).await.unwrap();
        let mut _models = Vec::<crate::models::Anime>::new();
        unimplemented!()
    }

    pub(crate) async fn request(
        &self,
        media_type: &str,
        action: &str,
        variables: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, reqwest::Error> {
        let query = Client::get_query(media_type, action).unwrap();
        let json = serde_json::json!({"query": query, "variables": variables});
        let mut body = reqwest::Client::new()
            .post("https://graphql.anilist.co/")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .timeout(Duration::from_secs(self.timeout))
            .body(json.to_string());

        if let Some(token) = &self.api_token {
            body = body.bearer_auth(token);
        }

        let response = body.send().await?.text().await?;
        let result: serde_json::Value = serde_json::from_str(&response).unwrap();

        Ok(result)
    }

    pub(crate) fn get_query(media_type: &str, action: &str) -> Result<String> {
        let media_type = media_type.to_lowercase();
        let media_types = ["anime", "manga", "character", "user", "person", "studio"];
        if !media_types.contains(&media_type.as_str()) {
            panic!("The media type '{}' does not exits", { media_type });
        }

        let graphql_query = match media_type.as_str() {
            "anime" => include_str!("../queries/get_anime.graphql").to_string(),
            "manga" => include_str!("../queries/get_manga.graphql").to_string(),
            "character" => include_str!("../queries/get_character.graphql").to_string(),
            // "user" => include_str!("../queries/get_user.graphql").to_string(),
            "person" => include_str!("../queries/get_person.graphql").to_string(),
            // "studio" => include_str!("../queries/get_studio.graphql").to_string(),
            _ => unimplemented!(),
        };

        Ok(graphql_query)
    }
}
