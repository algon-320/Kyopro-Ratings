use serde_json::Value;

use super::Color;
use super::ContestService;
use super::Rating;
use crate::cache::FreshCache;
use crate::util::tomorrow;

lazy_static! {
    static ref ALGORITHM_CACHE: FreshCache<Option<Rating>> = FreshCache::new();
    static ref MARATHON_CACHE: FreshCache<Option<Rating>> = FreshCache::new();
}

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
        match self.contest_type {
            TopCoderContestType::Algorithm => match ALGORITHM_CACHE.get(handle) {
                Some(r) => r,
                None => fetch_and_store(handle)?.0,
            },
            TopCoderContestType::Marathon => match MARATHON_CACHE.get(handle) {
                Some(r) => r,
                None => fetch_and_store(handle)?.1,
            },
        }
    }
}

fn fetch_and_store(handle: &str) -> Option<(Option<Rating>, Option<Rating>)> {
    println!(
        "{} # topcoder: fetch {}'s rating",
        chrono::Utc::now(),
        handle
    );
    let json: Value = reqwest::get(&format!("http://api.topcoder.com/v2/users/{}", handle))
        .ok()?
        .json()
        .ok()?;

    let mut algorithm = None;
    let mut marathon = None;
    match &json["ratingSummary"] {
        Value::Array(v) => {
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
        _ => {}
    }

    ALGORITHM_CACHE.store(handle, algorithm.clone(), tomorrow(0));
    MARATHON_CACHE.store(handle, marathon.clone(), tomorrow(0));
    Some((algorithm, marathon))
}
