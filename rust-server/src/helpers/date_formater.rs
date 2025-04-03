use chrono::Local;

pub struct DateFormater;

impl DateFormater {
    pub fn datetime() -> String {
        Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
