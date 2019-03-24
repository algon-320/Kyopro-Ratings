use scraper::{Html, Selector};

pub fn get_atcoder_rating(handle: &str) -> Option<i64> {
    const ATCODER_RATING_CSS_SELECTOR : &str = "table.dl-table:nth-child(3) > tbody:nth-child(1) > tr:nth-child(2) > td:nth-child(2) > span:nth-child(1)";
    let html = reqwest::get(&format!("https://atcoder.jp/users/{}", handle))
        .ok()?
        .text()
        .ok()?;
    let document = Html::parse_document(&html);
    let selector = Selector::parse(ATCODER_RATING_CSS_SELECTOR).ok()?;
    let span = document.select(&selector).next()?;
    let rating_str = span.text().next()?;
    rating_str.parse::<i64>().ok()
}
