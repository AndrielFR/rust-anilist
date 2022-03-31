// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

// #![deny(missing_docs)]

#[macro_use]
pub mod models;
mod client;
mod error;
// mod page;

pub use self::client::Client;
pub use self::error::{Error, Result};
// pub use self::page::Page;
