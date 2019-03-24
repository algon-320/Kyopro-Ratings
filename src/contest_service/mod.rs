mod atcoder;
mod codeforces;
mod topcoder;

pub enum ContestService {
    AtCoder,
    Codeforces,
    TopCoder(topcoder::TopCoderContestType),
}
impl ContestService {
    pub fn from_name(service_name: &str) -> Option<ContestService> {
        match service_name {
            "atcoder" => Some(ContestService::AtCoder),
            "codeforces" => Some(ContestService::Codeforces),
            "topcoder_algorithm" => Some(ContestService::TopCoder(
                topcoder::TopCoderContestType::Algorithm,
            )),
            "topcoder_marathon" => Some(ContestService::TopCoder(
                topcoder::TopCoderContestType::Marathon,
            )),
            _ => None,
        }
    }
    pub fn name(&self) -> &str {
        match self {
            ContestService::AtCoder => "atcoder",
            ContestService::Codeforces => "codeforces",
            ContestService::TopCoder(topcoder::TopCoderContestType::Algorithm) => {
                "topcoder_algorithm"
            }
            ContestService::TopCoder(topcoder::TopCoderContestType::Marathon) => {
                "topcoder_marathon"
            }
        }
    }
    pub fn get_rating(&self, handle: &str) -> Option<i64> {
        match self {
            ContestService::AtCoder => atcoder::get_atcoder_rating(handle),
            ContestService::Codeforces => codeforces::get_codeforces_rating(handle),
            ContestService::TopCoder(kind) => topcoder::get_topcoder_rating(handle, kind),
        }
    }
    pub fn get_color(&self, rating: i64) -> (u8, u8, u8) {
        unimplemented!()
    }
}
