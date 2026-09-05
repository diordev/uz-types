//! Sir tiplari: `AccessToken`, `RefreshToken`, `ClientSecret`.
//!
//! Sir tiplarida **yo'q**: `Display`, `AsRef<str>`, `Borrow<str>`, `Deref`, `into_inner`,
//! derive `PartialEq`, `Hash`, `Ord`, (default'da) `Serialize`.
//! **Bor**: `expose_secret()`, yashirilgan `Debug`, constant-time `PartialEq` (`subtle`),
//! `Deserialize` (serde), `Serialize` faqat `serialize-secrets` feature'da,
//! `zeroize` feature'da `Drop` xotirani tozalaydi.

/// Token / sir uzunligi chegarasi (bayt). 8 KiB — JWT/Bearer uchun yetarli.
///
/// Diqqat: bu **xotira DoS'idan himoya emas** — `String` bu tekshiruvga kelguncha
/// allaqachon ajratilgan bo'ladi; body-limit HTTP qatlamida turishi kerak.
pub const MAX_TOKEN_LEN: usize = 8192;

/// Token / sir xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum TokenError {
    /// Trim'dan keyin bo'sh.
    #[error("token cannot be empty")]
    Empty,
    /// `MAX_TOKEN_LEN` dan uzun.
    #[error("token is too long, maximum is {MAX_TOKEN_LEN} bytes")]
    TooLong,
}

fn validate_token(s: &str) -> Result<(), TokenError> {
    if s.is_empty() {
        return Err(TokenError::Empty);
    }
    if s.len() > MAX_TOKEN_LEN {
        return Err(TokenError::TooLong);
    }
    Ok(())
}

macro_rules! secret_newtype {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        #[derive(Clone)]
        $vis struct $Name(String);

        impl $Name {
            /// Ruxsat etilgan maksimal uzunlik — [`MAX_TOKEN_LEN`].
            pub const MAX_LEN: usize = MAX_TOKEN_LEN;

            /// Trim → validatsiya. Bitta allocation.
            pub fn parse(value: &str) -> Result<Self, TokenError> {
                Self::try_from(value.to_owned())
            }

            /// Sirni **ochiq** qaytaradi. Nomi ataylab uzun — code review'da ko'zga tashlanadi.
            #[inline]
            #[must_use]
            pub fn expose_secret(&self) -> &str {
                &self.0
            }
        }

        impl TryFrom<String> for $Name {
            type Error = TokenError;

            fn try_from(mut value: String) -> Result<Self, TokenError> {
                $crate::macros::trim_in_place(&mut value);
                match validate_token(&value) {
                    Ok(()) => Ok(Self(value)),
                    Err(e) => {
                        // Rad etilgan sir ham xotirada qolmasin.
                        #[cfg(feature = "zeroize")]
                        ::zeroize::Zeroize::zeroize(&mut value);
                        Err(e)
                    }
                }
            }
        }

        impl TryFrom<&str> for $Name {
            type Error = TokenError;
            fn try_from(value: &str) -> Result<Self, TokenError> {
                Self::parse(value)
            }
        }

        impl ::core::str::FromStr for $Name {
            type Err = TokenError;
            fn from_str(s: &str) -> Result<Self, TokenError> {
                Self::parse(s)
            }
        }

        /// Loglarda sir hech qachon ko'rinmaydi.
        impl ::core::fmt::Debug for $Name {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(concat!(stringify!($Name), "([REDACTED])"))
            }
        }

        /// Constant-time taqqoslash — timing side-channel yo'q (uzunlik farqi bundan mustasno).
        impl PartialEq for $Name {
            fn eq(&self, other: &Self) -> bool {
                ::subtle::ConstantTimeEq::ct_eq(self.0.as_bytes(), other.0.as_bytes()).into()
            }
        }
        impl Eq for $Name {}

        #[cfg(feature = "zeroize")]
        impl Drop for $Name {
            fn drop(&mut self) {
                ::zeroize::Zeroize::zeroize(&mut self.0);
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $Name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                $crate::serde_support::deserialize_string_newtype(deserializer, "a non-empty secret string")
            }
        }

        /// Faqat `serialize-secrets` feature'da — masalan, auth-servis token javobi uchun.
        #[cfg(feature = "serialize-secrets")]
        impl ::serde::Serialize for $Name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.0)
            }
        }
    };
}

secret_newtype! {
    /// Qisqa muddatli kirish tokeni (JWT/Bearer).
    pub struct AccessToken;
}

secret_newtype! {
    /// Uzoq muddatli yangilash tokeni.
    pub struct RefreshToken;
}

secret_newtype! {
    /// OAuth client secret.
    pub struct ClientSecret;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secrets_are_redacted_and_ct_eq() {
        let a = AccessToken::parse(" top-secret ").unwrap();
        let b = AccessToken::parse("top-secret").unwrap();
        assert_eq!(a, b);
        assert_eq!(a.expose_secret(), "top-secret");
        assert_eq!(format!("{a:?}"), "AccessToken([REDACTED])");
        assert_eq!(AccessToken::parse("   "), Err(TokenError::Empty));
        assert_eq!(
            AccessToken::parse(&"a".repeat(MAX_TOKEN_LEN + 1)),
            Err(TokenError::TooLong)
        );
    }
}
