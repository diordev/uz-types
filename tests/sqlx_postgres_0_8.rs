//! Jonli PostgreSQL roundtrip'lari — sqlx 0.8.
//!
//! Tana [`common/postgres_suite.rs`](common/postgres_suite.rs) da; bu yerda faqat
//! versiya tanlanadi. Barcha testlar `#[ignore]` — oddiy `cargo test` DB so'ramaydi.
//! Ishga tushirish: `just postgres-test-08`.
#![cfg(feature = "sqlx-0_8-postgres")]

extern crate sqlx_0_8 as sqlx;

include!("common/postgres_suite.rs");
