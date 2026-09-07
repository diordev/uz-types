//! Ikkala sqlx liniyasi bir xil DB xulqini berishini qulflaydi (DB kerak emas).
//!
//! `sqlx_0_8::Type` va `sqlx_0_9::Type` — turli trait'lar, shuning uchun ularning
//! natijalarini bevosita `assert_eq!` qilib bo'lmaydi: PostgreSQL tip nomi
//! (`PgTypeInfo: Display`) va boolean natijalar orqali solishtiriladi.
#![cfg(all(feature = "sqlx-0_8-postgres", feature = "sqlx-0_9-postgres"))]

use uz_types::prelude::*;

/// `Type::type_info()` ning PostgreSQL tip nomi.
macro_rules! type_name {
    ($sqlx:ident, $T:ty) => {
        <$T as ::$sqlx::Type<::$sqlx::postgres::Postgres>>::type_info().to_string()
    };
}

/// `PgHasArrayType::array_type_info()` ning tip nomi.
macro_rules! array_name {
    ($sqlx:ident, $T:ty) => {
        <$T as ::$sqlx::postgres::PgHasArrayType>::array_type_info().to_string()
    };
}

/// `$T` `$Probe` ustuniga mos keladimi.
macro_rules! compatible_with {
    ($sqlx:ident, $T:ty, $Probe:ty) => {
        <$T as ::$sqlx::Type<::$sqlx::postgres::Postgres>>::compatible(&<$Probe as ::$sqlx::Type<
            ::$sqlx::postgres::Postgres,
        >>::type_info())
    };
}

/// `Vec<$T>` `$Probe` massiv ustuniga mos keladimi.
macro_rules! array_compatible_with {
    ($sqlx:ident, $T:ty, $Probe:ty) => {
        <$T as ::$sqlx::postgres::PgHasArrayType>::array_compatible(&<$Probe as ::$sqlx::Type<
            ::$sqlx::postgres::Postgres,
        >>::type_info())
    };
}

/// Bitta tip uchun to'rtala sirtni ikki versiyada solishtiradi.
macro_rules! assert_parity {
    ($T:ty, $Probe:ty, $ArrayProbe:ty) => {
        assert_eq!(
            type_name!(sqlx_0_8, $T),
            type_name!(sqlx_0_9, $T),
            concat!(stringify!($T), ": type_info() nomi farq qildi")
        );
        assert_eq!(
            array_name!(sqlx_0_8, $T),
            array_name!(sqlx_0_9, $T),
            concat!(stringify!($T), ": array_type_info() nomi farq qildi")
        );
        assert_eq!(
            compatible_with!(sqlx_0_8, $T, $Probe),
            compatible_with!(sqlx_0_9, $T, $Probe),
            concat!(stringify!($T), ": compatible() natijasi farq qildi")
        );
        assert_eq!(
            array_compatible_with!(sqlx_0_8, $T, $ArrayProbe),
            array_compatible_with!(sqlx_0_9, $T, $ArrayProbe),
            concat!(stringify!($T), ": array_compatible() natijasi farq qildi")
        );
    };
}

#[test]
fn string_newtypes_behave_identically_in_both_sqlx_lines() {
    assert_parity!(Passport, String, Vec<String>);
    assert_parity!(Pinfl, String, Vec<String>);
    assert_parity!(PhoneNumber, String, Vec<String>);
    assert_parity!(EmailAddress, String, Vec<String>);

    // Salbiy yo'l ham bir xil: string tiplari sonli massivga mos kelmaydi.
    assert_parity!(Passport, i64, Vec<i64>);
    assert!(!array_compatible_with!(sqlx_0_8, Passport, Vec<i64>));
    assert!(!array_compatible_with!(sqlx_0_9, Passport, Vec<i64>));
}

#[cfg(feature = "date")]
#[test]
fn birth_date_behaves_identically_in_both_sqlx_lines() {
    assert_parity!(BirthDate, chrono::NaiveDate, Vec<chrono::NaiveDate>);
}

#[cfg(feature = "id")]
#[test]
fn ids_behave_identically_in_both_sqlx_lines() {
    enum Order {}

    assert_parity!(Id<Order>, uuid::Uuid, Vec<uuid::Uuid>);
    assert_parity!(NumId<Order>, i64, Vec<i64>);
    assert_parity!(NumId<Order, i64>, i64, Vec<i64>);

    // `NumId` ikkala reprda ham `BIGINT`, ikkala versiyada ham.
    assert_eq!(
        type_name!(sqlx_0_8, NumId<Order>),
        type_name!(sqlx_0_8, i64)
    );
    assert_eq!(
        type_name!(sqlx_0_9, NumId<Order>),
        type_name!(sqlx_0_9, i64)
    );
}
