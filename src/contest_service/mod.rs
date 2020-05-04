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

pub trait ContestService {
    fn name(&self) -> &str;
    fn get_rating(&self, handle: &str) -> Option<Rating>;
}

pub fn from_name(name: &str) -> Option<Box<dyn ContestService>> {
    let services = vec![
        atcoder::AtCoder::get_service(),
        codeforces::Codeforces::get_service(),
        topcoder::TopCoder::get_service(topcoder::TopCoderContestType::Algorithm),
        topcoder::TopCoder::get_service(topcoder::TopCoderContestType::Marathon),
    ];

    for s in services.into_iter() {
        if name == s.name() {
            return Some(s);
        }
    }
    None
}
