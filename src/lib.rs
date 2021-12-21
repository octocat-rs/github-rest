pub mod methods;

use core::fmt;
use github_api::end_points::{EndPoints, Methods};
use reqwest::{header, Body};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

pub struct DefaultRequest {}

#[async_trait]
impl Requester for DefaultRequest {
    async fn raw_req<T, V>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<String, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Body> + std::marker::Send,
    {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::USER_AGENT,
            header::HeaderValue::from_str("tricked.pro/v2").unwrap(),
        );
        headers.insert(
            header::ACCEPT,
            header::HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();
        let path = format!("https://api.github.com{}", url.path());
        let mut req = match url.method() {
            Methods::Get => client.get(path),
            Methods::Post => client.post(path),
            Methods::Put => client.put(path),
            Methods::Patch => client.patch(path),
            Methods::Delete => client.delete(path),
        };
        if let Some(query) = query {
            req = req.query(query)
        }
        if let Some(body) = body {
            req = req.body(body)
        }
        let txt = req.send().await?.text().await?;
        Ok(txt)
    }
    async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, GithubRestError>
    where
        T: Serialize + ?Sized + std::marker::Send + std::marker::Sync,
        V: Into<Body> + std::marker::Send,
    {
        let r = self.raw_req(url, query, body).await?;
        Ok(serde_json::from_str(&r)?)
    }
}

#[derive(Debug)]
pub enum GithubRestError {
    ReqwestError(reqwest::Error),
    JsonError(serde_json::Error),
}

impl fmt::Display for GithubRestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "MONKEY")
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

use async_trait::async_trait;
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
