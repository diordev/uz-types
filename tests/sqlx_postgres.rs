#![cfg(feature = "sqlx-postgres")]

use core::fmt::Debug;

use sqlx::postgres::{PgPool, Postgres};
use sqlx::{Decode, Encode, Row, Type};
use uz_types::prelude::*;

async fn assert_roundtrip<T>(pool: &PgPool, sql: &'static str, value: T) -> Result<(), sqlx::Error>
where
    T: Clone
        + Debug
        + Eq
        + Send
        + Unpin
        + Type<Postgres>
        + for<'q> Encode<'q, Postgres>
        + for<'r> Decode<'r, Postgres>
        + 'static,
{
    let actual = sqlx::query_scalar::<_, T>(sql)
        .bind(value.clone())
        .fetch_one(pool)
        .await?;
    assert_eq!(actual, value);
    Ok(())
}

#[sqlx::test(migrations = false)]
#[ignore = "jonli PostgreSQL: `just postgres-test` bilan ishga tushiring"]
async fn string_scalars_roundtrip_normalize_and_decode_null(
    pool: PgPool,
) -> Result<(), sqlx::Error> {
    let passport = Passport::parse("AA1234567").unwrap();
    let pinfl = Pinfl::parse("31210932040247").unwrap();
    let phone = PhoneNumber::parse("998901234567").unwrap();
    let email = EmailAddress::parse("user@example.uz").unwrap();

    for sql in ["SELECT $1::TEXT", "SELECT $1::VARCHAR"] {
        assert_roundtrip(&pool, sql, passport.clone()).await?;
        assert_roundtrip(&pool, sql, pinfl.clone()).await?;
        assert_roundtrip(&pool, sql, phone.clone()).await?;
        assert_roundtrip(&pool, sql, email.clone()).await?;
    }

    sqlx::query(
        "CREATE TABLE domain_values (
            id INTEGER PRIMARY KEY,
            passport_text TEXT,
            passport_varchar VARCHAR,
            pinfl_text TEXT,
            pinfl_varchar VARCHAR,
            phone_text TEXT,
            phone_varchar VARCHAR,
            email_text TEXT,
            email_varchar VARCHAR
        )",
    )
    .execute(&pool)
    .await?;

    sqlx::query(
        "INSERT INTO domain_values VALUES
            (1, $1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(passport.clone())
    .bind(passport.clone())
    .bind(pinfl.clone())
    .bind(pinfl.clone())
    .bind(phone.clone())
    .bind(phone.clone())
    .bind(email.clone())
    .bind(email.clone())
    .execute(&pool)
    .await?;

    let row = sqlx::query("SELECT * FROM domain_values WHERE id = 1")
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.try_get::<Passport, _>("passport_text")?, passport);
    assert_eq!(row.try_get::<Passport, _>("passport_varchar")?, passport);
    assert_eq!(row.try_get::<Pinfl, _>("pinfl_text")?, pinfl);
    assert_eq!(row.try_get::<Pinfl, _>("pinfl_varchar")?, pinfl);
    assert_eq!(row.try_get::<PhoneNumber, _>("phone_text")?, phone);
    assert_eq!(row.try_get::<PhoneNumber, _>("phone_varchar")?, phone);
    assert_eq!(row.try_get::<EmailAddress, _>("email_text")?, email);
    assert_eq!(row.try_get::<EmailAddress, _>("email_varchar")?, email);

    sqlx::query(
        "INSERT INTO domain_values VALUES
            (2,
             ' aa 1234567 ', ' aa 1234567 ',
             ' 31210932040247 ', ' 31210932040247 ',
             '+998 (90) 123-45-67', '+998 (90) 123-45-67',
             ' USER@EXAMPLE.UZ ', ' USER@EXAMPLE.UZ '),
            (3, NULL, NULL, NULL, NULL, NULL, NULL, NULL, NULL)",
    )
    .execute(&pool)
    .await?;

    let row = sqlx::query("SELECT * FROM domain_values WHERE id = 2")
        .fetch_one(&pool)
        .await?;
    assert_eq!(
        row.try_get::<Passport, _>("passport_varchar")?.as_str(),
        "AA1234567"
    );
    assert_eq!(
        row.try_get::<Pinfl, _>("pinfl_text")?.as_str(),
        "31210932040247"
    );
    assert_eq!(
        row.try_get::<PhoneNumber, _>("phone_varchar")?.as_str(),
        "998901234567"
    );
    assert_eq!(
        row.try_get::<EmailAddress, _>("email_text")?.as_str(),
        "user@example.uz"
    );

    let row = sqlx::query("SELECT * FROM domain_values WHERE id = 3")
        .fetch_one(&pool)
        .await?;
    assert_eq!(row.try_get::<Option<Passport>, _>("passport_text")?, None);
    assert_eq!(row.try_get::<Option<Pinfl>, _>("pinfl_varchar")?, None);
    assert_eq!(row.try_get::<Option<PhoneNumber>, _>("phone_text")?, None);
    assert_eq!(
        row.try_get::<Option<EmailAddress>, _>("email_varchar")?,
        None
    );

    Ok(())
}

#[cfg(feature = "date")]
#[sqlx::test(migrations = false)]
#[ignore = "jonli PostgreSQL: `just postgres-test` bilan ishga tushiring"]
async fn birth_date_roundtrips_and_decodes_null(pool: PgPool) -> Result<(), sqlx::Error> {
    let birth_date = BirthDate::parse("1990-05-15").unwrap();
    assert_roundtrip(&pool, "SELECT $1::DATE", birth_date).await?;

    let null = sqlx::query_scalar::<_, Option<BirthDate>>("SELECT NULL::DATE")
        .fetch_one(&pool)
        .await?;
    assert_eq!(null, None);
    Ok(())
}

#[cfg(feature = "id")]
#[sqlx::test(migrations = false)]
#[ignore = "jonli PostgreSQL: `just postgres-test` bilan ishga tushiring"]
async fn ids_roundtrip_boundaries_and_decode_null(pool: PgPool) -> Result<(), sqlx::Error> {
    enum Live {}

    let id = Id::<Live>::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap();
    assert_roundtrip(&pool, "SELECT $1::UUID", id).await?;

    for value in [0, i64::MAX as u64] {
        assert_roundtrip(&pool, "SELECT $1::BIGINT", NumId::<Live>::new(value)).await?;
    }
    for value in [i64::MIN, -1, 0, i64::MAX] {
        assert_roundtrip(&pool, "SELECT $1::BIGINT", NumId::<Live, i64>::new(value)).await?;
    }

    let id_null = sqlx::query_scalar::<_, Option<Id<Live>>>("SELECT NULL::UUID")
        .fetch_one(&pool)
        .await?;
    let unsigned_null = sqlx::query_scalar::<_, Option<NumId<Live>>>("SELECT NULL::BIGINT")
        .fetch_one(&pool)
        .await?;
    let signed_null = sqlx::query_scalar::<_, Option<NumId<Live, i64>>>("SELECT NULL::BIGINT")
        .fetch_one(&pool)
        .await?;
    assert_eq!(id_null, None);
    assert_eq!(unsigned_null, None);
    assert_eq!(signed_null, None);
    Ok(())
}

#[sqlx::test(migrations = false)]
#[ignore = "jonli PostgreSQL: `just postgres-test` bilan ishga tushiring"]
async fn postgres_arrays_roundtrip(pool: PgPool) -> Result<(), sqlx::Error> {
    let expected = vec![
        Passport::parse("AA1234567").unwrap(),
        Passport::parse("BB7654321").unwrap(),
    ];

    let text =
        sqlx::query_scalar::<_, Vec<Passport>>("SELECT ARRAY['AA1234567', ' bb 7654321 ']::TEXT[]")
            .fetch_one(&pool)
            .await?;
    let varchar = sqlx::query_scalar::<_, Vec<Passport>>(
        "SELECT ARRAY['AA1234567', ' bb 7654321 ']::VARCHAR[]",
    )
    .fetch_one(&pool)
    .await?;
    let aggregated = sqlx::query_scalar::<_, Vec<Passport>>(
        "SELECT array_agg(value ORDER BY ordinal)
         FROM (VALUES (1, 'AA1234567'::VARCHAR), (2, 'bb7654321'::VARCHAR))
              AS rows(ordinal, value)",
    )
    .fetch_one(&pool)
    .await?;
    assert_eq!(text, expected);
    assert_eq!(varchar, expected);
    assert_eq!(aggregated, expected);

    let matched = sqlx::query_scalar::<_, String>(
        "SELECT value FROM (VALUES ('AA1234567'::VARCHAR), ('CC0000000'::VARCHAR)) AS rows(value) WHERE value = ANY($1) ORDER BY value",
    )
    .bind(expected.clone())
    .fetch_all(&pool)
    .await?;
    assert_eq!(matched, vec![String::from("AA1234567")]);

    #[cfg(feature = "id")]
    {
        enum Live {}
        let ids = vec![
            NumId::<Live, i64>::new(i64::MIN),
            NumId::<Live, i64>::new(-1),
            NumId::<Live, i64>::new(0),
            NumId::<Live, i64>::new(i64::MAX),
        ];
        assert_roundtrip(&pool, "SELECT $1::BIGINT[]", ids).await?;
    }

    Ok(())
}

#[sqlx::test(migrations = false)]
#[ignore = "jonli PostgreSQL: `just postgres-test` bilan ishga tushiring"]
async fn domain_errors_survive_sqlx_wrapping(pool: PgPool) -> Result<(), sqlx::Error> {
    let row = sqlx::query("SELECT 'nope'::TEXT AS value")
        .fetch_one(&pool)
        .await?;
    let error = row.try_get::<Passport, _>("value").unwrap_err();
    match error {
        sqlx::Error::ColumnDecode { source, .. } => {
            assert_eq!(
                source.downcast_ref::<PassportError>(),
                Some(&PassportError::Length)
            );
        }
        other => panic!("ColumnDecode kutilgan edi, olindi: {other:?}"),
    }

    #[cfg(feature = "id")]
    {
        enum Live {}

        let too_large_value = i64::MAX as u64 + 1;
        let too_large = NumId::<Live>::new(too_large_value);
        let mut query = sqlx::query::<Postgres>("SELECT $1::BIGINT");
        let bind_error = query.try_bind(too_large).unwrap_err();
        assert_eq!(
            bind_error.downcast_ref::<IdError>(),
            Some(&IdError::NumberTooLarge {
                value: too_large_value
            })
        );

        // `bind()` SQLx ichida encode xatosini matnga aylantiradi; shu sabab
        // tashqi SQLx varianti alohida, aniq domen xatosi esa `try_bind()` bilan
        // tekshiriladi.
        let error = sqlx::query_scalar::<_, i64>("SELECT $1::BIGINT")
            .bind(NumId::<Live>::new(too_large_value))
            .fetch_one(&pool)
            .await
            .unwrap_err();
        assert!(matches!(error, sqlx::Error::Encode(_)));

        let row = sqlx::query("SELECT -1::BIGINT AS value")
            .fetch_one(&pool)
            .await?;
        let error = row.try_get::<NumId<Live>, _>("value").unwrap_err();
        match error {
            sqlx::Error::ColumnDecode { source, .. } => {
                assert_eq!(
                    source.downcast_ref::<IdError>(),
                    Some(&IdError::NumberNegative { value: -1 })
                );
            }
            other => panic!("ColumnDecode kutilgan edi, olindi: {other:?}"),
        }
    }

    Ok(())
}
