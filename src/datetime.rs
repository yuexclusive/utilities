#![cfg(feature = "datetime")]

use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, Utc};

pub trait FormatDateTime {
    fn to_rfc3339(&self) -> String;
    fn to_default(&self) -> String;
}

const FORMAT_DEFAULT: &str = "%Y-%m-%d %H:%M";
const FORMAT_RFC3339: &str = "%Y-%m-%dT%H:%M:%S%.6f%:z";

pub trait ToDateTime {
    fn default_to_utc(&self) -> Option<DateTime<Utc>>;
    fn default_to_local(&self) -> Option<DateTime<Local>>;
    fn rfc3339_to_utc(&self) -> Option<DateTime<Utc>>;
    fn rfc3339_to_local(&self) -> Option<DateTime<Local>>;
    fn to_utc(&self, format: &str) -> Option<DateTime<Utc>>;
    fn to_local(&self, format: &str) -> Option<DateTime<Local>>;
}

impl FormatDateTime for DateTime<Utc> {
    fn to_rfc3339(&self) -> String {
        DateTime::<Local>::from(self.clone())
            .format(FORMAT_RFC3339)
            .to_string()
    }

    fn to_default(&self) -> String {
        DateTime::<Local>::from(self.clone())
            .format(FORMAT_DEFAULT)
            .to_string()
    }
}

impl<T> ToDateTime for T
where
    T: AsRef<str>,
{
    fn default_to_utc(&self) -> Option<DateTime<Utc>> {
        let local = self.default_to_local()?;
        Some(DateTime::<Utc>::from(local))
    }

    fn default_to_local(&self) -> Option<DateTime<Local>> {
        let time = NaiveDateTime::parse_from_str(self.as_ref(), FORMAT_DEFAULT).ok()?;
        let dt = DateTime::<Local>::from_local(time, FixedOffset::east_opt(60 * 60 * 8).unwrap());
        Some(dt)
    }

    fn rfc3339_to_utc(&self) -> Option<DateTime<Utc>> {
        let local = self.rfc3339_to_local()?;
        Some(DateTime::<Utc>::from(local))
    }

    fn rfc3339_to_local(&self) -> Option<DateTime<Local>> {
        let time = NaiveDateTime::parse_from_str(self.as_ref(), FORMAT_RFC3339).ok()?;
        let dt = DateTime::<Local>::from_local(time, FixedOffset::east_opt(60 * 60 * 8).unwrap());
        Some(dt)
    }

    fn to_utc(&self, format: &str) -> Option<DateTime<Utc>> {
        let local = self.to_local(format)?;
        Some(DateTime::<Utc>::from(local))
    }

    fn to_local(&self, format: &str) -> Option<DateTime<Local>> {
        let time = NaiveDateTime::parse_from_str(self.as_ref(), format).ok()?;
        let dt = DateTime::<Local>::from_local(time, FixedOffset::east_opt(60 * 60 * 8).unwrap());
        Some(dt)
    }
}
