//! Crate-ichki makrolar va yordamchilar.
//!
//! `string_newtype!` — `String` ustiga qurilgan barcha value object'lar uchun
//! **yagona** boilerplate manbai. Har bir tip faqat ikkita narsani beradi:
//!
//! - `fn normalize(s: &mut String)` (yoki `&mut str`, agar uzunlik o'zgarmasa) — in-place,
//!   allocation'siz; trim makro tomonidan allaqachon bajarilgan bo'ladi;
//! - `fn validate(s: &str) -> Result<(), Error>` — **normalizatsiya qilingan** matn ustida.

/// `String` ni allocation'siz, in-place trim qiladi (memmove + truncate).
pub(crate) fn trim_in_place(s: &mut String) {
    let end = s.trim_end().len();
    s.truncate(end);
    let start = s.len() - s.trim_start().len();
    if start > 0 {
        s.drain(..start);
    }
}

macro_rules! string_newtype {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
        error = $Error:ty;
        expecting = $expecting:literal;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
        $vis struct $Name(String);

        impl $Name {
            /// Matndan tip yaratadi: trim → normalizatsiya → validatsiya.
            ///
            /// Bitta allocation (normalizatsiya qilingan nusxa uchun).
            pub fn parse(value: &str) -> Result<Self, $Error> {
                Self::try_from(value.to_owned())
            }

            /// Normalizatsiya qilingan qiymat.
            #[inline]
            #[must_use]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Ichki `String` (ownership ko'chadi, nusxa olinmaydi).
            #[inline]
            #[must_use]
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        /// Yagona "owned" yo'l: hech qachon qo'shimcha allocation qilmaydi.
        impl TryFrom<String> for $Name {
            type Error = $Error;

            fn try_from(mut value: String) -> Result<Self, $Error> {
                $crate::macros::trim_in_place(&mut value);
                Self::normalize(&mut value);
                Self::validate(&value)?;
                Ok(Self(value))
            }
        }

        impl TryFrom<&str> for $Name {
            type Error = $Error;

            #[inline]
            fn try_from(value: &str) -> Result<Self, $Error> {
                Self::parse(value)
            }
        }

        impl ::core::str::FromStr for $Name {
            type Err = $Error;

            #[inline]
            fn from_str(s: &str) -> Result<Self, $Error> {
                Self::parse(s)
            }
        }

        impl From<$Name> for String {
            #[inline]
            fn from(value: $Name) -> Self {
                value.0
            }
        }

        impl AsRef<str> for $Name {
            #[inline]
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        /// `HashMap<$Name, V>::get("...")` va `BTreeMap` uchun.
        /// `Hash`/`Eq`/`Ord` ichki `String` dan derive qilingani uchun `str` bilan mos.
        impl ::core::borrow::Borrow<str> for $Name {
            #[inline]
            fn borrow(&self) -> &str {
                &self.0
            }
        }

        impl ::core::fmt::Display for $Name {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                f.write_str(&self.0)
            }
        }

        #[cfg(feature = "serde")]
        impl ::serde::Serialize for $Name {
            #[inline]
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                serializer.serialize_str(&self.0)
            }
        }

        #[cfg(feature = "serde")]
        impl<'de> ::serde::Deserialize<'de> for $Name {
            #[inline]
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                $crate::serde_support::deserialize_string_newtype(deserializer, $expecting)
            }
        }

        #[cfg(feature = "sqlx")]
        $crate::sqlx_support::sqlx_via!($Name, String, |s: String| Self::try_from(s), |this: &Self| &this.0);
    };
}

pub(crate) use string_newtype;
