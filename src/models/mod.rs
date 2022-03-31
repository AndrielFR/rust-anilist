//·SPDX-License-Identifier:·MIT↴
// Copyright·(c)·2022·Andriel·Ferreira·<https://github.com/AndrielFR>↴

mod anime;
mod character;
mod color;
mod cover;
mod date;
mod format;
mod gender;
mod image;
mod language;
mod name;
mod notification;
mod person;
mod relation;
mod score;
mod season;
mod source;
mod status;
mod tag;
mod title;
mod user;
mod manga;
mod studio;
mod link;

pub use anime::Anime;
pub use character::{Character, Role as CharacterRole};
pub use color::Color;
pub use cover::Cover;
pub use date::Date;
pub use format::Format;
pub use gender::Gender;
pub use image::Image;
pub use language::Language;
pub use name::Name;
pub use notification::{Notification, NotificationOption, Type as NotificationType};
pub use person::Person;
pub use relation::{Relation, Type as RelationType};
pub use score::{Format as ScoreFormat, Score};
pub use season::Season;
pub use source::Source;
pub use status::Status;
pub use tag::Tag;
pub use title::Title;
pub use user::User;
pub use manga::Manga;
pub use studio::Studio;
pub use link::{Link, Type as LinkType};

#[derive(Debug, Clone)]
pub enum Model {
    Anime(Anime),
    Manga(Manga),
    Character(Character),
    User(User),
    Person(Person),
    Studio(Studio),
}

impl Model {
    pub(crate) fn new(media_type: &str, data: &serde_json::Value) -> Option<Self> {
        match media_type {
            "anime" => Some(Self::Anime(Anime::parse(data, None))),
            "manga" => Some(Self::Manga(Manga::parse(data, None))),
            "character" => Some(Self::Character(Character::parse(data, None))),
            "user" => Some(Self::User(User::parse(data, None))),
            "person" => Some(Self::Person(Person::parse(data, None))),
            "studio" => Some(Self::Studio(Studio::parse(data, None))),
            _ => None,
        }
    }
}
