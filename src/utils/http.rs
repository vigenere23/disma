use std::collections::HashMap;

use reqwest::{
    blocking::{Client, RequestBuilder},
    header::{HeaderMap, HeaderName, HeaderValue},
};
use serde::{de::DeserializeOwned, Serialize};
use serde_json::Value;

pub struct HttpClient {
    base_url: String,
    headers: HeaderMap,
}

impl HttpClient {
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::new()
    }
}

impl HttpClient {
    pub fn get<ResponseBody>(&self, url: &str) -> ResponseBody
    where
        ResponseBody: DeserializeOwned,
    {
        let client = Client::new();
        let request = client.get(self.full_url(url));

        self.send_request(request)
    }

    pub fn post<RequestBody, ResponseBody>(&self, url: &str, body: RequestBody) -> ResponseBody
    where
        RequestBody: Serialize,
        ResponseBody: DeserializeOwned,
    {
        let client = Client::new();
        let request = client.post(self.full_url(url));

        self.send_body_request(request, body)
    }

    pub fn patch<RequestBody>(&self, url: &str, body: RequestBody)
    where
        RequestBody: Serialize,
    {
        let client = Client::new();
        let request = client.patch(self.full_url(url));

        self.send_body_request::<RequestBody, HashMap<String, Value>>(request, body);
    }

    pub fn delete(&self, url: &str) {
        let client = Client::new();
        let request = client.delete(self.full_url(url));

        self.send_request(request)
    }

    fn send_request<ResponseBody>(&self, builder: RequestBuilder) -> ResponseBody
    where
        ResponseBody: DeserializeOwned,
    {
        let request = builder.headers(self.headers.clone());
        let response = request.send().unwrap();

        if response.status().is_success() {
            response.json().unwrap()
        } else {
            panic!(
                "Error sending request. Returned body : {}",
                response.text().unwrap()
            )
        }
    }

    fn send_body_request<RequestBody, ResponseBody>(
        &self,
        builder: RequestBuilder,
        body: RequestBody,
    ) -> ResponseBody
    where
        RequestBody: Serialize,
        ResponseBody: DeserializeOwned,
    {
        let request = builder.headers(self.headers.clone()).json(&body);
        let response = request.send().unwrap();

        if response.status().is_success() {
            response.json().unwrap()
        } else {
            panic!(
                "Error sending request. Returned body : {}",
                response.text().unwrap()
            )
        }
    }

    fn full_url(&self, url_suffix: &str) -> String {
        return format!("{}{}", self.base_url, url_suffix);
    }
}

pub struct HttpClientBuilder {
    headers: HeaderMap,
    base_url: String,
}

impl HttpClientBuilder {
    pub fn new() -> HttpClientBuilder {
        Self {
            headers: HeaderMap::new(),
            base_url: String::new(),
        }
    }

    pub fn header(mut self, key: HeaderName, value: String) -> HttpClientBuilder {
        self.headers
            .insert(key, HeaderValue::from_str(value.as_str()).unwrap());
        self
    }

    pub fn base_url(mut self, url: &str) -> HttpClientBuilder {
        self.base_url = url.to_string();
        self
    }

    pub fn build(self) -> HttpClient {
        HttpClient {
            base_url: self.base_url,
            headers: self.headers,
        }
    }
}
