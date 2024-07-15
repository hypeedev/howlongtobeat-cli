use std::time::Duration;

pub trait DurationFormatter {
    fn format(&self) -> String;
}

impl DurationFormatter for Duration {
    fn format(&self) -> String {
        match self.as_secs() {
            seconds if seconds < 60 => format!("{}s", seconds),
            seconds if seconds < 3600 => format!("{}m", seconds / 60),
            seconds => format!("{}h", seconds / 3600),
        }
    }
}