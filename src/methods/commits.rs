use super::prelude::*;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetCommitsBody {
    ///SHA or branch to start listing commits from. Default: the repository’s default branch (usually master).
    sha: Option<String>,
    ///Only commits containing this file path will be returned.
    path: Option<String>,
    ///GitHub login or email address by which to filter by commit author.
    author: Option<String>,
    ///Only show notifications updated after the given time. This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
    since: Option<String>,
    ///Only commits before this date will be returned. This is a timestamp in ISO 8601 format: YYYY-MM-DDTHH:MM:SSZ.
    until: Option<String>,
    ///Results per page (max 100)
    ///Default: 30
    per_page: Option<String>,
    ///Page number of the results to fetch.
    ///Default: 1
    page: Option<String>,
}

/// * tags repos
/// * get `/repos/{owner}/{repo}/commits`
/// * docs <https://docs.github.com/rest/reference/repos#list-commits>
///
/// List commits
/// **Signature verification object**
///
/// The response will include a `verification` object that describes the result of verifying the commit's signature. The following fields are included in the `verification` object:
///
/// | Name | Type | Description |
/// | ---- | ---- | ----------- |
/// | `verified` | `boolean` | Indicates whether GitHub considers the signature in this commit to be verified. |
/// | `reason` | `string` | The reason for verified value. Possible values and their meanings are enumerated in table below. |
/// | `signature` | `string` | The signature that was extracted from the commit. |
/// | `payload` | `string` | The value that was signed. |
///
/// These are the possible values for `reason` in the `verification` object:
///
/// | Value | Description |
/// | ----- | ----------- |
/// | `expired_key` | The key that made the signature is expired. |
/// | `not_signing_key` | The "signing" flag is not among the usage flags in the GPG key that made the signature. |
/// | `gpgverify_error` | There was an error communicating with the signature verification service. |
/// | `gpgverify_unavailable` | The signature verification service is currently unavailable. |
/// | `unsigned` | The object does not include a signature. |
/// | `unknown_signature_type` | A non-PGP signature was found in the commit. |
/// | `no_user` | No user was associated with the `committer` email address in the commit. |
/// | `unverified_email` | The `committer` email address in the commit was associated with a user, but the email address is not verified on her/his account. |
/// | `bad_email` | The `committer` email address in the commit is not included in the identities of the PGP key that made the signature. |
/// | `unknown_key` | The key that made the signature has not been registered with any user's account. |
/// | `malformed_signature` | There was an error parsing the signature. |
/// | `invalid` | The signature could not be cryptographically verified using the key whose key-id was found in the signature. |
/// | `valid` | None of the above errors applied, so the signature is considered to be verified. |
pub async fn get_commits<T>(
    client: &T,
    owner: String,
    repo: String,
    options: Option<&GetCommitsBody>,
) -> Result<Commits, GithubRestError>
where
    T: Requester,
{
    client
        .req::<GetCommitsBody, String, Commits>(EndPoints::GetReposownerrepoCommits(owner, repo), options, None)
        .await
}

#[cfg(test)]
mod tests {
    use crate::client::DefaultRequest;

    use super::*;

    #[tokio::test]
    async fn test_get_commits() {
        let reqester = DefaultRequest::new_none();

        let r = get_commits(&reqester, "microsoft".to_owned(), "vscode".to_owned(), None)
            .await
            .unwrap();
        println!("{:#?}", r)
    }
}
