use serde::Deserialize;
use strum::{EnumString, EnumVariantNames};

/// Used to represent all possible values for the `x-github-event` header sent
/// with all webhook payloads.
///
/// See also: <https://docs.github.com/en/developers/webhooks-and-events/webhooks/webhook-events-and-payloads>
#[derive(Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EventTypes { // TODO: Sort variants
    BranchProtectionRule,
    CodeScanningAlert,
    CommitComment,
    DeployKey,
    Deployment,
    DeploymentStatus,
    CheckRun,
    Discussion,
    DiscussionComment,
    GithubAppAuthorization,
    Gollum, // Wiki page created/updated
    Installation,
    InstallationRepositories,
    Label,
    MarketplacePurchase,
    Member,
    Membership,
    Meta,
    Milestone,
    Organization,
    OrgBlock,
    Package,
    PageBuild,
    Project,
    ProjectCard,
    ProjectColumn,
    Public,
    PullRequestReview,
    PullRequestReviewComment,
    RepositoryDispatch,
    Repository,
    RepositoryImport,
    RepositoryVulnerabilityAlert,
    SecretScanningAlert,
    SecurityAdvisory,
    Sponsorship,
    Status,
    Team,
    TeamAdd,
    WorkflowDispatch,
    CheckSuite,
    Create,
    Delete,
    Fork,
    IssueComment,
    Issues,
    Ping,
    PullRequest,
    Push,
    Release,
    Star,
    Watch,
    WorkflowJob,
    WorkflowRun,
}
