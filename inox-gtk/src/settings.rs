use gtk::prelude::GtkWindowExt;
use gio::{Settings, traits::SettingsExt};

use chrono::prelude::*;
use chrono::Duration;


pub fn get_refresh_interval(settings: &Settings) -> Duration {
    let time = i64::from(settings.int("refresh-interval-time"));
    let period = settings.string("refresh-interval-period");

    time_period_to_duration(time, period.as_str())
}

pub fn get_cleanup_date(settings: &Settings) -> DateTime<Utc> {
    let time = i64::from(settings.int("cleanup-age-time"));
    let period = settings.string("cleanup-age-period");
    let duration = time_period_to_duration(time, period.as_str());

    Utc::now() - duration
}

pub fn time_period_to_duration(time: i64, period: &str) -> Duration {
    match period {
        "weeks" => Duration::weeks(time),
        "days" => Duration::days(time),
        "hours" => Duration::hours(time),
        "minutes" => Duration::minutes(time),
        _ => Duration::seconds(time),
    }
}

#[test]
fn test_time_period_to_duration() {
    let time = 2;
    let week = 604800 * time;
    let day = 86400 * time;
    let hour = 3600 * time;
    let minute = 60 * time;

    assert_eq!(week, time_period_to_duration(time, "weeks").num_seconds());
    assert_eq!(day, time_period_to_duration(time, "days").num_seconds());
    assert_eq!(hour, time_period_to_duration(time, "hours").num_seconds());
    assert_eq!(
        minute,
        time_period_to_duration(time, "minutes").num_seconds()
    );
    assert_eq!(time, time_period_to_duration(time, "seconds").num_seconds());
}

// #[test]
// fn test_apply_window_geometry() {
//     gtk::init().expect("Error initializing gtk.");

//     let window = gtk::Window::new(gtk::WindowType::Toplevel);
//     let _geometry = WindowGeometry {
//         left: 0,
//         top: 0,
//         width: 100,
//         height: 100,
//         is_maximized: true
//     };

//     assert!(!window.is_maximized());

//     window.show();
// window.activate();
//     geometry.apply(&window);

//     assert!(window.is_maximized());
// }
