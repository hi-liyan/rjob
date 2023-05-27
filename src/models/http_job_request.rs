use std::fmt::{Debug, Display, Formatter};
use reqwest::header::HeaderMap;

#[derive(Debug, Clone)]
pub struct HttpJobRequest {
    pub url: String,
    pub method: String,
    pub headers: Option<HeaderMap>,
    pub body: Option<String>,
}

impl HttpJobRequest {
    pub fn new(url: String, method: String, headers: Option<HeaderMap>, body: Option<String>) -> Self {
        HttpJobRequest {
            url,
            method,
            headers,
            body,
        }
    }
}

impl Display for HttpJobRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let headers = match &self.headers {
            Some(h) => format!("{:?}", h),
            None => "None".to_string()
        };
        let body = match &self.body {
            Some(b) => b,
            None => "None"
        };
        write!(f, "url: {}, method: {}, headers: {}, body: {}",
               self.url,
               self.method,
               headers,
               body)
    }
}