//! `serde` integratsiyasi (feature = "serde").
//!
//! Barcha `String`-asosli tiplar uchun **bitta** `Visitor`:
//!
//! - `visit_str`     → `FromStr` (`parse`) — bitta allocation (normalizatsiya qilingan nusxa);
//! - `visit_string`  → `TryFrom<String>` — deserializer bergan `String` xotirasi
//!   **qayta ishlatiladi** (allocation yo'q). Bu yo'l `serde_json::Value`,
//!   `bincode`, `postcard` kabi formatlarda ishlaydi; `serde_json::from_str`
//!   esa `visit_str`/`visit_borrowed_str` chaqiradi (u yerda farq yo'q).
//!
//! `deserialize_string` hint'i formatga "ownership bera olsang — ber" deydi.
//! Haqiqiy zero-copy bu tiplar uchun **printsipial mumkin emas** — ular `String`
//! saqlaydi, `&'de str` emas.

use core::fmt;
use core::marker::PhantomData;
use core::str::FromStr;

use serde::de::{self, Deserializer, Visitor};

pub(crate) fn deserialize_string_newtype<'de, D, T>(
    deserializer: D,
    expecting: &'static str,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + TryFrom<String>,
    <T as FromStr>::Err: fmt::Display,
    <T as TryFrom<String>>::Error: fmt::Display,
{
    struct StringNewtypeVisitor<T> {
        expecting: &'static str,
        _marker: PhantomData<fn() -> T>,
    }

    impl<'de, T> Visitor<'de> for StringNewtypeVisitor<T>
    where
        T: FromStr + TryFrom<String>,
        <T as FromStr>::Err: fmt::Display,
        <T as TryFrom<String>>::Error: fmt::Display,
    {
        type Value = T;

        fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str(self.expecting)
        }

        fn visit_str<E: de::Error>(self, v: &str) -> Result<T, E> {
            v.parse().map_err(E::custom)
        }

        fn visit_string<E: de::Error>(self, v: String) -> Result<T, E> {
            T::try_from(v).map_err(E::custom)
        }
    }

    deserializer.deserialize_string(StringNewtypeVisitor {
        expecting,
        _marker: PhantomData,
    })
}
