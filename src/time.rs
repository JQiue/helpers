use chrono::{DateTime, Duration, Local, Utc};

/// get local datetime
pub fn now() -> DateTime<Local> {
  chrono::Local::now()
}

/// get utc datetime
pub fn utc_now() -> DateTime<Utc> {
  chrono::Utc::now()
}

/// gets the current timestamp（s）
pub fn timestamp() -> i64 {
  Utc::now().timestamp()
}

/// gets the current timestamp（ms）
pub fn timestamp_millis() -> i64 {
  Utc::now().timestamp_millis()
}

/// format the date time as a string
pub fn format(dt: DateTime<Local>, fmt: &str) -> String {
  dt.format(fmt).to_string()
}

/// converts UTC time to local time
pub fn utc_to_local(dt: DateTime<Utc>) -> DateTime<Local> {
  dt.with_timezone(&Local)
}

/// converts local time to UTC time
pub fn local_to_utc(dt: DateTime<Local>) -> DateTime<Utc> {
  dt.with_timezone(&Utc)
}

/// days sales of inventory
pub fn add_days(dt: DateTime<Utc>, days: i64) -> DateTime<Utc> {
  dt + Duration::days(days)
}

/// AddHours
pub fn add_hours(dt: DateTime<Utc>, hours: i64) -> DateTime<Utc> {
  dt + Duration::hours(hours)
}

/// calculate the difference between two times（s）
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
