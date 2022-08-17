use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE},
    Method, StatusCode,
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub struct ClientBuilder {
    base_url: String,
    base_headers: HeaderMap,
}

impl ClientBuilder {
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

    pub fn build(&self) -> Client {
        Client::new(&self.base_url, self.base_headers.clone())
    }
}

pub struct Client {
    base_url: String,
    base_headers: HeaderMap,
}

impl Client {
    pub fn new(base_url: &str, base_headers: HeaderMap) -> Self {
        Self {
            base_url: base_url.to_string(),
            base_headers,
        }
    }

    pub fn request(&self) -> RequestBuilder {
        RequestBuilder::new(&self.base_url, self.base_headers.clone())
    }
}

pub struct RequestBuilder {
    base_url: String,
    url: Option<String>,
    headers: HeaderMap,
    body: Option<String>,
}

impl RequestBuilder {
    pub fn new(base_url: &str, headers: HeaderMap) -> Self {
        Self {
            base_url: base_url.to_string(),
            url: None,
            headers,
            body: None,
        }
    }

    pub fn url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }

    pub fn header(mut self, key: HeaderName, value: &str) -> Self {
        self.headers
            .insert(key, HeaderValue::from_str(value).unwrap());
        self
    }

    pub fn json_body<T: Serialize>(mut self, body: T) -> Self {
        self.body = Some(serde_json::to_string(&body).unwrap());
        self.header(CONTENT_TYPE, "application/json")
    }

    pub fn get(&self) -> Request {
        Request::new(
            Method::GET,
            self.full_url(),
            self.headers.clone(),
            self.body.clone(),
        )
    }

    pub fn post(&self) -> Request {
        Request::new(
            Method::POST,
            self.full_url(),
            self.headers.clone(),
            self.body.clone(),
        )
    }

    pub fn put(&self) -> Request {
        Request::new(
            Method::PUT,
            self.full_url(),
            self.headers.clone(),
            self.body.clone(),
        )
    }

    pub fn patch(&self) -> Request {
        Request::new(
            Method::PATCH,
            self.full_url(),
            self.headers.clone(),
            self.body.clone(),
        )
    }

    pub fn delete(&self) -> Request {
        Request::new(
            Method::DELETE,
            self.full_url(),
            self.headers.clone(),
            self.body.clone(),
        )
    }

    fn full_url(&self) -> String {
        match &self.url {
            Some(url) => format!("{}/{}", &self.base_url, &url),
            None => self.base_url.to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Request {
    method: Method,
    full_url: String,
    headers: HeaderMap,
    body: Option<String>,
}

impl Request {
    pub fn new(method: Method, full_url: String, headers: HeaderMap, body: Option<String>) -> Self {
        Self {
            method,
            full_url,
            headers,
            body,
        }
    }

    pub fn send(&self) -> Response {
        let client = reqwest::blocking::Client::new();
        let mut request = client
            .request(self.method.clone(), self.full_url.clone())
            .headers(self.headers.clone());

        request = match &self.body {
            Some(body) => request.body(body.clone()),
            None => request,
        };

        let http_response = request.send().unwrap();

        Response::new(
            self.clone(),
            http_response.status(),
            http_response.text().unwrap(),
        )
    }
}

pub struct Response {
    request: Request,
    status: StatusCode,
    content: String,
}

impl Response {
    pub fn new(request: Request, status: StatusCode, content: String) -> Self {
        Self {
            request,
            status,
            content,
        }
    }

    pub fn text_body(&self) -> &str {
        &self.content
    }

    pub fn json_body(&self) -> HashMap<String, Value> {
        serde_json::from_str(&self.content).unwrap()
    }

    pub fn parsed_body<T: DeserializeOwned>(&self) -> T {
        serde_json::from_str(&self.content).unwrap()
    }
}
