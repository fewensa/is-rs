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

pub fn is_today(dt: &NaiveDate) -> bool {
    *dt == current_local_date()
}

pub fn is_yesterday(dt: &NaiveDate) -> bool {
    current_local_date()
        .checked_sub_days(Days::new(1))
        .is_some_and(|yesterday| *dt == yesterday)
}

pub fn is_tomorrow(dt: &NaiveDate) -> bool {
    current_local_date()
        .checked_add_days(Days::new(1))
        .is_some_and(|tomorrow| *dt == tomorrow)
}

pub fn is_past(dt: &DateTime<Utc>) -> bool {
    *dt < current_utc_time()
}

pub fn is_future(dt: &DateTime<Utc>) -> bool {
    *dt > current_utc_time()
}

pub fn is_day(dt: &NaiveDate, day: Weekday) -> bool {
    dt.weekday() == day
}

pub fn is_month(dt: &NaiveDate, month: u32) -> bool {
    (1..=12).contains(&month) && dt.month() == month
}

pub fn is_year(dt: &NaiveDate, year: i32) -> bool {
    dt.year() == year
}

pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || year % 400 == 0
}

pub fn is_weekday(dt: &NaiveDate) -> bool {
    matches!(
        dt.weekday(),
        Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri
    )
}

pub fn is_weekend(dt: &NaiveDate) -> bool {
    matches!(dt.weekday(), Weekday::Sat | Weekday::Sun)
}

pub fn in_date_range(dt: &NaiveDate, start: &NaiveDate, end: &NaiveDate) -> bool {
    start <= dt && dt <= end
}

pub fn in_last_week(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let start = now - TimeDelta::days(7);

    *dt <= now && *dt >= start
}

pub fn in_last_month(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let start = now
        .checked_sub_months(Months::new(1))
        .unwrap_or(now - TimeDelta::days(31));

    *dt <= now && *dt >= start
}

pub fn in_last_year(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let start = now
        .checked_sub_months(Months::new(12))
        .unwrap_or(now - TimeDelta::days(366));

    *dt <= now && *dt >= start
}

pub fn in_next_week(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let end = now + TimeDelta::days(7);

    *dt >= now && *dt <= end
}

pub fn in_next_month(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let end = now
        .checked_add_months(Months::new(1))
        .unwrap_or(now + TimeDelta::days(31));

    *dt >= now && *dt <= end
}

pub fn in_next_year(dt: &DateTime<Utc>) -> bool {
    let now = current_utc_time();
    let end = now
        .checked_add_months(Months::new(12))
        .unwrap_or(now + TimeDelta::days(366));

    *dt >= now && *dt <= end
}

pub fn quarter_of_year(dt: &NaiveDate, quarter: u8) -> bool {
    matches!(quarter, 1..=4) && ((dt.month0() / 3) + 1) == u32::from(quarter)
}

pub fn is_daylight_saving_time(dt: &DateTime<Local>) -> bool {
    daylight_saving_offset(dt.year())
        .is_some_and(|dst_offset| dt.offset().local_minus_utc() == dst_offset)
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let end = NaiveDate::from_ymd_opt(2024, 3, 31).expect("valid date");
        let outside = NaiveDate::from_ymd_opt(2024, 4, 1).expect("valid date");

        assert!(is_weekday(&friday));
        assert!(!is_weekday(&saturday));

        assert!(is_weekend(&saturday));
        assert!(!is_weekend(&friday));

        assert!(in_date_range(&start, &start, &end));
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
}
