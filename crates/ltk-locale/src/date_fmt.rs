//! Date/time formatting per locale.

use std::time::{SystemTime, UNIX_EPOCH};

pub struct DateFormatter { pub locale: String, pub use_24h: bool }

impl DateFormatter {
    pub fn new(locale: impl Into<String>) -> Self {
        Self { locale: locale.into(), use_24h: true }
    }

    pub fn format_date(&self, year: i32, month: u8, day: u8) -> String {
        match self.locale.as_str() {
            l if l.starts_with("en-US") => format!("{:02}/{:02}/{}", month, day, year),
            l if l.starts_with("de") | l.starts_with("fr") | l.starts_with("ru") =>
                format!("{:02}.{:02}.{}", day, month, year),
            _ => format!("{}-{:02}-{:02}", year, month, day),
        }
    }

    pub fn format_time(&self, hour: u8, minute: u8) -> String {
        if self.use_24h { format!("{:02}:{:02}", hour, minute) }
        else {
            let h = if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour };
            format!("{}:{:02} {}", h, minute, if hour >= 12 { "PM" } else { "AM" })
        }
    }

    pub fn format_relative(&self, seconds_ago: i64) -> String {
        match seconds_ago.abs() {
            0..=59   => format!("{} seconds ago", seconds_ago.abs()),
            60..=3599 => format!("{} minutes ago", seconds_ago.abs() / 60),
            3600..=86399 => format!("{} hours ago", seconds_ago.abs() / 3600),
            s => format!("{} days ago", s / 86400),
        }
    }
}
