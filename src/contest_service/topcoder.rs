use serde_json::Value;

use super::Color;
use super::ContestService;
use super::Rating;

pub enum TopCoderContestType {
    Algorithm,
    Marathon,
}

pub struct TopCoder {
    contest_type: TopCoderContestType,
}
impl TopCoder {
    pub fn get_service(contest_type: TopCoderContestType) -> Box<dyn ContestService> {
        Box::new(TopCoder { contest_type })
    }
}

impl ContestService for TopCoder {
    fn name(&self) -> &str {
        match self.contest_type {
            TopCoderContestType::Algorithm => "topcoder_algorithm",
            TopCoderContestType::Marathon => "topcoder_marathon",
        }
    }
    fn get_rating(&self, handle: &str) -> Option<Rating> {
        let json: Value = reqwest::get(&format!("http://api.topcoder.com/v2/users/{}", handle))
            .ok()?
            .json()
            .ok()?;
        match &json["ratingSummary"] {
            Value::Array(v) => {
                for obj in v {
                    match self.contest_type {
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
}
