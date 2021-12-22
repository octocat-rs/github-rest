/// Generated by https://quicktype.io
extern crate serde_json;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateIssueResponse {
    id: i64,
    node_id: String,
    url: String,
    repository_url: String,
    labels_url: String,
    comments_url: String,
    events_url: String,
    html_url: String,
    number: i64,
    state: String,
    title: String,
    body: String,
    user: Assignee,
    labels: Vec<Label>,
    assignee: Assignee,
    assignees: Vec<Assignee>,
    milestone: Milestone,
    locked: bool,
    active_lock_reason: String,
    comments: i64,
    pull_request: PullRequest,
    closed_at: Option<serde_json::Value>,
    created_at: String,
    updated_at: String,
    closed_by: Assignee,
    author_association: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Assignee {
    login: String,
    id: i64,
    node_id: String,
    avatar_url: String,
    gravatar_id: String,
    url: String,
    html_url: String,
    followers_url: String,
    following_url: String,
    gists_url: String,
    starred_url: String,
    subscriptions_url: String,
    organizations_url: String,
    repos_url: String,
    events_url: String,
    received_events_url: String,
    assignee_type: String,
    site_admin: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    id: i64,
    node_id: String,
    url: String,
    name: String,
    description: String,
    color: String,
    label_default: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Milestone {
    url: String,
    html_url: String,
    labels_url: String,
    id: i64,
    node_id: String,
    number: i64,
    state: String,
    title: String,
    description: String,
    creator: Assignee,
    open_issues: i64,
    closed_issues: i64,
    created_at: String,
    updated_at: String,
    closed_at: String,
    due_on: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PullRequest {
    #[serde(rename = "url")]
    url: String,

    #[serde(rename = "html_url")]
    html_url: String,

    #[serde(rename = "diff_url")]
    diff_url: String,

    #[serde(rename = "patch_url")]
    patch_url: String,
}
