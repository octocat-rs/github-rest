use core::fmt;
use std::error::Error;

use async_trait::async_trait;
use github_api::end_points::EndPoints;
use reqwest::{Body, StatusCode};
use serde::{de::DeserializeOwned, Serialize};

pub use github_api::end_points;
#[cfg(feature = "builders")]
pub mod builders;
#[cfg(feature = "client")]
pub mod client;
pub mod methods;
pub mod structs;

#[derive(Debug)]
pub enum GithubRestError {
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
    ResponseError(StatusCode, String),
    AnyError(),
}

impl fmt::Display for GithubRestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error ocurred as you can see")
    }
}

impl Error for GithubRestError {}

impl From<reqwest::Error> for GithubRestError {
    fn from(e: reqwest::Error) -> Self {
        GithubRestError::ReqwestError(e)
    }
}

impl From<serde_json::Error> for GithubRestError {
    fn from(e: serde_json::Error) -> Self {
        GithubRestError::JsonError(e)
    }
}

#[async_trait]
pub trait Requester {
    async fn raw_req<T, V>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Body> + std::marker::Send;

    async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Body> + std::marker::Send;
}
