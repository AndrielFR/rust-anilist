// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Error` enum.

/// A specialized `Result` type for operations that can return an `Error`.
///
/// This is defined as a convenience to avoid writing out `std::result::Result`
/// with the `rust_anilist::Error` type repeatedly.
pub type Result<T> = std::result::Result<T, Error>;

/// Represents the various errors that can occur in the application.
///
/// This enum defines different types of errors that can be encountered,
/// such as invalid IDs and API errors.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// An error indicating that the ID is invalid.
    #[error("invalid ID")]
    InvalidId,
    /// An error indicating that the API returned an error.
    #[error("api error: `{0}`")]
    ApiError(String),
    /// An error indicating that the API returned an invalid response.
    #[error("Failed to parse JSON")]
    JsonParseError(#[from] serde_json::Error),
    /// An error indicating that the request failed.
    #[error("http error: `{0}`")]
    HttpError(#[from] reqwest::Error),
    /// An error indicating that the media type or action is not supported.
    #[error("unsupported media type or action")]
    UnsupportedMediaType,
}
