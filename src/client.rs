// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Client` struct and its related types.

use std::time::Duration;

use crate::{
    models::{Anime, Character, Manga, MediaType, Person, User},
    Error, Result,
};

/// Represents a client for interacting with an API.
///
/// The `Client` struct contains the necessary configuration for making
/// requests to an API, including the API token and the timeout duration.
#[derive(Clone, Debug, PartialEq)]
pub struct Client {
    /// The API token to use for requests.
    api_token: Option<String>,
    /// The timeout for requests (in seconds).
    timeout: Duration,
}

impl Client {
    /// Creates a new client instance with the specified timeout duration.
    ///
    /// This method initializes a new `Client` instance with the provided
    /// timeout duration.
    ///
    /// # Arguments
    ///
    /// * `timeout` - The timeout duration for requests, in seconds.
    pub fn with_timeout(duration: Duration) -> Self {
        Self {
            api_token: None,
            timeout: duration,
        }
    }

    /// Creates a new client instance with the specified API token.
    ///
    /// This method initializes a new `Client` instance with the provided
    /// API token and a default timeout duration of 20 seconds.
    ///
    /// # Arguments
    ///
    /// * `token` - A string slice that holds the API token.
    pub fn with_token(token: &str) -> Self {
        Self {
            api_token: Some(token.to_string()),
            timeout: Duration::from_secs(20),
        }
    }

    /// Sets the timeout duration for the client.
    ///
    /// This method allows you to set the timeout duration for the client
    /// in seconds. The timeout duration determines how long the client
    /// will wait for a response before timing out.
    ///
    /// # Arguments
    ///
    /// * `seconds` - The timeout duration in seconds.
    pub fn timeout(mut self, duration: Duration) -> Self {
        self.timeout = duration;
        self
    }

    /// Sets the API token for the client.
    ///
    /// This method allows you to set the API token for the client, which
    /// will be used for authenticating API requests.
    ///
    /// # Arguments
    ///
    /// * `token` - A string slice that holds the API token.
    pub fn token(mut self, token: &str) -> Self {
        self.api_token = Some(token.to_string());
        self
    }

    /// Get an anime by its ID or MAL ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the anime.
    /// * `mal_id` - The MAL ID of the anime.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let anime = client.get_anime(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_anime(&self, id: i64) -> Result<Anime> {
        let data = self
            .request(
                MediaType::Anime,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        match serde_json::from_str::<Anime>(&data["data"]["Media"].to_string()) {
            Ok(mut anime) => {
                anime.client = self.clone();
                anime.is_full_loaded = true;

                Ok(anime)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a manga by its ID or MAL ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the manga.
    /// * `mal_id` - The MAL ID of the manga.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let manga = client.get_manga(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_manga(&self, id: i64) -> Result<Manga> {
        let data = self
            .request(
                MediaType::Manga,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        match serde_json::from_str::<Manga>(&data["data"]["Media"].to_string()) {
            Ok(mut manga) => {
                manga.client = self.clone();
                manga.is_full_loaded = true;

                Ok(manga)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a character by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the character.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let character = client.get_character(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_character(&self, id: i64) -> Result<Character> {
        let data = self
            .request(
                MediaType::Character,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        match serde_json::from_str::<Character>(&data["data"]["Character"].to_string()) {
            Ok(mut character) => {
                character.client = self.clone();
                character.is_full_loaded = true;

                Ok(character)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a character by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the character.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let character = client.get_char(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_char(&self, id: i64) -> Result<Character> {
        self.get_character(id).await
    }

    /// Get a user by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the user.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let user = client.get_user(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user(&self, id: i32) -> Result<User> {
        let data = self
            .request(
                MediaType::User,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        match serde_json::from_str::<User>(&data["data"]["User"].to_string()) {
            Ok(user) => Ok(user),
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a user by its name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the user.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let user = client.get_user_by_name("andrielfr").await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_user_by_name<N: ToString>(&self, name: N) -> Result<User> {
        let name = name.to_string();

        let data = self
            .request(
                MediaType::User,
                Action::Get,
                serde_json::json!({ "name": name }),
            )
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        match serde_json::from_str::<User>(&data["data"]["User"].to_string()) {
            Ok(mut user) => {
                user.client = self.clone();
                user.is_full_loaded = true;

                Ok(user)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Get a person by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the person.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let person = client.get_person(1).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_person(&self, id: i64) -> Result<Person> {
        let data = self
            .request(
                MediaType::Person,
                Action::Get,
                serde_json::json!({ "id": id }),
            )
            .await
            .map_err(|e| Error::ApiError(e.to_string()))?;

        match serde_json::from_str::<Person>(&data["data"]["Staff"].to_string()) {
            Ok(mut person) => {
                person.client = self.clone();
                person.is_full_loaded = true;

                Ok(person)
            }
            Err(e) => Err(crate::Error::ApiError(e.to_string())),
        }
    }

    /// Search for animes.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the anime to search.
    /// * `page` - The page number to get.
    /// * `limit` - The number of animes to get per page.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let animes = client.search_anime("Naruto", 1, 10).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_anime(&self, title: &str, page: u16, limit: u16) -> Result<Vec<Anime>> {
        let result = self
            .request(
                MediaType::Anime,
                Action::Search,
                serde_json::json!({ "search": title, "page": page, "per_page": limit, }),
            )
            .await?;

        let mut animes = Vec::new();

        if let Some(medias) = result["data"]["Page"]["media"].as_array() {
            for media in medias.iter() {
                if let Ok(mut anime) = serde_json::from_value::<Anime>(media.clone()) {
                    anime.client = self.clone();
                    animes.push(anime);
                }
            }
        }

        Ok(animes)
    }

    /// Search for mangas.
    ///
    /// # Arguments
    ///
    /// * `title` - The title of the manga to search.
    /// * `page` - The page number to get.
    /// * `limit` - The number of mangas to get per page.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let mangas = client.search_manga("Naruto", 1, 10).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_manga(&self, title: &str, page: u16, limit: u16) -> Result<Vec<Manga>> {
        let result = self
            .request(
                MediaType::Manga,
                Action::Search,
                serde_json::json!({ "search": title, "page": page, "per_page": limit, }),
            )
            .await?;

        let mut mangas = Vec::new();

        if let Some(medias) = result["data"]["Page"]["media"].as_array() {
            for media in medias.iter() {
                if let Ok(mut manga) = serde_json::from_value::<Manga>(media.clone()) {
                    manga.client = self.clone();
                    mangas.push(manga);
                }
            }
        }

        Ok(mangas)
    }

    /// Search for users.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the user to search.
    /// * `page` - The page number to get.
    /// * `limit` - The number of users to get per page.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    ///
    /// # Example
    ///
    /// ```
    /// # async fn f(client: rust_anilist::Client) -> rust_anilist::Result<()> {
    /// let users = client.search_user("andrielfr", 1, 10).await?;
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub async fn search_user(&self, name: &str, page: u16, limit: u16) -> Result<Vec<User>> {
        let result = self
            .request(
                MediaType::User,
                Action::Search,
                serde_json::json!({ "search": name, "page": page, "per_page": limit, }),
            )
            .await?;

        let mut vec = Vec::new();

        if let Some(users) = result["data"]["Page"]["users"].as_array() {
            for user in users.iter() {
                if let Ok(mut user) = serde_json::from_value::<User>(user.clone()) {
                    user.client = self.clone();
                    vec.push(user);
                }
            }
        }

        Ok(vec)
    }

    /// Send a request to the AniList API.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The type of media to request.
    /// * `action` - The action to perform.
    /// * `variables` - The variables to send with the request.
    ///
    /// # Errors
    ///
    /// Returns an error if the request fails.
    async fn request(
        &self,
        media_type: MediaType,
        action: Action,
        variables: serde_json::Value,
    ) -> Result<serde_json::Value> {
        let query = Client::get_query(media_type, action)?;
        let json = serde_json::json!({"query": query, "variables": variables});
        let mut body = reqwest::Client::new()
            .post("https://graphql.anilist.co/")
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .timeout(self.timeout)
            .body(json.to_string());

        if let Some(token) = &self.api_token {
            body = body.bearer_auth(token);
        }

        let response = body.send().await?.text().await?;
        let result = serde_json::from_str::<serde_json::Value>(&response)?;

        Ok(result)
    }

    /// Get the GraphQL query for a specific media type.
    ///
    /// # Arguments
    ///
    /// * `media_type` - The type of media to get the query for.
    /// * `action` - The action to perform.
    ///
    /// # Errors
    ///
    /// Returns an error if the media type is not valid.
    fn get_query(media_type: MediaType, action: Action) -> Result<String> {
        let graphql_query = match action {
            Action::Get => {
                match media_type {
                    MediaType::Anime => include_str!("../queries/get_anime.graphql").to_string(),
                    MediaType::Manga => include_str!("../queries/get_manga.graphql").to_string(),
                    MediaType::Character => {
                        include_str!("../queries/get_character.graphql").to_string()
                    }
                    MediaType::User => include_str!("../queries/get_user.graphql").to_string(),
                    MediaType::Person => include_str!("../queries/get_person.graphql").to_string(),
                    // MediaType::Studio => include_str!("../queries/get_studio.graphql").to_string(),
                    _ => return Err(Error::UnsupportedMediaType),
                }
            }
            Action::Search => {
                match media_type {
                    MediaType::Anime => include_str!("../queries/search_anime.graphql").to_string(),
                    MediaType::Manga => include_str!("../queries/search_manga.graphql").to_string(),
                    // MediaType::Character => {
                    //     include_str!("../queries/search_character.graphql").to_string()
                    // }
                    MediaType::User => include_str!("../queries/search_user.graphql").to_string(),
                    // MediaType::Person => {
                    //     include_str!("../queries/search_person.graphql").to_string()
                    // }
                    // MediaType::Studio => include_str!("../queries/search_studio.graphql").to_string(),
                    _ => return Err(Error::UnsupportedMediaType),
                }
            }
        };

        Ok(graphql_query)
    }
}

impl Default for Client {
    fn default() -> Self {
        Client {
            api_token: None,
            timeout: Duration::from_secs(20),
        }
    }
}

/// Represents an action that can be performed by the client.
///
/// The `Action` enum defines various actions that the client can perform,
/// such as getting media by ID or searching for media.
enum Action {
    /// Get media by ID.
    Get,
    /// Search for media.
    Search,
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn test_with_timeout() {
        let duration = Duration::from_secs(30);
        let client = Client::with_timeout(duration);

        assert_eq!(client.timeout, duration);
        assert!(client.api_token.is_none());
    }

    #[test]
    fn test_with_token() {
        let api_token = "test_token";
        let client = Client::with_token(api_token);

        assert_eq!(client.timeout, Duration::from_secs(20));
        assert_eq!(client.api_token, Some(api_token.to_string()));
    }

    #[test]
    fn test_timeout() {
        let initial_duration = Duration::from_secs(30);
        let new_duration = Duration::from_secs(60);
        let client = Client::with_timeout(initial_duration).timeout(new_duration);

        assert_eq!(client.timeout, new_duration);
    }

    #[test]
    fn test_token() {
        let initial_token = "initial_token";
        let new_token = "new_token";
        let client = Client::with_token(initial_token).token(new_token);

        assert_eq!(client.api_token, Some(new_token.to_string()));
    }
}
