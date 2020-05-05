use serde_json::Value;

use super::Color;
use super::ContestService;
use super::Rating;
use crate::cache::FreshCache;
use crate::util::tomorrow;

lazy_static! {
    static ref CACHE: FreshCache<Rating> = FreshCache::new();
}

pub struct Codeforces;

impl Codeforces {
    pub fn get_service() -> Box<dyn ContestService> {
        Box::new(Self)
    }
}

impl ContestService for Codeforces {
    fn name(&self) -> &str {
        "codeforces"
    }
    fn get_rating(&self, handle: &str) -> Option<Rating> {
        match CACHE.get(handle) {
            Some(x) => Some(x),
            None => fetch_and_store(handle),
        }
    }
}

fn fetch_and_store(handle: &str) -> Option<Rating> {
    println!(
        "{} # codeforces: fetch {}'s rating",
        chrono::Utc::now(),
        handle
    );
    let json: Value = reqwest::get(&format!(
        "https://codeforces.com/api/user.info?handles={}",
        handle
    ))
    .ok()?
    .json()
    .ok()?;

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
    let rating = Rating { value, color };
    CACHE.store(handle, rating.clone(), tomorrow(0));
    Some(rating)
}
