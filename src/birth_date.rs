use chrono::{Datelike, NaiveDate, Utc};

/// Sana formatlari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DateFormat {
    /// `YYYY-MM-DD`
    YmdHyphen,
    /// `DD-MM-YYYY`
    DmyHyphen,
    /// `YYYY.MM.DD`
    YmdDot,
    /// `DD.MM.YYYY`
    DmyDot,
}

impl DateFormat {
    /// `strftime` shabloni (chrono uchun).
    #[must_use]
    pub const fn pattern(self) -> &'static str {
        match self {
            Self::YmdHyphen => "%Y-%m-%d",
            Self::DmyHyphen => "%d-%m-%Y",
            Self::YmdDot => "%Y.%m.%d",
            Self::DmyDot => "%d.%m.%Y",
        }
    }

    /// Kun/yil tartibini teskarisiga o'giradi.
    #[must_use]
    pub const fn reversed(self) -> Self {
        match self {
            Self::YmdHyphen => Self::DmyHyphen,
            Self::DmyHyphen => Self::YmdHyphen,
            Self::YmdDot => Self::DmyDot,
            Self::DmyDot => Self::YmdDot,
        }
    }
}

/// Tug'ilgan sana: `MIN_YEAR` dan keyin va (yaratilish paytida) kelajakda emas.
///
/// Kelajak tekshiruvi **monoton**: bir marta qabul qilingan sana keyinchalik hech
/// qachon rad etilmaydi — shuning uchun replay/`Deserialize` uchun xavfsiz.
/// Deterministik testlar uchun `*_at(…, today)` variantlari bor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct BirthDate(NaiveDate);

impl BirthDate {
    /// Standart format.
    pub const DEFAULT_FORMAT: DateFormat = DateFormat::YmdHyphen;
    /// Eng erta qabul qilinadigan yil.
    pub const MIN_YEAR: i32 = 1900;

    /// `YYYY-MM-DD` dan (bugun = UTC).
    pub fn parse(value: &str) -> Result<Self, BirthDateError> {
        Self::parse_with_format(value, Self::DEFAULT_FORMAT)
    }

    /// Berilgan formatdan (bugun = UTC).
    pub fn parse_with_format(value: &str, format: DateFormat) -> Result<Self, BirthDateError> {
        Self::parse_with_format_at(value, format, Self::today_utc())
    }

    /// Deterministik variant: "bugun" tashqaridan beriladi.
    pub fn parse_with_format_at(
        value: &str,
        format: DateFormat,
        today: NaiveDate,
    ) -> Result<Self, BirthDateError> {
        let date = NaiveDate::parse_from_str(value.trim(), format.pattern())
            .map_err(|_| BirthDateError::Date)?;
        Self::from_naive_date_at(date, today)
    }

    /// `NaiveDate` dan (bugun = UTC).
    pub fn from_naive_date(date: NaiveDate) -> Result<Self, BirthDateError> {
        Self::from_naive_date_at(date, Self::today_utc())
    }

    /// `NaiveDate` dan, deterministik. Yuqori chegara: `today + 1` (UTC+14 gacha yon berish).
    pub fn from_naive_date_at(date: NaiveDate, today: NaiveDate) -> Result<Self, BirthDateError> {
        if date.year() < Self::MIN_YEAR {
            return Err(BirthDateError::TooOld);
        }
        if date > today.succ_opt().unwrap_or(today) {
            return Err(BirthDateError::FutureDate);
        }
        Ok(Self(date))
    }

    /// Soatga murojaat qilinadigan **yagona** joy.
    fn today_utc() -> NaiveDate {
        Utc::now().date_naive()
    }

    /// Ichki `NaiveDate`.
    #[inline]
    #[must_use]
    pub fn as_naive_date(&self) -> NaiveDate {
        self.0
    }

    /// Formatlangan matn.
    #[must_use]
    pub fn format_as(&self, format: DateFormat) -> String {
        self.0.format(format.pattern()).to_string()
    }

    /// Yil / oy / kun.
    #[must_use]
    pub fn year(&self) -> i32 {
        self.0.year()
    }
    /// Oy (1..=12).
    #[must_use]
    pub fn month(&self) -> u32 {
        self.0.month()
    }
    /// Kun (1..=31).
    #[must_use]
    pub fn day(&self) -> u32 {
        self.0.day()
    }

    /// Bugungi (UTC) to'liq yosh.
    #[must_use]
    pub fn age(&self) -> u32 {
        self.age_at(Self::today_utc())
    }

    /// Berilgan sanadagi to'liq yosh (deterministik).
    #[must_use]
    pub fn age_at(&self, date: NaiveDate) -> u32 {
        if date <= self.0 {
            return 0;
        }
        let mut years = date.year() - self.0.year();
        if (date.month(), date.day()) < (self.0.month(), self.0.day()) {
            years -= 1;
        }
        u32::try_from(years).unwrap_or(0)
    }
}

impl core::fmt::Display for BirthDate {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.0.format(Self::DEFAULT_FORMAT.pattern()))
    }
}

impl core::str::FromStr for BirthDate {
    type Err = BirthDateError;
    fn from_str(s: &str) -> Result<Self, BirthDateError> {
        Self::parse(s)
    }
}

impl TryFrom<&str> for BirthDate {
    type Error = BirthDateError;
    fn try_from(value: &str) -> Result<Self, BirthDateError> {
        Self::parse(value)
    }
}

impl TryFrom<String> for BirthDate {
    type Error = BirthDateError;
    fn try_from(value: String) -> Result<Self, BirthDateError> {
        Self::parse(&value)
    }
}

impl TryFrom<NaiveDate> for BirthDate {
    type Error = BirthDateError;
    fn try_from(value: NaiveDate) -> Result<Self, BirthDateError> {
        Self::from_naive_date(value)
    }
}

impl From<BirthDate> for NaiveDate {
    fn from(value: BirthDate) -> Self {
        value.0
    }
}

impl AsRef<NaiveDate> for BirthDate {
    fn as_ref(&self) -> &NaiveDate {
        &self.0
    }
}

#[cfg(feature = "serde")]
impl serde::Serialize for BirthDate {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.collect_str(self)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for BirthDate {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        crate::serde_support::deserialize_string_newtype(
            deserializer,
            "a birth date in YYYY-MM-DD format",
        )
    }
}

#[cfg(feature = "sqlx")]
crate::sqlx_support::sqlx_via!(
    BirthDate,
    NaiveDate,
    |d: NaiveDate| BirthDate::from_naive_date(d),
    |this: &BirthDate| &this.0
);

/// `BirthDate` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum BirthDateError {
    /// Format yoki qiymat noto'g'ri.
    #[error("invalid birth date format or value")]
    Date,
    /// Kelajak sanasi.
    #[error("birth date cannot be in the future")]
    FutureDate,
    /// `MIN_YEAR` dan oldin.
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
    fn deterministic_boundaries() {
        let today = NaiveDate::from_ymd_opt(2026, 9, 3).unwrap();
        assert!(BirthDate::from_naive_date_at(today.succ_opt().unwrap(), today).is_ok());
        assert_eq!(
            BirthDate::parse_with_format_at("2026-09-05", DateFormat::YmdHyphen, today),
            Err(BirthDateError::FutureDate)
        );
        assert_eq!(BirthDate::parse("1899-12-31"), Err(BirthDateError::TooOld));
        assert_eq!(BirthDate::parse("15.05.1990"), Err(BirthDateError::Date));
        assert_eq!(BirthDate::parse("1990-05-15").unwrap().age_at(today), 36);
    }
}
