//! sqlx trait'lari compile-time'da mavjudligini qulflaydi (DB kerak emas).
#![cfg(feature = "sqlx-postgres")]

use sqlx::postgres::{PgHasArrayType, Postgres};
use sqlx::{Decode, Encode, Type};
use uz_types::prelude::*;

fn assert_pg_type<T>()
where
    T: Type<Postgres>
        + for<'q> Encode<'q, Postgres>
        + for<'r> Decode<'r, Postgres>
        + PgHasArrayType,
{
}

#[test]
fn all_types_implement_postgres_traits() {
    assert_pg_type::<Passport>();
    assert_pg_type::<Pinfl>();
    assert_pg_type::<PhoneNumber>();
    assert_pg_type::<EmailAddress>();

    #[cfg(feature = "date")]
    assert_pg_type::<BirthDate>();

    #[cfg(feature = "id")]
    {
        enum Order {}
        assert_pg_type::<Id<Order>>();
        assert_pg_type::<NumId<Order>>();
        assert_pg_type::<NumId<Order, i64>>();
        assert_eq!(
            <Id<Order> as Type<Postgres>>::type_info(),
            <uuid::Uuid as Type<Postgres>>::type_info()
        );
        // Ikkala repr ham bir xil ustunga (`BIGINT`) tushadi — farq faqat
        // Encode/Decode ning xato yo'li bor-yo'qligida.
        assert_eq!(
            <NumId<Order> as Type<Postgres>>::type_info(),
            <i64 as Type<Postgres>>::type_info()
        );
        assert_eq!(
            <NumId<Order, i64> as Type<Postgres>>::type_info(),
            <i64 as Type<Postgres>>::type_info()
        );
        assert_vec::<Vec<NumId<Order, i64>>>();
    }

    assert_eq!(
        <Passport as Type<Postgres>>::type_info(),
        <String as Type<Postgres>>::type_info()
    );
    // Vec<T> (= ANY($1)) ham ishlaydi
    fn assert_vec<T: Type<Postgres>>() {}
    assert_vec::<Vec<Passport>>();
}

fn assert_string_array_compatibility<T: PgHasArrayType>() {
    let text_array = <Vec<String> as Type<Postgres>>::type_info();
    let bigint_array = <Vec<i64> as Type<Postgres>>::type_info();
    let integer_array = <Vec<i32> as Type<Postgres>>::type_info();

    assert_eq!(
        T::array_type_info(),
        <String as PgHasArrayType>::array_type_info()
    );
    assert_eq!(
        T::array_compatible(&text_array),
        <String as PgHasArrayType>::array_compatible(&text_array)
    );
    assert!(!T::array_compatible(&bigint_array));
    assert!(!T::array_compatible(&integer_array));
}

#[test]
fn string_newtypes_delegate_array_compatibility() {
    assert_string_array_compatibility::<Passport>();
    assert_string_array_compatibility::<Pinfl>();
    assert_string_array_compatibility::<PhoneNumber>();
    assert_string_array_compatibility::<EmailAddress>();
}

#[cfg(feature = "id")]
#[test]
fn num_ids_delegate_array_compatibility() {
    enum Order {}

    assert_eq!(
        <NumId<Order> as PgHasArrayType>::array_type_info(),
        <i64 as PgHasArrayType>::array_type_info()
    );
    assert_eq!(
        <NumId<Order, i64> as PgHasArrayType>::array_type_info(),
        <i64 as PgHasArrayType>::array_type_info()
    );

    for ty in [
        <Vec<i64> as Type<Postgres>>::type_info(),
        <Vec<i32> as Type<Postgres>>::type_info(),
        <Vec<String> as Type<Postgres>>::type_info(),
    ] {
        assert_eq!(
            <NumId<Order> as PgHasArrayType>::array_compatible(&ty),
            <i64 as PgHasArrayType>::array_compatible(&ty)
        );
        assert_eq!(
            <NumId<Order, i64> as PgHasArrayType>::array_compatible(&ty),
            <i64 as PgHasArrayType>::array_compatible(&ty)
        );
    }
}
