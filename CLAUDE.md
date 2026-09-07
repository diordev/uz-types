# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

`uz-types` — O'zbekiston domeni uchun value object va tipli ID kutubxonasi (crates.io da nashr qilinadi).
Hujjatlar, izohlar, commit va CHANGELOG **o'zbek tilida**; kod identifikatorlari ingliz tilida.

> **Kod yozishdan oldin:** `.claude/skills/idiomatic-rust/SKILL.md` ni o'qing — bu repodagi
> idiomatik Rust qoidalari (type-driven design, allocation intizomi, xato dizayni, MSRV chegaralari).

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, invoke the `skill` tool with `skill: "graphify"` before doing anything else.

Rules:

- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).

## Buyruqlar

Talab: [`just`](https://just.systems). Qo'shimcha: `cargo-hack`, `cargo-audit`, `cargo-machete`,
`cargo-semver-checks`, `rustup toolchain install 1.85.0 1.94.0`.

```bash
just check   # TEZ (~3s warm): fmt-check + clippy + test + doc-check — commit'dan oldin
just ci      # DB-SIZ (~80s): check + example + features + msrv + package + audit + semver — push'dan oldin
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres just postgres-test
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres just postgres-test-08
```

`just ci` — PostgreSQL service talab qilmaydigan CI suite'i. Jonli DB tekshiruvi alohida:
`postgres-test` (sqlx 0.9) va `postgres-test-08` (sqlx 0.8) — ikkalasi Rust 1.94da; ularning
`DATABASE_URL`i CREATE DATABASE huquqli disposable instansga qarashi kerak.
Alohida: `just fmt`, `just lint`, `just test`, `just features`, `just msrv`,
`just semver-detail`, `just bench`, `just doc`, `just tree`.

Bitta test:

```bash
cargo test --all-features passport::tests::errors_are_precise   # unit (modul ichida)
cargo test --all-features --test props valid_passports          # integration fayl + filtr
cargo test --all-features --doc                                 # doctest (README ham shu yerda)
```

`--all-features` deyarli har doim kerak: testlarning katta qismi `#![cfg(feature = ...)]` ostida
(`tests/serde.rs`, `tests/sqlx_bounds.rs`).

Justfile `RUSTFLAGS=-D warnings` eksport qiladi (CI bilan parity uchun) — `just test` va oddiy
`cargo test` orasida almashganda cargo fingerprint o'zgaradi va qayta build bo'ladi. Bu normal.

## Arxitektura

To'liq xarita — [`docs/architecture.md`](docs/architecture.md): modul-fayl jadvali, public API
indeksi, feature grafi, parse/serde/sqlx oqimlari, kengaytirish retseptlari va invariantlar
jadvali. Arxitektura savoli tug'ilsa **avval o'sha faylni** o'qing.

Quyida faqat qaytarilmaydigan qarorlar — buzilmasligi kerak bo'lgan invariantlar:

- **Barcha `String`-asosli tiplar `string_newtype!` dan chiqadi** (`src/macros.rs`). Yangi tip =
  makro chaqiruvi + `normalize` + `validate`; boilerplate'ni qo'lda takrorlamang.
- **Tartib qat'iy: `trim_in_place` → `normalize` → `validate`.** Trim'ni makro bajaradi.
- **`TryFrom<String>` yo'li qo'shimcha allocation qilmaydi** — `benches/parse.rs` va
  `try_from_string_reuses_buffer` uslubidagi unit testlar bilan qulflangan.
- **Base parse'ga registry yoki qo'shimcha strict semantikani ko'chirmang**: DB/Kafka/Serde replay
  → `parse()`, joriy foydalanuvchi inputi → `parse_strict()`. PINFL strict tekshiruvi `date`
  feature'idan mustaqil ravishda checksum, jins/asr va to'liq Gregorian sanani qamraydi.
- **Crate ID uchun domen nomi bermaydi** — `OrderId`, `SessionId` iste'molchida. Tayyor alias'lar
  0.20.0–0.21.0 da ataylab olib tashlangan (CHANGELOG); bu qaror qaytarilmasin.
- **`NumIdRepr` sealed** — faqat `u64` va `i64`. `PhantomData<fn() -> Tag>`, `PhantomData<Tag>` emas.
- **Konversiya sirti uchala oilada bir xil** — `FromStr` + `TryFrom<&str>` + `TryFrom<String>` +
  `From<Self> for String`. `T: TryFrom<String>` ostidagi generic kod uchalasida ishlashi shart.
- **`secret_newtype!` ataylab kambag'al** — `Display`, `AsRef`, `Borrow`, `into_inner`, derive
  `PartialEq`/`Hash`/`Ord`, default `Serialize` va sqlx yo'q. Sir tipiga trait qo'shishdan oldin
  nega yo'qligini o'ylang.
- **README = doctest** — `src/lib.rs` uni `include_str!` qiladi (`date` + `id` ostida). README ni
  tahrirlagach `cargo test --all-features --doc`.

## Konvensiyalar

- `#![warn(missing_docs)]` + `#![deny(unsafe_code)]` + CI da `-D warnings`. Har bir public element
  hujjatlangan bo'lishi shart, `cargo doc` ogohlantirishi ham xato.
- Barcha public enum'lar `#[non_exhaustive]`. Har `parse()` **o'zining aniq** xatosini qaytaradi
  (`PassportError`, ...); `TypeError` — `#[from]` orqali yig'iladigan aggregate. Yangi xato tipi
  qo'shsangiz `TypeError` ga variant qo'shing.
- Telefonning exact public registrlari slice (`MOBILE_CODES`, `GEOGRAPHIC_CODES`, `SIP_CODES`,
  `NON_GEOGRAPHIC_FIXED_CODES`); `REGIONAL_CODES` deprecated compatibility oralig'i. Element
  qo'shish breaking bo'lmasin. Registry eskirishi mumkin — iste'molchi uchun chiqish yo'li
  `operator_code()`; buni olib tashlamang va hujjatdan o'chirmang.
- Yangi public tip qo'shganda tekshiring: `lib.rs` (`mod` + `pub use` + feature gate),
  `prelude.rs`, `TypeError`, `tests/props.rs`, `tests/sqlx_bounds.rs`. To'liq ro'yxat:
  [`docs/architecture.md` § 7](docs/architecture.md#7-kengaytirish-retseptlari).
- Unit testlar modul ichida (`#[cfg(test)] mod tests`), integration `tests/` da. `tests/props.rs`
  ikki invariantni qulflaydi: hech qanday input panic qilmaydi, `parse` idempotent.

### MSRV — ikkita pol

`rust-version = "1.85"` (edition 2024) — bu iste'molchi uchun. `sqlx-0_9` feature'i **1.94+** talab
qiladi (sqlx 0.9 ning o'z MSRV'i). `sqlx-0_8` da 1.94 poli yo'q, ammo uning tranzitiv
`url`→`idna`→`icu_*` zanjiri eng yangi versiyalarda 1.86–1.88 talab qiladi; MSRV-aware resolve
bilan 1.85 da ishlaydi — shuni `just msrv-sqlx-08` (lockfile qayta yaratiladi va `trap` bilan
tiklanadi) qulflaydi. Cargo per-feature MSRV'ni bilmaydi, shuning uchun manifestda eng past
umumiy qiymat turadi. 1.85 MSRV `cargo check` bilan, `--all-targets`siz o'lchanadi: dev-dep'lar
(criterion → 1.86, sqlx 0.9 dev-dep → 1.94) downstreamga kirmaydi — shu sabab repo ichidagi
`cargo test` (jumladan `postgres-test-08`) 1.94 talab qiladi.

### Reliz

Versiya bump'idan oldin `just semver-detail` — aynan nima breaking ekanini ko'rsatadi.
CHANGELOG: Keep a Changelog, breaking o'zgarishlar ⚠️ bilan va reliz oxirida **migratsiya jadvali**
bilan. Nashr: `just publish-check` (dry-run, toza tree kerak) → `just publish`.
