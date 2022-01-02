pub use commits::*;
pub use issues::*;
pub use misc::*;

mod commits;
mod issues;
mod misc;

///Prelude mod used for methods
pub mod prelude {
    pub use github_api::end_points::*;
    pub use reqwest::Body;
    pub use serde::{Deserialize, Serialize};

    pub use crate::{
        structs::{nested::*, *}, // You're gonna have to fix this I'm tired & have no idea what I should do (I just named the conflicting module push_event_nested )
        GithubRestError, Requester,
    };
}
