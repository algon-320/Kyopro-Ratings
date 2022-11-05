use super::Color;
use super::Rating;
use crate::cache::FreshCache;
use crate::util::japan_tomorrow;

lazy_static::lazy_static! {
    static ref CACHE: FreshCache<Option<Rating>> = FreshCache::new();
}

pub const NAME: &str = "codeforces";

pub async fn get_rating(handle: &str) -> Option<Rating> {
    if let Some(r) = CACHE.get(handle) {
        return r;
    }

    println!(
        "{} # codeforces: fetch {}'s rating",
        chrono::Utc::now(),
        handle
    );
    let json: serde_json::Value = reqwest::get(&format!(
        "https://codeforces.com/api/user.info?handles={}",
        handle
    ))
    .await
    .ok()?
    .json()
    .await
    .ok()?;

    let rating = || -> Option<Rating> {
        let value = if json["status"] == "OK" {
            json["result"][0]["rating"].as_i64()?
        } else {
            return None;
        };
        let color = Color::from_str(match value {
            r if r >= 2400 => "#FF0000",
            r if r >= 2100 => "#FF8C00",
            r if r >= 1900 => "#AA00AA",
            r if r >= 1600 => "#0000FF",
            r if r >= 1400 => "#03A89E",
            r if r >= 1200 => "#008000",
            r if r >= 0000 => "#808080",
            _ => "#000000",
        })?;
        Some(Rating { value, color })
    }();

    CACHE.store(handle, rating.clone(), japan_tomorrow());

    rating
}
