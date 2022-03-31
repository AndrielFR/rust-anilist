//·SPDX-License-Identifier:·MIT
//·Copyright·(c)·2022·Andriel·Ferreira·<https://github.com/AndrielFR>

#[derive(Debug, Default, Clone)]
pub struct Title {
    pub romaji: Option<String>,
    pub english: Option<String>,
    pub native: String,
    pub user_preferred: Option<String>,
}
