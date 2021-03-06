use super::{push_event_nested::HeadCommit, Repository, User};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::{EnumString, EnumVariantNames};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum WorkflowRunAction {
    Requested,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRunEvent {
    pub action: WorkflowRunAction,
    pub workflow_run: WorkflowRun,
    pub workflow: Workflow,
    pub repository: Repository,
    pub sender: User,
}
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Workflow {
    pub id: i64,
    pub node_id: String,
    pub name: String,
    pub path: String,
    pub state: String,
    pub created_at: String,
    pub updated_at: String,
    pub url: String,
    pub html_url: String,
    pub badge_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRun {
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub head_branch: String,
    pub head_sha: String,
    pub run_number: i64,
    pub event: String,
    pub status: String,
    pub conclusion: Value,
    pub workflow_id: i64,
    pub check_suite_id: i64,
    pub check_suite_node_id: String,
    pub url: String,
    pub html_url: String,
    pub pull_requests: Vec<Value>,
    pub created_at: String,
    pub updated_at: String,
    pub run_attempt: i64,
    pub run_started_at: String,
    pub jobs_url: String,
    pub logs_url: String,
    pub check_suite_url: String,
    pub artifacts_url: String,
    pub cancel_url: String,
    pub rerun_url: String,
    pub previous_attempt_url: Value,
    pub workflow_url: String,
    pub head_commit: HeadCommit,
    pub repository: Repository,
    pub head_repository: Repository,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum WorkflowJobAction {
    Queued,
    Completed,
    InProgress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowJobEvent {
    pub action: WorkflowRunAction,
    pub workflow_job: WorkflowJob,
    pub repository: Repository,
    pub sender: User,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum CheckRunAction {
    Completed,
    Created,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckRunEvent {
    action: CheckRunAction,
    check_run: CheckRun,
    repository: Repository,
    sender: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkflowJob {
    pub id: i64,
    pub run_id: i64,
    pub run_url: String,
    pub run_attempt: i64,
    pub node_id: String,
    pub head_sha: String,
    pub url: String,
    pub html_url: String,
    pub status: String,
    pub conclusion: Value,
    pub started_at: String,
    pub completed_at: Value,
    pub name: String,
    pub steps: Vec<Step>,
    pub check_run_url: String,
    pub labels: Vec<String>,
    pub runner_id: i64,
    pub runner_name: String,
    pub runner_group_id: i64,
    pub runner_group_name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Step {
    pub name: String,
    pub status: String,
    pub conclusion: Value,
    pub number: i64,
    pub started_at: String,
    pub completed_at: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckRun {
    pub id: i64,
    pub name: String,
    pub node_id: String,
    pub head_sha: String,
    pub external_id: String,
    pub url: String,
    pub html_url: String,
    pub details_url: String,
    pub status: String,
    pub conclusion: String,
    pub started_at: String,
    pub completed_at: String,
    pub output: Output,
    pub check_suite: CheckSuite,
    pub app: App,
    pub pull_requests: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Output {
    pub title: Value,
    pub summary: Value,
    pub text: Value,
    pub annotations_count: i64,
    pub annotations_url: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckSuite {
    pub id: i64,
    pub node_id: String,
    pub head_branch: String,
    pub head_sha: String,
    pub status: String,
    pub conclusion: Value,
    pub url: String,
    pub before: String,
    pub after: String,
    pub pull_requests: Vec<Value>,
    pub app: App,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct App {
    pub id: i64,
    pub slug: String,
    pub node_id: String,
    pub owner: User,
    pub name: String,
    pub description: String,
    pub external_url: String,
    pub html_url: String,
    pub created_at: String,
    pub updated_at: String,
    pub permissions: Permissions,
    pub events: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Permissions {
    pub actions: String,
    pub administration: String,
    pub checks: String,
    pub contents: String,
    pub deployments: String,
    pub discussions: String,
    pub issues: String,
    pub metadata: String,
    pub organization_packages: String,
    pub packages: String,
    pub pages: String,
    pub pull_requests: String,
    pub repository_hooks: String,
    pub repository_projects: String,
    pub security_events: String,
    pub statuses: String,
    pub vulnerability_alerts: String,
}
