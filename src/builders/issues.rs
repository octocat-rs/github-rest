use crate::{
    methods::{get_issues, prelude::Issues, GetIssueBody, IssueState},
    GithubRestError, Requester,
};

/// * tags issues
/// * get `/repos/{owner}/{repo}/issues`
/// * docs <https://docs.github.com/rest/reference/issues#list-repository-issues>
///
/// List repository issues
/// List issues in a repository.
///
/// **Note**: GitHub's REST API v3 considers every pull request an issue, but
/// not every issue is a pull request. For this reason, "Issues" endpoints may
/// return both issues and pull requests in the response. You can identify pull
/// requests by the `pull_request` key. Be aware that the `id` of a pull request
/// returned from "Issues" endpoints will be an _issue id_. To find out the pull request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
pub struct GetIssuesBuilder {
    data: (String, String),
    body: GetIssueBody,
}

impl GetIssuesBuilder {
    pub fn new(user: String, repo: String) -> Self {
        GetIssuesBuilder {
            data: (user, repo),
            body: GetIssueBody {
                milestone: None,
                state: None,
                assignee: None,
                creator: None,
                mentioned: None,
                labels: None,
                sort: None,
                direction: None,
                since: None,
                per_page: None,
                page: None,
            },
        }
    }

    pub fn milestone(self, milestone: String) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                milestone: Some(milestone),
                ..self.body
            },
            ..self
        }
    }

    pub fn state(self, state: IssueState) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                state: Some(state),
                ..self.body
            },
            ..self
        }
    }

    pub fn assignee(self, assignee: String) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                assignee: Some(assignee),
                ..self.body
            },
            ..self
        }
    }

    pub fn creator(self, creator: String) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                creator: Some(creator),
                ..self.body
            },
            ..self
        }
    }

    pub fn mentioned(self, mentioned: String) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                mentioned: Some(mentioned),
                ..self.body
            },
            ..self
        }
    }

    pub fn labels(self, labels: String) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                labels: Some(labels),
                ..self.body
            },
            ..self
        }
    }

    pub fn sort(self, sort: String) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                sort: Some(sort),
                ..self.body
            },
            ..self
        }
    }

    pub fn direction(self, direction: String) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                direction: Some(direction),
                ..self.body
            },
            ..self
        }
    }

    pub fn since(self, since: String) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                since: Some(since),
                ..self.body
            },
            ..self
        }
    }

    pub fn per_page(self, count: i32) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                per_page: Some(count.to_string()),
                ..self.body
            },
            ..self
        }
    }

    pub fn page(self, page: i32) -> GetIssuesBuilder {
        GetIssuesBuilder {
            body: GetIssueBody {
                page: Some(page.to_string()),
                ..self.body
            },
            ..self
        }
    }

    pub async fn execute<T>(self, client: &T) -> Result<Issues, GithubRestError>
    where
        T: Requester,
    {
        get_issues(client, self.data.0, self.data.1, Some(&self.body)).await
    }
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequest;

    use super::*;

    #[tokio::test]
    async fn test_get_issues_builder() {
        let reqester = DefaultRequest::new_none();

        let builder = GetIssuesBuilder::new("microsoft".to_owned(), "vscode".to_owned())
            .per_page(1)
            .page(2)
            .state(IssueState::Open);

        let r = builder.execute(&reqester).await.unwrap();
        println!("{:#?}", r)
    }
}
