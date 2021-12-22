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
pub async fn create_issue<T>(
    client: &T,
    owner: String,
    repo: String,
    body: CreateIssueBody,
) -> Result<CreateIssueResponse, GithubRestError>
where
    T: Requester,
{
    client
        .req::<String, String, CreateIssueResponse>(
            EndPoints::PostReposownerrepoIssues(owner, repo),
            None,
            Some(serde_json::to_string(&body)?),
        )
        .await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::DefaultRequest;

    #[tokio::test]
    async fn test_create_issue() {
        let reqester = DefaultRequest::new("TOKEN".to_owned());

        let bdy = CreateIssueBody {
            title: "tricked is cool".to_owned(),
            body: Some("This is very true".to_owned()),
            assignee: None,
            milestone: None,
            labels: None,
            assignees: None,
        };

        let r = create_issue(
            &reqester,
            "Tricked-dev".to_owned(),
            "octo-computing-machine".to_owned(),
            bdy,
        )
        .await
        .unwrap();
        println!("{:#?}", r)
    }
}
// TODO: add this function
// pub async fn get_issues<T: Requester>(client: T, repo: String,
//     owner: String,
//     body: CreateIssueBody,) -> Result<GetReposownerrepoIssuesResponse, CoolError> where T:Requester{
//     client
//         .req::<String, String, GetResponse>(EndPoints::GetReposownerrepoIssues(repo, owner), None, None)
//         .await
// }
