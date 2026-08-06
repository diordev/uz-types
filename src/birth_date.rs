use chrono::{Datelike, NaiveDate};
use serde::{self, Deserialize, Serialize};
use std::ops::Deref;

use crate::error::TypeError;

/// Tug'ilgan sanani ifodalovchi value object.
///
/// Faqat `YYYY-MM-DD` formatdagi sanalarni va kelajakda bo'lmagan qiymatlarni qabul qiladi.
/// Ichki qiymat sifatida `NaiveDate` saqlaydi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirthDate(NaiveDate);

impl BirthDate {
    /// Sana uchun yagona rasmiy format (`YYYY-MM-DD`).
    pub const FORMAT: &'static str = "%Y-%m-%d";

    /// String qiymatdan `BirthDate` yaratadi.
    ///
    /// Validatsiyalar:
    /// - Format `YYYY-MM-DD` bo'lishi kerak.
    /// - Kelajak sanasi qabul qilinmaydi.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        let raw = value.as_ref().trim();

        let date =
            NaiveDate::parse_from_str(raw, Self::FORMAT).map_err(|_| BirthDateError::Date)?;

        Self::from_naive_date(date)
    }

    /// `NaiveDate` obyektidan `BirthDate` yaratadi (kelajak sanasi tekshiriladi).
    pub fn from_naive_date(date: NaiveDate) -> Result<Self, TypeError> {
        let today = chrono::Local::now().date_naive();

        if date > today {
            return Err(BirthDateError::FutureDate.into());
        }

        Ok(Self(date))
    }

    /// Ichki `NaiveDate` qiymatiga reference qaytaradi.
    #[inline]
    pub fn as_naive_date(&self) -> &NaiveDate {
        &self.0
    }

    /// Ichki `NaiveDate` qiymatini qaytaradi (`Copy` bo'lgani uchun ownership yo'qolmaydi).
    #[inline]
    pub fn into_inner(self) -> NaiveDate {
        self.0
    }

    /// Tug'ilgan yilni qaytaradi.
    #[inline]
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Tug'ilgan oyni qaytaradi (1..=12).
    #[inline]
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Tug'ilgan kunni qaytaradi (1..=31).
    #[inline]
    pub fn day(&self) -> u32 {
        self.0.day()
    }
}

// ==========================================
// DEFAULT TRAITLAR
// ==========================================

/// `Deref` tufayli `BirthDate` obyektida `NaiveDate` metodlarini (masalan, .leap_year(), .weekday())
/// to'g'ridan-to'g'ri chaqirish mumkin bo'ladi.
impl Deref for BirthDate {
    type Target = NaiveDate;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `BirthDate`ni `NaiveDate` reference sifatida ishlatish imkonini beradi.
impl AsRef<NaiveDate> for BirthDate {
    #[inline]
    fn as_ref(&self) -> &NaiveDate {
        &self.0
    }
}

/// `BirthDate` qiymatini `YYYY-MM-DD` formatida chiqaradi.
impl std::fmt::Display for BirthDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(Self::FORMAT))
    }
}

/// `&str` dan `BirthDate` yaratish.
impl TryFrom<&str> for BirthDate {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, TypeError> {
        Self::parse(value)
    }
}

/// `String` dan `BirthDate` yaratish.
impl TryFrom<String> for BirthDate {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, TypeError> {
        Self::parse(value)
    }
}

/// `NaiveDate` dan `BirthDate` yaratish (kelajak sanasi validatsiya qilinadi).
impl TryFrom<NaiveDate> for BirthDate {
    type Error = TypeError;

    fn try_from(value: NaiveDate) -> Result<Self, TypeError> {
        Self::from_naive_date(value)
    }
}

/// `BirthDate` ni `NaiveDate` ga o'tkazadi.
impl From<BirthDate> for NaiveDate {
    fn from(value: BirthDate) -> Self {
        value.into_inner()
    }
}

/// `BirthDate` ni `String` ga o'tkazadi (YYYY-MM-DD formatida).
impl From<BirthDate> for String {
    fn from(value: BirthDate) -> Self {
        value.to_string()
    }
}

// ==========================================
// SERDE OPTIMIZATSIYASI
// ==========================================

/// `Display` trait orqali intermediate `String` yaratmasdan to'g'ridan-to'g'ri serializatsiya qilish.
impl Serialize for BirthDate {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

/// JSON string qiymatini zero-allocation (`&str`) orqali `BirthDate` ga o'tkazish.
#[allow(unknown_lints)]
impl<'de> Deserialize<'de> for BirthDate {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str>::deserialize(deserializer)?;
        Self::parse(s).map_err(serde::de::Error::custom)
    }
}

// ==========================================
// XATOLIKLAR ENUMI
// ==========================================

/// `BirthDate` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BirthDateError {
    /// Sana formati yoki qiymati noto'g'ri (YYYY-MM-DD kutilgan).
    #[error("invalid birth date format or value, expected YYYY-MM-DD")]
    Date,

    /// Kelajak sanasi berilgan (tug'ilgan sana kelajakda bo'lishi mumkin emas).
    #[error("birth date cannot be in the future")]
    FutureDate,
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. To'g'ri tug'ilgan sanani parse qilish
    #[test]
    fn parse_valid_birth_date() {
        let date = BirthDate::parse("1990-05-15").unwrap();
        assert_eq!(date.to_string(), "1990-05-15");
        assert_eq!(date.year(), 1990);
        assert_eq!(date.month(), 5);
        assert_eq!(date.day(), 15);
    }

    // 2. Noto'g'ri formatlar uchun xatolik qaytarishi
    #[test]
    fn parse_should_fail_for_invalid_format() {
        assert!(matches!(
            BirthDate::parse("15-05-1990").unwrap_err(),
            TypeError::BirthDate(BirthDateError::Date)
        ));
        assert!(matches!(
            BirthDate::parse("1990/05/15").unwrap_err(),
            TypeError::BirthDate(BirthDateError::Date)
        ));
    }

    // 3. Noto'g'ri oy (13-oy) uchun xatolik
    #[test]
    fn parse_should_fail_for_invalid_month() {
        assert!(matches!(
            BirthDate::parse("1990-13-01").unwrap_err(),
            TypeError::BirthDate(BirthDateError::Date)
        ));
    }

    // 4. Noto'g'ri kun (32-kun) uchun xatolik
    #[test]
    fn parse_should_fail_for_invalid_day() {
        assert!(matches!(
            BirthDate::parse("1990-01-32").unwrap_err(),
            TypeError::BirthDate(BirthDateError::Date)
        ));
    }

    // 5. Kabisa yili (Leap year) validatsiyasi
    #[test]
    fn parse_should_validate_leap_year() {
        assert!(BirthDate::parse("2000-02-29").is_ok());
        assert!(matches!(
            BirthDate::parse("1999-02-29").unwrap_err(),
            TypeError::BirthDate(BirthDateError::Date)
        ));
    }

    // 6. Kelajak sanasi kelsa xatolik berishi
    #[test]
    fn parse_should_fail_for_future_date() {
        assert!(matches!(
            BirthDate::parse("2100-01-01").unwrap_err(),
            TypeError::BirthDate(BirthDateError::FutureDate)
        ));
    }

    // 7. Boshidagi va oxiridagi bo'sh joylarni trim qilishi
    #[test]
    fn parse_should_trim_whitespace() {
        let date = BirthDate::parse(" 1990-05-15 ").unwrap();
        assert_eq!(date.to_string(), "1990-05-15");
    }

    // 8. Display, Deref va NaiveDate helper metodlarining ishlashi
    #[test]
    fn display_and_deref_should_work() {
        let date = BirthDate::parse("2000-01-01").unwrap();
        assert_eq!(format!("{}", date), "2000-01-01");

        // Deref yordamida NaiveDate metodlarini to'g'ridan-to'g mevalarni chaqirish:
        assert!(date.leap_year()); // 2000-yil kabisa yili
        assert_eq!(date.as_naive_date(), &date.into_inner());
    }

    // 9. Copy trait va qiymatlarni o'tkazish (ownership o'zgarmasligi)
    #[test]
    fn birth_date_should_implement_copy() {
        let first = BirthDate::parse("1985-03-20").unwrap();
        let second = first; // Copy yuz beradi
        assert_eq!(first, second);
    }

    // 10. Type o'g'irishlar: TryFrom va From (str, String, NaiveDate)
    #[test]
    fn test_try_from_and_from_conversions() {
        let date_str = BirthDate::try_from("1990-05-15").unwrap();
        let date_string = BirthDate::try_from(String::from("1990-05-15")).unwrap();
        assert_eq!(date_str, date_string);

        let naive = NaiveDate::from_ymd_opt(1990, 5, 15).unwrap();
        let date_naive = BirthDate::try_from(naive).unwrap();
        assert_eq!(date_str, date_naive);

        let value_string: String = date_str.into();
        assert_eq!(value_string, "1990-05-15");

        let value_naive: NaiveDate = date_str.into();
        assert_eq!(value_naive, naive);
    }

    // 11. Serde Serializatsiya va Deserializatsiya (Roundtrip)
    #[test]
    fn birth_date_should_support_serde_roundtrip() {
        let date = BirthDate::parse("1990-05-15").unwrap();

        let json = serde_json::to_string(&date).unwrap();
        assert_eq!(json, "\"1990-05-15\"");

        let restored: BirthDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, restored);
    }

    // 12. Deserializatsiya paytida validatsiya ishlashi
    #[test]
    fn deserialize_should_validate_format_and_future_dates() {
        let valid_json = "\"2025-01-01\"";
        let date: BirthDate = serde_json::from_str(valid_json).unwrap();
        assert_eq!(date.to_string(), "2025-01-01");

        let invalid_json = "\"2100-01-01\"";
        assert!(serde_json::from_str::<BirthDate>(invalid_json).is_err());
    }
}
