#![deny(missing_docs)]
#![deny(unsafe_code)]

//! # `uz-types`
//!
//! Rust loyihalari uchun qat'iy tiplangan (strongly typed), xavfsiz va qayta ishlatiladigan
//! *Domain Type* va *Value Object* kutubxonasi.
//!
//! Bu kutubxona backend tizimlarida ko'p uchraydigan ma'lumotlarni (masalan, O'zbekiston pasporti,
//! PINFL, telefon raqami va tug'ilgan sana) xavfsiz qabul qilish, avtomatik tekshirish (validatsiya)
//! va saqlash vazifalarini standartlashtiradi va osonlashtiradi.
//!
//! ## Asosiy tiplar:
//! - [`Passport`] - O'zbekiston pasport seriyasi va raqami (masalan: `AA1234567`).
//! - [`Pinfl`] - Jismoniy shaxsning shaxsiy identifikatsiya raqami (14 xonali raqam).
//! - [`PhoneNumber`] - O'zbekiston telefon raqami (operator/hudud kodi bilan).
//! - [`EmailAddress`] - Validatsiya qilingan elektron pochta (email) manzili.
//! - [`BirthDate`] - Tug'ilgan sana (kelajak va juda uzoq o'tmishga qarshi himoya, yosh hisoblash).
//! - [`JobId`], [`SessionId`], [`RequestId`], [`Reuid`] - UUID (v4/v7) yoki `u64` qabul qiluvchi ID turlari.
//! - [`AccessToken`], [`RefreshToken`], [`ClientId`], [`ClientSecret`] - API xizmatlariga ulanish uchun matnli sir turlari.
//!
//! ## Foydalanish
//!
//! ```rust
//! use uz_types::{Passport, PassportError, TypeError};
//!
//! let passport = Passport::parse("aa1234567")?;
//! assert_eq!(passport.series(), "AA");
//!
//! // Xato turini aniq ajratish mumkin
//! assert!(matches!(
//!     Passport::parse("AA123"),
//!     Err(TypeError::Passport(PassportError::Length))
//! ));
//! # Ok::<(), TypeError>(())
//! ```
//!
//! Ko'p tip kerak bo'lsa, [`prelude`] dan foydalaning.
//!
//! ## ⚠️ Bilib qo'yish kerak bo'lgan cheklovlar
//!
//! - [`Pinfl`] **faqat formatni** tekshiradi — checksum tekshirilmaydi.
//! - Token tiplarining `Debug` chiqishi yashiringan, lekin `Display`
//!   (`{}` va `.to_string()`) sirni to'liq ochadi — loglashda `{:?}` ishlating.
//! - ID tiplari serde'da ikki xil JSON shakl beradi: UUID → string,
//!   raqam → integer.

mod birth_date;
mod email;
mod error;
mod passport;
mod phone_number;
mod pinfl;
mod token_types;
mod uuid_types;

/// Tashqi foydalanish uchun umumiy type eksportlari.
///
/// Eng ko'p ishlatiladigan tiplarni bitta qatorda chaqirib olish uchun mo'ljallangan.
///
/// ## Misol:
/// ```rust
/// use uz_types::prelude::*;
/// ```
pub mod prelude;

// ==========================================
// PUBLIC API (crate root)
// ==========================================
//
// Har bir tip va uning xato (error) turi crate ildizidan to'g'ridan-to'g'ri
// chaqirilishi kerak: `use uz_types::{Passport, PassportError};`
//
// Modullarning o'zi `private` bo'lib qoladi — shu sababli har bir tipga
// faqat bitta rasmiy yo'l (canonical path) mavjud bo'ladi.

pub use crate::birth_date::{BirthDate, BirthDateError, DateFormat};
pub use crate::email::{EmailAddress, EmailAddressError};
pub use crate::error::TypeError;
pub use crate::passport::{Passport, PassportError};
pub use crate::phone_number::{PhoneNumber, PhoneNumberError};
pub use crate::pinfl::{Pinfl, PinflError};
pub use crate::token_types::{
    AccessToken, ClientId, ClientSecret, MAX_TOKEN_LEN, RefreshToken, TokenError,
};
pub use crate::uuid_types::{IdError, JobId, RequestId, Reuid, SessionId};
