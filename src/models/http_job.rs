use std::fmt::{Display, Formatter};
use crate::models::http_job_request::HttpJobRequest;

#[derive(Debug, Clone)]
pub struct HttpJob {
    pub name: String,
    pub enable: bool,
    pub cron: String,
    pub request: HttpJobRequest
}

impl HttpJob {
    pub fn new(name: String, enable: bool, cron: String, request: HttpJobRequest) -> Self {
        HttpJob {
            name,
            enable,
            cron,
            request
        }
    }
}

impl Display for HttpJob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, enable: {}, cron: {}, request: [{}]", self.name, self.enable, self.cron, self.request)
    }
}