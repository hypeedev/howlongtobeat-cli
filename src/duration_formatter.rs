use std::time::Duration;

fn round_to_half(num: f32) -> f32 {
    (num * 2.0).round() / 2.0
}

pub trait DurationFormatter {
    fn format(&self) -> String;
}

impl DurationFormatter for Duration {
    fn format(&self) -> String {
        match self.as_secs() {
            seconds if seconds < 60 => format!("{}s", seconds),
            seconds if seconds < 3600 => format!("{}m", round_to_half(seconds as f32 / 60.0)),
            seconds => format!("{}h", round_to_half(seconds as f32 / 3600.0)),
        }
    }
}