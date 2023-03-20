use chrono::{Duration, Utc};


pub fn format_current_time() -> String{
    let u = Utc::now();
    u.format("[%Y-%m-%d|%H:%M:%S,%f]").to_string()
}
pub fn format_corrected_current_time(theta: Duration) -> String{
    let u = Utc::now() - theta;
    u.format("[%Y-%m-%d|%H:%M:%S,%f]").to_string()
}


