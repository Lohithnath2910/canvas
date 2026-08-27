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

pub fn day_stamp(ts_ms: i64, offset_ms: i64) -> String {
    const MONTHS: [&str; 12] = [
        "JAN", "FEB", "MAR", "APR", "MAY", "JUN", "JUL", "AUG", "SEP", "OCT", "NOV", "DEC",
    ];
    let (_, month, day) = civil_from_days((ts_ms + offset_ms).div_euclid(DAY_MS));
    format!("{day} {}", MONTHS[(month - 1) as usize])
}

pub fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

pub fn hhmm(ts_ms: i64, offset_ms: i64) -> String {
    let minutes = minute_of_day(ts_ms, offset_ms);
    format!("{:02}:{:02}", minutes / 60, minutes % 60)
}

pub const MINUTES_PER_DAY: i64 = 1440;
pub const MIN_WINDOW_MINUTES: i64 = 360;
pub const DEFAULT_WINDOW_START: i64 = 8 * 60;
pub const DEFAULT_WINDOW_END: i64 = 18 * 60;

pub fn timeline_window(spans: &[(i64, i64)], now: Option<i64>) -> (i64, i64) {
    let mut start = DEFAULT_WINDOW_START;
    let mut end = DEFAULT_WINDOW_END;

    for (from, to) in spans {
        start = start.min(*from);
        end = end.max(*to);
    }
    if let Some(now) = now {
        start = start.min(now);
        end = end.max(now);
    }

    start = (start.clamp(0, MINUTES_PER_DAY) / 60) * 60;
    end = end.clamp(0, MINUTES_PER_DAY);
    if end % 60 != 0 {
        end = (end / 60 + 1) * 60;
    }
    end = end.min(MINUTES_PER_DAY);

    if end - start < MIN_WINDOW_MINUTES {
        end = (start + MIN_WINDOW_MINUTES).min(MINUTES_PER_DAY);
        start = (end - MIN_WINDOW_MINUTES).max(0);
    }
    (start, end)
}

pub fn tick_step(window_minutes: i64, width_px: f32) -> i64 {
    const LABEL_PX: f32 = 34.0;
    let hours = (window_minutes / 60).max(1);
    let per_hour = if hours > 0 {
        width_px / hours as f32
    } else {
        width_px
    };
    for step in [1i64, 2, 3, 4, 6, 12] {
        if per_hour * step as f32 >= LABEL_PX {
            return step;
        }
    }
    12
}

pub fn span_fraction(from: i64, to: i64, window: (i64, i64)) -> (f32, f32) {
    let (ws, we) = window;
    let width = (we - ws).max(1) as f32;
    let from = from.clamp(ws, we);
    let to = to.clamp(from, we);
    let start = (from - ws) as f32 / width;
    let span = (to - from) as f32 / width;
    (start, span)
}

pub fn minute_fraction(minute: i64, window: (i64, i64)) -> f32 {
    let (ws, we) = window;
    let width = (we - ws).max(1) as f32;
    (minute - ws) as f32 / width
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

pub fn days_from_civil(year: i64, month: u32, day: u32) -> i64 {
    let y = if month <= 2 { year - 1 } else { year };
    let era = y.div_euclid(400);
    let yoe = y - era * 400;
    let mp = (month as i64 + 9).rem_euclid(12);
    let doy = (153 * mp + 2) / 5 + day as i64 - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    era * 146_097 + doe - 719_468
}

pub fn weekday(days_since_epoch: i64) -> u32 {
    (days_since_epoch + 4).rem_euclid(DAYS_PER_WEEK) as u32
}

pub fn local_civil(ts_ms: i64, offset_ms: i64) -> (i64, u32, u32) {
    civil_from_days((ts_ms + offset_ms).div_euclid(DAY_MS))
}

pub fn compose_local_ms(
    year: i64,
    month: u32,
    day: u32,
    minute_of_day: i64,
    offset_ms: i64,
) -> i64 {
    days_from_civil(year, month, day) * DAY_MS + minute_of_day * MINUTE_MS - offset_ms
}

pub fn month_step(year: i64, month: u32, delta: i32) -> (i64, u32) {
    let zero = year * 12 + month as i64 - 1 + delta as i64;
    (zero.div_euclid(12), zero.rem_euclid(12) as u32 + 1)
}

pub fn month_grid(year: i64, month: u32, first_weekday: u32) -> Vec<i64> {
    let first = days_from_civil(year, month, 1);
    let lead = (weekday(first) + DAYS_PER_WEEK as u32 - first_weekday) % DAYS_PER_WEEK as u32;
    let start = first - lead as i64;
    (0..42).map(|i| start + i).collect()
}

pub fn month_name(month: u32) -> &'static str {
    const NAMES: [&str; 12] = [
        "JANUARY",
        "FEBRUARY",
        "MARCH",
        "APRIL",
        "MAY",
        "JUNE",
        "JULY",
        "AUGUST",
        "SEPTEMBER",
        "OCTOBER",
        "NOVEMBER",
        "DECEMBER",
    ];
    NAMES
        .get(month.wrapping_sub(1) as usize)
        .copied()
        .unwrap_or("")
}

pub fn weekday_initial(weekday: u32) -> &'static str {
    const HEADS: [&str; 7] = ["S", "M", "T", "W", "T", "F", "S"];
    HEADS.get(weekday as usize).copied().unwrap_or("")
}

pub fn date_stamp(ts_ms: i64, offset_ms: i64) -> String {
    const DAYS: [&str; 7] = ["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    let days = (ts_ms + offset_ms).div_euclid(DAY_MS);
    let (_, month, day) = civil_from_days(days);
    format!(
        "{}, {day} {}",
        DAYS[weekday(days) as usize],
        &month_name(month)[..3]
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const IST: i64 = 5 * 3_600_000 + 30 * MINUTE_MS;

    fn is_leap(year: i64) -> bool {
        year.rem_euclid(4) == 0 && (year.rem_euclid(100) != 0 || year.rem_euclid(400) == 0)
    }

    fn days_in_month(year: i64, month: u32) -> u32 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 if is_leap(year) => 29,
            2 => 28,
            _ => 0,
        }
    }

    #[test]
    fn the_test_month_lengths_add_up_to_a_year() {
        for year in 1970..2100 {
            let total: u32 = (1..=12).map(|m| days_in_month(year, m)).sum();
            assert_eq!(total, if is_leap(year) { 366 } else { 365 });
        }
    }

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
    fn civil_from_days_agrees_with_known_dates() {
        assert_eq!(civil_from_days(0), (1970, 1, 1));
        assert_eq!(civil_from_days(-1), (1969, 12, 31));
        assert_eq!(civil_from_days(59), (1970, 3, 1));
        assert_eq!(civil_from_days(20_691), (2026, 8, 26));
    }

    #[test]
    fn civil_from_days_handles_leap_days() {
        let leap = 11_016;
        assert_eq!(civil_from_days(leap), (2000, 2, 29));
        assert_eq!(civil_from_days(leap + 1), (2000, 3, 1));
    }

    #[test]
    fn day_stamp_reads_as_a_card_meta_date() {
        assert_eq!(day_stamp(20_691 * DAY_MS, 0), "26 AUG");
        assert_eq!(day_stamp(0, 0), "1 JAN");
    }

    #[test]
    fn day_stamp_follows_the_local_day() {
        let late = 20_691 * DAY_MS + 23 * 3_600_000;
        assert_eq!(day_stamp(late, 0), "26 AUG");
        assert_eq!(day_stamp(late, 2 * 3_600_000), "27 AUG");
    }

    #[test]
    fn days_from_civil_inverts_civil_from_days() {
        for days in -40_000..40_000 {
            let (y, m, d) = civil_from_days(days);
            assert_eq!(days_from_civil(y, m, d), days, "{y}-{m}-{d}");
        }
    }

    #[test]
    fn days_from_civil_agrees_with_known_dates() {
        assert_eq!(days_from_civil(1970, 1, 1), 0);
        assert_eq!(days_from_civil(1969, 12, 31), -1);
        assert_eq!(days_from_civil(2026, 8, 26), 20_691);
        assert_eq!(days_from_civil(2000, 2, 29), 11_016);
    }

    #[test]
    fn weekday_reads_zero_as_sunday() {
        assert_eq!(weekday(0), 4);
        assert_eq!(weekday(days_from_civil(2026, 8, 26)), 3);
        assert_eq!(weekday(days_from_civil(2026, 8, 30)), 0);
        for days in -1000..1000 {
            assert_eq!(weekday(days + 7), weekday(days));
        }
    }

    #[test]
    fn month_step_wraps_both_year_boundaries() {
        assert_eq!(month_step(2026, 12, 1), (2027, 1));
        assert_eq!(month_step(2026, 1, -1), (2025, 12));
        assert_eq!(month_step(2026, 8, 0), (2026, 8));
        assert_eq!(month_step(2026, 8, 12), (2027, 8));
        assert_eq!(month_step(2026, 8, -12), (2025, 8));
        let (mut y, mut m) = (2026, 1);
        for _ in 0..24 {
            let stepped = month_step(y, m, 1);
            y = stepped.0;
            m = stepped.1;
        }
        assert_eq!((y, m), (2028, 1));
    }

    #[test]
    fn a_month_grid_is_six_weeks_starting_on_the_asked_for_weekday() {
        for month in 1..=12 {
            let grid = month_grid(2026, month, 1);
            assert_eq!(grid.len(), 42);
            assert_eq!(weekday(grid[0]), 1);
            for (i, day) in grid.iter().enumerate() {
                assert_eq!(*day, grid[0] + i as i64);
            }
        }
        assert_eq!(weekday(month_grid(2026, 8, 0)[0]), 0);
    }

    #[test]
    fn a_month_grid_contains_every_day_of_its_month() {
        for year in [1999, 2024, 2026] {
            for month in 1..=12 {
                let grid = month_grid(year, month, 1);
                for day in 1..=days_in_month(year, month) {
                    assert!(
                        grid.contains(&days_from_civil(year, month, day)),
                        "{year}-{month}-{day} missing"
                    );
                }
            }
        }
    }

    #[test]
    fn a_month_grid_never_starts_inside_its_own_month() {
        for year in [1970, 2026, 2100] {
            for month in 1..=12 {
                let grid = month_grid(year, month, 1);
                let first = days_from_civil(year, month, 1);
                assert!(grid[0] <= first);
                assert!(first - grid[0] < 7);
            }
        }
    }

    #[test]
    fn compose_local_ms_round_trips_a_picked_date_and_time() {
        for offset in [0, IST, -8 * 3_600_000, 45 * MINUTE_MS] {
            for (y, m, d, minute) in [
                (2026, 8, 26, 0),
                (2026, 8, 26, 9 * 60 + 30),
                (2026, 1, 1, 23 * 60 + 59),
                (2024, 2, 29, 12 * 60),
                (1999, 12, 31, 18 * 60 + 5),
            ] {
                let ts = compose_local_ms(y, m, d, minute, offset);
                assert_eq!(local_civil(ts, offset), (y, m, d), "offset {offset}");
                assert_eq!(minute_of_day(ts, offset), minute, "offset {offset}");
            }
        }
    }

    #[test]
    fn compose_local_ms_disagrees_across_timezones_by_the_offset() {
        let utc = compose_local_ms(2026, 8, 26, 9 * 60, 0);
        let ist = compose_local_ms(2026, 8, 26, 9 * 60, IST);
        assert_eq!(utc - ist, IST);
    }

    #[test]
    fn compose_local_ms_and_hhmm_agree_on_the_clock() {
        for minute in 0..1440 {
            let ts = compose_local_ms(2026, 8, 26, minute, IST);
            assert_eq!(
                hhmm(ts, IST),
                format!("{:02}:{:02}", minute / 60, minute % 60)
            );
        }
    }

    #[test]
    fn date_stamp_reads_as_a_picker_header() {
        assert_eq!(date_stamp(20_691 * DAY_MS, 0), "WED, 26 AUG");
        assert_eq!(date_stamp(0, 0), "THU, 1 JAN");
    }

    #[test]
    fn month_and_weekday_names_never_panic_on_a_bad_index() {
        assert_eq!(month_name(1), "JANUARY");
        assert_eq!(month_name(12), "DECEMBER");
        assert_eq!(month_name(0), "");
        assert_eq!(month_name(13), "");
        assert_eq!(weekday_initial(0), "S");
        assert_eq!(weekday_initial(6), "S");
        assert_eq!(weekday_initial(7), "");
    }

    #[test]
    fn an_empty_week_shows_the_working_day() {
        assert_eq!(timeline_window(&[], None), (8 * 60, 18 * 60));
    }

    #[test]
    fn the_window_grows_to_hold_every_event() {
        let spans = [(9 * 60, 10 * 60), (19 * 60 + 15, 20 * 60 + 45)];
        assert_eq!(timeline_window(&spans, None), (8 * 60, 21 * 60));
    }

    #[test]
    fn an_early_event_pulls_the_window_back_to_a_whole_hour() {
        let spans = [(6 * 60 + 25, 7 * 60)];
        assert_eq!(timeline_window(&spans, None), (6 * 60, 18 * 60));
    }

    #[test]
    fn now_is_always_inside_the_window() {
        let spans = [(9 * 60, 10 * 60)];
        let (start, end) = timeline_window(&spans, Some(23 * 60 + 30));
        assert!(start <= 23 * 60 + 30 && 23 * 60 + 30 <= end);
        assert_eq!(end, MINUTES_PER_DAY);
    }

    #[test]
    fn a_tiny_window_is_widened_to_the_minimum() {
        let (start, end) = timeline_window(&[(10 * 60, 10 * 60 + 30)], Some(10 * 60));
        assert!(end - start >= MIN_WINDOW_MINUTES);
    }

    #[test]
    fn a_late_night_event_does_not_push_the_window_past_midnight() {
        let (start, end) = timeline_window(&[(23 * 60, MINUTES_PER_DAY)], None);
        assert_eq!(end, MINUTES_PER_DAY);
        assert!(start >= 0);
        assert!(end - start >= MIN_WINDOW_MINUTES);
    }

    #[test]
    fn a_span_maps_onto_the_window_as_fractions() {
        let window = (8 * 60, 18 * 60);
        let (start, span) = span_fraction(9 * 60, 10 * 60, window);
        assert!((start - 0.1).abs() < 0.001, "start was {start}");
        assert!((span - 0.1).abs() < 0.001, "span was {span}");
    }

    #[test]
    fn a_span_outside_the_window_is_clamped_rather_than_drawn_off_the_end() {
        let window = (8 * 60, 18 * 60);
        let (start, span) = span_fraction(2 * 60, 23 * 60, window);
        assert_eq!(start, 0.0);
        assert!((span - 1.0).abs() < 0.001);

        let (late, tail) = span_fraction(20 * 60, 21 * 60, window);
        assert!((late - 1.0).abs() < 0.001);
        assert_eq!(tail, 0.0);
    }

    #[test]
    fn the_tick_step_opens_up_when_hours_are_cramped() {
        assert_eq!(tick_step(10 * 60, 600.0), 1);
        assert!(tick_step(24 * 60, 300.0) > 1);
        assert!(tick_step(24 * 60, 120.0) >= 4);
    }

    #[test]
    fn a_minute_maps_to_its_place_in_the_window() {
        let window = (8 * 60, 20 * 60);
        assert_eq!(minute_fraction(8 * 60, window), 0.0);
        assert_eq!(minute_fraction(20 * 60, window), 1.0);
        assert!((minute_fraction(14 * 60, window) - 0.5).abs() < 0.001);
    }
}
