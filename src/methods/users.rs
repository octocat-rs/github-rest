use super::prelude::*;

/// * tags users
/// * get `/user/followers`
/// * docs <https://docs.github.com/rest/reference/users#list-followers-of-the-authenticated-user>
///
/// List followers of the authenticated user
/// Lists the people following the authenticated user.
pub async fn get_followers<T>(client: &T, params: Option<&Pagination>) -> Result<Vec<User>, GithubRestError>
where
    T: Requester,
{
    client
        .req::<Pagination, String, Vec<User>>(EndPoints::GetUserFollowers(), params, None)
        .await
}

/// * tags users
/// * get `/user/following`
/// * docs <https://docs.github.com/rest/reference/users#list-the-people-the-authenticated-user-follows>
///
/// List the people the authenticated user follows
/// Lists the people who the authenticated user follows.
pub async fn get_following<T>(client: &T, params: Option<&Pagination>) -> Result<Vec<User>, GithubRestError>
where
    T: Requester,
{
    client
        .req::<Pagination, String, Vec<User>>(EndPoints::GetUserFollowing(), params, None)
        .await
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Pagination {
    ///Results per page (max 100)
    ///Default: 30
    pub per_page: Option<String>,
    ///Page number of the results to fetch.
    ///Default: 1
    pub page: Option<String>,
}

#[cfg(feature = "client")]
#[cfg(test)]
mod tests {
    use crate::client::DefaultRequest;

    use super::*;

    #[tokio::test]
    async fn test_get_followers() {
        //Note this requires auth
        let reqester = DefaultRequest::new_none();
        let r = get_followers(&reqester, None).await.unwrap();
        println!("{:?}", r)
    }

    #[tokio::test]
    async fn test_get_following() {
        //Note this requires auth
        let reqester = DefaultRequest::new_none();
        let r = get_followers(&reqester, None).await.unwrap();
        println!("{:?}", r)
    }
}
