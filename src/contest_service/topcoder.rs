use serde_json::Value;

use super::Color;
use super::Rating;

pub enum TopCoderContestType {
    Algorithm,
    Marathon,
}
pub fn get_topcoder_rating(handle: &str, kind: &TopCoderContestType) -> Option<Rating> {
    let json: Value = reqwest::get(&format!("http://api.topcoder.com/v2/users/{}", handle))
        .ok()?
        .json()
        .ok()?;
    match &json["ratingSummary"] {
        Value::Array(v) => {
            for obj in v {
                match kind {
                    TopCoderContestType::Algorithm => {
                        if obj["name"] == "Algorithm" {
                            let value = obj["rating"].as_i64()?;
                            let code = &obj["colorStyle"].as_str()?[7..]; // "color: #RRGGBB"
                            let color = Color::from_str(&code)?;
                            return Some(Rating { value, color });
                        }
                    }
                    TopCoderContestType::Marathon => {
                        if obj["name"] == "Marathon Match" {
                            let value = obj["rating"].as_i64()?;
                            let code = &obj["colorStyle"].as_str()?[7..]; // "color: #RRGGBB"
                            let color = Color::from_str(&code)?;
                            return Some(Rating { value, color });
                        }
                    }
                }
            }
        }
        _ => {}
    }
    None
}
