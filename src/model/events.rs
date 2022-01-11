use serde::Deserialize;
use strum::{EnumString, EnumVariantNames};

/// Used to represent all possible values for the `x-github-event` header sent
/// with all webhook payloads.
///
/// Currently non-exhaustive- feel free to yell at me with a link to a
/// comprehensive list of its possible values, I can't find one anywhere...
#[derive(Deserialize, EnumString, EnumVariantNames)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
#[non_exhaustive]
pub enum EventTypes {
    CheckRun,
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
