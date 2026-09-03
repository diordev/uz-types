//! `sqlx` integratsiyasi (feature = "sqlx").
//!
//! Har bir tip DB'da o'zining "ichki" tipi orqali ifodalanadi:
//! `String` → `TEXT`/`VARCHAR`, `Uuid` → `UUID`, `NaiveDate` → `DATE`, `i64` → `BIGINT`.
//!
//! Implementatsiya **driver'ga bog'liq emas** (`DB: Database`), ya'ni `postgres`,
//! `mysql`, `sqlite` — foydalanuvchi qaysi driver'ni yoqsa, shu bilan ishlaydi.
//! `PgHasArrayType` (`Vec<T>` / `= ANY($1)`) faqat `sqlx-postgres` feature'da.
//!
//! **Decode har doim validatsiyadan o'tadi** (`#[sqlx(transparent)]` derive'dan farqli):
//! DB'dagi buzuq yozuv `Row::try_get` da xato beradi — jimgina ichkariga kirmaydi.

macro_rules! sqlx_via {
    (
        $Name:ty $([$($G:ident),+])?,
        $Inner:ty,
        $decode:expr,
        $encode_ref:expr
    ) => {
        impl<DB: ::sqlx::Database $($(, $G)+)?> ::sqlx::Type<DB> for $Name
        where
            $Inner: ::sqlx::Type<DB>,
        {
            fn type_info() -> <DB as ::sqlx::Database>::TypeInfo {
                <$Inner as ::sqlx::Type<DB>>::type_info()
            }

            fn compatible(ty: &<DB as ::sqlx::Database>::TypeInfo) -> bool {
                <$Inner as ::sqlx::Type<DB>>::compatible(ty)
            }
        }

        impl<'q, DB: ::sqlx::Database $($(, $G)+)?> ::sqlx::Encode<'q, DB> for $Name
        where
            $Inner: ::sqlx::Encode<'q, DB>,
        {
            fn encode_by_ref(
                &self,
                buf: &mut <DB as ::sqlx::Database>::ArgumentBuffer,
            ) -> Result<::sqlx::encode::IsNull, ::sqlx::error::BoxDynError> {
                // fn-pointer'ga coercion: closure'ga `for<'a> fn(&'a Self) -> &'a Inner`
                // imzosini beradi (closure'larda qaytariladigan lifetime elision ishlamaydi).
                let get: fn(&Self) -> &$Inner = $encode_ref;
                <$Inner as ::sqlx::Encode<'q, DB>>::encode_by_ref(get(self), buf)
            }
        }

        impl<'r, DB: ::sqlx::Database $($(, $G)+)?> ::sqlx::Decode<'r, DB> for $Name
        where
            $Inner: ::sqlx::Decode<'r, DB>,
        {
            fn decode(
                value: <DB as ::sqlx::Database>::ValueRef<'r>,
            ) -> Result<Self, ::sqlx::error::BoxDynError> {
                let inner = <$Inner as ::sqlx::Decode<'r, DB>>::decode(value)?;
                Ok(($decode)(inner)?)
            }
        }

        #[cfg(feature = "sqlx-postgres")]
        impl$(<$($G),+>)? ::sqlx::postgres::PgHasArrayType for $Name
        where
            $Inner: ::sqlx::postgres::PgHasArrayType,
        {
            fn array_type_info() -> ::sqlx::postgres::PgTypeInfo {
                <$Inner as ::sqlx::postgres::PgHasArrayType>::array_type_info()
            }
        }
    };
}

pub(crate) use sqlx_via;
