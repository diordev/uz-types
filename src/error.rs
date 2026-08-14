use crate::birth_date::BirthDateError;
use crate::email::EmailAddressError;
use crate::passport::PassportError;
use crate::phone_number::PhoneNumberError;
use crate::pinfl::PinflError;
use crate::token_types::TokenError;
use crate::uuid_types::IdError;

/// `uz-types` crate ichidagi umumiy error turi.
///
/// Har bir domain type o'zining maxsus erroriga ega — [`BirthDateError`],
/// [`PassportError`], [`PhoneNumberError`], [`PinflError`],
/// [`EmailAddressError`], [`IdError`], [`TokenError`] — va barchasi
/// `TypeError` orqali qaytariladi.
///
/// # Misol
///
/// ```rust
/// use uz_types::{Passport, PassportError, TypeError};
///
/// match Passport::parse("AA123") {
///     Ok(p) => println!("{p}"),
///     Err(TypeError::Passport(PassportError::Length)) => println!("uzunlik xato"),
///     Err(TypeError::Passport(PassportError::Format)) => println!("format xato"),
///     Err(e) => println!("boshqa xato: {e}"),
/// }
/// ```
///
/// Enum `#[non_exhaustive]` — kelajakda yangi variantlar qo'shilishi mumkin,
/// shu sababli `match` da har doim `_` yoki `Err(e)` tarmog'i bo'lishi kerak.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum TypeError {
    /// Umumiy validation xatolari.
    ///
    /// Deprecated: crate ichida ishlatilmaydi, 1.0 da olib tashlanadi.
    #[error("validation error: {message}")]
    Validation {
        /// Xato haqida batafsil ma'lumot.
        message: String,
    },

    /// Tug'ilgan sana bilan bog'liq xatolar.
    #[error(transparent)]
    BirthDate(#[from] BirthDateError),

    /// Passport bilan bog'liq xatolar.
    #[error(transparent)]
    Passport(#[from] PassportError),

    /// Phone number bilan bog'liq xatolar.
    #[error(transparent)]
    PhoneNumber(#[from] PhoneNumberError),

    /// Pinfl bilan bog'liq xatolar.
    ///
    /// Nomlash: 1.0 da bu variant Rust konvensiyasiga moslab `Pinfl`
    /// deb qayta nomlanadi.
    #[error(transparent)]
    PINFL(#[from] PinflError),

    /// Email bilan bog'liq xatolar.
    #[error(transparent)]
    EmailAddress(#[from] EmailAddressError),

    /// UUID yoki raqamli (ID) tiplar bilan bog'liq xatolar.
    #[error(transparent)]
    Id(#[from] IdError),

    /// Tokenlar (`AccessToken`, `RefreshToken`, `ClientId`, `ClientSecret`)
    /// bilan bog'liq xatolar.
    #[error(transparent)]
    Token(#[from] TokenError),
}

impl TypeError {
    /// Umumiy validation error yaratadi.
    #[inline]
    #[deprecated(
        since = "0.17.0",
        note = "crate ichida ishlatilmaydi va 1.0 da `TypeError::Validation` bilan birga olib tashlanadi; \
                o'z domeningiz uchun alohida error tipini yarating"
    )]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
}
