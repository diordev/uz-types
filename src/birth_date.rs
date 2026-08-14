use chrono::{Datelike, NaiveDate, Utc};
use serde::{self, Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

use crate::error::TypeError;

/// Sana formatlarini belgilovchi state (holat) enumi.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DateFormat {
    /// YYYY-MM-DD formati
    YmdHyphen,
    /// DD-MM-YYYY formati
    DmyHyphen,
    /// YYYY.MM.DD formati
    YmdDot,
    /// DD.MM.YYYY formati
    DmyDot,
}

impl DateFormat {
    /// Chrono kutubxonasi uchun mos format stringini qaytaradi.
    #[inline]
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::YmdHyphen => "%Y-%m-%d",
            Self::DmyHyphen => "%d-%m-%Y",
            Self::YmdDot => "%Y.%m.%d",
            Self::DmyDot => "%d.%m.%Y",
        }
    }

    /// Format holatini (state) compile-time darajasida teskarisiga o'zgartiradi.
    ///
    /// - `YYYY-MM-DD` -> `DD-MM-YYYY`
    /// - `DD-MM-YYYY` -> `YYYY-MM-DD`
    /// - `YYYY.MM.DD` -> `DD.MM.YYYY`
    /// - `DD.MM.YYYY` -> `YYYY.MM.DD`
    #[inline]
    #[must_use]
    pub const fn reversed(&self) -> Self {
        match self {
            Self::YmdHyphen => Self::DmyHyphen,
            Self::DmyHyphen => Self::YmdHyphen,
            Self::YmdDot => Self::DmyDot,
            Self::DmyDot => Self::YmdDot,
        }
    }
}

/// Tug'ilgan sanani ifodalovchi value object.
///
/// Sana ikki tomondan cheklanadi:
/// - **Yuqoridan**: kelajakdagi sana qabul qilinmaydi
///   ([`BirthDateError::FutureDate`]);
/// - **Pastdan**: [`Self::MIN_YEAR`] dan oldingi yil qabul qilinmaydi
///   ([`BirthDateError::TooOld`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct BirthDate(NaiveDate);
impl BirthDate {
    /// Standart rasmiy format (`YYYY-MM-DD`).
    pub const DEFAULT_FORMAT: DateFormat = DateFormat::YmdHyphen;

    /// Qabul qilinadigan eng erta tug'ilgan yil.
    ///
    /// Bu chegara `0001-01-01` kabi ma'nosiz qiymatlar va noto'g'ri
    /// parse qilingan sanalarni to'sadi.
    pub const MIN_YEAR: i32 = 1900;

    /// Standart `YYYY-MM-DD` formatidan `BirthDate` yaratadi.
    #[inline]
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        Self::parse_with_format(value, Self::DEFAULT_FORMAT)
    }

    /// Berilgan format (`DateFormat` state) asosida satrdan `BirthDate` yaratadi.
    #[inline]
    pub fn parse_with_format(
        value: impl AsRef<str>,
        format: DateFormat,
    ) -> Result<Self, TypeError> {
        let raw = value.as_ref().trim();

        let date =
            NaiveDate::parse_from_str(raw, format.as_str()).map_err(|_| BirthDateError::Date)?;

        Self::from_naive_date(date)
    }

    /// `NaiveDate` obyektidan `BirthDate` yaratadi.
    ///
    /// # Timezone
    ///
    /// Server UTC'da ishlashi mumkin, foydalanuvchi esa boshqa mintaqada
    /// (O'zbekiston — UTC+5). Shu sababli yuqori chegara sifatida
    /// **UTC bo'yicha ertangi kun** olinadi: bu Yer yuzidagi eng oldinda
    /// turgan mintaqa (UTC+14) uchun ham to'g'ri bo'ladi va bugun
    /// tug'ilgan chaqaloq xato `FutureDate` olmaydi.
    #[inline]
    pub fn from_naive_date(date: NaiveDate) -> Result<Self, TypeError> {
        Self::from_naive_date_with_today(date, Utc::now().date_naive())
    }

    /// `from_naive_date` ning deterministik (testlanadigan) varianti —
    /// "bugun" tashqaridan beriladi, tizim soatiga murojaat qilinmaydi.
    ///
    /// `today` UTC bo'yicha joriy sana bo'lishi kutiladi.
    #[inline]
    pub fn from_naive_date_with_today(
        date: NaiveDate,
        today: NaiveDate,
    ) -> Result<Self, TypeError> {
        if date.year() < Self::MIN_YEAR {
            return Err(BirthDateError::TooOld.into());
        }

        // UTC+14 gacha bo'lgan mintaqalar uchun bir kunlik yon berish.
        let max_allowed = today.succ_opt().unwrap_or(today);
        if date > max_allowed {
            return Err(BirthDateError::FutureDate.into());
        }

        Ok(Self(date))
    }

    /// Sanani berilgan format (`DateFormat` state) bo'yicha `String` ko'rinishiga o'tkazadi.
    #[inline]
    #[must_use]
    pub fn format_as(&self, format: DateFormat) -> String {
        self.0.format(format.as_str()).to_string()
    }

    /// Joriy formatni olib, uni `reversed()` state yordamida teskari formatga o'tkazib qaytaradi.
    #[inline]
    #[must_use]
    pub fn format_reversed(&self, current_format: DateFormat) -> String {
        self.format_as(current_format.reversed())
    }

    /// Ichki `NaiveDate` qiymatini qaytaradi (`Copy` bo'lgani uchun ownership yo'qolmaydi).
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> NaiveDate {
        self.0
    }

    /// Tug'ilgan yilni qaytaradi.
    #[inline]
    #[must_use]
    pub fn year(&self) -> i32 {
        self.0.year()
    }

    /// Tug'ilgan oyni qaytaradi (1..=12).
    #[inline]
    #[must_use]
    pub fn month(&self) -> u32 {
        self.0.month()
    }

    /// Tug'ilgan kunni qaytaradi (1..=31).
    #[inline]
    #[must_use]
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// To'liq yoshni qaytaradi (bugungi UTC sanasi bo'yicha).
    ///
    /// Tug'ilgan kun bu yil hali kelmagan bo'lsa, yosh bir birlik kam bo'ladi.
    #[inline]
    #[must_use]
    pub fn age(&self) -> u32 {
        self.age_at(Utc::now().date_naive())
    }

    /// Berilgan sanadagi to'liq yoshni qaytaradi (deterministik, testlanadigan).
    ///
    /// `date` tug'ilgan sanadan oldin bo'lsa `0` qaytaradi.
    #[must_use]
    pub fn age_at(&self, date: NaiveDate) -> u32 {
        if date <= self.0 {
            return 0;
        }

        let mut years = date.year() - self.0.year();

        // Tug'ilgan kun bu yil hali kelmagan bo'lsa, bir yil ayiramiz.
        // Kortej solishtiruvi 29-fevral holatini ham to'g'ri hal qiladi.
        if (date.month(), date.day()) < (self.0.month(), self.0.day()) {
            years -= 1;
        }

        u32::try_from(years).unwrap_or(0)
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

/// `BirthDate` qiymatini standart `YYYY-MM-DD` formatida chiqaradi.
impl std::fmt::Display for BirthDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.format(Self::DEFAULT_FORMAT.as_str()))
    }
}

/// Rust'ning standart idiomatik parse uslubi (`"1990-05-15".parse::<BirthDate>()`).
///
/// Faqat standart [`BirthDate::DEFAULT_FORMAT`] (`YYYY-MM-DD`) qabul qilinadi;
/// boshqa format uchun [`BirthDate::parse_with_format`] dan foydalaning.
impl std::str::FromStr for BirthDate {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
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
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

/// JSON string qiymatini zero-allocation (`&str`) orqali `BirthDate` ga o'tkazish.
impl<'de> Deserialize<'de> for BirthDate {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = Cow::<'de, str>::deserialize(deserializer)?;

        match s {
            // Borrow qilingan bo'lsa: zero-allocation &str orqali yasaladi
            Cow::Borrowed(borrowed) => Self::try_from(borrowed).map_err(serde::de::Error::custom),
            // Owned (String) bo'lsa: tayyor String xotirasi TryFrom<String> ga uzatiladi
            Cow::Owned(owned) => Self::try_from(owned).map_err(serde::de::Error::custom),
        }
    }
}

// ==========================================
// XATOLIKLAR ENUMI
// ==========================================

/// `BirthDate` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BirthDateError {
    /// Sana formati yoki qiymati noto'g'ri.
    #[error("invalid birth date format or value")]
    Date,

    /// Kelajak sanasi berilgan (tug'ilgan sana kelajakda bo'lishi mumkin emas).
    #[error("birth date cannot be in the future")]
    FutureDate,

    /// Sana juda uzoq o'tmishda (yil [`BirthDate::MIN_YEAR`] dan kichik).
    #[error(
        "birth date is too far in the past, year must be >= {}",
        BirthDate::MIN_YEAR
    )]
    TooOld,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_valid_birth_date() {
        let date = BirthDate::parse("1990-05-15").unwrap();
        assert_eq!(date.to_string(), "1990-05-15");
    }

    // 1-va 2-variantlar: Hyphen formatlar va ularning reverse state'i
    #[test]
    fn test_format_state_hyphen_reversal() {
        // YYYY-MM-DD formatidan o'qish
        let date = BirthDate::parse_with_format("1990-05-15", DateFormat::YmdHyphen).unwrap();

        // Uni DD-MM-YYYY ga o'tkazish
        let dmy = date.format_as(DateFormat::DmyHyphen);
        assert_eq!(dmy, "15-05-1990");

        // reversed() state yordamida teskarisiga o'girish (YmdHyphen -> DmyHyphen)
        let reversed_from_ymd = date.format_reversed(DateFormat::YmdHyphen);
        assert_eq!(reversed_from_ymd, "15-05-1990");

        // DD-MM-YYYY formatidan o'qib, uni YYYY-MM-DD ga reverse qilish
        let date_dmy = BirthDate::parse_with_format("15-05-1990", DateFormat::DmyHyphen).unwrap();
        let reversed_from_dmy = date_dmy.format_reversed(DateFormat::DmyHyphen);
        assert_eq!(reversed_from_dmy, "1990-05-15");
    }

    // 3-variant: Dot (nuqtali) formatlar va ularning reverse state'i
    #[test]
    fn test_format_state_dot_reversal() {
        // YYYY.MM.DD formatidan o'qish
        let date_ymd_dot = BirthDate::parse_with_format("1990.05.15", DateFormat::YmdDot).unwrap();

        // reversed() orqali DD.MM.YYYY ga o'tkazish
        assert_eq!(
            date_ymd_dot.format_reversed(DateFormat::YmdDot),
            "15.05.1990"
        );

        // DD.MM.YYYY formatidan o'qib, uni YYYY.MM.DD ga reverse qilish
        let date_dmy_dot = BirthDate::parse_with_format("15.05.1990", DateFormat::DmyDot).unwrap();
        assert_eq!(
            date_dmy_dot.format_reversed(DateFormat::DmyDot),
            "1990.05.15"
        );
    }

    #[test]
    fn parse_should_fail_for_invalid_format() {
        assert!(BirthDate::parse_with_format("15-05-1990", DateFormat::YmdHyphen).is_err());
    }

    #[test]
    fn birth_date_should_support_serde_roundtrip() {
        let date = BirthDate::parse("1990-05-15").unwrap();
        let json = serde_json::to_string(&date).unwrap();
        let restored: BirthDate = serde_json::from_str(&json).unwrap();
        assert_eq!(date, restored);
    }

    // --- Chegaralar ---

    #[test]
    fn parse_should_reject_dates_before_min_year() {
        assert!(matches!(
            BirthDate::parse("0001-01-01").unwrap_err(),
            TypeError::BirthDate(BirthDateError::TooOld)
        ));
        assert!(matches!(
            BirthDate::parse("1899-12-31").unwrap_err(),
            TypeError::BirthDate(BirthDateError::TooOld)
        ));
        // Chegaraning o'zi qabul qilinadi
        assert!(BirthDate::parse("1900-01-01").is_ok());
    }

    #[test]
    fn parse_should_reject_future_dates() {
        assert!(matches!(
            BirthDate::parse("3000-01-01").unwrap_err(),
            TypeError::BirthDate(BirthDateError::FutureDate)
        ));
    }

    #[test]
    fn today_and_tomorrow_utc_should_be_accepted() {
        let today = NaiveDate::from_ymd_opt(2026, 8, 14).unwrap();

        // UTC bo'yicha bugun — albatta qabul qilinadi
        assert!(BirthDate::from_naive_date_with_today(today, today).is_ok());

        // UTC+5 (Toshkent) da kun allaqachon almashgan bo'lishi mumkin —
        // ertangi UTC sanasi ham qabul qilinishi kerak
        let tomorrow = today.succ_opt().unwrap();
        assert!(BirthDate::from_naive_date_with_today(tomorrow, today).is_ok());

        // Ikki kundan keyingisi esa yo'q
        let day_after = tomorrow.succ_opt().unwrap();
        assert!(matches!(
            BirthDate::from_naive_date_with_today(day_after, today).unwrap_err(),
            TypeError::BirthDate(BirthDateError::FutureDate)
        ));
    }

    // --- Yosh hisoblash ---

    #[test]
    fn age_at_should_account_for_upcoming_birthday() {
        let date = BirthDate::parse("1990-05-15").unwrap();

        // Tug'ilgan kundan keyin
        let after = NaiveDate::from_ymd_opt(2026, 5, 16).unwrap();
        assert_eq!(date.age_at(after), 36);

        // Aynan tug'ilgan kunda
        let on = NaiveDate::from_ymd_opt(2026, 5, 15).unwrap();
        assert_eq!(date.age_at(on), 36);

        // Tug'ilgan kundan oldin — bir yil kam
        let before = NaiveDate::from_ymd_opt(2026, 5, 14).unwrap();
        assert_eq!(date.age_at(before), 35);
    }

    #[test]
    fn age_at_should_handle_leap_day_birthday() {
        let date = BirthDate::parse("2000-02-29").unwrap();

        // Kabisa bo'lmagan yilda 28-fevral — tug'ilgan kun hali kelmagan
        let feb_28 = NaiveDate::from_ymd_opt(2025, 2, 28).unwrap();
        assert_eq!(date.age_at(feb_28), 24);

        // 1-mart — o'tgan hisoblanadi
        let mar_1 = NaiveDate::from_ymd_opt(2025, 3, 1).unwrap();
        assert_eq!(date.age_at(mar_1), 25);
    }

    #[test]
    fn age_at_should_return_zero_for_dates_before_birth() {
        let date = BirthDate::parse("2020-06-01").unwrap();
        let earlier = NaiveDate::from_ymd_opt(2019, 1, 1).unwrap();
        assert_eq!(date.age_at(earlier), 0);
        assert_eq!(date.age_at(date.into_inner()), 0);
    }

    #[test]
    fn age_should_be_consistent_with_age_at_today() {
        let date = BirthDate::parse("1990-05-15").unwrap();
        assert_eq!(date.age(), date.age_at(Utc::now().date_naive()));
    }
}
