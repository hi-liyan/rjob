use chrono_tz::Tz;
use crate::models::http_job::HttpJob;

#[derive(Debug, Clone)]
pub struct Jobs {
    pub timezone: Tz,
    pub http_jobs: Vec<HttpJob>
}

impl Jobs {
    pub fn new(timezone: Tz, http_jobs: Vec<HttpJob>) -> Self {
        Jobs {
            timezone,
            http_jobs
        }
    }
}