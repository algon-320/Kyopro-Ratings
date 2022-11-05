use super::Color;
use super::Rating;
use crate::cache::FreshCache;
use crate::util::japan_tomorrow;

lazy_static::lazy_static! {
    static ref ALGORITHM_CACHE: FreshCache<Option<Rating>> = FreshCache::new();
    static ref MARATHON_CACHE: FreshCache<Option<Rating>> = FreshCache::new();
}

pub const NAME_ALGOLITHM: &str = "topcoder_algorithm";
pub const NAME_MARATHON: &str = "topcoder_marathon";

pub async fn get_rating_algorithm(handle: &str) -> Option<Rating> {
    match ALGORITHM_CACHE.get(handle) {
        Some(r) => r,
        None => fetch_and_store(handle).await?.0,
    }
}

pub async fn get_rating_marathon(handle: &str) -> Option<Rating> {
    match MARATHON_CACHE.get(handle) {
        Some(r) => r,
        None => fetch_and_store(handle).await?.1,
    }
}

async fn fetch_and_store(handle: &str) -> Option<(Option<Rating>, Option<Rating>)> {
    println!(
        "{} # topcoder: fetch {}'s rating",
        chrono::Utc::now(),
        handle
    );
    let json: serde_json::Value =
        reqwest::get(&format!("http://api.topcoder.com/v2/users/{}", handle))
            .await
            .ok()?
            .json()
            .await
            .ok()?;

    let mut algorithm = None;
    let mut marathon = None;
    if let serde_json::Value::Array(v) = &json["ratingSummary"] {
        for obj in v {
            if obj["name"] == "Algorithm" {
                algorithm = || -> Option<Rating> {
                    let value = obj["rating"].as_i64()?;
                    let code = &obj["colorStyle"].as_str()?[7..]; // "color: #RRGGBB"
                    let color = Color::from_str(&code)?;
                    let r = Rating { value, color };
                    Some(r)
                }();
            }
            if obj["name"] == "Marathon Match" {
                marathon = || -> Option<Rating> {
                    let value = obj["rating"].as_i64()?;
                    let code = &obj["colorStyle"].as_str()?[7..]; // "color: #RRGGBB"
                    let color = Color::from_str(&code)?;
                    let r = Rating { value, color };
                    Some(r)
                }();
            }
        }
    }

    ALGORITHM_CACHE.store(handle, algorithm.clone(), japan_tomorrow());
    MARATHON_CACHE.store(handle, marathon.clone(), japan_tomorrow());

    Some((algorithm, marathon))
}
