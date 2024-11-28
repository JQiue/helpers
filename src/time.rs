use chrono::{DateTime, Duration, Local, Utc};

/// Returns the current local date and time.
///
/// This function provides a convenient way to retrieve the current timestamp
/// using the local timezone from the `chrono` crate.
///
/// # Returns
///
/// A `DateTime<Local>` representing the current moment in the local time zone.
///
/// # Examples
///
/// ```rust
/// use chrono::Local;
///
/// let current_time = now();
/// println!("Current time: {}", current_time);
/// ```
///
/// # Note
///
/// - The returned time is based on the system's local time zone
/// - Precision depends on the system clock
/// - Recommended for logging, timestamps, and local time-related operations
///
/// # Panics
///
/// This function will not panic under normal circumstances.
pub fn now() -> DateTime<Local> {
  chrono::Local::now()
}

/// Get the current UTC date and time
///
/// Returns the current timestamp using the UTC timezone
///
/// # Returns
///
/// `DateTime<Utc>` representing the current UTC time
///
/// # Examples
///
/// ```rust
/// let current_utc_time = utc_now();
/// println!("Current UTC time: {}", current_utc_time);
/// ```
pub fn utc_now() -> DateTime<Utc> {
  chrono::Utc::now()
}

/// Get the current Unix timestamp in seconds
///
/// Returns the number of seconds since the Unix epoch (1970-01-01 00:00:00 UTC)
///
/// # Returns
///
/// `i64` Unix timestamp in seconds
///
/// # Examples
///
/// ```rust
/// let current_timestamp = timestamp();
/// println!("Current timestamp (seconds): {}", current_timestamp);
/// ```
pub fn timestamp() -> i64 {
  Utc::now().timestamp()
}

/// Get the current Unix timestamp in milliseconds
///
/// Returns the number of milliseconds since the Unix epoch (1970-01-01 00:00:00 UTC)
///
/// # Returns
///
/// `i64` Unix timestamp in milliseconds
///
/// # Examples
///
/// ```rust
/// let current_timestamp_ms = timestamp_millis();
/// println!("Current timestamp (milliseconds): {}", current_timestamp_ms);
/// ```
pub fn timestamp_millis() -> i64 {
  Utc::now().timestamp_millis()
}

/// Format a datetime as a string
///
/// Converts a datetime to a string using the specified format
///
/// # Parameters
///
/// - `dt`: Local datetime to be formatted
/// - `fmt`: Format pattern string
///
/// # Returns
///
/// Formatted datetime string
///
/// # Examples
///
/// ```rust
/// let dt = Local::now();
/// let formatted = format(dt, "%Y-%m-%d %H:%M:%S");
/// println!("Formatted time: {}", formatted);
/// ```
///
/// # Format Specifiers
/// - `%Y`: 4-digit year
/// - `%m`: 2-digit month
/// - `%d`: 2-digit day
/// - `%H`: Hour in 24-hour format
/// - `%M`: Minutes
/// - `%S`: Seconds
pub fn format(dt: DateTime<Local>, fmt: &str) -> String {
  dt.format(fmt).to_string()
}

/// Convert UTC time to local time
///
/// # Parameters
///
/// - `dt`: UTC datetime to convert
///
/// # Returns
///
/// Converted local datetime
///
/// # Examples
///
/// ```rust
/// let utc_time = Utc::now();
/// let local_time = utc_to_local(utc_time);
/// ```
pub fn utc_to_local(dt: DateTime<Utc>) -> DateTime<Local> {
  dt.with_timezone(&Local)
}

/// Convert local time to UTC time
///
/// # Parameters
///
/// - `dt`: Local datetime to convert
///
/// # Returns
///
/// Converted UTC datetime
///
/// # Examples
///
/// ```rust
/// let local_time = Local::now();
/// let utc_time = local_to_utc(local_time);
/// ```
pub fn local_to_utc(dt: DateTime<Local>) -> DateTime<Utc> {
  dt.with_timezone(&Utc)
}

/// Add specified number of days to a UTC datetime
///
/// # Parameters
///
/// - `dt`: Original UTC datetime
/// - `days`: Number of days to add (can be negative)
///
/// # Returns
///
/// UTC datetime after adding specified days
///
/// # Examples
///
/// ```rust
/// let now = Utc::now();
/// let future_date = add_days(now, 7);  // 7 days later
/// let past_date = add_days(now, -7);   // 7 days ago
/// ```
pub fn add_days(dt: DateTime<Utc>, days: i64) -> DateTime<Utc> {
  dt + Duration::days(days)
}

/// Add specified number of hours to a UTC datetime
///
/// # Parameters
///
/// - `dt`: Original UTC datetime
/// - `hours`: Number of hours to add (can be negative)
///
/// # Returns
///
/// UTC datetime after adding specified hours
///
/// # Examples
///
/// ```rust
/// let now = Utc::now();
/// let later = add_hours(now, 3);   // 3 hours later
/// let earlier = add_hours(now, -3);  // 3 hours ago
/// ```
pub fn add_hours(dt: DateTime<Utc>, hours: i64) -> DateTime<Utc> {
  dt + Duration::hours(hours)
}

/// Calculate the difference between two UTC times in seconds
///
/// # Parameters
///
/// - `dt1`: First UTC datetime
/// - `dt2`: Second UTC datetime
///
/// # Returns
///
/// Number of seconds between two times (dt2 - dt1)
///
/// # Examples
///
/// ```rust
/// let time1 = Utc::now();
/// // Some operations
/// let time2 = Utc::now();
/// let seconds_diff = diff_seconds(time1, time2);
/// ```
///
/// # Notes
///
/// - Return value can be positive or negative depending on time order
/// - Precise to the second level
pub fn diff_seconds(dt1: DateTime<Utc>, dt2: DateTime<Utc>) -> i64 {
  (dt2 - dt1).num_seconds()
}

#[cfg(test)]
mod tests {
  use crate::time::{now, timestamp, timestamp_millis, utc_now};

  #[test]
  fn test_time() {
    println!("{:?}", now());
    println!("{:?}", utc_now());
    println!("{:?}", timestamp());
    println!("{:?}", timestamp_millis());
  }
}
