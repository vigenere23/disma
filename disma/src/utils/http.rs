#![allow(dead_code)]

use std::{error::Error, fmt::Display};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Method, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

#[derive(Clone, Default)]
pub struct Client {
    base_url: String,
    base_headers: HeaderMap,
}

impl Client {
    pub fn new() -> Self {
        Self {
            base_url: String::new(),
            base_headers: HeaderMap::new(),
        }
    }

    pub fn base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    pub fn header(mut self, key: HeaderName, value: &str) -> Self {
        self.base_headers
            .insert(key, HeaderValue::from_str(value).unwrap());
        self
    }

    pub fn get(self, url: &str) -> Request {
        self.request(Method::GET, url)
    }

    pub fn post(self, url: &str) -> Request {
        self.request(Method::POST, url)
    }

    pub fn patch(self, url: &str) -> Request {
        self.request(Method::PATCH, url)
    }

    pub fn put(self, url: &str) -> Request {
        self.request(Method::PUT, url)
    }

    pub fn delete(self, url: &str) -> Request {
        self.request(Method::DELETE, url)
    }

    pub fn request(self, method: Method, url: &str) -> Request {
        Request::new(method, &self.full_url(url)).headers(&self.base_headers)
    }

    fn full_url(&self, url_suffix: &str) -> String {
        if self.base_url.is_empty() {
            url_suffix.to_string()
        } else {
            format!("{}{}", &self.base_url, &url_suffix)
        }
    }
}

#[derive(Debug)]
pub enum HttpError {
    SendingRequest(String),
    ParsingTextResponse(String),
    SendingJsonPayload(String),
}

impl Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SendingRequest(description) => {
                f.write_str(&format!("Could not send request. Error: {description}"))
            }
            Self::SendingJsonPayload(description) => f.write_str(&format!(
                "Error occured while serializing json body. Error: {description}"
            )),
            Self::ParsingTextResponse(description) => f.write_str(&format!(
                "Could not fetch response text content. Error: {description}"
            )),
        }
    }
}

impl Error for HttpError {}

#[derive(Debug)]
pub struct Request {
    method: Method,
    url: String,
    headers: HeaderMap,
    body: Option<String>,
}

impl Request {
    pub fn new(method: Method, url: &str) -> Self {
        Self {
            method,
            url: url.to_string(),
            headers: HeaderMap::new(),
            body: None,
        }
    }

    pub fn get(url: &str) -> Self {
        Self::new(Method::GET, url)
    }

    pub fn post(url: &str) -> Self {
        Self::new(Method::POST, url)
    }

    pub fn patch(url: &str) -> Self {
        Self::new(Method::PATCH, url)
    }

    pub fn put(url: &str) -> Self {
        Self::new(Method::PUT, url)
    }

    pub fn delete(url: &str) -> Self {
        Self::new(Method::DELETE, url)
    }

    pub fn header(mut self, key: HeaderName, value: &str) -> Self {
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
        self
    }

    pub fn headers(mut self, headers: &HeaderMap) -> Self {
        self.headers = headers.clone();
        self
    }

    pub fn json_body<T: Serialize>(mut self, body: T) -> Result<Self, HttpError> {
        let json_body = serde_json::to_string(&body)
            .map_err(|error| HttpError::SendingJsonPayload(error.to_string()))?;

        self.body = Some(json_body);
        Ok(self.header(CONTENT_TYPE, "application/json"))
    }

    pub fn send(self) -> Result<Response, HttpError> {
        let client = reqwest::blocking::Client::new();
        let mut request = client
            .request(self.method.clone(), self.url.clone())
            .headers(self.headers.clone());

        request = match &self.body {
            Some(body) => request.body(body.clone()),
            None => request,
        };

        let http_response = request
            .send()
            .map_err(|error| HttpError::SendingRequest(error.to_string()))?;

        let status = http_response.status();
        let text_content = http_response
            .text()
            .map_err(|error| HttpError::ParsingTextResponse(error.to_string()))?;

        let response = Response::new(self, status, &text_content);
        Ok(response)
    }
}

#[derive(Debug)]
pub struct Response {
    pub request: Request,
    pub status: StatusCode,
    pub content: String,
}

impl Response {
    pub fn new(request: Request, status: StatusCode, content: &str) -> Self {
        Self {
            request,
            status,
            content: content.to_string(),
        }
    }

    pub fn text_body(&self) -> &str {
        &self.content
    }

    pub fn json_body(&self) -> Result<Value, String> {
        serde_json::from_str(&self.content).map_err(|error| {
            format!(
                "Could not parse response body to json.\nBody: {}\nError: {}",
                &self.content, error
            )
        })
    }

    pub fn parsed_body<T: DeserializeOwned>(&self) -> Result<T, String> {
        serde_json::from_str(&self.content).map_err(|error| {
            format!(
                "Could not parse response body to specified type.\nBody: {}\nError: {}",
                &self.content, error
            )
        })
    }
}
