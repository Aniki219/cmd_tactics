use serde::Deserialize;

#[derive(Deserialize, Clone, Copy)]
#[serde(try_from = "String")]
pub struct Grade(i32);

impl Grade {
    pub fn value(self) -> i32 {
        self.0
    }
}

impl TryFrom<String> for Grade {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut chars = s.chars();

        let base = match chars.next() {
            Some('S') => 16,
            Some('A') => 13,
            Some('B') => 10,
            Some('C') => 7,
            Some('D') => 4,
            Some('F') => 1,
            _ => return Err(format!("invalid grade '{s}'")),
        };

        let modifier = match chars.next() {
            Some('+') => 1,
            Some('-') => -1,
            None => 0,
            Some(_) => return Err(format!("invalid grade modifier in '{s}'")),
        };

        if chars.next().is_some() {
            return Err(format!("invalid grade '{s}'"));
        }

        Ok(Grade(base + modifier))
    }
}