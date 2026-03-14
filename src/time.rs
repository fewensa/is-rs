use chrono::{
    DateTime, Datelike, Days, Local, LocalResult, Months, NaiveDate, TimeDelta, TimeZone, Utc,
    Weekday,
};

fn current_local_date() -> NaiveDate {
    Local::now().date_naive()
}

fn current_utc_time() -> DateTime<Utc> {
    Utc::now()
}

fn offset_for_local_date(date: NaiveDate) -> Option<i32> {
    let naive = date.and_hms_opt(12, 0, 0)?;

    match Local.from_local_datetime(&naive) {
        LocalResult::Single(dt) => Some(dt.offset().local_minus_utc()),
        LocalResult::Ambiguous(first, _) => Some(first.offset().local_minus_utc()),
        LocalResult::None => None,
    }
}

fn daylight_saving_offset(year: i32) -> Option<i32> {
    let january_offset = offset_for_local_date(NaiveDate::from_ymd_opt(year, 1, 15)?)?;
    let july_offset = offset_for_local_date(NaiveDate::from_ymd_opt(year, 7, 15)?)?;

    if january_offset == july_offset {
        None
    } else {
        Some(january_offset.max(july_offset))
    }
}

/// Returns `true` if the given date is today in local time.
///
/// # Examples
///
/// ```
/// use chrono::Local;
/// use is_rs::time::is_today;
///
/// let today = Local::now().date_naive();
/// assert!(is_today(&today));
/// ```
pub fn is_today(dt: &NaiveDate) -> bool {
    *dt == current_local_date()
}

/// Returns `true` if the given date is yesterday in local time.
///
/// # Examples
///
/// ```
/// use chrono::{Local, Days};
/// use is_rs::time::is_yesterday;
///
/// let yesterday = Local::now().date_naive().checked_sub_days(Days::new(1)).unwrap();
/// assert!(is_yesterday(&yesterday));
/// ```
pub fn is_yesterday(dt: &NaiveDate) -> bool {
    current_local_date()
        .checked_sub_days(Days::new(1))
        .is_some_and(|yesterday| *dt == yesterday)
}

/// Returns `true` if the given date is tomorrow in local time.
///
/// # Examples
///
/// ```
/// use chrono::{Local, Days};
/// use is_rs::time::is_tomorrow;
///
/// let tomorrow = Local::now().date_naive().checked_add_days(Days::new(1)).unwrap();
/// assert!(is_tomorrow(&tomorrow));
/// ```
pub fn is_tomorrow(dt: &NaiveDate) -> bool {
    current_local_date()
        .checked_add_days(Days::new(1))
        .is_some_and(|tomorrow| *dt == tomorrow)
}

/// Returns `true` if the given UTC datetime is in the past.
///
/// # Examples
///
/// ```
/// use chrono::{Utc, TimeDelta};
/// use is_rs::time::is_past;
///
/// let past = Utc::now() - TimeDelta::seconds(10);
/// assert!(is_past(&past));
/// ```
pub fn is_past(dt: &DateTime<Utc>) -> bool {
    *dt < current_utc_time()
}

/// Returns `true` if the given UTC datetime is in the future.
///
/// # Examples
///
/// ```
/// use chrono::{Utc, TimeDelta};
/// use is_rs::time::is_future;
///
/// let future = Utc::now() + TimeDelta::seconds(10);
/// assert!(is_future(&future));
/// ```
pub fn is_future(dt: &DateTime<Utc>) -> bool {
    *dt > current_utc_time()
}

/// Returns `true` if the given date falls on the specified weekday.
///
/// # Examples
///
/// ```
/// use chrono::{NaiveDate, Weekday};
/// use is_rs::time::is_day;
///
/// let monday = NaiveDate::from_ymd_opt(2024, 2, 26).unwrap();
/// assert!(is_day(&monday, Weekday::Mon));
/// assert!(!is_day(&monday, Weekday::Tue));
/// ```
pub fn is_day(dt: &NaiveDate, day: Weekday) -> bool {
    dt.weekday() == day
}

/// Returns `true` if the given date falls in the specified month (1–12).
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use is_rs::time::is_month;
///
/// let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
/// assert!(is_month(&date, 6));
/// assert!(!is_month(&date, 7));
/// ```
pub fn is_month(dt: &NaiveDate, month: u32) -> bool {
    (1..=12).contains(&month) && dt.month() == month
}

/// Returns `true` if the given date falls in the specified year.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use is_rs::time::is_year;
///
/// let date = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
/// assert!(is_year(&date, 2024));
/// assert!(!is_year(&date, 2023));
/// ```
pub fn is_year(dt: &NaiveDate, year: i32) -> bool {
    dt.year() == year
}

/// Returns `true` if the given year is a leap year.
///
/// A year is a leap year if it is divisible by 4, except for century years,
/// which must be divisible by 400.
///
/// # Examples
///
/// ```
/// use is_rs::time::is_leap_year;
///
/// assert!(is_leap_year(2024));
/// assert!(!is_leap_year(2023));
/// assert!(!is_leap_year(1900));
/// assert!(is_leap_year(2000));
/// ```
pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

/// Returns `true` if the given date falls on a weekday (Monday–Friday).
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use is_rs::time::is_weekday;
///
/// let friday = NaiveDate::from_ymd_opt(2024, 3, 1).unwrap();
/// assert!(is_weekday(&friday));
///
/// let saturday = NaiveDate::from_ymd_opt(2024, 3, 2).unwrap();
/// assert!(!is_weekday(&saturday));
/// ```
pub fn is_weekday(dt: &NaiveDate) -> bool {
    matches!(
        dt.weekday(),
        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri
    )
}

/// Returns `true` if the given date falls on a weekend (Saturday or Sunday).
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use is_rs::time::is_weekend;
///
/// let saturday = NaiveDate::from_ymd_opt(2024, 3, 2).unwrap();
/// assert!(is_weekend(&saturday));
///
/// let monday = NaiveDate::from_ymd_opt(2024, 3, 4).unwrap();
/// assert!(!is_weekend(&monday));
/// ```
pub fn is_weekend(dt: &NaiveDate) -> bool {
    matches!(dt.weekday(), Weekday::Sat | Weekday::Sun)
}

/// Returns `true` if `dt` is within the inclusive date range `[start, end]`.
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use is_rs::time::in_date_range;
///
/// let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
/// let end   = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();
/// let mid   = NaiveDate::from_ymd_opt(2024, 6, 15).unwrap();
///
/// assert!(in_date_range(&mid, &start, &end));
/// assert!(!in_date_range(&NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(), &start, &end));
/// ```
pub fn in_date_range(dt: &NaiveDate, start: &NaiveDate, end: &NaiveDate) -> bool {
    start <= dt && dt <= end
}

/// Returns `true` if `dt` is within the last 7 days (inclusive of now).
///
/// # Examples
///
/// ```
/// use chrono::{Utc, TimeDelta};
/// use is_rs::time::in_last_week;
///
/// let three_days_ago = Utc::now() - TimeDelta::days(3);
/// assert!(in_last_week(&three_days_ago));
///
/// let eight_days_ago = Utc::now() - TimeDelta::days(8);
/// assert!(!in_last_week(&eight_days_ago));
/// ```
pub fn in_last_week(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let start = now - TimeDelta::days(7);

    *dt <= now && *dt >= start
}

/// Returns `true` if `dt` is within the last calendar month (inclusive of now).
///
/// # Examples
///
/// ```
/// use chrono::{Utc, TimeDelta};
/// use is_rs::time::in_last_month;
///
/// let two_weeks_ago = Utc::now() - TimeDelta::days(14);
/// assert!(in_last_month(&two_weeks_ago));
///
/// let two_months_ago = Utc::now() - TimeDelta::days(65);
/// assert!(!in_last_month(&two_months_ago));
/// ```
pub fn in_last_month(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let start = now
        .checked_sub_months(Months::new(1))
        .unwrap_or(now - TimeDelta::days(31));

    *dt <= now && *dt >= start
}

/// Returns `true` if `dt` is within the last 12 calendar months (inclusive of now).
///
/// # Examples
///
/// ```
/// use chrono::{Utc, TimeDelta};
/// use is_rs::time::in_last_year;
///
/// let six_months_ago = Utc::now() - TimeDelta::days(180);
/// assert!(in_last_year(&six_months_ago));
///
/// let two_years_ago = Utc::now() - TimeDelta::days(730);
/// assert!(!in_last_year(&two_years_ago));
/// ```
pub fn in_last_year(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let start = now
        .checked_sub_months(Months::new(12))
        .unwrap_or(now - TimeDelta::days(366));

    *dt <= now && *dt >= start
}

/// Returns `true` if `dt` is within the next 7 days from now (inclusive).
///
/// # Examples
///
/// ```
/// use chrono::{Utc, TimeDelta};
/// use is_rs::time::in_next_week;
///
/// let three_days_ahead = Utc::now() + TimeDelta::days(3);
/// assert!(in_next_week(&three_days_ahead));
///
/// let eight_days_ahead = Utc::now() + TimeDelta::days(8);
/// assert!(!in_next_week(&eight_days_ahead));
/// ```
pub fn in_next_week(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let end = now + TimeDelta::days(7);

    *dt >= now && *dt <= end
}

/// Returns `true` if `dt` is within the next calendar month from now (inclusive).
///
/// # Examples
///
/// ```
/// use chrono::{Utc, TimeDelta};
/// use is_rs::time::in_next_month;
///
/// let two_weeks_ahead = Utc::now() + TimeDelta::days(14);
/// assert!(in_next_month(&two_weeks_ahead));
///
/// let two_months_ahead = Utc::now() + TimeDelta::days(65);
/// assert!(!in_next_month(&two_months_ahead));
/// ```
pub fn in_next_month(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let end = now
        .checked_add_months(Months::new(1))
        .unwrap_or(now + TimeDelta::days(31));

    *dt >= now && *dt <= end
}

/// Returns `true` if `dt` is within the next 12 calendar months from now (inclusive).
///
/// # Examples
///
/// ```
/// use chrono::{Utc, TimeDelta};
/// use is_rs::time::in_next_year;
///
/// let six_months_ahead = Utc::now() + TimeDelta::days(180);
/// assert!(in_next_year(&six_months_ahead));
///
/// let two_years_ahead = Utc::now() + TimeDelta::days(730);
/// assert!(!in_next_year(&two_years_ahead));
/// ```
pub fn in_next_year(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let end = now
        .checked_add_months(Months::new(12))
        .unwrap_or(now + TimeDelta::days(366));

    *dt >= now && *dt <= end
}

/// Returns `true` if the given date falls in the specified quarter (1–4).
///
/// - Q1: January–March
/// - Q2: April–June
/// - Q3: July–September
/// - Q4: October–December
///
/// # Examples
///
/// ```
/// use chrono::NaiveDate;
/// use is_rs::time::quarter_of_year;
///
/// let may = NaiveDate::from_ymd_opt(2024, 5, 10).unwrap();
/// assert!(quarter_of_year(&may, 2));
/// assert!(!quarter_of_year(&may, 1));
/// ```
pub fn quarter_of_year(dt: &NaiveDate, quarter: u8) -> bool {
    matches!(quarter, 1..=4) && ((dt.month0() / 3) + 1) == u32::from(quarter)
}

/// Returns `true` if the given local datetime is currently observing daylight saving time.
///
/// Returns `false` for timezones that do not observe DST (e.g., UTC, Asia/Shanghai).
///
/// # Examples
///
/// ```no_run
/// use chrono::Local;
/// use is_rs::time::is_daylight_saving_time;
///
/// let now = Local::now();
/// // Result depends on the local timezone and current date.
/// let _ = is_daylight_saving_time(&now);
/// ```
pub fn is_daylight_saving_time(dt: &DateTime<Local>) -> bool {
    daylight_saving_offset(dt.year())
        .is_some_and(|dst_offset| dt.offset().local_minus_utc() == dst_offset)
}
