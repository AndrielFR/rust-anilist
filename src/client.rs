// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use std::fs::File;
use std::io::Read;
use std::time::Duration;

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
        let mut anime = crate::models::Anime::parse(&data["data"]["Media"]);
        anime.is_full_loaded = true;

        Ok(anime)
    }

    pub async fn get_manga(&self, variables: serde_json::Value) -> Result<crate::models::Manga> {
        let data = self.request("manga", "get", variables).await.unwrap();
        let mut manga = crate::models::Manga::parse(&data["data"]["Media"]);
        manga.is_full_loaded = true;

        Ok(manga)
    }

    pub async fn get_character(
        &self,
        variables: serde_json::Value,
    ) -> Result<crate::models::Character> {
        let data = self.request("character", "get", variables).await.unwrap();
        let mut character = crate::models::Character::parse(&data["data"]["Character"]);
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
        let mut person = crate::models::Person::parse(&data["data"]["Staff"]);
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

    pub fn get_query(media_type: &str, action: &str) -> Result<String> {
        let mut graphql_query = String::new();

        let media_type = media_type.to_lowercase();
        let media_types = vec!["anime", "manga", "character", "user", "person", "studio"];
        if !media_types.contains(&media_type.as_str()) {
            panic!("The media type '{}' does not exits", { media_type });
        }

        let file_name = format!("{}_{}.graphql", action, media_type);
        let file_path = format!("./queries/{}", file_name);
        File::open(file_path)
            .and_then(|mut file| file.read_to_string(&mut graphql_query))
            .unwrap();

        Ok(graphql_query)
    }
}
