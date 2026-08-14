use serde::{self, Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

use crate::error::TypeError;

/// Jismoniy shaxsning Yagona Identifikatsiya Raqami (PINFL).
///
/// O'zbekiston fuqarolarining 14 xonali identifikatsiya raqami.
///
/// # Format
///
/// - Aynan **14 ta** ASCII raqam
/// - Harflar, bo'shliqlar yoki maxsus belgilar qabul qilinmaydi
/// - Kirish: `12345678901234`
///
/// # ⚠️ Tekshiruv doirasi
///
/// **Faqat format tekshiriladi** — uzunlik va raqamlardan iboratlik.
/// Quyidagilar tekshirilmaydi:
///
/// - nazorat summasi (checksum, 14-raqam);
/// - 1-raqamdagi jins/asr belgisi;
/// - 2–7 raqamlardagi tug'ilgan sana strukturasi.
///
/// Shu sababli `00000000000000` kabi mavjud bo'lmagan raqam ham qabul
/// qilinadi. Agar sizga haqiqiy PINFL kafolati kerak bo'lsa, uni davlat
/// xizmati (masalan `my.gov.uz` yoki idoraviy API) orqali alohida
/// tasdiqlashingiz zarur.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pinfl(String);

impl Pinfl {
    /// PINFL uzunligi — aynan 14 ta raqam.
    pub const LEN: usize = 14;

    /// Ichki validatsiya logikasi (Xotira ajratmaydi).
    fn validate(pinfl: &str) -> Result<(), PinflError> {
        if pinfl.len() != Self::LEN {
            return Err(PinflError::Length);
        }
        if !pinfl.bytes().all(|b| b.is_ascii_digit()) {
            return Err(PinflError::Format);
        }
        Ok(())
    }

    /// `&str` yoki `String` kabi qiymatlardan `Pinfl` yaratadi:
    /// trim qiladi va formatni tekshiradi.
    ///
    /// # Xatolar
    ///
    /// [`PinflError`] qaytaradi agar:
    /// - Uzunlik [`Self::LEN`] ga teng bo'lmasa;
    /// - Raqamdan boshqa belgi uchrasa.
    #[inline]
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        let pinfl = value.as_ref().trim();

        Self::validate(pinfl)?; // `?` operatori avtomatik .into() ni chaqiradi

        Ok(Self(pinfl.to_owned()))
    }

    /// Pinfl qiymatini `&str` sifatida qaytaradi.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Ichki String qiymatni qaytaradi (Ownership ko'chadi).
    #[inline]
    #[must_use]
    pub fn into_inner(self) -> String {
        self.0
    }
}

// ==========================================
// DEFAULT TRAITLAR
// ==========================================

/// `Deref` tufayli `Pinfl` obyektida `String`/`&str` metodlarini (masalan, .starts_with(), .chars())
/// to'g'ridan-to'g'ri chaqirish mumkin bo'ladi.
impl Deref for Pinfl {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Pinfl qiymatini `&str` sifatida ishlatish imkonini beradi.
impl AsRef<str> for Pinfl {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Pinfl qiymatini string ko'rinishida chiqaradi.
impl std::fmt::Display for Pinfl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// Rust'ning standart idiomatik parse uslubi (`"12345678901234".parse::<Pinfl>()`).
impl std::str::FromStr for Pinfl {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
    }
}

/// `&str` dan `Pinfl` yaratish.
impl TryFrom<&str> for Pinfl {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, TypeError> {
        Self::parse(value)
    }
}

/// `String` dan `Pinfl` yaratish.
impl TryFrom<String> for Pinfl {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, TypeError> {
        let trimmed_len = value.trim().len();

        // Agar String ichida bo'sh joy bo'lmasa, XOTIRANI QAYTA ISHLATAMIZ (Zero-allocation)
        if trimmed_len == value.len() {
            Self::validate(&value)?;
            Ok(Self(value))
        } else {
            // Agar bo'sh joylar bo'lsa, trim qilib yangitdan parse qilamiz
            Self::parse(value.trim())
        }
    }
}

/// `Pinfl` ni `String` ga o'tkazadi (Ownership ko'chadi).
impl From<Pinfl> for String {
    fn from(value: Pinfl) -> Self {
        value.into_inner()
    }
}

// ==========================================
// SERDE OPTIMIZATSIYASI
// ==========================================

/// Serializatsiya paytida `.clone()` olinishining oldini olish uchun manual implementatsiya.
impl Serialize for Pinfl {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str()) // Nusxa (clone) olmaydi!
    }
}

/// JSON'dan o'qish jarayonida zero-copy (`&str`) va zarur hollarda `String` xotirasini qayta ishlash uchun.
impl<'de> Deserialize<'de> for Pinfl {
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

/// `Pinfl` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum PinflError {
    /// PINFL uzunligi noto'g'ri (14 ta raqam bo'lishi kerak).
    #[error("pinfl length is invalid, expected 14 digits")]
    Length,

    /// PINFL formati noto'g'ri (faqat raqamlardan iborat bo'lishi kerak).
    #[error("pinfl format is incorrect, expected only digits")]
    Format,
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_PINFL: &str = "12345678901234";

    // 1. To'g'ri PINFL parse qilinishi
    #[test]
    fn test_valid_pinfl_parse() {
        let pinfl = Pinfl::parse(VALID_PINFL).expect("Valid PINFL parse bo'lishi kerak");
        assert_eq!(pinfl.as_str(), VALID_PINFL);
    }

    // 2. Boshidagi va oxiridagi bo'sh joylar (whitespace) trim qilinishi
    #[test]
    fn test_pinfl_trim_whitespace() {
        let input = "  12345678901234 \n\t";
        let pinfl = Pinfl::parse(input).expect("Probel bo'lsa ham parse bo'lishi kerak");
        assert_eq!(pinfl.as_str(), VALID_PINFL);
    }

    // 3. Uzunlik xatolari (14 ta raqamdan kam yoki ko'p)
    #[test]
    fn test_invalid_length() {
        // Qisqa (13 ta)
        assert!(Pinfl::parse("1234567890123").is_err());

        // Uzun (15 ta)
        assert!(Pinfl::parse("123456789012345").is_err());

        // Bo'sh string
        assert!(Pinfl::parse("").is_err());
    }

    // 4. Format xatolari (Raqam bo'lmagan belgilar)
    #[test]
    fn test_invalid_format() {
        // Harf qatnashgan
        assert!(Pinfl::parse("1234567890123a").is_err());

        // Maxsus belgi qatnashgan
        assert!(Pinfl::parse("123456789012-4").is_err());

        // O'rtasida bo'sh joy bor
        assert!(Pinfl::parse("12345 67890123").is_err());
    }

    // 5. Deref va AsRef traitlari ishlashi
    #[test]
    fn test_deref_and_as_ref() {
        let pinfl = Pinfl::parse(VALID_PINFL).unwrap();

        // Deref tufayli &str metodlarini to'g'ridan-to'g'ri chaqirish
        assert_eq!(pinfl.len(), 14);
        assert!(pinfl.starts_with("123"));

        // AsRef ishlashi
        assert_eq!(pinfl.as_ref(), VALID_PINFL);
    }

    // 6. TryFrom<&str> va TryFrom<String>
    #[test]
    fn test_try_from_conversions() {
        // &str -> Pinfl
        let pinfl1 = Pinfl::try_from(VALID_PINFL).unwrap();
        assert_eq!(pinfl1.as_str(), VALID_PINFL);

        // String -> Pinfl (Zero-allocation yo'li)
        let s = String::from(VALID_PINFL);
        let pinfl2 = Pinfl::try_from(s).unwrap();
        assert_eq!(pinfl2.as_str(), VALID_PINFL);

        // String -> Pinfl (Trim qilinadigan yo'li)
        let s_padded = String::from("  12345678901234  ");
        let pinfl3 = Pinfl::try_from(s_padded).unwrap();
        assert_eq!(pinfl3.as_str(), VALID_PINFL);
    }

    // 7. From<Pinfl> for String va Display
    #[test]
    fn test_display_and_into_string() {
        let pinfl = Pinfl::parse(VALID_PINFL).unwrap();

        // Display trait
        assert_eq!(format!("{}", pinfl), VALID_PINFL);

        // Into<String>
        let s: String = pinfl.into();
        assert_eq!(s, VALID_PINFL);
    }

    // 8. Serde (Serialize / Deserialize) mosligi
    #[test]
    fn test_serde_serialization() {
        let pinfl = Pinfl::parse(VALID_PINFL).unwrap();

        // JSON ga o'girish (Serializatsiya)
        let json = serde_json::to_string(&pinfl).unwrap();
        assert_eq!(json, format!("\"{}\"", VALID_PINFL));

        // JSON dan o'qish (To'g'ri qiymat)
        let deserialized: Pinfl = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, pinfl);

        // JSON dan o'qish (Noto'g'ri qiymat deserialization xatosini berishi kerak)
        let invalid_json = "\"12345\"";
        let res: Result<Pinfl, _> = serde_json::from_str(invalid_json);
        assert!(res.is_err());
    }
}
