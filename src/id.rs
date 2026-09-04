//! Tipli identifikatorlar: [`Id<Tag>`] (UUID) va [`NumId<Tag, R>`] (`u64` yoki `i64`).
//!
//! Crate tayyor ID **nomlarini bermaydi**: `OrderId`, `SessionId` — bu sizning
//! domeningiz. Crate faqat mexanizmni beradi, nomni va ko'rinishni siz tanlaysiz:
//!
//! ```ignore
//! pub mod tag {
//!     pub enum Order {}
//!     pub enum LegacyInvoice {}
//! }
//!
//! pub type OrderId = uz_types::Id<tag::Order>;                    // UUID
//! pub type LegacyInvoiceId = uz_types::NumId<tag::LegacyInvoice, i64>; // BIGINT
//! ```
//!
//! `Tag` — faqat compile-time belgisi (marker), hech qachon instansiyalanmaydi.
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
    /// Butun son sifatida o'qib bo'lmadi (`NumId<_, u64>` uchun `-` ham qabul qilinmaydi).
    #[error("invalid numeric id")]
    Number,
    /// Qiymat `BIGINT` (`i64`) ustuniga sig'maydi — faqat `NumId<_, u64>` da bo'lishi mumkin.
    ///
    /// Buni query paytiga qoldirmaslik uchun input chegarasida
    /// [`NumId::try_new_db_safe`] / [`NumId::parse_db_safe`] ishlating.
    #[error("numeric id {value} does not fit into BIGINT (i64)")]
    NumberTooLarge {
        /// Sig'magan qiymat.
        value: u64,
    },
    /// DB'dagi manfiy `BIGINT` `u64` ko'rinishiga sig'maydi.
    ///
    /// Manfiy legacy ID'lar kutilsa, `NumId<Tag, i64>` ishlating.
    #[error("negative numeric id {value} in database does not fit into an unsigned id")]
    NumberNegative {
        /// DB'dan kelgan qiymat.
        value: i64,
    },
}

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
// NumId<Tag, R> — u64 (default) yoki i64 (legacy / BIGINT)
// ==========================================

mod sealed {
    pub trait Sealed {}
    impl Sealed for u64 {}
    impl Sealed for i64 {}
}

/// [`NumId`] ning ichki raqam ko'rinishi — **faqat** `u64` va `i64` (sealed).
///
/// DB `BIGINT` ustuni `i64`. `u64` ko'rinishi kengroq, shuning uchun uning yuqori
/// yarmi (`> i64::MAX`) `Encode` da xato beradi; `i64` ko'rinishida esa `Encode` va
/// `Decode` **total** — hech qachon xato bermaydi.
pub trait NumIdRepr:
    sealed::Sealed + Copy + Ord + Hash + fmt::Display + FromStr + Send + Sync + 'static
{
    /// `Debug` da tip nomiga qo'shiladigan qism (`u64` — bo'sh, `i64` — `", i64"`).
    const DEBUG_SUFFIX: &'static str;

    /// DB `BIGINT` ga. `i64` ko'rinishida hech qachon `Err` bo'lmaydi.
    fn to_bigint(self) -> Result<i64, IdError>;

    /// DB `BIGINT` dan. `i64` ko'rinishida hech qachon `Err` bo'lmaydi.
    fn from_bigint(value: i64) -> Result<Self, IdError>;

    /// Matndan. `u64` da `-` va `+` rad etiladi; `i64` da faqat `+` rad etiladi.
    fn parse_repr(value: &str) -> Result<Self, IdError>;

    /// JSON'da **har doim** integer: `u64` → `serialize_u64`, `i64` → `serialize_i64`.
    #[cfg(feature = "serde")]
    fn serialize_repr<S: serde::Serializer>(self, serializer: S) -> Result<S::Ok, S::Error>;

    /// Integer'dan. String (`"42"`) qabul qilinmaydi.
    #[cfg(feature = "serde")]
    fn deserialize_repr<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error>;
}

impl NumIdRepr for u64 {
    const DEBUG_SUFFIX: &'static str = "";

    fn to_bigint(self) -> Result<i64, IdError> {
        i64::try_from(self).map_err(|_| IdError::NumberTooLarge { value: self })
    }

    fn from_bigint(value: i64) -> Result<Self, IdError> {
        Self::try_from(value).map_err(|_| IdError::NumberNegative { value })
    }

    fn parse_repr(value: &str) -> Result<Self, IdError> {
        if value.is_empty() || !value.bytes().all(|b| b.is_ascii_digit()) {
            return Err(IdError::Number);
        }
        value.parse().map_err(|_| IdError::Number)
    }

    #[cfg(feature = "serde")]
    fn serialize_repr<S: serde::Serializer>(self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self)
    }

    #[cfg(feature = "serde")]
    fn deserialize_repr<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        serde::Deserialize::deserialize(deserializer)
    }
}

impl NumIdRepr for i64 {
    const DEBUG_SUFFIX: &'static str = ", i64";

    fn to_bigint(self) -> Result<Self, IdError> {
        Ok(self)
    }

    fn from_bigint(value: Self) -> Result<Self, IdError> {
        Ok(value)
    }

    fn parse_repr(value: &str) -> Result<Self, IdError> {
        let digits = value.strip_prefix('-').unwrap_or(value);
        if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(IdError::Number);
        }
        value.parse().map_err(|_| IdError::Number)
    }

    #[cfg(feature = "serde")]
    fn serialize_repr<S: serde::Serializer>(self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_i64(self)
    }

    #[cfg(feature = "serde")]
    fn deserialize_repr<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Self, D::Error> {
        serde::Deserialize::deserialize(deserializer)
    }
}

/// Tipli raqamli identifikator (legacy tizimlar, `BIGINT` ustunlar).
/// JSON'da **har doim** integer, DB'da `BIGINT`.
///
/// `R` — ichki ko'rinish, [`NumIdRepr`] (`u64` yoki `i64`); default `u64`, shuning uchun
/// `NumId<Order>` avvalgidek ishlaydi.
///
/// | | `NumId<Tag>` (`u64`) | `NumId<Tag, i64>` |
/// | --- | --- | --- |
/// | Diapazon | `0..=u64::MAX` | `i64::MIN..=i64::MAX` |
/// | `Encode` (`BIGINT`) | `> i64::MAX` → [`IdError::NumberTooLarge`] | **hech qachon xato yo'q** |
/// | `Decode` (`BIGINT`) | manfiy → [`IdError::NumberNegative`] | **hech qachon xato yo'q** |
/// | Manfiy legacy ID | ❌ | ✅ |
///
/// DB bilan ishlaganda `i64` ko'rinishi runtime xatolarining butun sinfini yo'q qiladi.
/// `u64` kerak bo'lsa, chegarani **input tomonida** [`try_new_db_safe`](Self::try_new_db_safe)
/// yoki [`parse_db_safe`](Self::parse_db_safe) bilan qo'ying — query paytida emas.
pub struct NumId<Tag, R: NumIdRepr = u64>(R, PhantomData<fn() -> Tag>);

impl<Tag, R: NumIdRepr> NumId<Tag, R> {
    /// Qiymatdan.
    #[inline]
    #[must_use]
    pub const fn new(value: R) -> Self {
        Self(value, PhantomData)
    }

    /// Matndan. `u64` ko'rinishida `-`/`+` rad etiladi, `i64` da `-` qabul qilinadi.
    pub fn parse(value: &str) -> Result<Self, IdError> {
        R::parse_repr(value.trim()).map(Self::new)
    }

    /// Qiymat.
    #[inline]
    #[must_use]
    pub const fn get(&self) -> R {
        self.0
    }

    /// DB `BIGINT` (`i64`) qiymati — `sqlx::Encode` aynan shuni chaqiradi.
    ///
    /// Query'ni yuborishdan **oldin** tekshirish uchun: `i64` ko'rinishida har doim `Ok`.
    pub fn to_bigint(&self) -> Result<i64, IdError> {
        self.0.to_bigint()
    }

    /// `BIGINT` ustuniga sig'adimi (`i64` ko'rinishida har doim `true`).
    #[must_use]
    pub fn is_db_safe(&self) -> bool {
        self.0.to_bigint().is_ok()
    }
}

impl<Tag> NumId<Tag, u64> {
    /// `BIGINT` (`i64`) ustuniga sig'adigan eng katta qiymat.
    pub const MAX_DB_SAFE: u64 = i64::MAX as u64;

    /// [`new`](Self::new) ning tekshiradigan varianti: `> MAX_DB_SAFE` → `Err`.
    ///
    /// [`IdError::NumberTooLarge`] ni query paytidan **konstruksiya paytiga** ko'chiradi.
    pub const fn try_new_db_safe(value: u64) -> Result<Self, IdError> {
        if value > Self::MAX_DB_SAFE {
            return Err(IdError::NumberTooLarge { value });
        }
        Ok(Self::new(value))
    }

    /// [`parse`](Self::parse) ning tekshiradigan varianti: `> MAX_DB_SAFE` → `Err`.
    pub fn parse_db_safe(value: &str) -> Result<Self, IdError> {
        Self::try_new_db_safe(Self::parse(value)?.0)
    }
}

impl<Tag, R: NumIdRepr> Clone for NumId<Tag, R> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Tag, R: NumIdRepr> Copy for NumId<Tag, R> {}
impl<Tag, R: NumIdRepr> PartialEq for NumId<Tag, R> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl<Tag, R: NumIdRepr> Eq for NumId<Tag, R> {}
impl<Tag, R: NumIdRepr> PartialOrd for NumId<Tag, R> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<Tag, R: NumIdRepr> Ord for NumId<Tag, R> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}
impl<Tag, R: NumIdRepr> Hash for NumId<Tag, R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}
impl<Tag, R: NumIdRepr> fmt::Debug for NumId<Tag, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = core::any::type_name::<Tag>()
            .rsplit("::")
            .next()
            .unwrap_or("?");
        write!(f, "NumId<{tag}{}>({})", R::DEBUG_SUFFIX, self.0)
    }
}
impl<Tag, R: NumIdRepr> fmt::Display for NumId<Tag, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}
impl<Tag, R: NumIdRepr> FromStr for NumId<Tag, R> {
    type Err = IdError;
    fn from_str(s: &str) -> Result<Self, IdError> {
        Self::parse(s)
    }
}

// `From<R> for NumId<Tag, R>` yozib bo'lmaydi: core'dagi `impl<T> From<T> for T`
// bilan overlap deb hisoblanadi (rustc bu yerda where-clause'ga qaramaydi).
// Shuning uchun har bir ko'rinish uchun konkret impl.
impl<Tag> From<u64> for NumId<Tag, u64> {
    fn from(value: u64) -> Self {
        Self::new(value)
    }
}
impl<Tag> From<NumId<Tag, u64>> for u64 {
    fn from(id: NumId<Tag, u64>) -> Self {
        id.0
    }
}
impl<Tag> From<i64> for NumId<Tag, i64> {
    fn from(value: i64) -> Self {
        Self::new(value)
    }
}
impl<Tag> From<NumId<Tag, i64>> for i64 {
    fn from(id: NumId<Tag, i64>) -> Self {
        id.0
    }
}

/// `u64` → `i64` ko'rinishiga o'tish: `> i64::MAX` da [`IdError::NumberTooLarge`].
impl<Tag> TryFrom<NumId<Tag, u64>> for NumId<Tag, i64> {
    type Error = IdError;
    fn try_from(id: NumId<Tag, u64>) -> Result<Self, IdError> {
        id.0.to_bigint().map(Self::new)
    }
}

/// `i64` → `u64` ko'rinishiga o'tish: manfiy qiymatda [`IdError::NumberNegative`].
impl<Tag> TryFrom<NumId<Tag, i64>> for NumId<Tag, u64> {
    type Error = IdError;
    fn try_from(id: NumId<Tag, i64>) -> Result<Self, IdError> {
        u64::from_bigint(id.0).map(Self::new)
    }
}

#[cfg(feature = "serde")]
impl<Tag, R: NumIdRepr> serde::Serialize for NumId<Tag, R> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.0.serialize_repr(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, Tag, R: NumIdRepr> serde::Deserialize<'de> for NumId<Tag, R> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        R::deserialize_repr(deserializer).map(Self::new)
    }
}

/// DB'da har ikkala ko'rinish ham `BIGINT` (`i64`).
///
/// `NumId<Tag, i64>` da `Encode`/`Decode` **total** — xato yo'li yo'q.
/// `NumId<Tag, u64>` da esa [`IdError::NumberTooLarge`] (Encode) va
/// [`IdError::NumberNegative`] (Decode) bo'lishi mumkin — bu xatolar strukturali,
/// `BoxDynError` ichida ham `downcast_ref::<IdError>()` bilan ushlanadi.
#[cfg(feature = "sqlx")]
mod sqlx_impls {
    use super::{NumId, NumIdRepr};

    impl<DB: sqlx::Database, Tag, R: NumIdRepr> sqlx::Type<DB> for NumId<Tag, R>
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

    impl<'q, DB: sqlx::Database, Tag, R: NumIdRepr> sqlx::Encode<'q, DB> for NumId<Tag, R>
    where
        i64: sqlx::Encode<'q, DB>,
    {
        fn encode_by_ref(
            &self,
            buf: &mut DB::ArgumentBuffer,
        ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
            let value = self.0.to_bigint()?;
            <i64 as sqlx::Encode<'q, DB>>::encode_by_ref(&value, buf)
        }
    }

    impl<'r, DB: sqlx::Database, Tag, R: NumIdRepr> sqlx::Decode<'r, DB> for NumId<Tag, R>
    where
        i64: sqlx::Decode<'r, DB>,
    {
        fn decode(value: DB::ValueRef<'r>) -> Result<Self, sqlx::error::BoxDynError> {
            let raw = <i64 as sqlx::Decode<'r, DB>>::decode(value)?;
            Ok(Self::new(R::from_bigint(raw)?))
        }
    }

    #[cfg(feature = "sqlx-postgres")]
    impl<Tag, R: NumIdRepr> sqlx::postgres::PgHasArrayType for NumId<Tag, R> {
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
    type SignedOrderId = NumId<Order, i64>;

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
        assert_eq!(format!("{n:?}"), "NumId<Order>(42)");
        assert!(LegacyOrderId::parse("-1").is_err());
        assert!(LegacyOrderId::parse("+1").is_err());
        assert!(LegacyOrderId::parse("").is_err());
    }

    #[test]
    fn signed_repr_accepts_negative_legacy_ids() {
        let n = SignedOrderId::parse(" -1 ").unwrap();
        assert_eq!(n.get(), -1);
        assert_eq!(n.to_string(), "-1");
        assert_eq!(format!("{n:?}"), "NumId<Order, i64>(-1)");
        assert_eq!(SignedOrderId::parse(&n.to_string()).unwrap(), n); // roundtrip
        assert!(SignedOrderId::parse("+1").is_err());
        assert!(SignedOrderId::parse("-").is_err());
        assert!(SignedOrderId::parse("").is_err());

        // i64 ko'rinishida BIGINT konversiyasi total — xato yo'li yo'q.
        assert!(SignedOrderId::new(i64::MIN).is_db_safe());
        assert_eq!(SignedOrderId::new(i64::MAX).to_bigint(), Ok(i64::MAX));
    }

    #[test]
    fn db_safe_bounds_move_the_error_to_construction() {
        const MAX: u64 = LegacyOrderId::MAX_DB_SAFE;
        assert_eq!(MAX, i64::MAX as u64);

        // `new`/`parse` kengroq — xato faqat Encode'da chiqardi.
        let too_big = LegacyOrderId::new(MAX + 1);
        assert!(!too_big.is_db_safe());
        assert_eq!(
            too_big.to_bigint(),
            Err(IdError::NumberTooLarge { value: MAX + 1 })
        );

        // `*_db_safe` uni konstruksiya paytiga ko'chiradi.
        assert_eq!(LegacyOrderId::try_new_db_safe(MAX).unwrap().get(), MAX);
        assert_eq!(
            LegacyOrderId::try_new_db_safe(MAX + 1),
            Err(IdError::NumberTooLarge { value: MAX + 1 })
        );
        assert!(LegacyOrderId::parse_db_safe("9223372036854775808").is_err());
        assert!(LegacyOrderId::parse_db_safe("9223372036854775807").is_ok());
    }

    #[test]
    fn repr_conversions_are_checked_both_ways() {
        assert_eq!(
            SignedOrderId::try_from(LegacyOrderId::new(42))
                .unwrap()
                .get(),
            42
        );
        assert_eq!(
            SignedOrderId::try_from(LegacyOrderId::new(u64::MAX)),
            Err(IdError::NumberTooLarge { value: u64::MAX })
        );
        assert_eq!(
            LegacyOrderId::try_from(SignedOrderId::new(42))
                .unwrap()
                .get(),
            42
        );
        assert_eq!(
            LegacyOrderId::try_from(SignedOrderId::new(-1)),
            Err(IdError::NumberNegative { value: -1 })
        );
    }

    #[test]
    fn id_is_send_sync_regardless_of_tag() {
        fn assert_send_sync<T: Send + Sync>() {}
        #[allow(dead_code)]
        struct NotSync(core::cell::Cell<u8>);
        assert_send_sync::<Id<NotSync>>();
        assert_send_sync::<NumId<NotSync>>();
        assert_send_sync::<NumId<NotSync, i64>>();
    }
}
