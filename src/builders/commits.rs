use crate::{
    methods::{comment_on_commit, prelude::Comment, CommentOnCommitBody},
    GithubRestError, Requester,
};

/// * tags repos
/// * post `/repos/{owner}/{repo}/commits/{commit_sha}/comments`
/// * docs <https://docs.github.com/rest/reference/repos#create-a-commit-comment>
///
/// Create a commit comment
/// Create a comment for a commit using its `:commit_sha`.
///
/// This endpoint triggers [notifications](https://docs.github.com/en/github/managing-subscriptions-and-notifications-on-github/about-notifications). Creating content too quickly using this endpoint may result in secondary rate limiting. See "[Secondary rate limits](https://docs.github.com/rest/overview/resources-in-the-rest-api#secondary-rate-limits)" and "[Dealing with secondary rate limits](https://docs.github.com/rest/guides/best-practices-for-integrators#dealing-with-secondary-rate-limits)" for details.
pub struct CommentOnCommitBuilder {
    data: (String, String, String),
    body: CommentOnCommitBody,
}
impl CommentOnCommitBuilder {
    pub fn new(user: String, repo: String, hash: String, content: String) -> Self {
        CommentOnCommitBuilder {
            data: (user, repo, hash),
            body: CommentOnCommitBody {
                body: content,
                path: None,
                position: None,
                line: None,
            },
        }
    }
    pub fn body(self, body: String) -> CommentOnCommitBuilder {
        CommentOnCommitBuilder {
            body: CommentOnCommitBody { body, ..self.body },
            ..self
        }
    }
    pub fn path(self, path: String) -> CommentOnCommitBuilder {
        CommentOnCommitBuilder {
            body: CommentOnCommitBody {
                path: Some(path),
                ..self.body
            },
            ..self
        }
    }
    pub fn position(self, position: String) -> CommentOnCommitBuilder {
        CommentOnCommitBuilder {
            body: CommentOnCommitBody {
                position: Some(position),
                ..self.body
            },
            ..self
        }
    }

    pub async fn execute<T>(self, client: &T) -> Result<Comment, GithubRestError>
    where
        T: Requester,
    {
        comment_on_commit(client, self.data.0, self.data.1, self.data.2, self.body).await
    }
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::{builders::CommentOnCommitBuilder, client::DefaultRequest};

    #[cfg(feature = "client")]
    #[tokio::test]
    async fn test_comment_on_commit() {
        let comment = CommentOnCommitBuilder::new(
            "octocat-rs".to_owned(),
            "github-rest".to_owned(),
            "2eb7eeba66a6adf2168391d0cd6dcac995a34489".to_owned(),
            "Losing my mind".to_owned(),
        );

        // You'll need to add your auth to get this to pass
        let a = comment.execute(&DefaultRequest::new_none()).await.unwrap();

        dbg!(a);
    }
}
