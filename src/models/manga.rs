// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use crate::models::Anime;
use crate::models::Character;
use crate::models::Color;
use crate::models::Cover;
use crate::models::Date;
use crate::models::Format;
use crate::models::Language;
use crate::models::MediaType;
use crate::models::Person;
use crate::models::Score;
use crate::models::Source;
use crate::models::Status;
use crate::models::Studio;
use crate::models::Tag;
use crate::models::Title;
use crate::models::{Link, LinkType};
use crate::models::{Relation, RelationType};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Manga {
    pub id: i64,
    pub id_mal: Option<i64>,
    pub title: Title,
    pub format: Format,
    pub status: Status,
    pub description: String,
    pub start_date: Option<Date>,
    pub end_date: Option<Date>,
    pub chapters: Option<i64>,
    pub volumes: Option<i64>,
    pub country_of_origin: Option<String>,
    pub is_licensed: Option<bool>,
    pub source: Option<Source>,
    pub hashtag: Option<String>,
    pub updated_at: Option<i64>,
    pub cover: Cover,
    pub banner: Option<String>,
    pub genres: Option<Vec<String>>,
    pub synonyms: Option<Vec<String>>,
    pub score: Score,
    pub popularity: Option<i64>,
    pub is_locked: Option<bool>,
    pub trending: Option<i64>,
    pub favourites: Option<i64>,
    pub tags: Option<Vec<Tag>>,
    pub relations: Option<Vec<Relation>>,
    pub characters: Option<Vec<Character>>,
    pub staff: Option<Vec<Person>>,
    pub studios: Option<Vec<Studio>>,
    pub is_favourite: Option<bool>,
    pub is_favourite_blocked: Option<bool>,
    pub is_adult: Option<bool>,
    pub external_links: Option<Vec<Link>>,
    pub url: String,
    pub(crate) is_full_loaded: bool,
}

impl Manga {
    pub(crate) fn parse(data: &serde_json::Value) -> Self {
        let mut manga = Manga::default();

        manga.id = data["id"].as_i64().unwrap();

        if let Some(id_mal) = data["idMal"].as_i64() {
            manga.id_mal = Some(id_mal);
        }

        let title = data["title"].as_object().unwrap();
        manga.title = Title {
            romaji: Some(title["romaji"].as_str().unwrap().to_string()),
            english: match title["english"].as_str() {
                Some(title) => Some(title.to_string()),
                None => None,
            },
            native: title["native"].as_str().unwrap().to_string(),
            user_preferred: Some(title["userPreferred"].as_str().unwrap().to_string()),
        };

        let format = data["format"].as_str().unwrap();
        manga.format = match format {
            "TV_SHORT" => Format::TvShort,
            "MOVIE" => Format::Movie,
            "SPECIAL" => Format::Special,
            "OVA" => Format::Ova,
            "ONA" => Format::Ona,
            "MUSIC" => Format::Music,
            "MANGA" => Format::Manga,
            "NOVEL" => Format::Novel,
            "ONE_SHOT" => Format::OneShot,
            _ => Format::default(),
        };

        let status = data["status"].as_str().unwrap();
        manga.status = match status {
            "FINISHED" => Status::Finished,
            "RELEASING" => Status::Releasing,
            "CANCELLED" => Status::Cancelled,
            "HIATUS" => Status::Hiatus,
            _ => Status::default(),
        };

        manga.description = data["description"].as_str().unwrap().to_string();

        if let Some(start_date) = data["startDate"].as_object() {
            let mut date = Date::default();

            if let Some(year) = start_date["year"].as_i64() {
                date.year = Some(year);
            }
            if let Some(month) = start_date["month"].as_i64() {
                date.month = Some(month);
            }
            if let Some(day) = start_date["day"].as_i64() {
                date.day = Some(day);
            }

            manga.start_date = Some(date);
        }

        if let Some(end_date) = data["endDate"].as_object() {
            let mut date = Date::default();

            if let Some(year) = end_date["year"].as_i64() {
                date.year = Some(year);
            }
            if let Some(month) = end_date["month"].as_i64() {
                date.month = Some(month);
            }
            if let Some(day) = end_date["day"].as_i64() {
                date.day = Some(day);
            }

            manga.end_date = Some(date);
        }

        if let Some(chapters) = data["chapters"].as_i64() {
            manga.chapters = Some(chapters);
        }

        if let Some(volumes) = data["volumes"].as_i64() {
            manga.volumes = Some(volumes);
        }

        if let Some(country_of_origin) = data["countryOfOrigin"].as_str() {
            manga.country_of_origin = Some(country_of_origin.to_string());
        }

        if let Some(is_licensed) = data["isLicensed"].as_bool() {
            manga.is_licensed = Some(is_licensed);
        }

        if let Some(source) = data["source"].as_str() {
            manga.source = match source {
                "MANGA" => Some(Source::Manga),
                "LIGHT_NOVEL" => Some(Source::LightNovel),
                "VISUAL_NOVEL" => Some(Source::VisualNovel),
                "VIDEO_GAME" => Some(Source::VideoGame),
                "OTHER" => Some(Source::Other),
                "NOVEL" => Some(Source::Novel),
                _ => Some(Source::default()),
            };
        }

        if let Some(hashtag) = data["hashtag"].as_str() {
            manga.hashtag = Some(hashtag.to_string());
        }

        if let Some(updated_at) = data["updatedAt"].as_i64() {
            manga.updated_at = Some(updated_at);
        }

        if let Some(cover_image) = data["coverImage"].as_object() {
            let mut cover = Cover::default();

            if let Some(extra_large) = cover_image["extraLarge"].as_str() {
                cover.extra_large = Some(extra_large.to_string());
            }

            if let Some(large) = cover_image["large"].as_str() {
                cover.large = Some(large.to_string());
            }

            if let Some(medium) = cover_image["medium"].as_str() {
                cover.medium = Some(medium.to_string());
            }

            if let Some(color) = cover_image["color"].as_str() {
                cover.color = Some(Color::Hex(color.to_string()));
            }

            manga.cover = cover;
        }

        if let Some(banner) = data["bannerImage"].as_str() {
            manga.banner = Some(banner.to_string());
        }

        if let Some(genres_array) = data["genres"].as_array() {
            let mut genres = Vec::with_capacity(genres_array.len());

            for genre in genres_array {
                genres.push(genre.as_str().unwrap().to_string());
            }

            manga.genres = Some(genres);
        }

        if let Some(synonyms_array) = data["synonyms"].as_array() {
            let mut synonyms = Vec::with_capacity(synonyms_array.len());

            for synonym in synonyms_array {
                synonyms.push(synonym.as_str().unwrap().to_string());
            }

            manga.synonyms = Some(synonyms);
        }

        let mut score = Score::default();
        if let Some(average) = data["averageScore"].as_i64() {
            score.average = average;
        }

        if let Some(mean) = data["meanScore"].as_i64() {
            score.mean = mean;
        }
        manga.score = score;

        if let Some(popularity) = data["popularity"].as_i64() {
            manga.popularity = Some(popularity);
        }

        if let Some(is_locked) = data["isLocked"].as_bool() {
            manga.is_locked = Some(is_locked);
        }

        if let Some(trending) = data["trendig"].as_i64() {
            manga.trending = Some(trending);
        }

        if let Some(favourites) = data["favourites"].as_i64() {
            manga.favourites = Some(favourites);
        }

        if let Some(tags_array) = data["tags"].as_array() {
            let mut tags: Vec<Tag> = Vec::with_capacity(tags_array.len());

            for tag in tags_array {
                tags.push(Tag {
                    id: tag["id"].as_i64().unwrap(),
                    name: tag["name"].as_str().unwrap().to_string(),
                    description: tag["description"].as_str().unwrap().to_string(),
                    category: tag["category"].as_str().unwrap().to_string(),
                    rank: tag["rank"].as_i64().unwrap(),
                    is_general_spoiler: tag["isGeneralSpoiler"].as_bool().unwrap(),
                    is_media_spoiler: tag["isMediaSpoiler"].as_bool().unwrap(),
                    is_adult: tag["isAdult"].as_bool().unwrap(),
                    user_id: tag["userId"].as_i64(),
                });
            }

            manga.tags = Some(tags);
        }

        if let Some(relations) = data["relations"].as_object() {
            if let Some(edges) = relations["edges"].as_array() {
                let mut relations: Vec<Relation> = Vec::with_capacity(edges.len());

                for edge in edges {
                    let node = edge.get("node").unwrap();
                    let media_type = match node["type"].as_str().unwrap() {
                        "ANIME" => MediaType::Anime,
                        "MANGA" => MediaType::Manga,
                        _ => MediaType::default(),
                    };
                    relations.push(Relation {
                        media_type: media_type,
                        anime: match media_type {
                            MediaType::Anime => Some(Anime::parse(node)),
                            _ => None,
                        },
                        manga: match media_type {
                            MediaType::Manga => Some(Manga::parse(node)),
                            _ => None,
                        },
                        id: edge["id"].as_i64().unwrap(),
                        relation_type: match edge["relationType"].as_str().unwrap() {
                            "ADAPTATION" => RelationType::Adaptation,
                            "PREQUEL" => RelationType::Prequel,
                            "SEQUEL" => RelationType::Sequel,
                            "PARENT" => RelationType::Parent,
                            "SIDE_STORY" => RelationType::SideStory,
                            "CHARACTER" => RelationType::Character,
                            "SUMMARY" => RelationType::Summary,
                            "ALTERNATIVE" => RelationType::Alternative,
                            "SPIN_OFF" => RelationType::SpinOff,
                            "OTHER" => RelationType::Other,
                            "COMPILATION" => RelationType::Compilation,
                            "CONTAINS" => RelationType::Contains,
                            _ => RelationType::Source,
                        },
                        is_main_studio: edge["isMainStudio"].as_bool().unwrap(),
                    });
                }

                manga.relations = Some(relations);
            }
        }

        if let Some(characters) = data["characters"].as_object() {
            if let Some(nodes) = characters["nodes"].as_array() {
                let mut characters: Vec<Character> = Vec::with_capacity(nodes.len());

                for node in nodes {
                    characters.push(Character::parse(node));
                }

                manga.characters = Some(characters);
            }
        }

        if let Some(persons) = data["staff"].as_object() {
            if let Some(nodes) = persons["nodes"].as_array() {
                let mut staff: Vec<Person> = Vec::with_capacity(nodes.len());

                for node in nodes {
                    staff.push(Person::parse(node, None));
                }

                manga.staff = Some(staff);
            }
        }

        if let Some(studios) = data["studios"].as_object() {
            if let Some(nodes) = studios["nodes"].as_array() {
                let mut studios: Vec<Studio> = Vec::with_capacity(nodes.len());

                for node in nodes {
                    studios.push(Studio::parse(node, None));
                }

                manga.studios = Some(studios);
            }
        }

        if let Some(is_favourite) = data["isFavourite"].as_bool() {
            manga.is_favourite = Some(is_favourite);
        }

        if let Some(is_favourite_blocked) = data["isFavouriteBlocked"].as_bool() {
            manga.is_favourite_blocked = Some(is_favourite_blocked);
        }

        if let Some(is_adult) = data["isAdult"].as_bool() {
            manga.is_adult = Some(is_adult);
        }

        if let Some(external_links_array) = data["externalLinks"].as_array() {
            let mut external_links: Vec<Link> = Vec::with_capacity(external_links_array.len());

            for external_link in external_links_array {
                external_links.push(Link {
                    id: external_link["id"].as_i64(),
                    url: external_link["url"].as_str().unwrap().to_string(),
                    site: external_link["site"].as_str().unwrap().to_string(),
                    site_id: external_link["siteId"].as_i64(),
                    link_type: match external_link["type"].as_str().unwrap() {
                        "STREAMING" => Some(LinkType::Streaming),
                        "SOCIAL" => Some(LinkType::Social),
                        _ => Some(LinkType::default()),
                    },
                    language: match external_link["language"].as_str() {
                        Some(language) => {
                            // TODO: Add rest
                            match language {
                                "ENGLISH" => Some(Language::English),
                                "KOREAN" => Some(Language::Korean),
                                _ => Some(Language::default()),
                            }
                        }
                        None => None,
                    },
                    color: match external_link["color"].as_str() {
                        Some(hex) => Some(Color::Hex(hex.to_string())),
                        None => None,
                    },
                    icon: match external_link["icon"].as_str() {
                        Some(url) => Some(url.to_string()),
                        None => None,
                    },
                    ..Default::default()
                })
            }

            manga.external_links = Some(external_links);
        }

        manga.url = data["siteUrl"].as_str().unwrap().to_string();

        manga
    }

    pub async fn load_full(self) -> crate::Result<Self> {
        if !self.is_full_loaded {
            let mut manga = crate::Client::new()
                .get_manga(serde_json::json!({"id": self.id}))
                .await
                .unwrap();
            manga.is_full_loaded = true;

            Ok(manga)
        } else {
            panic!("This manga is already full loaded")
        }
    }
}
