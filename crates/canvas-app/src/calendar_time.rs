pub const DAY_MS: i64 = 86_400_000;
pub const MINUTE_MS: i64 = 60_000;
pub const DAYS_PER_WEEK: i64 = 7;

pub fn week_start_utc(now_ms: i64, offset_ms: i64) -> i64 {
    let local = now_ms + offset_ms;
    let days = local.div_euclid(DAY_MS);
    let weekday = (days + 3).rem_euclid(DAYS_PER_WEEK);
    (days - weekday) * DAY_MS - offset_ms
}

pub fn day_index(ts_ms: i64, week_start_utc: i64, offset_ms: i64) -> i32 {
    let local = ts_ms + offset_ms;
    let start_local = week_start_utc + offset_ms;
    (local.div_euclid(DAY_MS) - start_local.div_euclid(DAY_MS)).clamp(0, 6) as i32
}

pub fn minute_of_day(ts_ms: i64, offset_ms: i64) -> i64 {
    (ts_ms + offset_ms).rem_euclid(DAY_MS) / MINUTE_MS
}

pub fn day_fraction(ts_ms: i64, offset_ms: i64) -> f32 {
    minute_of_day(ts_ms, offset_ms) as f32 / 1440.0
}

pub fn hhmm(ts_ms: i64, offset_ms: i64) -> String {
    let minutes = minute_of_day(ts_ms, offset_ms);
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

pub fn chip_bucket(duration_ms: i64) -> (f32, i32) {
    let minutes = duration_ms.max(0) / MINUTE_MS;
    match minutes {
        ..=30 => (24.0, 1),
        31..=60 => (32.0, 2),
        61..=120 => (40.0, 3),
        _ => (56.0, 4),
    }
}

#[cfg(windows)]
pub fn local_offset_ms() -> i64 {
    use windows_sys::Win32::System::Time::{GetTimeZoneInformation, TIME_ZONE_INFORMATION};

    let mut info: TIME_ZONE_INFORMATION = unsafe { std::mem::zeroed() };
    let extra = match unsafe { GetTimeZoneInformation(&mut info) } {
        0 | 1 => info.StandardBias,
        2 => info.DaylightBias,
        _ => return 0,
    };
    -(info.Bias as i64 + extra as i64) * MINUTE_MS
}

#[cfg(not(windows))]
pub fn local_offset_ms() -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const IST: i64 = 5 * 3_600_000 + 30 * MINUTE_MS;

    fn utc(days_since_epoch: i64, hour: i64, minute: i64) -> i64 {
        days_since_epoch * DAY_MS + hour * 3_600_000 + minute * MINUTE_MS
    }

    #[test]
    fn the_epoch_was_a_thursday() {
        let start = week_start_utc(0, 0);
        assert_eq!(start, -3 * DAY_MS);
    }

    #[test]
    fn a_week_start_is_midnight_on_a_monday() {
        for day in 0..400 {
            let start = week_start_utc(utc(day, 13, 47), 0);
            assert_eq!(start.rem_euclid(DAY_MS), 0);
            assert_eq!((start.div_euclid(DAY_MS) + 3).rem_euclid(7), 0);
        }
    }

    #[test]
    fn every_moment_in_a_week_shares_one_week_start() {
        let anchor = week_start_utc(utc(20_000, 9, 0), 0);
        for hour in 0..(24 * 7) {
            assert_eq!(week_start_utc(anchor + hour * 3_600_000, 0), anchor);
        }
        assert_ne!(week_start_utc(anchor + 7 * DAY_MS, 0), anchor);
    }

    #[test]
    fn a_week_start_follows_the_local_day_not_the_utc_one() {
        let sunday_night = utc(20_002, 18, 45);
        assert_eq!(week_start_utc(sunday_night, 0), 19_996 * DAY_MS);
        let ist_start = week_start_utc(sunday_night, IST);
        assert_eq!(ist_start, 20_003 * DAY_MS - IST);
        assert_eq!((ist_start + IST).rem_euclid(DAY_MS), 0);
    }

    #[test]
    fn day_index_spreads_a_week_across_seven_columns() {
        let start = week_start_utc(utc(20_000, 12, 0), 0);
        for day in 0..7 {
            assert_eq!(
                day_index(start + day * DAY_MS + 3_600_000, start, 0),
                day as i32
            );
        }
    }

    #[test]
    fn day_index_clamps_events_that_straddle_the_week() {
        let start = week_start_utc(utc(20_000, 12, 0), 0);
        assert_eq!(day_index(start - 2 * 3_600_000, start, 0), 0);
        assert_eq!(day_index(start + 9 * DAY_MS, start, 0), 6);
    }

    #[test]
    fn minute_of_day_reads_local_clock_time() {
        let ts = utc(20_000, 4, 0);
        assert_eq!(minute_of_day(ts, 0), 240);
        assert_eq!(minute_of_day(ts, IST), 9 * 60 + 30);
    }

    #[test]
    fn minute_of_day_wraps_backwards_over_midnight() {
        let ts = utc(20_000, 1, 0);
        assert_eq!(minute_of_day(ts, -2 * 3_600_000), 23 * 60);
    }

    #[test]
    fn hhmm_is_a_zero_padded_24_hour_clock() {
        assert_eq!(hhmm(utc(20_000, 9, 30), 0), "09:30");
        assert_eq!(hhmm(utc(20_000, 0, 0), 0), "00:00");
        assert_eq!(hhmm(utc(20_000, 18, 5), 0), "18:05");
        assert_eq!(hhmm(utc(20_000, 23, 59), 0), "23:59");
    }

    #[test]
    fn day_fraction_runs_from_zero_to_just_under_one() {
        assert_eq!(day_fraction(utc(20_000, 0, 0), 0), 0.0);
        assert!((day_fraction(utc(20_000, 12, 0), 0) - 0.5).abs() < f32::EPSILON);
        assert!(day_fraction(utc(20_000, 23, 59), 0) < 1.0);
    }

    #[test]
    fn chip_buckets_step_through_the_four_heights() {
        assert_eq!(chip_bucket(0), (24.0, 1));
        assert_eq!(chip_bucket(30 * MINUTE_MS), (24.0, 1));
        assert_eq!(chip_bucket(45 * MINUTE_MS), (32.0, 2));
        assert_eq!(chip_bucket(60 * MINUTE_MS), (32.0, 2));
        assert_eq!(chip_bucket(90 * MINUTE_MS), (40.0, 3));
        assert_eq!(chip_bucket(120 * MINUTE_MS), (40.0, 3));
        assert_eq!(chip_bucket(8 * 3_600_000), (56.0, 4));
    }

    #[test]
    fn a_longer_event_never_gets_a_shorter_chip() {
        let mut previous = (0.0, 0);
        for minutes in 0..600 {
            let current = chip_bucket(minutes * MINUTE_MS);
            assert!(current.0 >= previous.0);
            assert!(current.1 >= previous.1);
            previous = current;
        }
    }
}
