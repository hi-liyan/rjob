use std::error::Error;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use crate::models::http_job::HttpJob;
use crate::models::http_job_request::HttpJobRequest;

/// Parses the JSON configuration and retrieves the list of HTTP jobs.
///
/// # Arguments
///
/// * `value` - The JSON configuration value.
///
/// # Returns
///
/// A `Result` containing a vector of `HttpJob` on success, or an error message on failure.
///
/// # Errors
///
/// This function can return an error under the following conditions:
///
/// * The 'http_jobs' field is missing in the JSON configuration.
/// * The 'http_jobs' field is not an array in the JSON configuration.
/// * The 'name' field is missing or not a string for any HTTP job.
/// * The 'enable' field is missing or not a boolean for any HTTP job.
/// * The 'cron' field is missing or not a string for any HTTP job.
/// * Failed to parse the 'request' field for any HTTP job.
///
pub fn get_http_jobs(value: Value) -> Result<Vec<HttpJob>, Box<dyn Error>> {
    let http_jobs_val = value.get("http_jobs")
        .ok_or("The 'http_jobs' field is missing in the JSON configuration.")?;

    let http_jobs_val = http_jobs_val.as_array()
        .ok_or("The 'http_jobs' field must be an array in the JSON configuration.")?;

    let mut http_jobs: Vec<HttpJob> = Vec::new();

    for it in http_jobs_val {
        let name = it.get("name")
            .and_then(|n| n.as_str())
            .ok_or("The 'name' field is missing or not a string.")?
            .to_string();

        let enable = it.get("enable")
            .and_then(|e| e.as_bool())
            .unwrap_or(true);

        let cron = it.get("cron")
            .and_then(|c| c.as_str())
            .ok_or("The 'cron' field is missing or not a string.")?
            .to_string();

        let timeout = it.get("timeout")
            .and_then(|t| t.as_u64())
            .unwrap_or(5000);

        let max_retry = it.get("max_retry")
            .and_then(|m| m.as_u64())
            .unwrap_or(3);

        let request = get_http_job_request(&it)?;

        let http_job = HttpJob::new(name, enable, cron, timeout, max_retry, request);
        http_jobs.push(http_job);
    }

    Ok(http_jobs)
}

/// Parses the given JSON value and constructs an HTTP request.
///
/// Parameters:
/// - `value`: JSON value containing the request information.
///
/// Returns:
/// The constructed `HttpJobRequest` instance.
///
/// # Examples
///
/// ```
/// use serde_json::json;
/// use crate::http_job::HttpJobRequest;
///
/// let json_value = json!({
///     "request": {
///         "url": "https://example.com",
///         "method": "GET",
///         "headers": {
///             "Content-Type": "application/json"
///         },
///         "body": {
///             "key": "value"
///         }
///     }
/// });
///
/// let request = get_http_job_request(&json_value);
/// println!("URL: {}", request.url);
/// println!("Method: {}", request.method);
/// // ...
/// ```
fn get_http_job_request(value: &Value) -> Result<HttpJobRequest, Box<dyn Error>> {
    let request = value.get("request")
        .ok_or("The 'request' field is required in the JSON value.")?;

    let url = request.get("url")
        .and_then(|u| u.as_str())
        .ok_or("The 'url' field is required and must be a string.")?
        .to_string();

    let method = request.get("method")
        .and_then(|m| m.as_str())
        .unwrap_or("GET")
        .to_string();

    let headers: Result<Option<HeaderMap>, Box<dyn Error>> = request.get("headers")
        .and_then(|h| h.as_object())
        .map(|map| {
            let mut header_map = HeaderMap::new();
            for (k, v) in map {
                let k = HeaderName::try_from(k)?;
                let v = v.as_str().ok_or("The value of the header must be a string.")?;
                let v = HeaderValue::try_from(v)?;
                header_map.append(k, v);
            }
            Ok(header_map)
        })
        .transpose();

    let body = request.get("body")
        .and_then(|b| b.as_object())
        .map(|body| {
            serde_json::to_string(body)
                .map_err(|_| format!("Error parsing request body."))
        })
        .transpose();

    Ok(HttpJobRequest::new(url, method, headers?, body?))
}