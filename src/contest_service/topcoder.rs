use serde_json::Value;

pub enum TopCoderContestType {
    Algorithm,
    Marathon,
}
pub fn get_topcoder_rating(handle: &str, kind: &TopCoderContestType) -> Option<i64> {
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
                            return obj["rating"].as_i64();
                        }
                    }
                    TopCoderContestType::Marathon => {
                        if obj["name"] == "Marathon Match" {
                            return obj["rating"].as_i64();
                        }
                    }
                }
            }
            None
        }
        _ => None,
    }
}
