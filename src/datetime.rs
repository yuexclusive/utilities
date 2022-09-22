#![cfg(feature = "datetime")]

use chrono::{DateTime, Local, Utc};

pub trait FormatDateTime {
    fn to_rfc3339(&self) -> String;
    fn to_show(&self) -> String;
}

impl FormatDateTime for DateTime<Utc> {
    fn to_rfc3339(&self) -> String {
        DateTime::<Local>::from(self.clone()).to_rfc3339()
    }

    fn to_show(&self) -> String {
        DateTime::<Local>::from(self.clone())
            .format("%Y-%m-%d %H:%M")
            .to_string()
    }
}
