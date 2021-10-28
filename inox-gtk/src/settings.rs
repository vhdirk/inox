use gtk::prelude::GtkWindowExt;
use gio::{Settings, traits::SettingsExt};
use gtk;

use chrono::prelude::*;
use chrono::Duration;

pub struct WindowGeometry {
    left: i32,
    top: i32,
    width: i32,
    height: i32,
    is_maximized: bool,
}

impl WindowGeometry {
    pub fn from_window(window: &gtk::ApplicationWindow) -> WindowGeometry {
        let position = window.position();
        let size = window.size();
        let left = position.0;
        let top = position.1;
        let width = size.0;
        let height = size.1;
        let is_maximized = window.is_maximized();

        WindowGeometry {
            left,
            top,
            width,
            height,
            is_maximized,
        }
    }

    pub fn from_settings(settings: &gio::Settings) -> WindowGeometry {
        let top = settings.int("persist-window-geometry-top");
        let left = settings.int("persist-window-geometry-left");
        let width = settings.int("persist-window-geometry-width");
        let height = settings.int("persist-window-geometry-height");
        let is_maximized = settings.boolean("persist-window-geometry-maximized");

        WindowGeometry {
            left,
            top,
            width,
            height,
            is_maximized,
        }
    }

    pub fn apply(&self, window: &gtk::ApplicationWindow) {
        if self.width > 0 && self.height > 0 {
            window.resize(self.width, self.height);
        }

        if self.is_maximized {
            window.maximize();
        } else if self.top > 0 && self.left > 0 {
            window.move_(self.left, self.top);
        }
    }

    pub fn write(&self, settings: &gio::Settings) {
        settings.set_int("persist-window-geometry-left", self.left);
        settings.set_int("persist-window-geometry-top", self.top);
        settings.set_int("persist-window-geometry-width", self.width);
        settings.set_int("persist-window-geometry-height", self.height);
        settings.set_boolean("persist-window-geometry-maximized", self.is_maximized);
    }
}

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
