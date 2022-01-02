use crate::structs::User;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Reactions {
    pub url: String,
    pub total_count: i64,
    #[serde(rename = "+1")]
    pub n1: i64,
    #[serde(rename = "-1")]
    pub n12: i64,
    pub laugh: i64,
    pub hooray: i64,
    pub confused: i64,
    pub heart: i64,
    pub rocket: i64,
    pub eyes: i64,
}

/// See also: <https://docs.github.com/en/rest/reference/reactions#reaction-types>
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Reaction {
    #[serde(rename = "+1")]
    ThumbsUp,
    #[serde(rename = "-1")]
    ThumbsDown,
    Laugh,
    Confused,
    Heart,
    Hooray,
    Rocket,
    Eyes,
}

impl Default for Reaction {
    fn default() -> Self {
        Reaction::ThumbsUp
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommitCommentReactionCreated {
    id: i64,
    node_id: String,
    user: User,
    #[serde(rename = "content")]
    reaction: Reaction,
    created_at: String,
}
