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
//! - `Passport` - O'zbekiston pasport seriyasi va raqami (masalan: `AA1234567`).
//! - `Pinfl` - Jismoniy shaxsning shaxsiy identifikatsiya raqami (14 xonali raqam).
//! - `PhoneNumber` - Xalqaro formatdagi telefon raqamlari.
//! - `BirthDate` - Tug'ilgan sana (YYYY-MM-DD formati va kelajak sanalariga qarshi himoya).

mod birth_date;
mod error;
mod passport;
mod phone_number;
mod pinfl;

/// Tashqi foydalanish uchun umumiy type eksportlari.
///
/// Eng ko'p ishlatiladigan tiplarni bitta qatorda chaqirib olish uchun mo'ljallangan.
///
/// ## Misol:
/// ```rust
/// use uz_types::prelude::*;
/// ```
pub mod prelude;
