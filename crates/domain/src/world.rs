//! The structure of a world that every genre shares: regions, nationalities,
//! competitions and teams (packages spec).

use crate::date::GameDate;
use crate::id::ContentId;

/// Settings that apply to the whole world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WorldSettings {
    pub start_date: GameDate,
}

/// Whether a full name is written given name first or family name first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum NameOrder {
    GivenFirst,
    FamilyFirst,
}

/// An ISO 3166-1 alpha-2 country code, such as `KR`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CountryCode([u8; 2]);

impl CountryCode {
    /// Two uppercase ASCII letters.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        match text.as_bytes() {
            &[a, b] if a.is_ascii_uppercase() && b.is_ascii_uppercase() => Some(Self([a, b])),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        // Both bytes are ASCII letters, so this never falls back.
        std::str::from_utf8(&self.0).unwrap_or("??")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nationality {
    pub id: ContentId,
    pub code: CountryCode,
    pub name_order: NameOrder,
}

/// The names people of one nationality are drawn from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamePool {
    pub nationality: ContentId,
    pub given: Vec<String>,
    pub family: Vec<String>,
}

/// A region, with how strong its scene is and who plays in it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    pub id: ContentId,
    /// Added to every team's mean strength in this region.
    pub level: i32,
    /// Nationalities and their weights, in id order.
    pub nationalities: Vec<(ContentId, u32)>,
}

/// A league. Its stages come with the competitions spec.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Competition {
    pub id: ContentId,
    pub region: ContentId,
    pub teams: Vec<ContentId>,
}

/// A team's three-character tag, such as `AZT`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tag([u8; 3]);

impl Tag {
    /// Three uppercase ASCII letters or digits.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let valid = |b: u8| b.is_ascii_uppercase() || b.is_ascii_digit();
        match text.as_bytes() {
            &[a, b, c] if valid(a) && valid(b) && valid(c) => Some(Self([a, b, c])),
            _ => None,
        }
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.0).unwrap_or("???")
    }
}

/// An sRGB colour.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Colour {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Colour {
    /// `#RRGGBB`, with uppercase or lowercase hex digits.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let hex = text.strip_prefix('#')?;
        if hex.len() != 6 || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
            return None;
        }
        let channel = |range| u8::from_str_radix(&hex[range], 16).ok();
        Some(Self {
            red: channel(0..2)?,
            green: channel(2..4)?,
            blue: channel(4..6)?,
        })
    }
}

impl std::fmt::Display for Colour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:02X}{:02X}{:02X}", self.red, self.green, self.blue)
    }
}

/// How well known and strong a team is, from 1 to 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Reputation(u8);

impl Reputation {
    #[must_use]
    pub fn new(value: u32) -> Option<Self> {
        u8::try_from(value)
            .ok()
            .filter(|value| (1..=100).contains(value))
            .map(Self)
    }

    #[must_use]
    pub const fn value(self) -> u8 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Team {
    pub id: ContentId,
    pub region: ContentId,
    pub tag: Tag,
    pub colours: [Colour; 2],
    pub reputation: Reputation,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn country_codes_are_two_uppercase_letters() {
        assert_eq!(
            CountryCode::parse("KR").map(|c| c.as_str().to_owned()),
            Some("KR".to_owned())
        );
        for text in ["kr", "K", "KOR", "K1", ""] {
            assert_eq!(CountryCode::parse(text), None, "{text}");
        }
    }

    #[test]
    fn tags_are_three_uppercase_letters_or_digits() {
        assert_eq!(
            Tag::parse("PS7").map(|t| t.as_str().to_owned()),
            Some("PS7".to_owned())
        );
        for text in ["azt", "AZ", "AZTX", "A-T", "ÇAP"] {
            assert_eq!(Tag::parse(text), None, "{text}");
        }
    }

    #[test]
    fn colours_parse_from_hex() {
        assert_eq!(
            Colour::parse("#1E5aA8"),
            Some(Colour {
                red: 0x1e,
                green: 0x5a,
                blue: 0xa8
            })
        );
        assert_eq!(
            Colour::parse("#1E5AA8").map(|c| c.to_string()),
            Some("#1E5AA8".to_owned())
        );
        for text in ["1E5AA8", "#1E5AA", "#1E5AA80", "#GG5AA8", "#+1+1+1", ""] {
            assert_eq!(Colour::parse(text), None, "{text}");
        }
    }

    #[test]
    fn reputation_runs_from_1_to_100() {
        assert_eq!(Reputation::new(1).map(Reputation::value), Some(1));
        assert_eq!(Reputation::new(100).map(Reputation::value), Some(100));
        assert_eq!(Reputation::new(0), None);
        assert_eq!(Reputation::new(101), None);
        assert_eq!(Reputation::new(300), None);
    }
}
