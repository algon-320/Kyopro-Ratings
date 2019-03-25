mod atcoder;
mod codeforces;
mod topcoder;

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl Color {
    pub fn to_string(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
    pub fn from_str(s: &str) -> Option<Color> {
        if s.len() < 4 || &s[0..1] != "#" {
            return None;
        }
        match s.len() {
            4 => {
                let r = u8::from_str_radix(&s[1..2], 16).ok()?;
                let g = u8::from_str_radix(&s[2..3], 16).ok()?;
                let b = u8::from_str_radix(&s[3..4], 16).ok()?;
                Some(Color { r, g, b })
            }
            7 => {
                let r = u8::from_str_radix(&s[1..3], 16).ok()?;
                let g = u8::from_str_radix(&s[3..5], 16).ok()?;
                let b = u8::from_str_radix(&s[5..7], 16).ok()?;
                Some(Color { r, g, b })
            }
            _ => None,
        }
    }
}
pub struct Rating {
    pub value: i64,
    pub color: Color,
}

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
    pub fn get_rating(&self, handle: &str) -> Option<Rating> {
        match self {
            ContestService::AtCoder => atcoder::get_atcoder_rating(handle),
            ContestService::Codeforces => codeforces::get_codeforces_rating(handle),
            ContestService::TopCoder(kind) => topcoder::get_topcoder_rating(handle, kind),
        }
    }
}
