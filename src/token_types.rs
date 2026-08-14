use serde::{self, Deserialize, Serialize};
use std::borrow::Cow;

use crate::error::TypeError;

// ==========================================
// TOKEN XATOLIKLARI
// ==========================================

/// Tokenlar bilan ishlash jarayonidagi xatoliklar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum TokenError {
    /// Token butunlay bo'sh bo'lishi mumkin emas.
    #[error("token cannot be empty")]
    Empty,

    /// Token ruxsat etilgan uzunlikdan oshib ketdi.
    #[error("token is too long, maximum is {} bytes", MAX_TOKEN_LEN)]
    TooLong,
}

/// Token uchun maksimal uzunlik (bayt).
///
/// Bu chegara ishonchsiz manbadan (masalan JSON body'dan) kelgan
/// cheksiz uzunlikdagi matn xotirani to'ldirib qo'yishining oldini oladi.
/// 8 KiB — amaldagi JWT va Bearer tokenlar uchun yetarlicha keng zaxira.
pub const MAX_TOKEN_LEN: usize = 8192;

// ==========================================
// MACRO DEFINITION
// ==========================================

/// Zero-allocation imkoniyatiga ega Token tiplarini (Access/Refresh) generatsiya qiluvchi makro.
macro_rules! define_token_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        // Debug ataylab olib tashlandi, uni pastda o'zimiz yozamiz
        #[derive(Clone, PartialEq, Eq, Hash)]
        $vis struct $Name(String);

        impl $Name {
            /// Ruxsat etilgan maksimal uzunlik — [`MAX_TOKEN_LEN`].
            pub const MAX_LEN: usize = $crate::token_types::MAX_TOKEN_LEN;

            /// Bo'sh bo'lmagan matndan token yasaydi (ikki yonidagi bo'sh joylarni olib tashlaydi).
            ///
            /// # Xatolar
            ///
            /// - [`TokenError::Empty`] — trim qilingandan keyin matn bo'sh qolsa;
            /// - [`TokenError::TooLong`] — uzunlik [`MAX_TOKEN_LEN`] dan oshsa.
            #[inline]
            pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
                let raw = value.as_ref().trim();

                Self::validate(raw)?;

                Ok(Self(raw.to_string()))
            }

            /// Ichki validatsiya (xotira ajratmaydi).
            #[inline]
            fn validate(raw: &str) -> Result<(), TokenError> {
                if raw.is_empty() {
                    return Err(TokenError::Empty);
                }

                if raw.len() > Self::MAX_LEN {
                    return Err(TokenError::TooLong);
                }

                Ok(())
            }

            /// Tokenning ichki matn qiymatini (`&str`) qaytaradi.
            #[inline]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Ichki `String`ni to'liq qaytaradi (ownership uzatiladi).
            #[inline]
            #[must_use]
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        // --- Default Traitlar ---

        /// Xavfsiz Debug: loglarda haqiqiy tokenni emas, yashiringan matnni ko'rsatadi.
        impl std::fmt::Debug for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.debug_tuple(stringify!($Name)).field(&"***REDACTED***").finish()
            }
        }

        /// Tokenni xavfsiz tarzda `&str` sifatda qabul qilishga yordam beradi.
        impl AsRef<str> for $Name {

            #[inline]
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        /// Tokenni matn ko'rinishida chiqarish (print) imkonini beradi.
        ///
        /// # ⚠️ Xavfsizlik ogohlantirishi
        ///
        /// `Debug` (`{:?}`) qiymatni yashiradi, lekin **`Display` (`{}`) va
        /// `.to_string()` sirni to'liq ochib beradi**. Quyidagilar tokenni
        /// log fayliga tushiradi:
        ///
        /// ```text
        /// tracing::info!("token: {}", token);   // ❌ sir logga tushadi
        /// println!("{}", token);                // ❌
        /// format!("{token}")                    // ❌
        /// ```
        ///
        /// Loglashda `{:?}` dan foydalaning. Haqiqiy qiymat faqat uni
        /// tashqi xizmatga uzatish kerak bo'lgan joyda — `as_str()` yoki
        /// `into_inner()` orqali olinsin.
        impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(&self.0)
            }
        }

        // Rust'ning standart idiomatik parse uslubi ("text".parse() uchun).
        impl std::str::FromStr for $Name {
            type Err = TypeError;

            // Rust'ning standart parse mantig'iga o'zimizning parse'ni ulab qo'yamiz
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::parse(s)
            }
        }

        // Matndan tokenga aylantirish (parse jarayoni).
        impl TryFrom<&str> for $Name {
            type Error = TypeError;

            fn try_from(value: &str) -> Result<Self, TypeError> {
                Self::parse(value)
            }
        }

        // Stringdan token yaratadi. Agar matnda bo'sh joy bo'lmasa, tayyor xotirani qayta ishlatadi (zero-alloc).
        impl TryFrom<String> for $Name {
            type Error = TypeError;

            fn try_from(value: String) -> Result<Self, TypeError> {
                let trimmed_len = value.trim().len();

                // Agar u allaqachon trim qilingan bo'lsa, xotirani qayta ishlatamiz.
                if value.len() == trimmed_len {
                    Self::validate(&value)?;
                    Ok(Self(value))
                } else {
                    Self::parse(value)
                }
            }
        }

        // Tokendan uning haqiqiy String formatini chiqarib oladi (nusxa olmaydi).
        impl From<$Name> for String {
            fn from(value: $Name) -> Self {
                value.into_inner()
            }
        }

        // --- Serde Optimizatsiyasi ---

        // Serializatsiyada tokenni oddiy matn sifatida (clone olmasdan) yozadi.
        impl Serialize for $Name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(self.as_str())
            }
        }

        // JSON'dan o'qilganda String ajratilgan xotirasini imkon boricha saqlab qoladi.
        impl<'de> Deserialize<'de> for $Name {
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
    };
}

// ==========================================
// TIP E'LONLARI
// ==========================================

define_token_type! {
    /// API xizmatlarga ulanish uchun ruxsat beruvchi qisqa muddatli token (JWT/Bearer).
    pub struct AccessToken;
}

define_token_type! {
    /// Yangi `AccessToken` olish uchun ishlatiladigan uzoq muddatli token.
    pub struct RefreshToken;
}

define_token_type! {
    /// Yangi `ClientId` olish uchun ishlatiladigan tur (String qabul qiladi).
    pub struct ClientId;
}

define_token_type! {
    /// Yangi `ClientSecret` olish uchun ishlatiladigan tur (String qabul qiladi).
    pub struct ClientSecret;
}

// ==========================================
// TESTLAR (Minimalist)
// ==========================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_should_create_valid_access_token() {
        let token = AccessToken::parse(" eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz ").unwrap();
        assert_eq!(token.as_str(), "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz"); // Trim ishladi
    }

    #[test]
    fn parse_should_fail_on_empty_token() {
        let result = RefreshToken::parse("   ");
        assert!(matches!(
            result.unwrap_err(),
            TypeError::Token(TokenError::Empty)
        ));
    }

    #[test]
    fn try_from_string_should_reuse_memory_if_clean() {
        let raw = String::from("clean_token_123");
        let token = AccessToken::try_from(raw).unwrap();
        // AsRef va Display ishlashiga tekshiruv
        assert_eq!(token.as_ref(), "clean_token_123");
        assert_eq!(format!("{}", token), "clean_token_123");
    }

    #[test]
    fn serde_should_support_roundtrip() {
        let token = AccessToken::parse("token123").unwrap();
        let json = serde_json::to_string(&token).unwrap();
        assert_eq!(json, "\"token123\"");

        let restored: AccessToken = serde_json::from_str(&json).unwrap();
        assert_eq!(token, restored);
    }

    #[test]
    fn parse_should_reject_tokens_over_max_len() {
        // Chegaraning o'zi qabul qilinadi
        let at_limit = "a".repeat(MAX_TOKEN_LEN);
        assert!(AccessToken::parse(&at_limit).is_ok());

        // Bir bayt ortiq — rad etiladi
        let over_limit = "a".repeat(MAX_TOKEN_LEN + 1);
        assert!(matches!(
            AccessToken::parse(&over_limit).unwrap_err(),
            TypeError::Token(TokenError::TooLong)
        ));
    }

    #[test]
    fn try_from_string_should_also_enforce_max_len() {
        let over_limit = "a".repeat(MAX_TOKEN_LEN + 1);

        // Zero-allocation yo'li (trim kerak emas)
        assert!(matches!(
            RefreshToken::try_from(over_limit.clone()).unwrap_err(),
            TypeError::Token(TokenError::TooLong)
        ));

        // Trim qilinadigan yo'l
        assert!(matches!(
            RefreshToken::try_from(format!("  {over_limit}  ")).unwrap_err(),
            TypeError::Token(TokenError::TooLong)
        ));
    }

    #[test]
    fn deserialization_should_reject_oversized_tokens() {
        let json = format!("\"{}\"", "a".repeat(MAX_TOKEN_LEN + 1));
        assert!(serde_json::from_str::<AccessToken>(&json).is_err());
    }

    #[test]
    fn client_id_and_secret_behave_like_tokens() {
        assert_eq!(ClientId::parse(" id-1 ").unwrap().as_str(), "id-1");
        assert_eq!(ClientSecret::parse("s3cr3t").unwrap().as_str(), "s3cr3t");

        assert!(ClientId::parse("").is_err());
        assert!(ClientSecret::parse("  ").is_err());
    }

    #[test]
    fn debug_should_redact_but_display_should_not() {
        let secret = ClientSecret::parse("top-secret").unwrap();

        // Debug — yashiringan
        assert!(!format!("{secret:?}").contains("top-secret"));
        assert!(format!("{secret:?}").contains("REDACTED"));

        // Display — ochiq (hujjatlashtirilgan xatti-harakat, 0.18 da o'zgaradi)
        assert_eq!(format!("{secret}"), "top-secret");
    }
}
