//! Tipli identifikatorlar: [`Id<Tag>`] (UUID) va [`NumId<Tag>`] (u64).
//!
//! `Tag` — faqat compile-time belgisi (marker). Foydalanuvchi o'z tag'ini o'zi e'lon qiladi:
//!
//! ```ignore
//! pub enum Order {}
//! pub type OrderId = uz_types::Id<Order>;
//! ```
//!
//! `PhantomData<fn() -> Tag>` — `Id<Tag>` har doim `Send + Sync + Unpin` bo'lishi
//! va `Tag` bo'yicha kovariant qolishi uchun (`PhantomData<Tag>` emas).

use core::fmt;
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::str::FromStr;

use uuid::Uuid;

/// ID tiplari xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum IdError {
    /// UUID sifatida o'qib bo'lmadi.
    #[error("invalid UUID format")]
    Uuid,
    /// Manfiy bo'lmagan butun son (u64) sifatida o'qib bo'lmadi.
    #[error("invalid numeric id, expected a non-negative integer")]
    Number,
}

/// Standart tag'lar (0.17 dagi `JobId`, `SessionId`, `RequestId`, `Reuid` uchun).
pub mod tag {
    /// `JobId` tag.
    #[derive(Debug)]
    pub enum Job {}
    /// `SessionId` tag.
    #[derive(Debug)]
    pub enum Session {}
    /// `RequestId` tag.
    #[derive(Debug)]
    pub enum Request {}
}

/// Ish (job) identifikatori.
pub type JobId = Id<tag::Job>;
/// Sessiya identifikatori.
pub type SessionId = Id<tag::Session>;
/// So'rov (request) identifikatori.
pub type RequestId = Id<tag::Request>;

// ==========================================
// Id<Tag> — UUID
// ==========================================

/// Tipli UUID identifikator. JSON'da **har doim** string, DB'da `UUID`.
///
/// `parse`/`from_uuid` har qanday RFC 9562 UUID'ni qabul qiladi (struktura);
/// versiya cheklovi kerak bo'lsa — [`Id::version`] bilan tekshiring.
pub struct Id<Tag>(Uuid, PhantomData<fn() -> Tag>);

impl<Tag> Id<Tag> {
    /// Tasodifiy (v4).
    #[must_use]
    pub fn new_v4() -> Self {
        Self::from_uuid(Uuid::new_v4())
    }

    /// Vaqt bo'yicha tartiblangan (v7) — DB primary key uchun tavsiya etiladi.
    #[must_use]
    pub fn now_v7() -> Self {
        Self::from_uuid(Uuid::now_v7())
    }

    /// Mavjud `Uuid` dan.
    #[inline]
    #[must_use]
    pub const fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid, PhantomData)
    }

    /// Matndan (hyphenated / simple / braced / urn).
    pub fn parse(value: &str) -> Result<Self, IdError> {
        Uuid::parse_str(value.trim())
            .map(Self::from_uuid)
            .map_err(|_| IdError::Uuid)
    }

    /// Ichki `Uuid`.
    #[inline]
    #[must_use]
    pub const fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    /// Ichki `Uuid` (`Copy`).
    #[inline]
    #[must_use]
    pub const fn into_uuid(self) -> Uuid {
        self.0
    }

    /// UUID versiyasi (`Some(Version::SortRand)` — v7).
    #[must_use]
    pub fn version(&self) -> Option<uuid::Version> {
        self.0.get_version()
    }

    /// Nil UUID (`0000…`) mi.
    #[must_use]
    pub const fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
}

// --- Tag'ga bog'liq BO'LMAGAN trait impl'lar (derive `Tag: Clone` va h.k. talab qilardi) ---

impl<Tag> Clone for Id<Tag> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Tag> Copy for Id<Tag> {}
impl<Tag> PartialEq for Id<Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<Tag> Eq for Id<Tag> {}
impl<Tag> PartialOrd for Id<Tag> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<Tag> Ord for Id<Tag> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<Tag> Hash for Id<Tag> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<Tag> fmt::Debug for Id<Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = core::any::type_name::<Tag>()
            .rsplit("::")
            .next()
            .unwrap_or("?");
        write!(f, "Id<{tag}>({})", self.0)
    }
}

impl<Tag> fmt::Display for Id<Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0.as_hyphenated(), f)
    }
}

impl<Tag> FromStr for Id<Tag> {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, IdError> {
        Self::parse(s)
    }
}

impl<Tag> TryFrom<&str> for Id<Tag> {
    type Error = IdError;
    fn try_from(value: &str) -> Result<Self, IdError> {
        Self::parse(value)
    }
}

impl<Tag> TryFrom<String> for Id<Tag> {
    type Error = IdError;
    fn try_from(value: String) -> Result<Self, IdError> {
        Self::parse(&value)
    }
}

impl<Tag> From<Uuid> for Id<Tag> {
    fn from(uuid: Uuid) -> Self {
        Self::from_uuid(uuid)
    }
}

impl<Tag> From<Id<Tag>> for Uuid {
    fn from(id: Id<Tag>) -> Self {
        id.0
    }
}

impl<Tag> From<Id<Tag>> for String {
    fn from(id: Id<Tag>) -> Self {
        id.to_string()
    }
}

/// serde: `Uuid` ga delegatsiya — JSON'da string, binary formatlarda (bincode/postcard) 16 bayt.
#[cfg(feature = "serde")]
impl<Tag> serde::Serialize for Id<Tag> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, Tag> serde::Deserialize<'de> for Id<Tag> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Uuid::deserialize(deserializer).map(Self::from_uuid)
    }
}

#[cfg(feature = "sqlx")]
crate::sqlx_support::sqlx_via!(
    Id<Tag> [Tag],
    Uuid,
    |u: Uuid| Ok::<_, core::convert::Infallible>(Id::from_uuid(u)),
    |this: &Id<Tag>| &this.0
);

// ==========================================
// NumId<Tag> — u64 (legacy / BIGINT)
// ==========================================

/// Tipli raqamli identifikator (legacy tizimlar, `BIGINT` ustunlar).
/// JSON'da **har doim** integer, DB'da `BIGINT` (`i64`; `> i64::MAX` qiymat Encode'da xato).
pub struct NumId<Tag>(u64, PhantomData<fn() -> Tag>);

impl<Tag> NumId<Tag> {
    /// Qiymatdan.
    #[inline]
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self(value, PhantomData)
    }

    /// Matndan (faqat ASCII raqamlar, `+`/`-` yo'q).
    pub fn parse(value: &str) -> Result<Self, IdError> {
        let raw = value.trim();
        if raw.is_empty() || !raw.bytes().all(|b| b.is_ascii_digit()) {
            return Err(IdError::Number);
        }
        raw.parse::<u64>()
            .map(Self::new)
            .map_err(|_| IdError::Number)
    }

    /// Qiymat.
    #[inline]
    #[must_use]
    pub const fn get(&self) -> u64 {
        self.0
    }
}

impl<Tag> Clone for NumId<Tag> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Tag> Copy for NumId<Tag> {}
impl<Tag> PartialEq for NumId<Tag> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<Tag> Eq for NumId<Tag> {}
impl<Tag> PartialOrd for NumId<Tag> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<Tag> Ord for NumId<Tag> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<Tag> Hash for NumId<Tag> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<Tag> fmt::Debug for NumId<Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = core::any::type_name::<Tag>()
            .rsplit("::")
            .next()
            .unwrap_or("?");
        write!(f, "NumId<{tag}>({})", self.0)
    }
}
impl<Tag> fmt::Display for NumId<Tag> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
impl<Tag> FromStr for NumId<Tag> {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, IdError> {
        Self::parse(s)
    }
}
impl<Tag> From<u64> for NumId<Tag> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
impl<Tag> From<NumId<Tag>> for u64 {
    fn from(id: NumId<Tag>) -> Self {
        id.0
    }
}

#[cfg(feature = "serde")]
impl<Tag> serde::Serialize for NumId<Tag> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0)
    }
}

#[cfg(feature = "serde")]
impl<'de, Tag> serde::Deserialize<'de> for NumId<Tag> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        u64::deserialize(deserializer).map(Self::new)
    }
}

/// DB'da `BIGINT` (`i64`): Encode `> i64::MAX` bo'lsa xato, Decode manfiy bo'lsa xato.
#[cfg(feature = "sqlx")]
mod sqlx_impls {
    use super::NumId;

    impl<DB: sqlx::Database, Tag> sqlx::Type<DB> for NumId<Tag>
    where
        i64: sqlx::Type<DB>,
    {
        fn type_info() -> DB::TypeInfo {
            <i64 as sqlx::Type<DB>>::type_info()
        }
        fn compatible(ty: &DB::TypeInfo) -> bool {
            <i64 as sqlx::Type<DB>>::compatible(ty)
        }
    }

    impl<'q, DB: sqlx::Database, Tag> sqlx::Encode<'q, DB> for NumId<Tag>
    where
        i64: sqlx::Encode<'q, DB>,
    {
        fn encode_by_ref(
            &self,
            buf: &mut DB::ArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
            let value = i64::try_from(self.0)
                .map_err(|_| format!("NumId {} does not fit into BIGINT (i64)", self.0))?;
            <i64 as sqlx::Encode<'q, DB>>::encode_by_ref(&value, buf)
        }
    }

    impl<'r, DB: sqlx::Database, Tag> sqlx::Decode<'r, DB> for NumId<Tag>
    where
        i64: sqlx::Decode<'r, DB>,
    {
        fn decode(value: DB::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
            let raw = <i64 as sqlx::Decode<'r, DB>>::decode(value)?;
            let value = u64::try_from(raw).map_err(|_| format!("negative id {raw} in database"))?;
            Ok(Self::new(value))
        }
    }

    #[cfg(feature = "sqlx-postgres")]
    impl<Tag> sqlx::postgres::PgHasArrayType for NumId<Tag> {
        fn array_type_info() -> sqlx::postgres::PgTypeInfo {
            <i64 as sqlx::postgres::PgHasArrayType>::array_type_info()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    enum Order {}
    type OrderId = Id<Order>;
    type LegacyOrderId = NumId<Order>;

    #[test]
    fn ids_are_typed_and_roundtrip() {
        let a = OrderId::now_v7();
        let b = OrderId::now_v7();
        assert!(a < b);
        assert_eq!(OrderId::parse(&a.to_string()).unwrap(), a);
        assert_eq!(format!("{a:?}"), format!("Id<Order>({})", a.as_uuid()));
        assert_eq!(a.version(), Some(uuid::Version::SortRand));
        assert!(OrderId::parse("not-a-uuid").is_err());
        assert!(OrderId::parse("42").is_err()); // raqam UUID emas

        let n = LegacyOrderId::parse(" 42 ").unwrap();
        assert_eq!(n.get(), 42);
        assert!(LegacyOrderId::parse("-1").is_err());
        assert!(LegacyOrderId::parse("+1").is_err());
        assert!(LegacyOrderId::parse("").is_err());
    }

    #[test]
    fn id_is_send_sync_regardless_of_tag() {
        fn assert_send_sync<T: Send + Sync>() {}
        #[allow(dead_code)]
        struct NotSync(core::cell::Cell<u8>);
        assert_send_sync::<Id<NotSync>>();
        assert_send_sync::<NumId<NotSync>>();
    }
}
