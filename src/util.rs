use chrono::{DateTime, Utc};
use rand::Rng;

/// Clean text containing HTML into regular text.
pub fn sanitize_html(html: &str) -> String {
    ammonia::clean_text(html)
}

/// A human-friendly display of a date time.
pub fn date_display(date: &DateTime<Utc>) -> String {
    date.format("%Y %B %d, %H:%M").to_string()
}

/// Get a random emoji within a relatively large range.
pub fn random_emoij() -> char {
    let mut rng = rand::thread_rng();
    rng.gen_range('🐵'..'🗿')
}
