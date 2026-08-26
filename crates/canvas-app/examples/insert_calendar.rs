const DAY_MS: i64 = 86_400_000;
const MINUTE_MS: i64 = 60_000;

fn week_start_utc(now_ms: i64, offset_ms: i64) -> i64 {
    let local = now_ms + offset_ms;
    let days = local.div_euclid(DAY_MS);
    let weekday = (days + 3).rem_euclid(7);
    (days - weekday) * DAY_MS - offset_ms
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let dir = canvas_storage::Store::default_path();
    let store = canvas_storage::Store::open(&dir.join("canvas_local.db"))?;

    if std::env::args().any(|a| a == "--remove") {
        let mut removed = 0;
        for row in store.list_widgets()? {
            if row.widget_type == canvas_storage::KIND_CALENDAR {
                store.delete_widget(&row.id)?;
                removed += 1;
            }
        }
        println!("removed {removed} calendar rows");
        return Ok(());
    }

    let offset = std::env::args()
        .skip_while(|a| a != "--offset-minutes")
        .nth(1)
        .and_then(|v| v.parse::<i64>().ok())
        .map(|m| m * MINUTE_MS)
        .unwrap_or_else(local_offset_ms);

    let now = canvas_storage::now_millis();
    let monday = week_start_utc(now, offset);

    let (row, _) = store.create_widget(canvas_storage::KIND_CALENDAR)?;
    store.set_title(&row.id, "THIS WEEK")?;

    let doc = canvas_doc::CalendarDoc::new();
    let plan = [
        (0, 9 * 60 + 30, 30, "stand-up"),
        (1, 11 * 60, 60, "strip spec review"),
        (2, 9 * 60 + 30, 30, "stand-up"),
        (2, 14 * 60, 180, "widget build block"),
        (3, 9 * 60 + 30, 30, "stand-up"),
        (3, 16 * 60, 90, "verification walk"),
        (4, 13 * 60, 45, "font binaries"),
        (6, 18 * 60, 120, "demo room"),
    ];

    for (day, minute, minutes, title) in plan {
        let start = monday + day * DAY_MS + minute * MINUTE_MS;
        doc.add_event(title, start, start + minutes * MINUTE_MS);
    }

    store.save_document(&row.id, &doc.snapshot(), &doc.state_vector())?;

    println!(
        "inserted {} type={} with {} events, week of {}",
        row.id,
        row.widget_type,
        doc.events().len(),
        monday
    );
    Ok(())
}

#[cfg(windows)]
fn local_offset_ms() -> i64 {
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
fn local_offset_ms() -> i64 {
    0
}
