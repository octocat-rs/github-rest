use async_trait::async_trait;
use core::fmt;
use github_api::end_points::{EndPoints, Methods};
use reqwest::{header, Body};
use serde::{de::DeserializeOwned, Serialize};
use std::error::Error;

#[async_trait]
pub trait Requester<E: Error> {
    // async fn raw_req<T, V>(
    //     &self,
    //     url: EndPoints,
    //     query: Option<&T>,
    //     body: Option<V>,
    // ) -> Result<String, E>
    // where
    //     T: Serialize + ?Sized,
    //     V: Into<Body>;

    async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, E>
    where
        T: Serialize + ?Sized,
        V: Into<Body>;
}

pub struct DefaultRequest {}

#[async_trait]
impl Requester<CoolError> for DefaultRequest {
    // async fn raw_req<T, V>(
    //     &self,
    //     url: EndPoints,
    //     query: Option<&T>,
    //     body: Option<V>,
    // ) -> Result<String, Error>
    // where
    //     T: Serialize + ?Sized,
    //     V: Into<Body>,
    // {
    //     let mut headers = header::HeaderMap::new();
    //     headers.insert(
    //         header::USER_AGENT,
    //         header::HeaderValue::from_str("tricked.pro/v2").unwrap(),
    //     );
    //     let client = reqwest::Client::builder()
    //         .default_headers(headers)
    //         .build()
    //         .unwrap();
    //     let path = format!("https://api.github.com{}", url.path());
    //     let mut req = match url.method() {
    //         Methods::Get => client.get(path),
    //         Methods::Post => client.post(path),
    //         Methods::Put => client.put(path),
    //         Methods::Patch => client.patch(path),
    //         Methods::Delete => client.delete(path),
    //     };
    //     if let Some(query) = query {
    //         req = req.query(query)
    //     }
    //     if let Some(body) = body {
    //         req = req.body(body)
    //     }
    //     let txt = req.send().await?.text().await?;

    //     Ok(txt)
    // }
    async fn req<T, V, A: DeserializeOwned>(
        &self,
        url: EndPoints,
        query: Option<&T>,
        body: Option<V>,
    ) -> Result<A, CoolError>
    where
        T: Serialize + ?Sized,
        V: Into<Body>,
    {
        let client = reqwest::Client::builder().build().unwrap();
        let req = client
            .get(format!("https://api.github.com{}", url.path()))
            .send()
            .await
            .unwrap();
        // let mut headers = header::HeaderMap::new();
        // headers.insert(
        //     header::USER_AGENT,
        //     header::HeaderValue::from_str("tricked.pro/v2").unwrap(),
        // );
        // let client = reqwest::Client::builder()
        //     .default_headers(headers)
        //     .build()
        //     .unwrap();
        // let path = format!("https://api.github.com{}", url.path());
        // let mut req = match url.method() {
        //     Methods::Get => client.get(path),
        //     Methods::Post => client.post(path),
        //     Methods::Put => client.put(path),
        //     Methods::Patch => client.patch(path),
        //     Methods::Delete => client.delete(path),
        // };
        // if let Some(query) = query {
        //     req = req.query(query)
        // }
        // if let Some(body) = body {
        //     req = req.body(body)
        // }
        let txt = &req.text().await?;
        // println!("{}", txt);
        Ok(serde_json::from_str(txt)?)
    }
}

#[derive(Debug)]
struct CoolError;

impl fmt::Display for CoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}

impl Error for CoolError {}

// #[derive(Debug)]
// pub enum Error {
//     ReqwestError(reqwest::Error),
//     SerdeError(serde_json::Error),
// }

impl From<reqwest::Error> for CoolError {
    fn from(e: reqwest::Error) -> Self {
        CoolError {}
    }
}
impl From<serde_json::Error> for CoolError {
    fn from(e: serde_json::Error) -> Self {
        CoolError {}
    }
}
