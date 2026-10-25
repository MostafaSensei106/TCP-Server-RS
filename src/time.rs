use std::time::{SystemTime, UNIX_EPOCH};

pub fn current_timestamp() -> String {
    let unix_seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = unix_seconds / 86_400;
    let seconds_in_day = unix_seconds % 86_400;
    let (year, month, day) = civil_from_days(days as i64);
    let hour_24 = seconds_in_day / 3_600;
    let hour_12 = match hour_24 % 12 {
        0 => 12,
        hour => hour,
    };
    let minute = (seconds_in_day % 3_600) / 60;
    let second = seconds_in_day % 60;
    let period = if hour_24 < 12 { "AM" } else { "PM" };
    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02} {}",
        year, month, day, hour_12, minute, second, period
    )
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += if month <= 2 { 1 } else { 0 };
    (year, month, day)
}
