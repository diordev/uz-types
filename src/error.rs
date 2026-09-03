/// Barcha tip xatolarini birlashtiruvchi aggregate error.
///
/// Har bir `parse()` **o'zining aniq** xatosini qaytaradi (`PassportError`, ...);
/// `TypeError` — `?` orqali avtomatik (`#[from]`) yig'iladigan umumiy tur.
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum TypeError {
    /// Pasport.
    #[error(transparent)]
    Passport(#[from] crate::PassportError),
    /// PINFL.
    #[error(transparent)]
    Pinfl(#[from] crate::PinflError),
    /// Telefon raqami.
    #[error(transparent)]
    PhoneNumber(#[from] crate::PhoneNumberError),
    /// Email.
    #[error(transparent)]
    EmailAddress(#[from] crate::EmailAddressError),
    /// Tug'ilgan sana (feature = "date").
    #[cfg(feature = "date")]
    #[error(transparent)]
    BirthDate(#[from] crate::BirthDateError),
    /// ID tiplari (feature = "id").
    #[cfg(feature = "id")]
    #[error(transparent)]
    Id(#[from] crate::IdError),
    /// Token / sir tiplari.
    #[error(transparent)]
    Token(#[from] crate::TokenError),
}
