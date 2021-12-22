pub use issues::*;
pub use misc::*;

mod issues;
mod misc;

///Prelude mod used for methods
pub mod prelude {
    pub use github_api::end_points::*;
    pub use reqwest::Body;
    pub use serde::{Deserialize, Serialize};

    pub use crate::{structs::CreateIssueResponse, GithubRestError, Requester};
}
