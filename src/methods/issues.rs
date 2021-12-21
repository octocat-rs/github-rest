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
) -> Result<GetResponse, GithubRestError> {
    client
        .req::<String, String, GetResponse>(
            EndPoints::PostReposownerrepoIssues(repo, owner),
            None,
            Some(serde_json::to_string(&body)?),
        )
        .await
}
// TODO: add this function
// pub async fn get_issues<T: Requester>(client: T, repo: String,
//     owner: String,
//     body: CreateIssueBody,) -> Result<GetReposownerrepoIssuesResponse, CoolError> where T:Requester{
//     client
//         .req::<String, String, GetResponse>(EndPoints::GetReposownerrepoIssues(repo, owner), None, None)
//         .await
// }
