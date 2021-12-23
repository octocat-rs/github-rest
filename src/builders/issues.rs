use crate::{
    methods::{get_issues, prelude::Issues, GetIssueBody},
    GithubRestError, Requester,
};

/// * tags issues
/// * get `/repos/{owner}/{repo}/issues`
/// * docs <https://docs.github.com/rest/reference/issues#list-repository-issues>
///
/// List repository issues
/// List issues in a repository.
///
/// **Note**: GitHub's REST API v3 considers every pull request an issue, but not every issue is a pull request. For this
/// reason, "Issues" endpoints may return both issues and pull requests in the response. You can identify pull requests by
/// the `pull_request` key. Be aware that the `id` of a pull request returned from "Issues" endpoints will be an _issue id_. To find out the pull
/// request id, use the "[List pull requests](https://docs.github.com/rest/reference/pulls#list-pull-requests)" endpoint.
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
    pub fn milestone(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.milestone = Some(value);
        self
    }
    pub fn state(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.state = Some(value);
        self
    }
    pub fn assignee(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.assignee = Some(value);
        self
    }
    pub fn creator(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.creator = Some(value);
        self
    }
    pub fn mentioned(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.mentioned = Some(value);
        self
    }
    pub fn labels(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.labels = Some(value);
        self
    }
    pub fn sort(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.sort = Some(value);
        self
    }
    pub fn direction(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.direction = Some(value);
        self
    }
    pub fn since(&mut self, value: String) -> &mut GetIssuesBuilder {
        self.body.since = Some(value);
        self
    }
    pub fn per_page(&mut self, value: i32) -> &mut GetIssuesBuilder {
        self.body.per_page = Some(value.to_string());
        self
    }
    pub fn page(&mut self, value: i32) -> &mut GetIssuesBuilder {
        self.body.page = Some(value.to_string());
        self
    }
    pub async fn execute<T>(self, client: &T) -> Result<Issues, GithubRestError>
    where
        T: Requester,
    {
        get_issues(client, self.data.0, self.data.1, Some(&self.body)).await
    }
}
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequest;

    use super::*;

    #[tokio::test]
    async fn test_get_issues_builder() {
        let reqester = DefaultRequest::new_none();

        let mut builder = GetIssuesBuilder::new("microsoft".to_owned(), "vscode".to_owned());
        builder.per_page(1).page(2).state("open".to_owned()); // builder.per_page(1).page(2).state("open".to_owned()).execute(&reqester).await.unwrap() - works too
        let r = builder.execute(&reqester).await.unwrap();
        println!("{:#?}", r)
    }
}
