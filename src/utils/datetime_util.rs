use chrono::{DateTime, Local};

/// Get the current local date and time as a formatted string.
/// The format of the string is "%Y-%m-%d %H:%M:%S.%3f".
///
/// # Examples
///
/// ```
/// let datetime = get_local_datetime();
/// println!("Current datetime: {}", datetime);
/// ```
pub fn get_local_datetime() -> String {
    let local_time: DateTime<Local> = Local::now();
    local_time.format("%Y-%m-%d %H:%M:%S.%3f").to_string()
}