// SPDX-License-Identifier: MIT
// Copyright (c) 2022-2025 Andriel Ferreira <https://github.com/AndrielFR>

//! This module contains the `Name` struct.

use serde::{Deserialize, Serialize};

/// Represents a name.
#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Name {
    /// The first name.
    pub first: Option<String>,
    /// The middle name, if any.
    pub middle: Option<String>,
    /// The last name, if any.
    pub last: Option<String>,
    /// The full name.
    pub full: Option<String>,
    /// The native name, if any.
    pub native: Option<String>,
    /// Alternative names.
    pub alternative: Vec<String>,
    /// Alternative names that may contain spoilers.
    pub alternative_spoiler: Option<Vec<String>>,
    /// The name preferred by the user, if any.
    pub user_preferred: Option<String>,
}

impl Name {
    /// Returns the full name.
    pub fn full(&self) -> Option<String> {
        self.full.clone()
    }

    /// Returns the native name, if any.
    pub fn native(&self) -> Option<String> {
        self.native.clone()
    }

    /// Returns the alternative names.
    pub fn alternative(&self) -> Vec<String> {
        self.alternative.clone()
    }

    /// Returns the alternative names that may contain spoilers.
    pub fn spoiler(&self) -> Option<Vec<String>> {
        self.alternative_spoiler.clone()
    }

    /// Returns the name preferred by the user, if any.
    pub fn user_preferred(&self) -> Option<String> {
        self.user_preferred.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full() {
        let name = Name {
            first: Some("John".to_string()),
            middle: Some("Doe".to_string()),
            last: Some("Smith".to_string()),
            full: Some("John Doe Smith".to_string()),
            native: Some("ジョン ドウ スミス".to_string()),
            alternative: vec!["Johnny".to_string()],
            alternative_spoiler: Some(vec!["J.D.".to_string()]),
            user_preferred: Some("John Smith".to_string()),
        };

        assert_eq!(name.full(), Some("John Doe Smith".to_string()));
    }

    #[test]
    fn test_native() {
        let name = Name {
            first: Some("John".to_string()),
            middle: Some("Doe".to_string()),
            last: Some("Smith".to_string()),
            full: Some("John Doe Smith".to_string()),
            native: Some("ジョン ドウ スミス".to_string()),
            alternative: vec!["Johnny".to_string()],
            alternative_spoiler: Some(vec!["J.D.".to_string()]),
            user_preferred: Some("John Smith".to_string()),
        };

        assert_eq!(name.native(), Some("ジョン ドウ スミス".to_string()));
    }

    #[test]
    fn test_alternative() {
        let name = Name {
            first: Some("John".to_string()),
            middle: Some("Doe".to_string()),
            last: Some("Smith".to_string()),
            full: Some("John Doe Smith".to_string()),
            native: Some("ジョン ドウ スミス".to_string()),
            alternative: vec!["Johnny".to_string()],
            alternative_spoiler: Some(vec!["J.D.".to_string()]),
            user_preferred: Some("John Smith".to_string()),
        };

        assert_eq!(name.alternative(), vec!["Johnny".to_string()]);
    }

    #[test]
    fn test_spoiler() {
        let name = Name {
            first: Some("John".to_string()),
            middle: Some("Doe".to_string()),
            last: Some("Smith".to_string()),
            full: Some("John Doe Smith".to_string()),
            native: Some("ジョン ドウ スミス".to_string()),
            alternative: vec!["Johnny".to_string()],
            alternative_spoiler: Some(vec!["J.D.".to_string()]),
            user_preferred: Some("John Smith".to_string()),
        };

        assert_eq!(name.spoiler(), Some(vec!["J.D.".to_string()]));
    }

    #[test]
    fn test_user_preferred() {
        let name = Name {
            first: Some("John".to_string()),
            middle: Some("Doe".to_string()),
            last: Some("Smith".to_string()),
            full: Some("John Doe Smith".to_string()),
            native: Some("ジョン ドウ スミス".to_string()),
            alternative: vec!["Johnny".to_string()],
            alternative_spoiler: Some(vec!["J.D.".to_string()]),
            user_preferred: Some("John Smith".to_string()),
        };

        assert_eq!(name.user_preferred(), Some("John Smith".to_string()));
    }
}
