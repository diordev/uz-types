use chrono::Datelike;
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::error::{TypeError, TypeResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
/// test_docs
/// BirthDate Struct orqlai siz
/// YYYY-MM-DD formatda qabul data qabul qilishingiz mumkun.
pub struct BirthDate(NaiveDate);

impl BirthDate {
    // BirthDate ning yagona rasmiy string formati.
    const FORMAT: &str = "%Y-%m-%d";

    /// test_docs
    /// String ni BirthDate ga parse qiladi.
    ///
    /// Qabul qilinadigan format:
    /// YYYY-MM-DD
    ///
    /// Misol:
    /// 2025-08-04
    pub fn parse(value: impl AsRef<str>) -> TypeResult<Self> {
        let raw: &str = value.as_ref().trim();

        // NaiveDate::parse_from_str o'zi format va uzunlikni tekshiradi
        let date: NaiveDate = NaiveDate::parse_from_str(raw, Self::FORMAT).map_err(|_| {
            TypeError::validation(format!("birth_date is not a valid YYYY-MM-DD date: {raw}"))
        })?;

        let today: NaiveDate = chrono::Local::now().date_naive();

        if date > today {
            return Err(TypeError::validation(format!(
                "birth_date cannot be in the future: {raw}"
            )));
        }

        Ok(Self(date))
    }
    /// Instance dagi year qaytaradi type=u32
    pub fn year(&self) -> u32 {
        self.0.year() as u32
    }

    /// Instance dagi month qaytaradi type=u32
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Instance dagi day qaytaradi type=u32
    pub fn day(&self) -> u32 {
        self.0.day()
    }
}

// BirthDate qiymatini belgilangan sana formatida chiqaradi.
// Ichki NaiveDate formatini tashqi ko'rinishga o'zgartiradi.
// Masalan: 2025-01-01 ko'rinishida qaytaradi.
impl std::fmt::Display for BirthDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(Self::FORMAT))
    }
}

// &str dan BirthDate yaratishning idiomatik usuli.
// Parse muvaffaqiyatsiz bo'lsa xato qaytaradi.
impl TryFrom<&str> for BirthDate {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::parse(value)
    }
}

// String dan BirthDate yaratishning idiomatik usuli.
// Parse muvaffaqiyatsiz bo'lsa xato qaytaradi.
impl TryFrom<String> for BirthDate {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::try_from(value.as_str())
    }
}

// BirthDate ni String ga aylantiradi.
// Natija: "YYYY-MM-DD"
impl From<BirthDate> for String {
    fn from(value: BirthDate) -> Self {
        value.to_string()
    }
}

// BirthDate JSON ga "YYYY-MM-DD"
// satri ko'rinishida serialize qilinadi.
impl Serialize for BirthDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.collect_str(self)
    }
}

// JSON satrini BirthDate ga aylantiradi.
// Parse orqali validatsiya amalga oshiriladi.
impl<'de> Deserialize<'de> for BirthDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = <&str>::deserialize(deserializer)?;

        Self::parse(s).map_err(serde::de::Error::custom)
    }
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
