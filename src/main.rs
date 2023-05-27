use std::time::Duration;

use crate::scheduler::cron_scheduler::start_cron_scheduler;

mod models;
mod configure;
mod scheduler;
mod utils;

#[tokio::main]
async fn main() {
    start_cron_scheduler().await;
    tokio::time::sleep(Duration::MAX).await;
}
