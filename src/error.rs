use crate::birth_date::BirthDateError;
use crate::passport::PassportError;
use crate::phone_number::PhoneNumberError;
use crate::pinfl::PinflError;

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
    ///
    /// Masalan:
    /// - noto'g'ri input
    /// - umumiy tekshiruv xatolari
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
