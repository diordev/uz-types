use crate::birth_date::BirthDateError;
use crate::email::EmailAddressError;
use crate::passport::PassportError;
use crate::phone_number::PhoneNumberError;
use crate::pinfl::PinflError;
use crate::token_types::TokenError;
use crate::uuid_types::IdError;

/// `uz-types` crate ichidagi umumiy error turi.
///
/// Har bir domain type o'zining maxsus erroriga ega:
/// - `BirthDateError`
/// - `PassportError`
///
/// Tashqi API uchun barcha xatolar `TypeError` orqali qaytariladi.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum TypeError {
    /// Umumiy validation xatolari.
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

    /// Pinfl bilan bo'gliq xatolar
    #[error(transparent)]
    PINFL(#[from] PinflError),

    /// Email bilan bo'gliq xatolar
    #[error(transparent)]
    EmailAddress(#[from] EmailAddressError),

    /// Uuid yoki raqamli (ID) bilan bog'liq hatolikar
    #[error(transparent)]
    Id(#[from] IdError),

    /// Tokenlar (AccessToken yoki RefreshToken) bilan bo'g'liq hatolikar.
    #[error(transparent)]
    Token(#[from] TokenError),
}

impl TypeError {
    /// Umumiy validation error yaratadi.
    #[inline]
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
}
