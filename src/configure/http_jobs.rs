use std::error::Error;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde_json::Value;
use crate::configure::get_configure;
use crate::models::http_job::HttpJob;
use crate::models::http_job_request::HttpJobRequest;

/// Retrieves a list of HTTP jobs from the JSON configuration.
///
/// Returns:
/// - A vector of `HttpJob` instances if successful.
/// - An `Err` containing the error message if any error occurs during reading or parsing the JSON configuration.
///
/// # Examples
///
/// ```
/// use crate::http_job::HttpJob;
///
/// match get_http_jobs() {
///     Ok(http_jobs) => {
///         for job in http_jobs {
///             // Process each HTTP job
///             println!("Name: {}", job.name);
///             println!("Enabled: {}", job.enable);
///             println!("Cron: {}", job.cron);
///             // ...
///         }
///     }
///     Err(err) => {
///         eprintln!("Error while retrieving HTTP jobs: {}", err);
///         // Handle the error accordingly
///     }
/// }
/// ```
pub fn get_http_jobs() -> Result<Vec<HttpJob>, Box<dyn Error>> {
    let json_configure = get_configure()?;

    let http_jobs_val = json_configure.get("http_jobs")
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
            .ok_or("The 'enable' field is missing or not a boolean.")?;

        let cron = it.get("cron")
            .and_then(|c| c.as_str())
            .ok_or("The 'cron' field is missing or not a string.")?
            .to_string();

        let request = get_http_job_request(&it)?;

        let http_job = HttpJob::new(name, enable, cron, request);
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
        .ok_or("The 'method' field is required and must be a string.")?
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