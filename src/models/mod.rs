// SPDX-License-Identifier: MIT↴
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>↴

mod anime;
mod character;
mod color;
mod cover;
mod date;
mod format;
mod gender;
mod image;
mod language;
mod link;
mod manga;
mod name;
mod notification;
mod person;
mod relation;
mod score;
mod season;
mod source;
mod status;
mod studio;
mod tag;
mod title;
mod user;

pub use anime::Anime;
pub use character::{Character, Role as CharacterRole};
pub use color::Color;
pub use cover::Cover;
pub use date::Date;
pub use format::Format;
pub use gender::Gender;
pub use image::Image;
pub use language::Language;
pub use link::{Link, Type as LinkType};
pub use manga::Manga;
pub use name::Name;
pub use notification::{Notification, NotificationOption, Type as NotificationType};
pub use person::Person;
pub use relation::{Relation, Type as RelationType};
pub use score::{Format as ScoreFormat, Score};
pub use season::Season;
pub use source::Source;
pub use status::Status;
pub use studio::Studio;
pub use tag::Tag;
pub use title::Title;
pub use user::User;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum MediaType {
    Anime,
    Manga,
    Unknown,
}

impl Default for MediaType {
    fn default() -> Self {
        MediaType::Unknown
    }
}
