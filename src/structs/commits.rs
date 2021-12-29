use serde::{Deserialize, Serialize};

use crate::{builders::CommentOnCommitBuilder, structs::nested::Comment, GithubRestError, Requester};

pub type Commits = Vec<Commit>;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Commit {
    pub sha: String,
    pub node_id: String,
    pub commit: nested::Commit,
    pub url: String,
    pub html_url: String,
    pub comments_url: String,
    pub author: nested::Committer,
    pub committer: nested::Committer,
    pub parents: Vec<nested::Parent>,
}

impl Commit {
    // TODO(octocat): Make end user not have to pass self.http_client everywhere.
    // The above will require a bit more brainstorming on my part, as this issue
    // will persist everywhere such functions are implemented.

    // Additionally, Octocat's command interface may need a rework as a result of
    // the aforementioned issue

    /// Adds a comment to the current instance.
    pub async fn add_comment<T>(
        &self,
        client: &T,
        body: String,
        path: Option<String>,
        position: Option<String>,
    ) -> Result<Comment, GithubRestError>
    where
        T: Requester,
    {
        let (owner, repo) = self.owner_and_repo();

        let mut comment = CommentOnCommitBuilder::new(owner, repo, self.sha.clone(), body);

        if let Some(s) = path {
            comment = comment.path(s);
        }

        if let Some(s) = position {
            comment = comment.position(s);
        }

        comment.execute(client).await
    }

    fn owner_and_repo(&self) -> (String, String) {
        // TODO: Make a less disgusting method
        // Filters out the useless portions
        let f = |s: &str| {
            if s.contains("https:") || s.is_empty() || s.eq("github.com") {
                None
            } else {
                Some(s.to_owned())
            }
        };

        let split: Vec<String> = self.html_url.split('/').filter_map(f).collect();

        (split[0].clone(), split[1].clone())
    }
}

pub mod nested {
    use serde::{Deserialize, Serialize};

    // TODO: Create better names for these structs
    use crate::structs::User;

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Commit {
        pub author: CommitAuthor,
        pub committer: CommitAuthor,
        pub message: String,
        pub tree: Tree,
        pub url: String,
        pub comment_count: i64,
        pub verification: Verification,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CommitAuthor {
        pub name: String,
        pub email: String,
        pub date: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Tree {
        pub sha: String,
        pub url: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Verification {
        pub verified: bool,
        pub reason: String,
        pub signature: Option<String>,
        pub payload: Option<String>,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Committer {
        pub login: String,
        pub id: i64,
        pub node_id: String,
        pub avatar_url: String,
        pub gravatar_id: String,
        pub url: String,
        pub html_url: String,
        pub followers_url: String,
        pub following_url: String,
        pub gists_url: String,
        pub starred_url: String,
        pub subscriptions_url: String,
        pub organizations_url: String,
        pub repos_url: String,
        pub events_url: String,
        pub received_events_url: String,
        #[serde(rename = "type")]
        pub type_field: String,
        pub site_admin: bool,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Parent {
        pub sha: String,
        pub url: String,
        pub html_url: String,
    }

    #[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct Comment {
        pub html_url: String,
        pub url: String,
        pub id: i64,
        pub node_id: String,
        pub body: String,
        pub path: String,
        pub position: i64,
        pub line: i64,
        pub commit_id: String,
        pub author_association: String,
        pub user: User,
        pub created_at: String,
        pub updated_at: String,
    }
}
