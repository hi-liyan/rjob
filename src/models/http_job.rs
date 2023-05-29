use std::fmt::{Display, Formatter};
use crate::models::http_job_request::HttpJobRequest;

#[derive(Debug, Clone)]
pub struct HttpJob {
    pub name: String,
    pub enable: bool,
    pub cron: String,
    pub timeout: u64,
    pub max_retry: u64,
    pub request: HttpJobRequest,
}

impl HttpJob {
    pub fn new(name: String, enable: bool, cron: String, timeout: u64, max_retry: u64, request: HttpJobRequest) -> Self {
        HttpJob {
            name,
            enable,
            cron,
            timeout,
            max_retry,
            request,
        }
    }
}

impl Display for HttpJob {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "name: {}, enable: {}, cron: {}, timeout: {}, max_retry: {}, request: [{}]",
               self.name, self.enable, self.cron, self.timeout, self.max_retry, self.request)
    }
}