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
