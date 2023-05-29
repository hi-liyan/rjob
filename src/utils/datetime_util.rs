use chrono::{DateTime, Local, TimeZone};
use chrono_tz::Tz;

/// Get the current local datetime as a formatted string.
/// The format of the string is "%Y-%m-%d %H:%M:%S.%3f".
///
/// # Examples
///
/// ```
/// let datetime = get_local_datetime();
/// println!("Current datetime: {}", datetime);
/// ```
#[allow(dead_code)]
pub fn get_local_datetime() -> String {
    let local_time: DateTime<Local> = Local::now();
    local_time.format("%Y-%m-%d %H:%M:%S.%3f").to_string()
}

/// Get the current local datetime in the specified timezone as a formatted string.
///
/// # Arguments
///
/// * `timezone` - The timezone to convert the datetime to.
///
/// # Example
///
/// ```
/// use chrono_tz::Tz;
/// use crate::get_local_datetime_in_timezone;
///
/// let timezone = Tz::UTC;
/// let datetime = get_local_datetime_in_timezone(timezone);
/// println!("Current datetime in UTC: {}", datetime);
/// ```
#[allow(dead_code)]
pub fn get_local_datetime_in_timezone(timezone: &Tz) -> String {
    let local_time: DateTime<Tz> = timezone.from_utc_datetime(&Local::now().naive_utc());
    local_time.format("%Y-%m-%d %H:%M:%S.%3f").to_string()
}
