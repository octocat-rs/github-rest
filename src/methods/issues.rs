use super::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateIssueBody {
    title: String,
    body: Option<String>,
    assignee: Option<String>,
    milestone: Option<String>,
    labels: Option<Vec<String>>,
    assignees: Option<Vec<String>>,
}
//TODO: TEST THIS
pub async fn create_issue(
    client: &DefaultRequest,
    repo: String,
    owner: String,
    body: CreateIssueBody,
) -> Result<GetResponse, CoolError> {
    client
        .req::<String, String, GetResponse>(
            EndPoints::PostReposownerrepoIssues(repo, owner),
            None,
            Some(serde_json::to_string(&body)?),
        )
        .await
}
