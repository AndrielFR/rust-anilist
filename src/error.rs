// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use std::error::Error as StdError;
use std::fmt;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidId,
    InvalidMediaType,
    ApiError(String),
}

#[derive(Debug)]
struct InvalidId;

impl StdError for InvalidId {}

impl fmt::Display for InvalidId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The given id is invalid")
    }
}

#[derive(Debug)]
struct InvalidMediaType;

impl StdError for InvalidMediaType {}

impl fmt::Display for InvalidMediaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The given media type is invalid")
    }
}
