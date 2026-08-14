use serde::{self, Deserialize, Serialize};
use std::borrow::Cow;
use std::ops::Deref;

use crate::error::TypeError;

/// Elektron pochta manzilini ifodalovchi value object.
///
/// Format: `local-part@domain.tld`
/// Barcha harflar kichik (lowercase) ga o'tkazilib, tekshiriladi va saqlanadi.
///
/// # Tekshiriladigan qoidalar
///
/// **Local-part** (`@` dan oldingi qism):
/// - bo'sh emas va [`Self::LOCAL_PART_MAX_LEN`] dan uzun emas;
/// - `.` bilan boshlanmaydi va tugamaydi, ketma-ket `..` bo'lmaydi;
/// - faqat ASCII harf/raqam va RFC 5322 ruxsat bergan belgilar (`!#$%&'*+-/=?^_`{|}~.`).
///
/// **Domain** (`@` dan keyingi qism):
/// - kamida ikkita label (`domain.tld`), har bir label bo'sh emas;
/// - label `-` bilan boshlanmaydi/tugamaydi va faqat `[a-z0-9-]` dan iborat;
/// - TLD kamida [`Self::TLD_MIN_LEN`] ta **harf** (raqam yoki `-` bo'lmaydi).
///
/// # Cheklovlar
///
/// Faqat ASCII manzillar qo'llab-quvvatlanadi — IDN (punycode'gacha bo'lgan
/// unicode domenlar), quoted local-part (`"a b"@x.com`) va IP-literal
/// (`a@[192.0.2.1]`) qabul qilinmaydi.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct EmailAddress(String);

impl EmailAddress {
    /// Email manzilining minimal uzunligi — arzon dastlabki tekshiruv uchun.
    ///
    /// Diqqat: bu faqat quyi chegara. Haqiqiy eng qisqa valid manzil
    /// `a@b.co` (6 belgi), chunki TLD kamida [`Self::TLD_MIN_LEN`] ta harf.
    pub const MIN_LEN: usize = 5;

    /// Email manzili maksimal uzunligi (RFC 5321 ga ko'ra 254 ta belgi).
    pub const MAX_LEN: usize = 254;

    /// Local-part (`@` dan oldingi qism) maksimal uzunligi (RFC 5321).
    pub const LOCAL_PART_MAX_LEN: usize = 64;

    /// Domain qismining maksimal uzunligi (RFC 1035).
    pub const DOMAIN_MAX_LEN: usize = 253;

    /// Domain'dagi bitta label maksimal uzunligi (RFC 1035).
    pub const DOMAIN_LABEL_MAX_LEN: usize = 63;

    /// TLD (oxirgi label) minimal uzunligi.
    pub const TLD_MIN_LEN: usize = 2;

    /// Ichki validatsiya logikasi (Xotira ajratmaydi).
    ///
    /// Faqat formatni tekshiradi — lowercase yoki trim amallarini bajarmaydi,
    /// shu sababli katta/kichik harfga bog'liq emas.
    fn validate(raw: &str) -> Result<(), EmailAddressError> {
        if raw.len() < Self::MIN_LEN || raw.len() > Self::MAX_LEN {
            return Err(EmailAddressError::Length);
        }

        // Email ichida ASCII bo'lmagan belgilar bo'lmasligi kerak
        if !raw.is_ascii() || raw.contains(char::is_whitespace) {
            return Err(EmailAddressError::Format);
        }

        // Bitta va faqat bitta '@' belgisi bo'lishi kerak.
        // `split_once` + qoldiqni tekshirish — `Vec` yig'ishdan farqli
        // o'laroq umuman xotira ajratmaydi.
        let Some((local_part, domain_part)) = raw.split_once('@') else {
            return Err(EmailAddressError::Format);
        };

        if domain_part.contains('@') {
            return Err(EmailAddressError::Format);
        }

        Self::validate_local_part(local_part)?;
        Self::validate_domain(domain_part)
    }

    /// RFC 5322 "atext" to'plamidagi belgi ekanini tekshiradi (`.` bundan mustasno
    /// — uning joylashuvi alohida tekshiriladi).
    #[inline]
    const fn is_allowed_local_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric()
            || matches!(
                b,
                b'.' | b'!'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'/'
                    | b'='
                    | b'?'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'{'
                    | b'|'
                    | b'}'
                    | b'~'
            )
    }

    /// `@` dan oldingi qismni tekshiradi.
    fn validate_local_part(local: &str) -> Result<(), EmailAddressError> {
        if local.is_empty() || local.len() > Self::LOCAL_PART_MAX_LEN {
            return Err(EmailAddressError::Format);
        }

        // `.` chekkalarda turolmaydi va ketma-ket kelolmaydi (`a..b@x.com`).
        if local.starts_with('.') || local.ends_with('.') || local.contains("..") {
            return Err(EmailAddressError::Format);
        }

        if !local.bytes().all(Self::is_allowed_local_byte) {
            return Err(EmailAddressError::Format);
        }

        Ok(())
    }

    /// `@` dan keyingi qismni tekshiradi.
    fn validate_domain(domain: &str) -> Result<(), EmailAddressError> {
        if domain.is_empty() || domain.len() > Self::DOMAIN_MAX_LEN {
            return Err(EmailAddressError::Format);
        }

        let mut label_count = 0usize;
        let mut tld = "";

        // Bo'sh label tekshiruvi bir vaqtning o'zida `a@.x.com`, `a@x.com.`
        // va `a@x..com` holatlarini ham qamrab oladi.
        for label in domain.split('.') {
            if label.is_empty() || label.len() > Self::DOMAIN_LABEL_MAX_LEN {
                return Err(EmailAddressError::Format);
            }

            if label.starts_with('-') || label.ends_with('-') {
                return Err(EmailAddressError::Format);
            }

            if !label
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            {
                return Err(EmailAddressError::Format);
            }

            label_count += 1;
            tld = label;
        }

        // Kamida `domain.tld` bo'lishi shart.
        if label_count < 2 {
            return Err(EmailAddressError::Format);
        }

        // TLD faqat harflardan iborat va yetarlicha uzun bo'lishi kerak.
        if tld.len() < Self::TLD_MIN_LEN || !tld.bytes().all(|b| b.is_ascii_alphabetic()) {
            return Err(EmailAddressError::Format);
        }

        Ok(())
    }

    /// `&str` yoki `String` kabi qiymatlardan `EmailAddress` yaratadi:
    /// trim qiladi, formatni tekshiradi va lowercase qilib saqlaydi.
    ///
    /// # Xatolar
    ///
    /// [`EmailAddressError`] qaytaradi agar:
    /// - Uzunlik [`Self::MIN_LEN`]–[`Self::MAX_LEN`] oralig'idan tashqarida bo'lsa;
    /// - Yuqoridagi local-part yoki domain qoidalaridan biri buzilsa.
    #[inline]
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        let raw = value.as_ref().trim();

        // Avval validatsiya — u xotira ajratmaydi va katta/kichik harfga
        // bog'liq emas. Shu sababli noto'g'ri input uchun umuman
        // allocation bo'lmaydi.
        Self::validate(raw)?;

        Ok(Self(raw.to_ascii_lowercase()))
    }

    /// To'liq email manzilini qaytaradi.
    #[inline]
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Email'ning "@" dan oldingi qismini (local-part) qaytaradi.
    #[inline]
    #[must_use]
    pub fn local_part(&self) -> &str {
        let at_index = self.0.find('@').expect("Valid email must have @");
        &self.0[..at_index]
    }

    /// Email'ning "@" dan keyingi qismini (domain) qaytaradi.
    #[inline]
    #[must_use]
    pub fn domain(&self) -> &str {
        let at_index = self.0.find('@').expect("Valid email must have @");
        &self.0[at_index + 1..]
    }

    /// Ichki `String` qiymatni qaytaradi (ownership ko'chadi).
    #[inline]
    #[must_use]
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

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// `EmailAddress`ni `&str` sifatida ishlatish imkonini beradi.
impl AsRef<str> for EmailAddress {
    #[inline]
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

/// Rust'ning standart idiomatik parse uslubi (`"a@b.com".parse::<EmailAddress>()`).
impl std::str::FromStr for EmailAddress {
    type Err = TypeError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse(s)
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
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// JSON'dan o'qish jarayonida tayyor `String` xotirasini qayta ishlash uchun.
impl<'de> Deserialize<'de> for EmailAddress {
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

    // --- 0.17.0 gacha noto'g'ri qabul qilingan holatlar (regression) ---

    #[test]
    fn parse_should_reject_empty_domain_label() {
        // `b..c` — o'rtada bo'sh label
        assert!(matches!(
            EmailAddress::parse("a@b..c").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_reject_domain_label_with_leading_or_trailing_hyphen() {
        assert!(matches!(
            EmailAddress::parse("a@-b.com").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
        assert!(matches!(
            EmailAddress::parse("a@b-.com").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_reject_consecutive_dots_in_local_part() {
        assert!(matches!(
            EmailAddress::parse("a..b@c.com").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_reject_too_short_or_non_alphabetic_tld() {
        // 1 harfli TLD
        assert!(matches!(
            EmailAddress::parse("a@b.c").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
        // Raqamli TLD
        assert!(matches!(
            EmailAddress::parse("a@b.c1").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_reject_multiple_at_symbols() {
        assert!(matches!(
            EmailAddress::parse("a@b@c.com").unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_enforce_local_part_length_limit() {
        let ok = format!(
            "{}@example.com",
            "a".repeat(EmailAddress::LOCAL_PART_MAX_LEN)
        );
        assert!(EmailAddress::parse(&ok).is_ok());

        let too_long = format!(
            "{}@example.com",
            "a".repeat(EmailAddress::LOCAL_PART_MAX_LEN + 1)
        );
        assert!(matches!(
            EmailAddress::parse(&too_long).unwrap_err(),
            TypeError::EmailAddress(EmailAddressError::Format)
        ));
    }

    #[test]
    fn parse_should_reject_illegal_characters_in_local_part() {
        for input in ["a(b@c.com", "a,b@c.com", "a:b@c.com", "a\\b@c.com"] {
            assert!(
                EmailAddress::parse(input).is_err(),
                "{input} rad etilishi kerak edi"
            );
        }
    }

    #[test]
    fn parse_should_accept_legitimate_addresses() {
        for input in [
            "user+tag@example.com",
            "user_name@sub.example.co.uk",
            "first.last@my-domain.uz",
            "a@b.co",
            "user123@x1.info",
        ] {
            assert!(
                EmailAddress::parse(input).is_ok(),
                "{input} qabul qilinishi kerak edi"
            );
        }
    }

    #[test]
    fn try_from_string_path_applies_the_same_rules() {
        // Zero-allocation yo'li (allaqachon lowercase) ham yangi qoidalarni tekshirishi kerak
        assert!(EmailAddress::try_from(String::from("a@b..c")).is_err());
        assert!(EmailAddress::try_from(String::from("a@b.c")).is_err());
        // Lowercase qilinadigan yo'l ham
        assert!(EmailAddress::try_from(String::from("A@B..C")).is_err());
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
