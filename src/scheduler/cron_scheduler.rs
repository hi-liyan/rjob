use std::time::Duration;
use reqwest::{Method};
use tokio_cron::{Job, Scheduler};
use crate::configure::get_jobs;

use crate::models::http_job::HttpJob;
use crate::utils::datetime_util::{get_local_datetime_in_timezone};
use crate::utils::uuid_util::generate_uuid_without_hyphens;

/// Starts the cron scheduler for executing HTTP jobs.
///
/// This function retrieves the HTTP jobs using the `get_http_jobs` function and schedules them
/// based on their cron expressions. Only enabled jobs are scheduled for execution.
///
/// # Examples
///
/// ```rust
/// use tokio::runtime::Runtime;
///
/// let rt = Runtime::new().unwrap();
/// rt.block_on(async {
///     start_cron_scheduler().await;
/// });
/// ```
pub async fn start_cron_scheduler() {
    let jobs = get_jobs();
    let http_jobs = &jobs.http_jobs;

    let mut scheduler = Scheduler::new_in_timezone(jobs.timezone);

    for it in http_jobs {
        if it.enable {
            let job = Job::new_sync(&it.cron, move || {
                tokio::spawn(start_http_job(it));
            });
            scheduler.add(job);
        }
    }
}

/// Asynchronously starts an HTTP job by sending an HTTP request.
///
/// # Arguments
///
/// * `http_job` - An `Arc`-wrapped `HttpJob` struct representing the job to be started.
///
/// # Examples
///
/// ```rust
/// use std::sync::Arc;
///
/// let http_job = Arc::new(HttpJob {
///     name: "Test job".to_string(),
///     enable: true,
///     cron: "*/5 * * * * * *".to_string(),
///     request: HttpJobRequest {
///         method: "GET".to_string(),
///         url: "https://www.google.com".to_string(),
///         headers: None,
///         body: None
///     }
/// });
///
/// start_http_job(http_job).await;
/// ```
async fn start_http_job(http_job: &HttpJob) {
    let jobs = get_jobs();
    let timezone = &jobs.timezone;
    let uuid = generate_uuid_without_hyphens();
    let local_time = get_local_datetime_in_timezone(timezone);

    println!("{} {} Http job start, job name: {}", uuid, local_time, &http_job.name);
    println!("{} {} Job: [{}]", uuid, local_time, &http_job);

    let request = &http_job.request;
    let method = get_method(&request.method);
    let timeout = http_job.timeout.clone();

    let client = reqwest::Client::builder()
        .user_agent("rjob")
        .timeout(Duration::from_millis(timeout))
        .build()
        .expect("Failed to create HTTP client");

    let mut attempts = 0;
    let max_attempts = http_job.max_retry.clone();

    while attempts < max_attempts {
        attempts += 1;

        let request_builder = client.request(method.clone(), &request.url)
            .header("Content-Type", "application/json")
            .headers(request.headers.clone().unwrap_or_default())
            .body(request.body.clone().unwrap_or_default());

        let resp = match request_builder.send().await {
            Ok(resp) => resp,
            Err(err) => {
                println!("{} {} Http request failed, job name: {}, error: {}. Retry attempt: {}/{}", uuid, local_time, &http_job.name, err, attempts, max_attempts);
                continue;
            }
        };

        let status = resp.status();
        let text = resp.text().await.unwrap();

        if status.is_success() {
            println!("{} {} Http request success, job name: {}", uuid, local_time, &http_job.name);
            println!("{} {} Http response: {}", uuid, local_time, text);
        } else {
            println!("{} {} Http request failed, job name: {}, http status: {}", uuid, local_time, &http_job.name, status.as_u16());
            println!("{} {} Http response: {}", uuid, local_time, text);
        }
        break;
    }

    println!("{} {} Http job end, job name: {}\n", uuid, local_time, &http_job.name);
}

/// Get the corresponding `Method` enum value for the given HTTP method string.
///
/// # Arguments
///
/// * `method` - The HTTP method as a string.
///
/// # Returns
///
/// The corresponding `Method` enum value. If the provided method is not recognized,
/// the default value `Method::GET` is returned.
///
/// # Examples
///
/// ```
/// let method = get_method("POST");
/// println!("HTTP method: {:?}", method);
/// ```
fn get_method(method: &str) -> Method {
    match method.to_lowercase().as_str() {
        "get" => Method::GET,
        "post" => Method::POST,
        "put" => Method::PUT,
        "patch" => Method::PATCH,
        "options" => Method::OPTIONS,
        "delete" => Method::DELETE,
        "head" => Method::HEAD,
        _ => Method::GET
    }
}