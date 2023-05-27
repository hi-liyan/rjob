use std::process;
use std::sync::Arc;
use reqwest::{Method};
use tokio_cron::{Job, Scheduler};

use crate::configure::http_jobs::get_http_jobs;
use crate::models::http_job::HttpJob;
use crate::utils::datetime_util::get_local_datetime;
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
    let http_jobs = match get_http_jobs() {
        Ok(jobs) => jobs,
        Err(err) => {
            eprintln!("Failed to retrieve HTTP jobs: {}", err);
            process::exit(1);
        }
    };

    let mut scheduler = Scheduler::local();

    for it in http_jobs {
        let it = Arc::new(it);
        if it.enable {
            let http_job = it.clone();
            let job = Job::new_sync(&it.cron, move || {
                let http_job = http_job.clone();
                tokio::spawn(start_http_job(http_job));
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
async fn start_http_job(http_job: Arc<HttpJob>) {
    let local_time = get_local_datetime();
    let uuid = generate_uuid_without_hyphens();

    println!("{} {} Http job start, job name: {}", uuid, local_time, &http_job.name);
    println!("{} {} Job: [{}]", uuid, local_time, &http_job);

    let request = &http_job.request;
    let method = get_method(&request.method);

    // Create an HTTP client
    let client = reqwest::Client::builder()
        .user_agent("rjob")
        .build()
        .unwrap();

    // Create the request builder
    let mut request_builder = client.request(method, &request.url);

    // Set request headers
    if let Some(headers) = &request.headers {
        request_builder = request_builder.headers(headers.clone());
    }

    // Set request body
    if let Some(body) = &request.body {
        request_builder = request_builder
            .header("Content-Type", "application/json")
            .body(body.to_owned());
    }

    // Send the request
    let resp = request_builder.send().await;

    // Handle the response
    match resp {
        Ok(resp) => {
            if resp.status().is_success() {
                println!("{} {} Http request success, job name: {}", uuid, local_time, &http_job.name);
                println!("{} {} Http response: {}", uuid, local_time, resp.text().await.unwrap());
            } else {
                println!("{} {} Http request failed, job name: {}, http status: {}", uuid, local_time, &http_job.name, resp.status().as_u16());
                println!("{} {} Http response: {}", uuid, local_time, resp.text().await.unwrap());
            }
        }
        Err(err) => {
            println!("{} {} Http request failed, job name: {}, error: {}", uuid, local_time, &http_job.name, err);
        }
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
        "options" => Method::OPTIONS,
        "delete" => Method::DELETE,
        _ => Method::GET
    }
}