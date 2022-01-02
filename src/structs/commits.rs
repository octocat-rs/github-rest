use serde::{Deserialize, Serialize};

use crate::{builders::CommentOnCommitBuilder, methods::util, structs::nested::Comment, GithubRestError, Requester};

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
        let (owner, repo) = util::owner_and_repo(self.html_url.clone());

        let mut comment = CommentOnCommitBuilder::new(owner, repo, self.sha.clone(), body);

        if let Some(s) = path {
            comment = comment.path(s);
        }

        if let Some(s) = position {
            comment = comment.position(s);
        }

        comment.execute(client).await
    }
}

pub mod nested {
    use crate::{
        methods::{react_to_commit_comment, util},
        GithubRestError, Requester,
    };
    use serde::{Deserialize, Serialize};

    // TODO: Create better names for these structs
    use crate::structs::{CommitCommentReactionCreated, Reaction, User};

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
        pub path: Option<String>,
        pub position: Option<i64>,
        pub line: Option<i64>,
        pub commit_id: String,
        pub author_association: Association,
        pub user: User,
        pub created_at: String,
        pub updated_at: String,
    }

    impl Comment {
        pub async fn add_reaction<T>(
            &self,
            client: &T,
            reaction: Reaction,
        ) -> Result<CommitCommentReactionCreated, GithubRestError>
        where
            T: Requester,
        {
            let (owner, repo) = util::owner_and_repo(self.html_url.clone());

            react_to_commit_comment(client, owner, repo, self.id, reaction).await
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "SCREAMING_SNAKE_CASE")]
    pub enum Association {
        Collaborator,
        Contributor,
        FirstTimer,
        FirstTimeContributor,
        Mannequin,
        Member,
        None,
        Owner,
    }

    impl Default for Association {
        fn default() -> Self {
            Association::None
        }
    }
}
