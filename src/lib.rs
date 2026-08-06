#![deny(missing_docs)]
#![deny(unsafe_code)]

//! `uz-types` — qayta ishlatiladigan domain type va value object kutubxonasi.

mod birth_date;
mod error;
mod passport;

/// Tashqi foydalanish uchun umumiy type eksportlari.
///
/// Misol:
/// ```
/// use uz_types::prelude::*;
/// ```
pub mod prelude;