#![warn(missing_docs)]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
// README kod bloklari `date` va `id` tiplarini ishlatadi (`BirthDate`,
// `Pinfl::birth_date`, `NumId`, `IdError`). Ikkalasi ham default feature;
// ularsiz doctest kompilyatsiya bo'lmaydi, shuning uchun README faqat shu
// ikkisi yoqilganda doc sifatida qo'shiladi. docs.rs `all-features` bilan
// quradi — render'ga ta'sir yo'q.
#![cfg_attr(
    all(feature = "date", feature = "id"),
    doc = include_str!("../README.md")
)]

//! # `uz-types` (verification skeleton)
//!
//! Feature'lar:
//! - `date` (default) — `BirthDate`, `DateFormat` (`chrono`);
//! - `id` (default) — `Id<Tag>`, `NumId<Tag>` (`uuid`);
//! - `serde` — `Serialize`/`Deserialize`;
//! - `sqlx`, `sqlx-postgres` — `Type`/`Encode`/`Decode` (+ `PgHasArrayType`);
//! - `zeroize` — sir tiplari `Drop` da xotirani tozalaydi;
//! - `serialize-secrets` — sir tiplari uchun `Serialize`.

mod macros;

#[cfg(feature = "serde")]
mod serde_support;
#[cfg(feature = "sqlx")]
mod sqlx_support;

mod email;
mod error;
mod passport;
mod phone_number;
mod pinfl;
mod secret;

#[cfg(feature = "date")]
mod birth_date;
#[cfg(feature = "id")]
mod id;

pub mod prelude;

pub use crate::email::{EmailAddress, EmailAddressError};
pub use crate::error::TypeError;
pub use crate::passport::{Passport, PassportError};
pub use crate::phone_number::{PhoneNumber, PhoneNumberError};
pub use crate::pinfl::{Gender, Pinfl, PinflError};
pub use crate::secret::{
    AccessToken, ClientId, ClientSecret, MAX_TOKEN_LEN, RefreshToken, TokenError,
};

#[cfg(feature = "date")]
#[cfg_attr(docsrs, doc(cfg(feature = "date")))]
pub use crate::birth_date::{BirthDate, BirthDateError, DateFormat};

#[cfg(feature = "id")]
#[cfg_attr(docsrs, doc(cfg(feature = "id")))]
pub use crate::id::{Id, IdError, NumId, NumIdRepr};
