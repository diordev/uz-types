use crate::error::TypeError;
use serde::{Deserialize, Serialize};
use std::ops::Deref;

/// O'zbekiston telefon raqami.
///
/// Ichki saqlashda faqat raqamlar saqlanadi (`+` olib tashlanadi).
/// Bu normalizatsiya API ga yuborishda izchillikni ta'minlaydi.
///
/// # Format
///
/// - Kirish: `+998901234567` yoki `998901234567`
/// - Ichki saqlash: `998901234567` (har doim 12 raqam)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhoneNumber(String);

impl PhoneNumber {
    /// Telefon raqam uzunligi (`+` siz) — aynan 12 raqam.
    pub const DIGIT_LEN: usize = 12;

    /// O'zbekiston telefon raqam kodi prefiksi (`+` siz).
    pub const COUNTRY_CODE: &str = "998";

    /// Ichki validatsiya logikasi (Xotira ajratishga ta'sir qilmaydi)
    fn validate(phone: &str) -> Result<(), PhoneNumberError> {
        // phone uzunligi 12 ekanligini tekshiradi
        if phone.len() != Self::DIGIT_LEN {
            return Err(PhoneNumberError::Length);
        }

        // phone barchasi ASCII belgi ichidagi raqamlarni tekshiradi
        if !phone.bytes().all(|b| b.is_ascii_digit()) {
            return Err(PhoneNumberError::Format);
        }

        // phone boshlanishi 998 dan boshlangami tekshiradi
        if !phone.starts_with(Self::COUNTRY_CODE) {
            return Err(PhoneNumberError::Prefix);
        }
        Ok(())
    }

    /// `&str` yoki `String` kabi qiymatlardan `PhoneNumber` yaratadi.
    ///
    /// Agar boshidagi `+` belgisi bo'lsahma qabul qilinadi va olib tashlanadi.
    ///
    /// # Xatolar
    ///
    /// [`PhoneNumberError`] qaytaradi agar:
    /// - Raqamlar soni 12 ta bo'lmasa
    /// - Prefiks `998` bo'lmasa yoki raqamdan boshqa belgi mavjud bo'lsa
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        let raw = value.as_ref().trim();
        let digits = raw.strip_prefix('+').unwrap_or(raw);

        Self::validate(digits)?; // `?` operatori avtomatik .into() ni chaqiradi

        Ok(Self(digits.to_owned()))
    }

    /// Raqamni `&str` sifatida qaytaradi (`+` siz, faqat 12 raqam).
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Raqamni `+` bilan qaytaradi type=String (masalan: `+998901234567`).
    #[inline]
    pub fn to_international(&self) -> String {
        format!("+{}", self.0)
    }

    /// Ichki String qiymatni qaytaradi (Ownership ko'chadi).
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}
// ==========================================
// DEFAULT TRAITLAR
// ==========================================

/// `Deref` tufayli `PhoneNumber` obyektida `String`/`&str` metodlarini (masalan, .starts_with(), .chars())
/// to'g'ridan-to'g'ri chaqirish mumkin bo'ladi.
impl Deref for PhoneNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Raqam qiymatini `&str` sifatida ishlatish imkonini beradi.
impl AsRef<str> for PhoneNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Raqam qiymatini string ko'rinishida chiqaradi.
impl std::fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// `&str` dan `PhoneNumber` yaratish.
impl TryFrom<&str> for PhoneNumber {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, TypeError> {
        Self::parse(value)
    }
}

/// `String` dan `PhoneNumber` yaratish.
impl TryFrom<String> for PhoneNumber {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, TypeError> {
        // Agar matnda bosh/oxirida bo'sh joy yoki '+' belgisi bo'lsa,
        // to'g'ridan-to'g'ri parse() ga uzatamiz.
        if value.starts_with('+') || value.trim().len() != value.len() {
            Self::parse(value)
        } else {
            // Agar bo'sh joy va '+' bo'lmasa, XOTIRANI QAYTA ISHLATAMIZ (Zero-allocation)
            Self::validate(&value)?;
            Ok(Self(value))
        }
    }
}

/// `PhoneNumber` ni `String` ga o'tkazadi (Ownership ko'chadi).
impl From<PhoneNumber> for String {
    fn from(value: PhoneNumber) -> Self {
        value.into_inner()
    }
}

// ==========================================
// SERDE OPTIMIZATSIYASI
// ==========================================

/// Serializatsiya paytida `.clone()` olinishining oldini olish uchun manual implementatsiya.
impl Serialize for PhoneNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str()) // Nusxa (clone) olmaydi!
    }
}

/// JSON'dan o'qish jarayonida tayyor `String` xotirasini qayta ishlash uchun.
impl<'de> Deserialize<'de> for PhoneNumber {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}

/// `PhoneNumber` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum PhoneNumberError {
    /// Telefon raqami uzunligi noto'g'ri.
    #[error("phone number length is invalid")]
    Length,

    /// Telefon raqamida raqamdan boshqa belgi mavjud.
    #[error("phone number format is invalid")]
    Format,

    /// Telefon raqami O'zbekiston kodi bilan boshlanishi kerak.
    #[error("phone number must start with 998")]
    Prefix,
}

#[cfg(test)]
mod tests {
    use super::*;
    const PHONE_NUMBER_WITH_PLUS: &str = "+998901234567";

    const PHONE_NUMBER_WITHOUT_PLUS: &str = "998901234567";

    // 1. To'g'ri Phone number parse qilinishi
    #[test]
    fn test_valid_with_or_without_plus() {
        let phone_with_plus = PhoneNumber::parse(PHONE_NUMBER_WITH_PLUS).expect("valid phone");
        let phone_without_plus =
            PhoneNumber::parse(PHONE_NUMBER_WITHOUT_PLUS).expect("valid phone");

        assert_eq!(phone_with_plus.as_str(), PHONE_NUMBER_WITHOUT_PLUS);
        assert_eq!(phone_without_plus.as_str(), PHONE_NUMBER_WITHOUT_PLUS);

        // plus bilan qaytarish uchun
        assert_eq!(
            phone_without_plus.to_international().as_str(),
            PHONE_NUMBER_WITH_PLUS
        );
    }

    // 2. Boshidagi va oxiridagi bo'sh joylar (whitespace) trim qilinishi
    #[test]
    fn parse_should_trim_whitespace() {
        let input = "  998901234567 \n\t";
        let phone = PhoneNumber::parse(input).expect("Probel bo'lsa ham parse bo'lishi kerak");

        assert_eq!(phone.as_str(), PHONE_NUMBER_WITHOUT_PLUS);
    }

    // 3. Uzunlik xatolari (12 ta raqamdan kam yoki ko'p)
    #[test]
    fn test_invalid_length() {
        // Qisqa (11 ta)
        assert!(PhoneNumber::parse("99890123456").is_err());

        // Uzun (13 ta)
        assert!(PhoneNumber::parse("9989012345678").is_err());

        // Bo'sh string
        assert!(PhoneNumber::parse("").is_err());
    }

    // 4. Format xatolari (Raqam bo'lmagan belgilar yoki noto'g'ri prefix)
    #[test]
    fn test_invalid_format() {
        // Prefix noto'g'ri
        assert!(PhoneNumber::parse("997901234567").is_err());

        // Maxsus belgi qatnashgan
        assert!(PhoneNumber::parse("998a01234567").is_err());

        // O'rtasida bo'sh joy bor
        assert!(PhoneNumber::parse("998 901234567").is_err());
    }

    // 5. Deref va AsRef traitlari ishlashi
    #[test]
    fn test_deref_and_as_ref() {
        let phone = PhoneNumber::parse(PHONE_NUMBER_WITHOUT_PLUS).unwrap();

        // Deref tufayli &str metodlarini to'g'ridan-to'g'ri chaqirish
        assert_eq!(phone.len(), 12);
        assert!(phone.starts_with("998"));

        // AsRef ishlashi
        assert_eq!(phone.as_ref(), PHONE_NUMBER_WITHOUT_PLUS);
    }

    // 6. TryFrom<&str> va TryFrom<String>
    #[test]
    fn test_try_from_conversions() {
        // &str -> PhoneNumber
        let phone1 = PhoneNumber::try_from(PHONE_NUMBER_WITHOUT_PLUS).unwrap();
        assert_eq!(phone1.as_str(), PHONE_NUMBER_WITHOUT_PLUS);

        // String -> PhoneNumber (Zero-allocation yo'li)
        let s = String::from(PHONE_NUMBER_WITHOUT_PLUS);
        let phone2 = PhoneNumber::try_from(s).unwrap();
        assert_eq!(phone2.as_str(), PHONE_NUMBER_WITHOUT_PLUS);

        // String -> PhoneNumber (Trim qilinadigan yo'li)
        let s_padded = String::from("  998901234567  ");
        let phone3 = PhoneNumber::try_from(s_padded).unwrap();
        assert_eq!(phone3.as_str(), PHONE_NUMBER_WITHOUT_PLUS);
    }

    // 7. From<PhoneNumber> for String va Display
    #[test]
    fn test_display_and_into_string() {
        let phone = PhoneNumber::parse(PHONE_NUMBER_WITHOUT_PLUS).unwrap();

        // Display trait
        assert_eq!(format!("{}", phone), PHONE_NUMBER_WITHOUT_PLUS);

        // Into<String>
        let s: String = phone.into();
        assert_eq!(s, PHONE_NUMBER_WITHOUT_PLUS);
    }

    // 8. Serde (Serialize / Deserialize) mosligi
    #[test]
    fn test_serde_serialization() {
        let phone = PhoneNumber::parse(PHONE_NUMBER_WITHOUT_PLUS).unwrap();

        // JSON ga o'girish (Serializatsiya)
        let json = serde_json::to_string(&phone).unwrap();
        assert_eq!(json, format!("\"{}\"", PHONE_NUMBER_WITHOUT_PLUS));

        // JSON dan o'qish (To'g'ri qiymat)
        let deserialized: PhoneNumber = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, phone);

        // JSON dan o'qish (Noto'g'ri qiymat deserialization xatosini berishi kerak)
        let invalid_json = "\"997881234567\"";
        let res: Result<PhoneNumber, _> = serde_json::from_str(invalid_json);
        assert!(res.is_err());
    }

    // 9. Phone numberni + bilan qaytarish (Heapdan allocation bo'ladi)
    #[test]
    fn to_international_should_add_plus() {
        let phone = PhoneNumber::parse(PHONE_NUMBER_WITHOUT_PLUS).unwrap();
        assert_eq!(phone.to_international(), PHONE_NUMBER_WITH_PLUS);
    }
}
