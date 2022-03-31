// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use std::fs::File;
use std::io::Read;

use crate::Result;

#[derive(Clone)]
pub struct Client {
    api_token: Option<String>,
    timeout: i32,
    max_retries: i32,
}

impl Default for Client {
    fn default() -> Self {
        Client {
            api_token: None,
            timeout: 7,
            max_retries: 2,
        }
    }
}

impl Client {
    pub fn new() -> Self {
        Client::default()
    }

    pub fn api_token(mut self, token: &str) -> Self {
        self.api_token = Some(token.to_string());
        self
    }

    pub fn timeout(mut self, seconds: i32) -> Self {
        self.timeout = seconds;
        self
    }

    pub fn max_retries(mut self, count: i32) -> Self {
        self.max_retries = count;
        self
    }

    pub async fn get(
        &self,
        media_type: &str,
        variables: serde_json::Value,
    ) -> Option<crate::models::Model> {
        let data = self.request(media_type, "get", variables).await.unwrap();
        crate::models::Model::new(media_type, &data["data"]["Media"])
    }

    pub async fn mutate(&self, _media_type: &str, _id: i32, _variables: serde_json::Value) -> Result<bool> {
        todo!()
    }

    pub async fn search(
        &self,
        media_type: &str,
        variables: serde_json::Value,
    ) -> Option<Vec<crate::models::Model>> {
        let result = self.request(media_type, "search", variables).await.unwrap();
        let mut _models = Vec::<crate::models::Model>::new();
        todo!()
    }

    async fn request(
        &self,
        media_type: &str,
        action: &str,
        variables: serde_json::Value,
    ) -> std::result::Result<serde_json::Value, reqwest::Error> {
        let query = Client::get_query(media_type, action);
        let json = serde_json::json!({"query": query, "variables": variables});
        let body = reqwest::Client::new()
            .post("https://graphql.anilist.co/")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .body(json.to_string());
        let response = body.send().await?.text().await?;
        let result: serde_json::Value = serde_json::from_str(&response).unwrap();
        Ok(result)
    }

    pub fn get_query(media_type: &str, action: &str) -> String {
        let mut graphql_query = String::new();

        let media_type = media_type.to_lowercase();
        match media_type.as_str() {
            "anime" => {},
            "manga" => {},
            "character" => {},
            "user" => {},
            "person" => {},
            "studio" => {},
            _ => panic!("The media_type '{}' does not exits!", {media_type}),
        }

        let file_name = format!("{}_{}.graphql", action, media_type);
        let file_path = format!("./queries/{}", file_name);
        File::open(file_path)
            .and_then(|mut file| file.read_to_string(&mut graphql_query))
            .unwrap();

        graphql_query
    }
}
