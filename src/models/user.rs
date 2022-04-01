// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Anime;
use crate::models::Character;
use crate::models::Color;
use crate::models::Format;
use crate::models::Image;
use crate::models::Manga;
use crate::models::NotificationOption;
use crate::models::Person;
use crate::models::Status;
use crate::models::Studio;
use crate::models::{Score, ScoreFormat};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct User {
    id: i32,
    name: String,
    about: Option<String>,
    avatar: Option<Image>,
    banner: Option<String>,
    is_following: Option<bool>,
    is_follower: Option<bool>,
    is_blocked: Option<bool>,
    options: Option<Options>,
    media_list_options: MediaListOptions,
    favourites: Favourites,
    statistics: UserStatisticTypes,
}

impl User {
    pub(crate) fn parse(data: &serde_json::Value, user: Option<User>) -> Self {
        let mut user = match user {
            Some(user) => user,
            None => User::default(),
        };

        user
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
struct Options {
    title_language: UserTitleLanguage,
    display_adult_content: bool,
    airing_notifications: bool,
    profile_color: Color,
    notifications_options: Vec<NotificationOption>,
    timezone: String,
    activity_merge_time: i32,
    staff_name_language: UserStaffNameLanguage,
    restrict_messages_to_following: bool,
    disabled_list_activity: Vec<ListActivityOption>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserTitleLanguage {
    Romaji,
    English,
    Native,
    RomajiStylised,
    EnglishStylised,
    NativeStylised,
}

impl Default for UserTitleLanguage {
    fn default() -> Self {
        UserTitleLanguage::Romaji
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserStaffNameLanguage {
    RomajiWestern,
    Romaji,
    Native,
}

impl Default for UserStaffNameLanguage {
    fn default() -> Self {
        UserStaffNameLanguage::Romaji
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ListActivityOption {
    status: Status,
    disabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MediaListOptions {
    score_format: ScoreFormat,
    row_order: String,
    anime_list: MediaListTypeOptions,
    manga_list: MediaListTypeOptions,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct MediaListTypeOptions {
    section_order: Vec<String>,
    split_completed_section_by_format: bool,
    custom_lists: Vec<String>,
    advanced_scoring: Vec<String>,
    advanced_scoring_enabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Favourites {
    anime: Vec<Anime>,
    manga: Vec<Manga>,
    characters: Vec<Character>,
    staff: Vec<Person>,
    studios: Vec<Studio>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UserStatisticTypes {
    anime: UserStatistics,
    manga: UserStatistics,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UserStatistics {
    count: i32,
    score: Score,
    standard_deviation: f32,
    minutes_watched: Option<i32>,
    episodes_watched: Option<i32>,
    chapters_read: Option<i32>,
    volumes_read: Option<i32>,
    formats: Vec<UserFormatStatistic>,
    statuses: Vec<UserStatusStatistic>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UserFormatStatistic {
    count: i32,
    score: Score,
    minutes_watched: Option<i32>,
    chapters_read: Option<i32>,
    media_ids: Vec<i32>,
    format: Format,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct UserStatusStatistic {
    count: i32,
    score: Score,
    minutes_watched: Option<i32>,
    chapters_read: Option<i32>,
    media_ids: Vec<i32>,
    status: Status,
}
