// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

// #![deny(missing_docs)]
#![allow(clippy::field_reassign_with_default)]
#![allow(clippy::let_and_return)]
#![allow(dead_code)] // TODO: Remove this
#![allow(unused_mut)] // TODO: Remove this
#![allow(unused_variables)] // TODO: Remove this

#[macro_use]
pub mod models;
mod client;
mod error;
// mod page;

pub use self::client::Client;
pub use self::error::{Error, Result};
// pub use self::page::Page;
