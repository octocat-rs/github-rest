mod issues;
mod misc;
pub use issues::*;
pub use misc::*;

///Prelude mod used for methods
pub mod prelude {

    pub use github_api::end_points::*;
    pub use reqwest::Body;
    pub use serde::{Deserialize, Serialize};

    pub use crate::structs::CreateIssueResponse;
    pub use crate::GithubRestError;
    pub use crate::Requester;
}
