use super::Color;
use super::Rating;
use crate::cache::FreshCache;
use crate::util::japan_tomorrow;

lazy_static::lazy_static! {
    static ref CACHE: FreshCache<Option<Rating>> = FreshCache::new();
}

pub const NAME: &str = "atcoder";

pub async fn get_rating(handle: &str) -> Option<Rating> {
    if let Some(r) = CACHE.get(handle) {
        return r;
    }

    println!(
        "{} # atcoder: fetch {}'s rating",
        chrono::Utc::now(),
        handle
    );
    let html = reqwest::get(&format!("https://atcoder.jp/users/{}", handle))
        .await
        .ok()?
        .text()
        .await
        .ok()?;

    let rating = || -> Option<Rating> {
        const ATCODER_RATING_CSS_SELECTOR : &str = "table.dl-table:nth-child(3) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(2) > span:nth-child(1)";

        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse(ATCODER_RATING_CSS_SELECTOR).ok()?;
        let span = document.select(&selector).next()?;
        let rating_str = span.text().next()?;
        let value = rating_str.parse::<i64>().ok()?;
        let class = span.value().attr("class");
        let color = Color::from_str(match class {
            Some("user-red") => "#FF0000",
            Some("user-orange") => "#FF8000",
            Some("user-yellow") => "#C0C000",
            Some("user-blue") => "#0000FF",
            Some("user-cyan") => "#00C0C0",
            Some("user-green") => "#008000",
            Some("user-brown") => "#804000",
            Some("user-gray") => "#808080",
            Some("user-unrated") => "#000000",
            Some("user-admin") => "#C000C0",
            _ => "#000000",
        })?;
        Some(Rating { value, color })
    }();

    CACHE.store(handle, rating.clone(), japan_tomorrow());

    rating
}
