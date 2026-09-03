//! `cargo bench` — "minimal allocation" da'vosini o'lchov bilan tasdiqlash uchun.
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use uz_types::prelude::*;

fn parse_benches(c: &mut Criterion) {
    let mut g = c.benchmark_group("parse");
    g.bench_function("passport/&str", |b| {
        b.iter(|| Passport::parse(black_box("aa1234567")).unwrap())
    });
    g.bench_function("passport/String", |b| {
        b.iter_batched(
            || String::from("AA1234567"),
            |s| Passport::try_from(s).unwrap(),
            criterion::BatchSize::SmallInput,
        )
    });
    g.bench_function("phone/separators", |b| {
        b.iter(|| PhoneNumber::parse(black_box("+998 (90) 123-45-67")).unwrap())
    });
    g.bench_function("email", |b| {
        b.iter(|| EmailAddress::parse(black_box("User.Name@Example.com")).unwrap())
    });
    g.bench_function("pinfl/strict", |b| {
        b.iter(|| Pinfl::parse_strict(black_box("31210932040247")).unwrap())
    });
    g.finish();
}

criterion_group!(benches, parse_benches);
criterion_main!(benches);
