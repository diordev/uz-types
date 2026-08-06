use serde::{Deserialize, Serialize};
use std::fmt;

use crate::error::{TypeError, TypeResult};

/// O'zbekiston pasport seriyasi va raqami.
///
/// Format:
/// - 2 ta lotin harfi
/// - 7 ta raqam
///
/// Misol:
/// `AA1234567`
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Passport(String);

impl Passport {
    /// Pasport seriyasi uzunligi.
    pub const SERIES_LEN: usize = 2;

    /// Pasport raqami uzunligi.
    pub const NUMBER_LEN: usize = 7;

    /// Pasport umumiy uzunligi.
    pub const LEN: usize = Self::SERIES_LEN + Self::NUMBER_LEN;

    /// String qiymatdan Passport yaratadi.
    ///
    /// Qiymat:
    /// - trim qilinadi;
    /// - seriya uppercase qilinadi;
    /// - format tekshiriladi.
    pub fn parse(value: impl AsRef<str>) -> TypeResult<Self> {
        let raw = value.as_ref().trim();

        if raw.len() != Self::LEN {
            return Err(PassportError::InvalidLength.into());
        }

        let (series, number) = raw.split_at(Self::SERIES_LEN);

        if !series.bytes().all(|b| b.is_ascii_alphabetic())
            || !number.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(PassportError::InvalidFormat.into());
        }

        Ok(Self(format!(
            "{}{}",
            series.to_ascii_uppercase(),
            number
        )))
    }

    /// To'liq pasport qiymatini qaytaradi.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Pasport seriyasini qaytaradi.
    #[inline]
    pub fn series(&self) -> &str {
        &self.0[..Self::SERIES_LEN]
    }

    /// Pasport raqamini qaytaradi.
    #[inline]
    pub fn number(&self) -> &str {
        &self.0[Self::SERIES_LEN..]
    }

    /// Ichki String qiymatni qaytaradi.
    ///
    /// Ownership Passport dan String ga o'tadi.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}

/// Passport qiymatini `&str` sifatida ishlatish imkonini beradi.
impl AsRef<str> for Passport {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

/// Passport qiymatini string ko'rinishida chiqaradi.
impl fmt::Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// `&str` dan Passport yaratish.
impl TryFrom<&str> for Passport {
    type Error = TypeError;

    fn try_from(value: &str) -> TypeResult<Self> {
        Self::parse(value)
    }
}

/// `String` dan Passport yaratish.
impl TryFrom<String> for Passport {
    type Error = TypeError;

    fn try_from(value: String) -> TypeResult<Self> {
        Self::parse(value)
    }
}

/// Passport ni String ga o'tkazadi.
///
/// Ownership ko'chadi, nusxa olinmaydi.
impl From<Passport> for String {
    fn from(value: Passport) -> Self {
        value.into_inner()
    }
}

/// Passport validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum PassportError {
    /// Passport uzunligi noto'g'ri.
    #[error("passport length is invalid")]
    InvalidLength,

    /// Passport formati noto'g'ri.
    ///
    /// Format:
    /// 2 ta harf + 7 ta raqam
    #[error("passport format is invalid")]
    InvalidFormat,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_should_create_valid_passport() {
        let passport = Passport::parse("AA1234567").unwrap();

        assert_eq!(passport.as_str(), "AA1234567");
    }

    #[test]
    fn parse_should_convert_series_to_uppercase() {
        let passport = Passport::parse("ab1234567").unwrap();

        assert_eq!(passport.as_str(), "AB1234567");
    }

    #[test]
    fn parse_should_trim_whitespace() {
        let passport = Passport::parse("  AA1234567  ").unwrap();

        assert_eq!(passport.as_str(), "AA1234567");
    }

    #[test]
    fn should_return_series_and_number() {
        let passport = Passport::parse("AA1234567").unwrap();

        assert_eq!(passport.series(), "AA");
        assert_eq!(passport.number(), "1234567");
    }

    #[test]
    fn parse_should_fail_when_length_is_invalid() {
        let result = Passport::parse("AA123");

        assert!(matches!(
            result.unwrap_err(),
            TypeError::Passport(PassportError::InvalidLength)
        ));
    }

    #[test]
    fn parse_should_fail_when_series_contains_numbers() {
        let result = Passport::parse("A11234567");

        assert!(matches!(
            result.unwrap_err(),
            TypeError::Passport(PassportError::InvalidFormat)
        ));
    }

    #[test]
    fn parse_should_fail_when_number_contains_letters() {
        let result = Passport::parse("AA12345AB");

        assert!(matches!(
            result.unwrap_err(),
            TypeError::Passport(PassportError::InvalidFormat)
        ));
    }

    #[test]
    fn display_should_return_passport_value() {
        let passport = Passport::parse("AA1234567").unwrap();

        assert_eq!(format!("{}", passport), "AA1234567");
    }

    #[test]
    fn try_from_str_should_create_passport() {
        let passport = Passport::try_from("AA1234567").unwrap();

        assert_eq!(passport.as_str(), "AA1234567");
    }

    #[test]
    fn try_from_string_should_create_passport() {
        let passport = Passport::try_from(String::from("AA1234567")).unwrap();

        assert_eq!(passport.as_str(), "AA1234567");
    }

    #[test]
    fn passport_should_convert_into_string() {
        let passport = Passport::parse("AA1234567").unwrap();

        let value: String = passport.into();

        assert_eq!(value, "AA1234567");
    }

    #[test]
    fn serde_should_support_roundtrip() {
        let passport = Passport::parse("AA1234567").unwrap();

        let json = serde_json::to_string(&passport).unwrap();

        let restored: Passport = serde_json::from_str(&json).unwrap();

        assert_eq!(passport, restored);
    }

    #[test]
    fn deserialize_should_validate_format() {
        let json = "\"AA1234567\"";

        let passport: Passport = serde_json::from_str(json).unwrap();

        assert_eq!(passport.as_str(), "AA1234567");
    }

    #[test]
    fn serialize_should_return_string_value() {
        let passport = Passport::parse("AA1234567").unwrap();

        let json = serde_json::to_string(&passport).unwrap();

        assert_eq!(json, "\"AA1234567\"");
    }
}