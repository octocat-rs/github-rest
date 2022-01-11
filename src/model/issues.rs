use super::{Repository, User as Assignee};
use serde::{Deserialize, Serialize};
use strum::{EnumString, EnumVariantNames};

pub type Issues = Vec<Issue>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IssueAction {
    Opened,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssueEvent {
    pub action: IssueAction,
    pub issue: Issue,
    pub repository: Repository,
    pub sender: Assignee,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Label {
    id: i64,
    node_id: String,
    url: String,
    name: String,
    description: String,
    color: String,
    label_default: Option<bool>,
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
    url: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Issue {
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
    assignee: Option<Assignee>,
    assignees: Vec<Assignee>,
    milestone: Option<Milestone>,
    locked: bool,
    active_lock_reason: Option<String>,
    comments: i64,
    pull_request: Option<PullRequest>,
    closed_at: Option<serde_json::Value>,
    created_at: String,
    updated_at: String,
    closed_by: Option<Assignee>,
    author_association: String,
}

#[derive(Serialize, Deserialize)]
pub struct RequestedTeam {
    id: i64,
    node_id: String,
    url: String,
    html_url: String,
    name: String,
    slug: String,
    description: String,
    privacy: String,
    permission: String,
    members_url: String,
    repositories_url: String,
    parent: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub enum EventsUrl {
    #[serde(rename = "https://api.github.com/users/hubot/events{/privacy}")]
    HttpsApiGithubComUsersHubotEventsPrivacy,

    #[serde(rename = "https://api.github.com/users/octocat/events{/privacy}")]
    HttpsApiGithubComUsersOctocatEventsPrivacy,

    #[serde(rename = "https://api.github.com/users/other_user/events{/privacy}")]
    HttpsApiGithubComUsersOtherUserEventsPrivacy,
}

#[derive(Serialize, Deserialize)]
pub enum FollowingUrl {
    #[serde(rename = "https://api.github.com/users/hubot/following{/other_user}")]
    HttpsApiGithubComUsersHubotFollowingOtherUser,

    #[serde(rename = "https://api.github.com/users/octocat/following{/other_user}")]
    HttpsApiGithubComUsersOctocatFollowingOtherUser,

    #[serde(rename = "https://api.github.com/users/other_user/following{/other_user}")]
    HttpsApiGithubComUsersOtherUserFollowingOtherUser,
}

#[derive(Serialize, Deserialize)]
pub enum GistsUrl {
    #[serde(rename = "https://api.github.com/users/hubot/gists{/gist_id}")]
    HttpsApiGithubComUsersHubotGistsGistId,

    #[serde(rename = "https://api.github.com/users/octocat/gists{/gist_id}")]
    HttpsApiGithubComUsersOctocatGistsGistId,

    #[serde(rename = "https://api.github.com/users/other_user/gists{/gist_id}")]
    HttpsApiGithubComUsersOtherUserGistsGistId,
}

#[derive(Serialize, Deserialize)]
pub enum StarredUrl {
    #[serde(rename = "https://api.github.com/users/hubot/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersHubotStarredOwnerRepo,

    #[serde(rename = "https://api.github.com/users/octocat/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersOctocatStarredOwnerRepo,

    #[serde(rename = "https://api.github.com/users/other_user/starred{/owner}{/repo}")]
    HttpsApiGithubComUsersOtherUserStarredOwnerRepo,
}
