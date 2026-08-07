use crate::error::TypeError;
use serde::{self, Deserialize, Serialize};
use std::ops::Deref;

/// Elektron pochta manzilini ifodalovchi value object.
///
/// Format: `local-part@domain.tld`
/// Barcha harflar kichik (lowercase) ga o'tkazilib, tekshiriladi va saqlanadi.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Email manzili minimal uzunligi (masalan: a@b.c)
    pub const MIN_LEN: usize = 5;

    /// Email manzili maksimal uzunligi (RFC 5321 ga ko'ra 254 ta belgi).
    pub const MAX_LEN: usize = 254;

    /// Ichki validatsiya logikasi (Xotira ajratishga ta'sir qilmaydi).
    /// Faqat formatni tekshiradi, lowercase yoki trim amallarini bajarmaydi.
    fn validate(raw: &str) -> Result<(), EmailAddressError> {
        if raw.len() < Self::MIN_LEN || raw.len() > Self::MAX_LEN {
            return Err(EmailAddressError::Length);
        }

        // Email ichida ASCII bo'lmagan belgilar bo'lmasligi kerak
        if !raw.is_ascii() || raw.contains(char::is_whitespace) {
            return Err(EmailAddressError::Format);
        }

        // Bitta va faqat bitta '@' belgisi bo'lishi kerak.
        let parts: Vec<&str> = raw.split('@').collect();
        if parts.len() != 2 {
            return Err(EmailAddressError::Format);
        }

        let local_part = parts[0];
        let domain_part = parts[1];

        // Local part validatsiyasi
        if local_part.is_empty() || local_part.starts_with('.') || local_part.ends_with('.') {
            return Err(EmailAddressError::Format);
        }

        // Domain part validatsiyasi (eng kamida x.y)
        if domain_part.len() < 3 || !domain_part.contains('.') {
            return Err(EmailAddressError::Format);
        }

        if domain_part.starts_with('.') || domain_part.ends_with('.') {
            return Err(EmailAddressError::Format);
        }

        Ok(())
    }

    /// `&str` yoki `String` kabi qiymatlardan `EmailAddress` yaratadi:
    /// trim qiladi, lowercase qiladi va formatni tekshiradi.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        let raw = value.as_ref().trim();

        // Trim qilingan qiymatni avval lowercase qilamiz, sababi validate formatni tekshiradi
        // Ascii lower case qilish bitta xotira ajratishni talab qiladi.
        let lowercased = raw.to_ascii_lowercase();

        Self::validate(&lowercased)?;

        Ok(Self(lowercased))
    }

    /// To'liq email manzilini qaytaradi.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Email'ning "@" dan oldingi qismini (local-part) qaytaradi.
    #[inline]
    pub fn local_part(&self) -> &str {
        let at_index = self.0.find('@').expect("Valid email must have @");
        &self.0[..at_index]
    }

    /// Email'ning "@" dan keyingi qismini (domain) qaytaradi.
    #[inline]
    pub fn domain(&self) -> &str {
        let at_index = self.0.find('@').expect("Valid email must have @");
        &self.0[at_index + 1..]
    }

    /// Ichki `String` qiymatni qaytaradi (ownership ko'chadi).
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}

// ==========================================
// DEFAULT TRAITLAR
// ==========================================

/// `Deref` orqali `EmailAddress` obyektida `String`/`&str` metodlarini to'g'ridan-to'g'ri chaqirish.
impl Deref for EmailAddress {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `EmailAddress`ni `&str` sifatida ishlatish imkonini beradi.
impl AsRef<str> for EmailAddress {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// `EmailAddress`ni string ko'rinishida chiqaradi.
impl std::fmt::Display for EmailAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// `&str` dan `EmailAddress` yaratish.
impl TryFrom<&str> for EmailAddress {
    type Error = TypeError;

    fn try_from(value: &str) -> Result<Self, TypeError> {
        Self::parse(value)
    }
}

/// `String` dan `EmailAddress` yaratish (Xotirani optimallashtirish bilan).
impl TryFrom<String> for EmailAddress {
    type Error = TypeError;

    fn try_from(value: String) -> Result<Self, TypeError> {
        let is_lowercase = value
            .bytes()
            .all(|b| b.is_ascii_lowercase() || !b.is_ascii_alphabetic());

        // Agar string tayyor bo'lsa (bo'sh joylarsiz va faqat kichik harflarda bo'lsa), xotirani qayta ishlatamiz.
        if value.trim().len() == value.len() && is_lowercase {
            EmailAddress::validate(&value)?;
            return Ok(Self(value));
        }

        Self::parse(value)
    }
}

/// `EmailAddress`ni `String`ga o'tkazadi (ownership ko'chadi, nusxa olinmaydi).
impl From<EmailAddress> for String {
    fn from(value: EmailAddress) -> Self {
        value.into_inner()
    }
}

// ==========================================
// SERDE OPTIMIZATSIYASI
// ==========================================
/// Serializatsiya paytida `.clone()` olinishining oldini olish uchun manual implementatsiya.
impl Serialize for EmailAddress {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// JSON'dan o'qish jarayonida tayyor `String` xotirasini qayta ishlash uchun.
#[allow(unknown_lints)]
impl<'de> Deserialize<'de> for EmailAddress {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::try_from(s).map_err(serde::de::Error::custom)
    }
}

// ==========================================
// XATOLIKLAR ENUMI
// ==========================================

/// `EmailAddress` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum EmailAddressError {
    /// Email uzunligi noto'g'ri (Juda qisqa yoki 254 belgidan uzun).
    #[error(
        "email length is invalid, must be between {} and {} characters",
        EmailAddress::MIN_LEN,
        EmailAddress::MAX_LEN
    )]
    Length,

    /// Email formati noto'g'ri (masalan: '@' yoki '.' belgisi yo'q, xato simvollar bor).
    #[error("email format is invalid, expected valid local-part and domain")]
    Format,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_should_create_valid_email() {
        let email = EmailAddress::parse("test@example.com").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn parse_should_convert_to_lowercase() {
        let email = EmailAddress::parse("TEST@EXAMPLE.COM").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn parse_should_trim_whitespace() {
        let email = EmailAddress::parse("  test@example.com  ").unwrap();
        assert_eq!(email.as_str(), "test@example.com");
    }

    #[test]
    fn should_return_local_part_and_domain() {
        let email = EmailAddress::parse("user.name@domain.com").unwrap();
        assert_eq!(email.local_part(), "user.name");
        assert_eq!(email.domain(), "domain.com");
    }

    #[test]
    fn parse_should_fail_on_invalid_length() {
        assert!(matches!(
            EmailAddress::parse("a@b").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Length)
        ));
    }

    #[test]
    fn parse_should_fail_when_missing_at_symbol() {
        assert!(matches!(
            EmailAddress::parse("test.example.com").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_fail_when_missing_domain() {
        assert!(matches!(
            EmailAddress::parse("test@").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_fail_when_domain_has_no_dot() {
        assert!(matches!(
            EmailAddress::parse("test@example").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_fail_on_invalid_dots() {
        assert!(matches!(
            EmailAddress::parse(".test@example.com").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
        assert!(matches!(
            EmailAddress::parse("test.@example.com").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
        assert!(matches!(
            EmailAddress::parse("test@.example.com").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
        assert!(matches!(
            EmailAddress::parse("test@example.com.").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn test_try_from_conversions() {
        // From &str
        let email_str = EmailAddress::try_from("user@test.com").unwrap();
        assert_eq!(email_str.as_str(), "user@test.com");

        // From String (Tayyor string, zero allocation yuz beradi)
        let s1 = String::from("user@test.com");
        let email_string1 = EmailAddress::try_from(s1).unwrap();
        assert_eq!(email_string1.as_str(), "user@test.com");

        // From String (Katta harflar bilan kelganda qayta almashtiradi)
        let s2 = String::from("USER@test.com");
        let email_string2 = EmailAddress::try_from(s2).unwrap();
        assert_eq!(email_string2.as_str(), "user@test.com");
    }

    #[test]
    fn email_should_support_serde_roundtrip() {
        let email = EmailAddress::parse("hello@world.com").unwrap();

        let json = serde_json::to_string(&email).unwrap();
        assert_eq!(json, "\"hello@world.com\"");

        let restored: EmailAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(email, restored);
    }
}
