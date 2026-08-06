use chrono::{Datelike, NaiveDate};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{TypeError, TypeResult};

/// Tug'ilgan sanani ifodalovchi value object.
///
/// Faqat `YYYY-MM-DD` formatdagi sanalarni qabul qiladi.
/// Ichki qiymat sifatida `NaiveDate` saqlaydi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirthDate(NaiveDate);

impl BirthDate {
    /// Sana uchun yagona rasmiy format.
    const FORMAT: &'static str = "%Y-%m-%d";

    /// String qiymatdan BirthDate yaratadi.
    ///
    /// Validatsiyalar:
    /// - Format `YYYY-MM-DD` bo'lishi kerak.
    /// - Kelajak sanasi qabul qilinmaydi.
    pub fn parse(value: impl AsRef<str>) -> TypeResult<Self> {
        let raw = value.as_ref().trim();

        let date = NaiveDate::parse_from_str(raw, Self::FORMAT)
            .map_err(|_| BirthDateError::InvalidDate)?;

        let today = chrono::Local::now().date_naive();

        if date > today {
            return Err(BirthDateError::FutureDate.into());
        }

        Ok(Self(date))
    }

    /// Tug'ilgan yilni qaytaradi.
    #[inline]
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Tug'ilgan oyni qaytaradi.
    #[inline]
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Tug'ilgan kunni qaytaradi.
    #[inline]
    pub fn day(&self) -> u32 {
        self.0.day()
    }
}

/// BirthDate qiymatini `YYYY-MM-DD` formatda chiqaradi.
impl std::fmt::Display for BirthDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(Self::FORMAT))
    }
}

/// `&str` dan BirthDate yaratish.
impl TryFrom<&str> for BirthDate {
    type Error = TypeError;

    fn try_from(value: &str) -> TypeResult<Self> {
        Self::parse(value)
    }
}

/// `String` dan BirthDate yaratish.
impl TryFrom<String> for BirthDate {
    type Error = TypeError;

    fn try_from(value: String) -> TypeResult<Self> {
        Self::parse(value)
    }
}

/// BirthDate ni String ga aylantiradi.
///
/// Ownership ko'chadi.
impl From<BirthDate> for String {
    fn from(value: BirthDate) -> Self {
        value.to_string()
    }
}

/// JSON serialize formati:
///
/// `"1990-05-15"`
impl Serialize for BirthDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

/// JSON string qiymatini BirthDate ga aylantiradi.
///
/// Parse orqali validatsiya ishlaydi.
impl<'de> Deserialize<'de> for BirthDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = <&str>::deserialize(deserializer)?;

        Self::parse(value).map_err(serde::de::Error::custom)
    }
}

/// BirthDate uchun domain xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
pub enum BirthDateError {
    /// Kelajak sanasi berilgan.
    #[error("birth date cannot be in the future")]
    FutureDate,

    /// Sana formati yoki qiymati noto'g'ri.
    #[error("invalid birth date")]
    InvalidDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_birth_date() {
        let date = BirthDate::parse("1990-05-15").unwrap();

        assert_eq!(date.to_string(), "1990-05-15");
    }

    #[test]
    fn parse_should_fail_for_invalid_format() {
        assert!(BirthDate::parse("15-05-1990").is_err());
        assert!(BirthDate::parse("1990/05/15").is_err());
    }

    #[test]
    fn parse_should_fail_for_invalid_month() {
        assert!(BirthDate::parse("1990-13-01").is_err());
    }

    #[test]
    fn parse_should_fail_for_invalid_day() {
        assert!(BirthDate::parse("1990-01-32").is_err());
    }

    #[test]
    fn parse_should_validate_leap_year() {
        assert!(BirthDate::parse("2000-02-29").is_ok());
        assert!(BirthDate::parse("1999-02-29").is_err());
    }

    #[test]
    fn parse_should_fail_for_future_date() {
        assert!(BirthDate::parse("2100-01-01").is_err());
    }

    #[test]
    fn parse_should_trim_whitespace() {
        let date = BirthDate::parse(" 1990-05-15 ").unwrap();

        assert_eq!(date.to_string(), "1990-05-15");
    }

    #[test]
    fn display_should_return_expected_format() {
        let date = BirthDate::parse("2000-01-01").unwrap();

        assert_eq!(format!("{}", date), "2000-01-01");
    }

    #[test]
    fn birth_date_should_implement_copy() {
        let first = BirthDate::parse("1985-03-20").unwrap();
        let second = first;

        assert_eq!(first, second);
    }

    #[test]
    fn birth_date_should_convert_into_string() {
        let date = BirthDate::parse("1990-05-15").unwrap();

        let value: String = date.into();

        assert_eq!(value, "1990-05-15");
    }

    #[test]
    fn birth_date_should_support_serde_roundtrip() {
        let date = BirthDate::parse("1990-05-15").unwrap();

        let json = serde_json::to_string(&date).unwrap();

        let restored: BirthDate = serde_json::from_str(&json).unwrap();

        assert_eq!(date, restored);
    }

    #[test]
    fn deserialize_should_create_valid_birth_date() {
        let json = "\"2025-01-01\"";

        let date: BirthDate = serde_json::from_str(json).unwrap();

        assert_eq!(date.to_string(), "2025-01-01");
    }

    #[test]
    fn serialize_should_use_expected_format() {
        let date = BirthDate::parse("2025-01-01").unwrap();

        let json = serde_json::to_string(&date).unwrap();

        assert_eq!(json, "\"2025-01-01\"");
    }
}
