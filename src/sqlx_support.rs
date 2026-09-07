//! `sqlx` integratsiyasi (feature = "sqlx-0_8" va/yoki "sqlx-0_9").
//!
//! Har bir tip DB'da o'zining "ichki" tipi orqali ifodalanadi:
//! `String` → `TEXT`/`VARCHAR`, `Uuid` → `UUID`, `NaiveDate` → `DATE`, `i64` → `BIGINT`.
//!
//! Implementatsiya **driver'ga bog'liq emas** (`DB: Database`), ya'ni `postgres`,
//! `mysql`, `sqlite` — foydalanuvchi qaysi driver'ni yoqsa, shu bilan ishlaydi.
//! `PgHasArrayType` (`Vec<T>` / `= ANY($1)`) faqat `*-postgres` feature'da.
//!
//! **Decode har doim validatsiyadan o'tadi** (`#[sqlx(transparent)]` derive'dan farqli):
//! DB'dagi buzuq yozuv `Row::try_get` da xato beradi — jimgina ichkariga kirmaydi.
//!
//! # Nega ikki versiya
//!
//! SQLx 0.8 va 0.9 orasidagi yagona nomuvofiqlik — `Database::ArgumentBuffer`:
//! 0.8 da u lifetime'li GAT (`ArgumentBuffer<'q>`), 0.9 da lifetime'siz. Qolgan
//! butun sirt (`Type`, `Encode`, `Decode`, `PgHasArrayType::array_compatible`,
//! `Query::try_bind`) bir xil.
//!
//! `sqlx_0_8::Type` va `sqlx_0_9::Type` — **turli crate'lardagi turli trait'lar**,
//! shuning uchun bitta tip uchun ikkalasini ham implement qilish coherence buzmaydi
//! va feature'lar o'zaro istisno emas. Shu sabab bu yerda `compile_error!` guard'i
//! yo'q: dependency grafida ikkala feature unify bo'lsa ham crate kompilyatsiya
//! bo'ladi. Amalda bittasini tanlang — ikkalasi ikkita sqlx daraxtini tortadi.

/// `Type` + `Decode` — ikkala sqlx liniyasida bir xil.
macro_rules! sqlx_via_type_decode {
    ($sqlx:ident, $Name:ty $([$($G:ident),+])?, $Inner:ty, $decode:expr) => {
        impl<DB: ::$sqlx::Database $($(, $G)+)?> ::$sqlx::Type<DB> for $Name
        where
            $Inner: ::$sqlx::Type<DB>,
        {
            fn type_info() -> <DB as ::$sqlx::Database>::TypeInfo {
                <$Inner as ::$sqlx::Type<DB>>::type_info()
            }

            fn compatible(ty: &<DB as ::$sqlx::Database>::TypeInfo) -> bool {
                <$Inner as ::$sqlx::Type<DB>>::compatible(ty)
            }
        }

        impl<'r, DB: ::$sqlx::Database $($(, $G)+)?> ::$sqlx::Decode<'r, DB> for $Name
        where
            $Inner: ::$sqlx::Decode<'r, DB>,
        {
            fn decode(
                value: <DB as ::$sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, ::$sqlx::error::BoxDynError> {
                let inner = <$Inner as ::$sqlx::Decode<'r, DB>>::decode(value)?;
                Ok(($decode)(inner)?)
            }
        }
    };
}

/// `Encode` — sqlx 0.8: `ArgumentBuffer<'q>` (lifetime'li GAT).
///
/// Tana 0.9 variantidan faqat shu tip bilan farq qiladi; lifetime tokenini
/// makrolar orasida uzatmaslik uchun ikkita alohida makro saqlanadi.
#[cfg(feature = "sqlx-0_8")]
macro_rules! sqlx_via_encode_0_8 {
    ($sqlx:ident, $Name:ty $([$($G:ident),+])?, $Inner:ty, $encode_ref:expr) => {
        impl<'q, DB: ::$sqlx::Database $($(, $G)+)?> ::$sqlx::Encode<'q, DB> for $Name
        where
            $Inner: ::$sqlx::Encode<'q, DB>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <DB as ::$sqlx::Database>::ArgumentBuffer<'q>,
            ) -> Result<::$sqlx::encode::IsNull, ::$sqlx::error::BoxDynError> {
                // fn-pointer'ga coercion: closure'ga `for<'a> fn(&'a Self) -> &'a Inner`
                // imzosini beradi (closure'larda qaytariladigan lifetime elision ishlamaydi).
                let get: fn(&Self) -> &$Inner = $encode_ref;
                <$Inner as ::$sqlx::Encode<'q, DB>>::encode_by_ref(get(self), buf)
            }
        }
    };
}

/// `Encode` — sqlx 0.9: `ArgumentBuffer` (lifetime'siz).
#[cfg(feature = "sqlx-0_9")]
macro_rules! sqlx_via_encode_0_9 {
    ($sqlx:ident, $Name:ty $([$($G:ident),+])?, $Inner:ty, $encode_ref:expr) => {
        impl<'q, DB: ::$sqlx::Database $($(, $G)+)?> ::$sqlx::Encode<'q, DB> for $Name
        where
            $Inner: ::$sqlx::Encode<'q, DB>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <DB as ::$sqlx::Database>::ArgumentBuffer,
            ) -> Result<::$sqlx::encode::IsNull, ::$sqlx::error::BoxDynError> {
                let get: fn(&Self) -> &$Inner = $encode_ref;
                <$Inner as ::$sqlx::Encode<'q, DB>>::encode_by_ref(get(self), buf)
            }
        }
    };
}

/// `PgHasArrayType` — `array_compatible` ichki tipga delegatsiya qilinadi, shuning
/// uchun `TEXT[]` bilan birga `VARCHAR[]`, `array_agg(VARCHAR)` va `= ANY($1)` ham
/// ishlaydi. Ikkala sqlx liniyasida bir xil.
#[cfg(any(feature = "sqlx-0_8-postgres", feature = "sqlx-0_9-postgres"))]
macro_rules! sqlx_via_pg_array {
    ($sqlx:ident, $Name:ty $([$($G:ident),+])?, $Inner:ty) => {
        impl$(<$($G),+>)? ::$sqlx::postgres::PgHasArrayType for $Name
        where
            $Inner: ::$sqlx::postgres::PgHasArrayType,
        {
            fn array_type_info() -> ::$sqlx::postgres::PgTypeInfo {
                <$Inner as ::$sqlx::postgres::PgHasArrayType>::array_type_info()
            }

            fn array_compatible(ty: &::$sqlx::postgres::PgTypeInfo) -> bool {
                <$Inner as ::$sqlx::postgres::PgHasArrayType>::array_compatible(ty)
            }
        }
    };
}

/// Ichki tip orqali SQLx trait'larini beradi — yoqilgan har bir sqlx liniyasi uchun.
macro_rules! sqlx_via {
    (
        $Name:ty $([$($G:ident),+])?,
        $Inner:ty,
        $decode:expr,
        $encode_ref:expr
    ) => {
        #[cfg(feature = "sqlx-0_8")]
        $crate::sqlx_support::sqlx_via_type_decode!(
            sqlx_0_8, $Name $([$($G),+])?, $Inner, $decode
        );
        #[cfg(feature = "sqlx-0_8")]
        $crate::sqlx_support::sqlx_via_encode_0_8!(
            sqlx_0_8, $Name $([$($G),+])?, $Inner, $encode_ref
        );
        #[cfg(feature = "sqlx-0_8-postgres")]
        $crate::sqlx_support::sqlx_via_pg_array!(
            sqlx_0_8, $Name $([$($G),+])?, $Inner
        );

        #[cfg(feature = "sqlx-0_9")]
        $crate::sqlx_support::sqlx_via_type_decode!(
            sqlx_0_9, $Name $([$($G),+])?, $Inner, $decode
        );
        #[cfg(feature = "sqlx-0_9")]
        $crate::sqlx_support::sqlx_via_encode_0_9!(
            sqlx_0_9, $Name $([$($G),+])?, $Inner, $encode_ref
        );
        #[cfg(feature = "sqlx-0_9-postgres")]
        $crate::sqlx_support::sqlx_via_pg_array!(
            sqlx_0_9, $Name $([$($G),+])?, $Inner
        );
    };
}

pub(crate) use {sqlx_via, sqlx_via_type_decode};

#[cfg(feature = "sqlx-0_8")]
pub(crate) use sqlx_via_encode_0_8;
#[cfg(feature = "sqlx-0_9")]
pub(crate) use sqlx_via_encode_0_9;
#[cfg(any(feature = "sqlx-0_8-postgres", feature = "sqlx-0_9-postgres"))]
pub(crate) use sqlx_via_pg_array;
