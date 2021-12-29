use super::User as Assignee;
use serde::{Deserialize, Serialize};

pub type Issues = Vec<Issue>;

pub type Pulls = Vec<Pull>;

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
pub struct Pull {
    url: String,
    id: i64,
    node_id: String,
    html_url: String,
    diff_url: String,
    patch_url: String,
    issue_url: String,
    commits_url: String,
    review_comments_url: String,
    review_comment_url: String,
    comments_url: String,
    statuses_url: String,
    number: i64,
    state: String,
    locked: bool,
    title: String,
    user: Assignee,
    body: String,
    labels: Vec<Label>,
    milestone: Milestone,
    active_lock_reason: String,
    created_at: String,
    updated_at: String,
    closed_at: String,
    merged_at: String,
    merge_commit_sha: String,
    assignee: Assignee,
    assignees: Vec<Assignee>,
    requested_reviewers: Vec<Assignee>,
    requested_teams: Vec<RequestedTeam>,
    head: Base,
    base: Base,
    #[serde(rename = "_links")]
    links: Links,
    author_association: String,
    auto_merge: Option<serde_json::Value>,
    draft: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Base {
    label: String,
    #[serde(rename = "ref")]
    base_ref: String,
    sha: String,
    user: Assignee,
    repo: Repo,
}

#[derive(Serialize, Deserialize)]
pub struct Repo {
    id: i64,
    node_id: String,
    name: String,
    full_name: String,
    owner: Assignee,
    private: bool,
    html_url: String,
    description: String,
    fork: bool,
    url: String,
    archive_url: String,
    assignees_url: String,
    blobs_url: String,
    branches_url: String,
    collaborators_url: String,
    comments_url: String,
    commits_url: String,
    compare_url: String,
    contents_url: String,
    contributors_url: String,
    deployments_url: String,
    downloads_url: String,
    events_url: String,
    forks_url: String,
    git_commits_url: String,
    git_refs_url: String,
    git_tags_url: String,
    git_url: String,
    issue_comment_url: String,
    issue_events_url: String,
    issues_url: String,
    keys_url: String,
    labels_url: String,
    languages_url: String,
    merges_url: String,
    milestones_url: String,
    notifications_url: String,
    pulls_url: String,
    releases_url: String,
    ssh_url: String,
    stargazers_url: String,
    statuses_url: String,
    subscribers_url: String,
    subscription_url: String,
    tags_url: String,
    teams_url: String,
    trees_url: String,
    clone_url: String,
    mirror_url: String,
    hooks_url: String,
    svn_url: String,
    homepage: String,
    language: Option<serde_json::Value>,
    forks_count: i64,
    stargazers_count: i64,
    watchers_count: i64,
    size: i64,
    default_branch: String,
    open_issues_count: i64,
    is_template: bool,
    topics: Vec<String>,
    has_issues: bool,
    has_projects: bool,
    has_wiki: bool,
    has_pages: bool,
    has_downloads: bool,
    archived: bool,
    disabled: bool,
    visibility: String,
    pushed_at: String,
    created_at: String,
    updated_at: String,
    permissions: String,
    allow_rebase_merge: bool,
    template_repository: Option<serde_json::Value>,
    temp_clone_token: String,
    allow_squash_merge: bool,
    allow_auto_merge: bool,
    delete_branch_on_merge: bool,
    allow_merge_commit: bool,
    subscribers_count: i64,
    network_count: i64,
    license: License,
    forks: i64,
    open_issues: i64,
    watchers: i64,
}

#[derive(Serialize, Deserialize)]
pub struct License {
    key: String,
    name: String,
    url: String,
    spdx_id: String,
    node_id: String,
    html_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Permissions {
    admin: bool,
    push: bool,
    pull: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Links {
    links_self: Comments,
    html: Comments,
    issue: Comments,
    comments: Comments,
    review_comments: Comments,
    review_comment: Comments,
    commits: Comments,
    statuses: Comments,
}

#[derive(Serialize, Deserialize)]
pub struct Comments {
    href: String,
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
