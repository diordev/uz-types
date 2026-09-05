# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

`uz-types` — O'zbekiston domeni uchun value object va tipli ID kutubxonasi (crates.io da nashr qilinadi).
Hujjatlar, izohlar, commit va CHANGELOG **o'zbek tilida**; kod identifikatorlari ingliz tilida.

> **Kod yozishdan oldin:** `.claude/skills/idiomatic-rust/SKILL.md` ni o'qing — bu repodagi
> idiomatik Rust qoidalari (type-driven design, allocation intizomi, xato dizayni, MSRV chegaralari).

## Buyruqlar

Talab: [`just`](https://just.systems). Qo'shimcha: `cargo-hack`, `cargo-audit`, `cargo-machete`,
`cargo-semver-checks`, `rustup toolchain install 1.85.0 1.94.0`.

```bash
just check   # TEZ (~3s, tarmoqsiz): fmt-check + clippy + test + doc-check — commit'dan oldin
just ci      # TO'LIQ (~80s): check + example + features + msrv + package + audit + semver — push'dan oldin
```

`just ci` — CI job'larining aynan o'zi. Alohida: `just fmt`, `just lint`, `just test`, `just features`,
`just msrv`, `just semver-detail`, `just bench`, `just doc`, `just tree`.

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

### Barcha `String`-asosli tiplar bitta makrodan chiqadi

`src/macros.rs` dagi `string_newtype!` — `Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress`, `ClientId`
uchun **yagona** boilerplate manbai: `parse`/`as_str`/`into_inner`, `TryFrom<String>`, `TryFrom<&str>`,
`FromStr`, `Display`, `AsRef<str>`, `Borrow<str>`, `serde`, `sqlx`.

Yangi string tip qo'shish = makroni chaqirish + **ikkita** funksiya yozish:

- `fn normalize(s: &mut String)` — uzunlik o'zgarsa; `&mut str` — o'zgarmasa (faqat case);
- `fn validate(s: &str) -> Result<(), Error>` — **normalizatsiya qilingandan keyingi** matn ustida.

Tartib qat'iy: `trim_in_place` → `normalize` → `validate`. Trim'ni makro bajaradi, takrorlamang.

Allocation intizomi: `TryFrom<String>` yo'li hech qachon qo'shimcha allocation qilmaydi (`trim_in_place`
memmove + truncate; `normalize` in-place). Bu da'vo `benches/parse.rs` va `try_from_string_reuses_buffer`
uslubidagi unit testlar bilan qulflangan — buzmang.

`src/secret.rs` dagi `secret_newtype!` — parallel, lekin **ataylab kambag'al** makro: `Display`,
`AsRef`, `Borrow`, `into_inner`, derive `PartialEq`/`Hash`/`Ord`, default `Serialize` **yo'q**.
O'rniga: `expose_secret()`, `Debug` da `[REDACTED]`, `subtle` orqali constant-time `PartialEq`,
`zeroize` feature'da `Drop`. Sir tipiga oddiy trait qo'shishdan oldin nega yo'qligini o'ylang.

### Ikki qatlamli validatsiya — asosiy dizayn qarori

| Qatlam | Nima | Qayerda |
| --- | --- | --- |
| **Struktura** (hech qachon o'zgarmaydi) | uzunlik, belgilar, prefiks, kalendar sanasi | `parse()` |
| **Registry / biznes** (vaqt bilan o'zgaradi) | operator kodi ro'yxatda bormi, PINFL checksum | `is_*()`, `parse_strict()` |

O'zgaruvchan faktni (`MOBILE_CODES`, checksum, jins/asr) **hech qachon** `parse()` ichiga
ko'chirmang: DB va Kafka'dagi eski yozuvlar o'qilmay qoladi. Qoida: DB/event → `parse()`,
foydalanuvchi kiritgan ma'lumot → `parse_strict()`.

### `Id<Tag>` / `NumId<Tag, R>` — crate nom bermaydi

Crate faqat mexanizmni beradi; `OrderId`, `SessionId` kabi nomlarni iste'molchi o'zi e'lon qiladi
(0.20.0 da tayyor alias'lar ataylab olib tashlangan — CHANGELOG ga qarang). Bu qaror qaytarilmasin.

`PhantomData<fn() -> Tag>` (`PhantomData<Tag>` emas) — `Send + Sync + Unpin` va kovariantlik uchun.
`Tag` ga bog'liq bo'lmagan trait'lar qo'lda impl qilinadi, derive ishlatilmaydi (derive `Tag: Clone`
talab qilardi).

`NumIdRepr` — **sealed** trait, faqat `u64` (default) va `i64`. `u64` da `Encode`/`Decode` xato
berishi mumkin (`BIGINT` ga sig'maslik, DB'da manfiy qiymat) — ya'ni **query paytida**; shuning
uchun `try_new_db_safe`/`parse_db_safe` xatoni konstruksiya paytiga ko'chiradi. `i64` da bu yo'l umuman yo'q.

### Feature'lar va integratsiyalar

`date` va `id` — default. `serde`, `sqlx`, `sqlx-postgres`, `zeroize`, `serialize-secrets` — opsional.
Feature nomlari 1.0 gacha qulflangan.

- **serde** (`src/serde_support.rs`): barcha string tiplar uchun bitta `Visitor`.
  `visit_str` → `FromStr`, `visit_string` → `TryFrom<String>` (deserializer bufferini qayta ishlatadi).
  Smart constructor chetlab o'tilmaydi — noto'g'ri JSON `Err` beradi.
- **sqlx** (`src/sqlx_support.rs`): `sqlx_via!` makrosi. Driver'ga bog'liq emas (`DB: Database`);
  `PgHasArrayType` faqat `sqlx-postgres` da. `Decode` **har doim validatsiyadan o'tadi**
  (`#[sqlx(transparent)]` derive'dan farqli) — DB'dagi buzuq yozuv `try_get` da xato beradi.
  Jonli DB testi yo'q; `tests/sqlx_bounds.rs` trait'lar borligini compile-time'da qulflaydi.

### README = doctest

`src/lib.rs` README ni crate doc sifatida `include_str!` qiladi, lekin faqat `date` **va** `id`
yoqilganda (README kod bloklari o'sha tiplarni ishlatadi). README dagi har bir `rust` kod bloki —
ishlaydigan doctest. README ni tahrirlagach `cargo test --all-features --doc` ishlating.

## Konvensiyalar

- `#![warn(missing_docs)]` + `#![deny(unsafe_code)]` + CI da `-D warnings`. Har bir public element
  hujjatlangan bo'lishi shart, `cargo doc` ogohlantirishi ham xato.
- Barcha public enum'lar `#[non_exhaustive]`. Har `parse()` **o'zining aniq** xatosini qaytaradi
  (`PassportError`, ...); `TypeError` — `#[from]` orqali yig'iladigan aggregate. Yangi xato tipi
  qo'shsangiz `TypeError` ga variant qo'shing.
- Public konstantalar slice yoki `RangeInclusive` (`MOBILE_CODES`, `REGIONAL_CODES`) — element
  qo'shish breaking bo'lmasin.
- Yangi public tip qo'shganda tekshiring: `prelude.rs`, `TypeError`, `tests/props.rs`,
  `tests/sqlx_bounds.rs`.
- Unit testlar modul ichida (`#[cfg(test)] mod tests`), integration `tests/` da. `tests/props.rs`
  ikki invariantni qulflaydi: hech qanday input panic qilmaydi, `parse` idempotent.

### MSRV — ikkita pol

`rust-version = "1.85"` (edition 2024) — bu iste'molchi uchun. `sqlx` feature'i **1.94+** talab qiladi
(sqlx 0.9), lekin cargo per-feature MSRV'ni bilmaydi, shuning uchun manifestda eng past umumiy qiymat
turadi va CI ikkala polni alohida job'da tekshiradi. MSRV tekshiruvi `--all-targets` **ishlatmaydi**:
u dev-dep'larni (criterion → 1.86) tortadi, downstream esa ularni yuklamaydi.

### Reliz

Versiya bump'idan oldin `just semver-detail` — aynan nima breaking ekanini ko'rsatadi.
CHANGELOG: Keep a Changelog, breaking o'zgarishlar ⚠️ bilan va reliz oxirida **migratsiya jadvali**
bilan. Nashr: `just publish-check` (dry-run, toza tree kerak) → `just publish`.

## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, invoke the `skill` tool with `skill: "graphify"` before doing anything else.

Rules:

- For codebase questions, first run `graphify query "<question>"` when graphify-out/graph.json exists. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
