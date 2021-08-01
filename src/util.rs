use chrono::{DateTime, Utc};

pub fn sanitize_html(html: &str) -> String {
    ammonia::clean_text(html)
}

pub fn date_display(date: &DateTime<Utc>) -> String {
    date.format("%Y %B %d, %H:%M").to_string()
}
