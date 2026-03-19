use is_rs::time::*;

use chrono::{DateTime, Days, Local, LocalResult, NaiveDate, TimeDelta, TimeZone, Utc, Weekday};
use serial_test::serial;
use std::ffi::OsString;
use std::sync::Mutex;

static TZ_LOCK: Mutex<()> = Mutex::new(());

unsafe extern "C" {
    fn tzset();
}

struct TimeZoneGuard {
    original: Option<OsString>,
}

impl Drop for TimeZoneGuard {
    fn drop(&mut self) {
        match &self.original {
            Some(value) => unsafe {
                std::env::set_var("TZ", value);
            },
            None => unsafe {
                std::env::remove_var("TZ");
            },
        }

        unsafe {
            tzset();
        }
    }
}

fn set_timezone(tz: &str) -> TimeZoneGuard {
    let original = std::env::var_os("TZ");

    unsafe {
        std::env::set_var("TZ", tz);
        tzset();
    }

    TimeZoneGuard { original }
}

fn local_datetime(year: i32, month: u32, day: u32) -> DateTime<Local> {
    let naive = NaiveDate::from_ymd_opt(year, month, day)
        .and_then(|date| date.and_hms_opt(12, 0, 0))
        .expect("valid local date");

    match Local.from_local_datetime(&naive) {
        LocalResult::Single(dt) => dt,
        LocalResult::Ambiguous(first, _) => first,
        LocalResult::None => panic!("local datetime should exist"),
    }
}

#[test]
#[serial]
fn today_yesterday_and_tomorrow_are_relative_to_local_date() {
    let today = Local::now().date_naive();
    let yesterday = today.checked_sub_days(Days::new(1)).expect("valid date");
    let tomorrow = today.checked_add_days(Days::new(1)).expect("valid date");

    assert!(is_today(&today));
    assert!(!is_today(&yesterday));

    assert!(is_yesterday(&yesterday));
    assert!(!is_yesterday(&today));

    assert!(is_tomorrow(&tomorrow));
    assert!(!is_tomorrow(&today));
}

#[test]
#[serial]
fn past_and_future_compare_against_current_utc_time() {
    let now = Utc::now();
    let past = now - TimeDelta::seconds(5);
    let future = now + TimeDelta::seconds(5);

    assert!(is_past(&past));
    assert!(!is_past(&future));

    assert!(is_future(&future));
    assert!(!is_future(&past));
}

#[test]
#[serial]
fn day_month_year_and_leap_year_checks_match_calendar_values() {
    let monday = NaiveDate::from_ymd_opt(2024, 2, 26).expect("valid date");
    let leap_day = NaiveDate::from_ymd_opt(2024, 2, 29).expect("valid date");

    assert!(is_day(&monday, Weekday::Mon));
    assert!(!is_day(&monday, Weekday::Tue));

    assert!(is_month(&leap_day, 2));
    assert!(!is_month(&leap_day, 13));

    assert!(is_year(&leap_day, 2024));
    assert!(!is_year(&leap_day, 2023));

    assert!(is_leap_year(2024));
    assert!(!is_leap_year(2023));
}

#[test]
#[serial]
fn weekday_weekend_and_date_range_handle_boundaries() {
    let friday = NaiveDate::from_ymd_opt(2024, 3, 1).expect("valid date");
    let saturday = NaiveDate::from_ymd_opt(2024, 3, 2).expect("valid date");
    let start = NaiveDate::from_ymd_opt(2024, 3, 1).expect("valid date");
    let inside = NaiveDate::from_ymd_opt(2024, 3, 15).expect("valid date");
    let end = NaiveDate::from_ymd_opt(2024, 3, 31).expect("valid date");
    let outside = NaiveDate::from_ymd_opt(2024, 4, 1).expect("valid date");

    assert!(is_weekday(&friday));
    assert!(!is_weekday(&saturday));

    assert!(is_weekend(&saturday));
    assert!(!is_weekend(&friday));

    assert!(!in_date_range(&start, &start, &end));
    assert!(in_date_range(&inside, &start, &end));
    assert!(!in_date_range(&end, &start, &end));
    assert!(!in_date_range(&outside, &start, &end));
}

#[test]
#[serial]
fn relative_week_month_and_year_windows_are_bounded() {
    let now = Utc::now();

    let last_week_inside = now - TimeDelta::days(3);
    let last_week_outside = now - TimeDelta::days(8);
    assert!(in_last_week(&last_week_inside));
    assert!(!in_last_week(&last_week_outside));

    let last_month_inside = now - TimeDelta::days(15);
    let last_month_outside = now - TimeDelta::days(40);
    assert!(in_last_month(&last_month_inside));
    assert!(!in_last_month(&last_month_outside));

    let last_year_inside = now - TimeDelta::days(200);
    let last_year_outside = now - TimeDelta::days(400);
    assert!(in_last_year(&last_year_inside));
    assert!(!in_last_year(&last_year_outside));

    let next_week_inside = now + TimeDelta::days(3);
    let next_week_outside = now + TimeDelta::days(8);
    assert!(in_next_week(&next_week_inside));
    assert!(!in_next_week(&next_week_outside));

    let next_month_inside = now + TimeDelta::days(15);
    let next_month_outside = now + TimeDelta::days(40);
    assert!(in_next_month(&next_month_inside));
    assert!(!in_next_month(&next_month_outside));

    let next_year_inside = now + TimeDelta::days(200);
    let next_year_outside = now + TimeDelta::days(400);
    assert!(in_next_year(&next_year_inside));
    assert!(!in_next_year(&next_year_outside));
}

#[test]
#[serial]
fn quarter_of_year_accepts_only_matching_quarters() {
    let may = NaiveDate::from_ymd_opt(2024, 5, 10).expect("valid date");

    assert!(quarter_of_year(&may, 2));
    assert!(!quarter_of_year(&may, 3));
    assert!(!quarter_of_year(&may, 0));
}

#[test]
#[serial]
fn daylight_saving_time_detects_dst_offsets_when_timezone_observes_it() {
    let _lock = TZ_LOCK.lock().expect("timezone lock");
    let _guard = set_timezone("America/New_York");
    let winter = local_datetime(2026, 1, 15);
    let summer = local_datetime(2026, 7, 15);

    assert!(is_daylight_saving_time(&summer));
    assert!(!is_daylight_saving_time(&winter));
}

#[test]
#[serial]
fn daylight_saving_time_is_false_when_timezone_has_no_dst() {
    let _lock = TZ_LOCK.lock().expect("timezone lock");
    let _guard = set_timezone("Asia/Shanghai");
    let sample = local_datetime(2026, 7, 15);

    assert!(!is_daylight_saving_time(&sample));
}
