// SPDX-License-Identifier: MIT
// Copyright (c) 2022 Andriel Ferreira <https://github.com/AndrielFR>

use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
pub struct Notification {}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct NotificationOption {
    notification_type: Type,
    enabled: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "SCREAMING_SNAKE_CASE"))]
pub enum Type {
    #[default]
    ActivityMessage,
    ActivityReply,
    Following,
    ActivityMention,
    ThreadCommentMention,
    Airing,
    ActivityLike,
    ActivityReplyLike,
    ThreadLike,
    ActivityReplySubscribed,
    RelatedMediaAddition,
    MediaDataChange,
    MediaMerge,
    MediaDeletion,
}
