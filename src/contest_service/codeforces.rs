use serde_json::Value;

pub fn get_codeforces_rating(handle: &str) -> Option<i64> {
    let json: Value = reqwest::get(&format!(
        "https://codeforces.com/api/user.info?handles={}",
        handle
    )).ok()?
    .json()
    .ok()?;
    if json["status"] == "OK" {
        json["result"][0]["rating"].as_i64()
    } else {
        None
    }
}
