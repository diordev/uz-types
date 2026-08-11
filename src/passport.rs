use serde::{self, Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

use crate::error::TypeError;

/// O'zbekiston pasport seriyasi va raqami.
///
/// Format: 2 ta lotin harfi + 7 ta raqam. Misol: `AA1234567`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Passport(String);

impl Passport {
    /// Seriya uzunligi (harflar soni).
    pub const SERIES_LEN: usize = 2;

    /// Raqam uzunligi (sonlar soni).
    pub const NUMBER_LEN: usize = 7;

    /// Umumiy uzunlik: seriya + raqam.
    pub const LEN: usize = Self::SERIES_LEN + Self::NUMBER_LEN;

    /// Ichki validatsiya logikasi (Xotira ajratishga ta'sir qilmaydi).
    /// Faqat formatni tekshiradi, uppercase yoki trim amallarini bajarmaydi.
    fn validate(raw: &str) -> Result<(), PassportError> {
        if raw.len() != Self::LEN {
            return Err(PassportError::Length);
        }

        // ASCII tekshiruvi split_at dan OLDIN kerak: aks holda ko'p baytli
        // UTF-8 belgi bo'lsa (lekin umumiy bayt uzunligi mos kelib qolsa),
        // split_at belgi chegarasidan tashqarida panic qiladi.
        if !raw.is_ascii() {
            return Err(PassportError::Format);
        }

        let (series, number) = raw.split_at(Self::SERIES_LEN);

        if !series.bytes().all(|b| b.is_ascii_alphabetic())
            || !number.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(PassportError::Format);
        }

        Ok(())
    }

    /// `&str` yoki `String` kabi qiymatlardan  `Passport` yaratadi: trim qiladi, seriyani uppercase qiladi,
    /// formatni tekshiradi.
    #[inline]
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        let raw = value.as_ref().trim();

        // Trim qilingan qiymatni validatsiya qilamiz.
        // `validate` da faqat lotin harf/raqam ekani tekshiriladi (katta/kichik harfligi ahamiyatsiz)
        Self::validate(raw)?; // TypeError ga o'tishi uchun `?` ishlatiladi.

        // Agar u allaqachon ASCII bo'lsa (validate dan o'tdi), uni bitta marta uppercase qilamiz.
        // Bu (series + number) ni alohida qo'shishdan ko'ra bitta allocation kam.
        Ok(Self(raw.to_ascii_uppercase()))
    }

    /// To'liq pasport qiymatini qaytaradi.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Pasport seriyasini qaytaradi.
    #[inline]
    #[must_use]
    pub fn series(&self) -> &str {
        &self.0[..Self::SERIES_LEN]
    }

    /// Pasport raqamini qaytaradi.
    #[inline]
    #[must_use]
    pub fn number(&self) -> &str {
        &self.0[Self::SERIES_LEN..]
    }

    /// Ichki `String` qiymatni qaytaradi (ownership `Passport`dan ko'chadi).
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

// ==========================================
// DEFAULT TRAITLAR
// ==========================================

/// `Deref` tufayli `Passport` obyektida `String`/`&str` metodlarini to'g'ridan-to'g'ri chaqirish.
impl Deref for Passport {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `Passport`ni `&str` sifatida ishlatish imkonini beradi.
impl AsRef<str> for Passport {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// `Passport`ni string ko'rinishida chiqaradi.
impl std::fmt::Display for Passport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// `&str` dan `Passport` yaratish.
impl TryFrom<&str> for Passport {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, TypeError> {
        Self::parse(value)
    }
}

/// `String` dan `Passport` yaratish (Xotirani optimallashtirish bilan).
impl TryFrom<String> for Passport {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, TypeError> {
        // Agar string uzunligi aniq 9 ta bo'lsa, probellarsiz kelsa va faqat ASCII belgilardan iborat bo'lsa:
        if value.len() == Passport::LEN && value.trim().len() == Passport::LEN && value.is_ascii() {
            // Endi is_ascii bo'lgani uchun indexing [..SERIES_LEN] xavfsiz (panic bo'lmaydi)
            let is_upper = value[..Passport::SERIES_LEN]
                .bytes()
                .all(|b| b.is_ascii_uppercase());

            if is_upper {
                // Agar allaqachon KATTA harflarda bo'lsa, tayyor xotirani (zero-allocation) qayta ishlatamiz!
                Passport::validate(&value)?;
                return Ok(Self(value));
            }
        }

        // Aks holda (masalan kichik harflar, bo'sh joylar bo'lsa), parse() orqali tozalab yangi xotiraga yozamiz.
        Self::parse(value)
    }
}

/// `Passport`ni `String`ga o'tkazadi (ownership ko'chadi, nusxa olinmaydi).
impl From<Passport> for String {
    fn from(value: Passport) -> Self {
        value.into_inner()
    }
}

// ==========================================
// SERDE OPTIMIZATSIYASI
// ==========================================

/// Serializatsiya paytida `.clone()` olinishining oldini olish uchun manual implementatsiya.
impl Serialize for Passport {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str()) // Nusxa (clone) olmaydi!
    }
}

/// JSON'dan o'qish jarayonida tayyor `String` xotirasini qayta ishlash uchun.
#[allow(unknown_lints)]
impl<'de> Deserialize<'de> for Passport {
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

/// `Passport` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum PassportError {
    /// Pasport uzunligi noto'g'ri (9 ta belgi bo'lishi kerak).
    #[error("passport length is invalid, expected {} characters", Passport::LEN)]
    Length,

    /// Pasport formati noto'g'ri (2 ta lotin harfi + 7 ta raqam emas).
    #[error("passport format is invalid, expected 2 letters followed by 7 digits")]
    Format,
}

#[cfg(test)]
mod tests {
    use super::*;

    // 1. Asosiy parse qilinishi
    #[test]
    fn parse_should_create_valid_passport() {
        let passport = Passport::parse("AA1234567").unwrap();
        assert_eq!(passport.as_str(), "AA1234567");
    }

    // 2. Kichik harflarni avtomatik KATTA harfga o'tkazishi
    #[test]
    fn parse_should_convert_series_to_uppercase() {
        let passport = Passport::parse("ab1234567").unwrap();
        assert_eq!(passport.as_str(), "AB1234567");
    }

    // 3. Boshidagi va oxiridagi bo'sh joylar trim qilinishi
    #[test]
    fn parse_should_trim_whitespace() {
        let passport = Passport::parse("  AA1234567  ").unwrap();
        assert_eq!(passport.as_str(), "AA1234567");
    }

    // 4. series() va number() helper metodlari
    #[test]
    fn should_return_series_and_number() {
        let passport = Passport::parse("AA1234567").unwrap();
        assert_eq!(passport.series(), "AA");
        assert_eq!(passport.number(), "1234567");
    }

    // 5. Uzunlik xatolari
    #[test]
    fn parse_should_fail_when_length_is_invalid() {
        let result = Passport::parse("AA123");
        assert!(matches!(
            result.unwrap_err(),
            TypeError::Passport(PassportError::Length)
        ));
    }

    // 6. Format xatolari (Lotin harfi o'rnida raqam kelsa)
    #[test]
    fn parse_should_fail_when_series_contains_numbers() {
        let result = Passport::parse("A11234567");
        assert!(matches!(
            result.unwrap_err(),
            TypeError::Passport(PassportError::Format)
        ));
    }

    // 7. Format xatolari (Raqam o'rnida harf kelsa)
    #[test]
    fn parse_should_fail_when_number_contains_letters() {
        let result = Passport::parse("AA12345AB");
        assert!(matches!(
            result.unwrap_err(),
            TypeError::Passport(PassportError::Format)
        ));
    }

    // 8. Panic-safe: UTF-8 belgilari split_at ni buza olmasligi kerak
    #[test]
    fn parse_should_not_panic_on_multibyte_utf8_of_same_byte_length() {
        // "a" (1 bayt) + "Ä" (2 bayt) + "234567" (6 bayt) = Jami 9 bayt bo'ladi
        let result = Passport::parse("a\u{00C4}234567");

        assert!(matches!(
            result.unwrap_err(),
            TypeError::Passport(PassportError::Format)
        ));
    }

    // 9. Deref, AsRef va Display traitlari
    #[test]
    fn display_and_deref_should_work() {
        let passport = Passport::parse("AA1234567").unwrap();

        assert_eq!(format!("{}", passport), "AA1234567");
        assert_eq!(passport.as_ref(), "AA1234567");
        // Deref yordamida to'g'ridan to'g'ri string metodini ishlatish:
        assert!(passport.starts_with("AA"));
    }

    // 10. TryFrom<&str> va TryFrom<String> (Zero-allocation tekshiruvi)
    #[test]
    fn test_try_from_conversions() {
        let passport_str = Passport::try_from("AA1234567").unwrap();
        assert_eq!(passport_str.as_str(), "AA1234567");

        // String (Katta harfda, zero allocation yuz beradi)
        let s1 = String::from("AA1234567");
        let passport_string1 = Passport::try_from(s1).unwrap();
        assert_eq!(passport_string1.as_str(), "AA1234567");

        // String (Kichik harfda, parse orqali upper'ga o'tkazadi)
        let s2 = String::from("ab1234567");
        let passport_string2 = Passport::try_from(s2).unwrap();
        assert_eq!(passport_string2.as_str(), "AB1234567");
    }

    // 11. Into<String> ishlashi
    #[test]
    fn passport_should_convert_into_string() {
        let passport = Passport::parse("AA1234567").unwrap();
        let value: String = passport.into();
        assert_eq!(value, "AA1234567");
    }

    // 12. Serde manual implementation tekshiruvi (Roundtrip)
    #[test]
    fn serde_should_support_roundtrip() {
        let passport = Passport::parse("AA1234567").unwrap();

        // Serilizatsiya qilishda to'g'ri string qaytarishi kerak
        let json = serde_json::to_string(&passport).unwrap();
        assert_eq!(json, "\"AA1234567\"");

        // Deserilizatsiya qilinganda qayta ob'ekt yasalishi kerak
        let restored: Passport = serde_json::from_str(&json).unwrap();
        assert_eq!(passport, restored);
    }
}
