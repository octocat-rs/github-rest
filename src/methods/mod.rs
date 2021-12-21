mod issues;
mod misc;
pub use issues::*;
pub use misc::*;

///Prelude mod used for methods
pub mod prelude {
    pub use crate::Requester;
    pub use crate::{CoolError, DefaultRequest};
    pub use github_api::end_points::*;
    pub use reqwest::Body;
    pub use serde::{Deserialize, Serialize};
}
