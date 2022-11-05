mod atcoder;
mod codeforces;
mod topcoder;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
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

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Rating {
    pub value: i64,
    pub color: Color,
}

pub async fn get_rating(service_name: &str, handle_name: &str) -> Option<Rating> {
    match service_name {
        atcoder::NAME => atcoder::get_rating(handle_name).await,
        codeforces::NAME => codeforces::get_rating(handle_name).await,
        topcoder::NAME_ALGOLITHM => topcoder::get_rating_algorithm(handle_name).await,
        topcoder::NAME_MARATHON => topcoder::get_rating_marathon(handle_name).await,
        _ => None,
    }
}
