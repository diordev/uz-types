# 🇺🇿 uz-types

[![crates.io](https://img.shields.io/crates/v/uz-types.svg)](https://crates.io/crates/uz-types)
[![docs.rs](https://docs.rs/uz-types/badge.svg)](https://docs.rs/uz-types)
[![CI](https://github.com/diordev/uz-types/actions/workflows/ci.yml/badge.svg)](https://github.com/diordev/uz-types/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.94-blue.svg)](#msrv-va-semver)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](#litsenziya)

Rust loyihalari (ayniqsa O'zbekiston domeniga oid backend tizimlar) uchun qat'iy tiplangan (strongly typed), xavfsiz va qayta ishlatiladigan **value object** kutubxonasi.

Oddiy `String` o'rniga `Passport`, `Pinfl`, `PhoneNumber` kabi tiplardan foydalanasiz — qiymat tipga **faqat validatsiyadan o'tib** kiradi, keyin esa u bilan ishlash xavfsiz.

```rust
use uz_types::prelude::*;

let passport = Passport::parse("  aa 1234567 ").unwrap();
assert_eq!(passport.as_str(), "AA1234567");          // trim + katta harf avtomatik

let phone = PhoneNumber::parse("+998 (90) 123-45-67").unwrap();
assert_eq!(phone.as_str(), "998901234567");          // ajratuvchilar tozalandi

let pinfl = Pinfl::parse_strict("31210932040247").unwrap();
assert_eq!(pinfl.gender(), Some(Gender::Male));      // rasmiy checksum + struktura tekshirildi
```

---

## Mundarija

- [Imkoniyatlar](#imkoniyatlar)
- [O'rnatish va feature'lar](#ornatish-va-featurelar)
- [Tezkor boshlash](#tezkor-boshlash)
- [Asosiy g'oya: ikki qatlamli validatsiya](#asosiy-goya-ikki-qatlamli-validatsiya)
- [Tiplar](#tiplar)
  - [Passport](#passport)
  - [Pinfl (JShShIR)](#pinfl-jshshir)
  - [PhoneNumber](#phonenumber)
  - [EmailAddress](#emailaddress)
  - [BirthDate](#birthdate-feature-date)
  - [Id\<Tag\> va NumId\<Tag\>](#idtag-va-numidtag-feature-id)
  - [Sir tiplari: AccessToken, RefreshToken, ClientSecret](#sir-tiplari-accesstoken-refreshtoken-clientsecret)
- [Xatolar bilan ishlash](#xatolar-bilan-ishlash)
- [serde integratsiyasi](#serde-integratsiyasi)
- [sqlx integratsiyasi](#sqlx-integratsiyasi)
- [Cheklovlar](#cheklovlar)
- [MSRV va semver](#msrv-va-semver)
- [Rivojlantirish](#rivojlantirish)
- [Litsenziya](#litsenziya)

---

## Imkoniyatlar

|                               | Nima beradi                                                                                                                                                            |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Qat'iy tiplash**            | `Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress`, `BirthDate`, `Id<Tag>`, `NumId<Tag>`, `AccessToken`, `RefreshToken`, `ClientId`, `ClientSecret`                    |
| **Yagona konstruktor yo'li**  | `parse()`, `FromStr`, `TryFrom`, serde `Deserialize`, sqlx `Decode` — hammasi **bitta** validatsiya yo'lidan o'tadi. Validatsiyani chetlab o'tib tip yaratib bo'lmaydi |
| **Ikki qatlamli validatsiya** | `parse()` — hech qachon o'zgarmaydigan struktura; `parse_strict()` — o'zgaruvchan qoidalar (operator kodlari ro'yxati, PINFL checksum)                                 |
| **Aniq xatolar**              | Har bir tipning o'z error enum'i (`PassportError`, `PinflError`, …); umumiy `TypeError` `?` orqali avtomatik yig'iladi                                                 |
| **Xavfsiz sirlar**            | Token tiplarida `Display` yo'q, `Debug` yashirilgan, taqqoslash constant-time; sirni faqat `expose_secret()` ochadi                                                    |
| **Minimal allocation**        | `String` dan yaratishda qo'shimcha allocation yo'q (normalizatsiya in-place); `&str` dan — bitta                                                                       |
| **Opt-in integratsiyalar**    | `serde`, `sqlx` (Postgres/MySQL/SQLite), `zeroize` — faqat feature yoqilganda kompilyatsiya bo'ladi                                                                    |
| **Xavfsiz kod**               | `#![deny(unsafe_code)]`, panic yo'llari yo'q (property-based testlar bilan qulflangan)                                                                                 |

---

## O'rnatish va feature'lar

```toml
[dependencies]
uz-types = "0.18"
```

Yoki kerakli feature'lar bilan:

```toml
[dependencies]
uz-types = { version = "0.18", features = ["serde", "sqlx-postgres"] }
```

| Feature             | Default | Nima yoqadi                                                                    | Qo'shimcha dependency |
| ------------------- | ------- | ------------------------------------------------------------------------------ | --------------------- |
| `date`              | ✅      | `BirthDate`, `DateFormat`, `Pinfl::birth_date()`                               | `chrono`              |
| `id`                | ✅      | `Id<Tag>`, `NumId<Tag>`, `JobId`, `SessionId`, `RequestId`                     | `uuid`                |
| `serde`             |         | `Serialize` / `Deserialize` barcha tiplar uchun (sirlar — faqat `Deserialize`) | `serde`               |
| `sqlx`              |         | `Type` / `Encode` / `Decode` — driver'ga bog'liq emas                          | `sqlx`                |
| `sqlx-postgres`     |         | `sqlx` + `PgHasArrayType` (`Vec<T>`, `= ANY($1)`)                              | `sqlx/postgres`       |
| `zeroize`           |         | Sir tiplari `Drop` da xotirani nolga to'ldiradi                                | `zeroize`             |
| `serialize-secrets` |         | Sir tiplari uchun `Serialize` (masalan, auth-servis token javobi)              | `serde`               |

**Qoida:** tiplar default'da bor, integratsiyalar — siz tanlaysiz. Faqat `Passport` kerak bo'lgan servis `chrono`/`uuid` ni ham xohlamasa:

```toml
uz-types = { version = "0.18", default-features = false }
```

---

## Tezkor boshlash

```rust
use uz_types::prelude::*;

fn main() -> Result<(), TypeError> {
    // Pasport — trim, ichki bo'sh joy va katta harf avtomatik
    let passport = Passport::parse("aa 1234567")?;
    println!("{passport} | seriya: {} | raqam: {}", passport.series(), passport.number());

    // PINFL — parse: faqat 14 raqam; parse_strict: + rasmiy checksum va struktura
    let pinfl = Pinfl::parse_strict("31210932040247")?;
    println!("{pinfl} | jins: {:?} | tug'ilgan: {:?}", pinfl.gender(), pinfl.birth_date_parts());

    // Telefon — istalgan ajratuvchi bilan, ichkarida har doim 998XXXXXXXXX
    let phone = PhoneNumber::parse("+998 90 123 45 67")?;
    println!("{} | kod: {} | mobil: {}", phone.to_international(), phone.operator_code(), phone.is_mobile());

    // Email — lowercase qilib saqlanadi
    let email = EmailAddress::parse("Diordev@iCloud.com")?;
    println!("{email} | domain: {}", email.domain());

    // Tug'ilgan sana — kelajak va 1900 dan oldingi sanalar rad etiladi
    let birth_date = BirthDate::parse("1995-08-31")?;
    println!("{birth_date} | yosh: {} | {}", birth_date.age(), birth_date.format_as(DateFormat::DmyDot));

    // ID — UUID v7 (DB primary key uchun tavsiya etiladi)
    let job_id = JobId::now_v7();
    println!("{job_id} | v7: {}", job_id.version() == Some(uuid::Version::SortRand));

    // Sir — loglarda ko'rinmaydi, faqat expose_secret() ochadi
    let token = AccessToken::parse("eyJhbGciOiJIUzI1NiJ9.xyz")?;
    println!("{token:?}");                                   // AccessToken([REDACTED])
    let _header = format!("Bearer {}", token.expose_secret());

    Ok(())
}
```

---

## Asosiy g'oya: ikki qatlamli validatsiya

Har bir qoida ikki turdan biriga kiradi:

| Qatlam                | Nima tekshiradi                                               | O'zgaradimi?         | Qayerda                               |
| --------------------- | ------------------------------------------------------------- | -------------------- | ------------------------------------- |
| **Struktura**         | uzunlik, belgilar, prefiks, kalendar sanasi                   | Hech qachon          | `parse()`                             |
| **Registry / biznes** | operator kodi ro'yxatda bormi, PINFL nazorat raqami to'g'rimi | Ha, vaqt-vaqti bilan | `is_*()` metodlar va `parse_strict()` |

Nega shunday? Agar operator kodlari ro'yxati `parse()` ichida bo'lsa, yangi kod ajratilganda **yangi foydalanuvchilar rad etiladi**, tuzatish uchun esa crate relizi va barcha servislarni deploy qilish kerak — DB yoki Kafka'dagi eski yozuvlar ham o'qilmay qoladi. Shuning uchun:

```rust
use uz_types::prelude::*;

// Struktura o'tadi — 998 + 9 raqam. Bu qiymat DB'dan yoki event'dan kelgan bo'lishi mumkin.
let phone = PhoneNumber::parse("998000000000").unwrap();
assert!(!phone.is_known_operator());

// Registratsiya formasida qat'iylik kerak — parse_strict
assert_eq!(
    PhoneNumber::parse_strict("998000000000"),
    Err(PhoneNumberError::UnknownOperatorCode)
);
```

Qisqa qoida: **DB/Kafka/ichki chegara → `parse()`; foydalanuvchi kiritgan ma'lumot → `parse_strict()`.**

---

## Tiplar

### Passport

O'zbekiston pasporti yoki ID-kartasi seriyasi va raqami: 2 ta lotin harfi + 7 ta raqam.

```rust
use uz_types::{Passport, PassportError};

let p = Passport::parse(" ab 1234567 ").unwrap();
assert_eq!(p.as_str(), "AB1234567");   // trim, ichki bo'sh joy olib tashlandi, katta harf
assert_eq!(p.series(), "AB");
assert_eq!(p.number(), "1234567");

assert_eq!(Passport::parse("AB123"), Err(PassportError::Length));
assert_eq!(Passport::parse("A11234567"), Err(PassportError::Format));
```

Faqat **format** tekshiriladi — seriya amaldagi ro'yxatga kiradimi, bu tekshirilmaydi (`ZZ0000000` ham qabul qilinadi).

### Pinfl (JShShIR)

14 raqamli shaxsiy identifikatsiya raqami. Struktura rasmiy hujjatga asoslangan (Vazirlar Mahkamasining 2022-yil 12-apreldagi 177-son qarori):

| Raqamlar | Ma'nosi                                                                                                     |
| -------- | ----------------------------------------------------------------------------------------------------------- |
| 1        | jins va asr: `1`,`2` — 1800-yillar; `3`,`4` — 1900-yillar; `5`,`6` — 2000-yillar (toq — erkak, juft — ayol) |
| 2–7      | tug'ilgan sana `DDMMYY`                                                                                     |
| 8–10     | tug'ilgan hudud kodi                                                                                        |
| 11–13    | tartib raqami                                                                                               |
| 14       | nazorat raqami: birinchi 13 raqam `7,3,1,7,3,1,…` vaznlar bilan ko'paytirilib qo'shiladi, `mod 10`          |

```rust
use uz_types::{Gender, Pinfl, PinflError};

// parse — faqat struktura (14 raqam). DB'dan o'qish, event'lar uchun.
let p = Pinfl::parse("31210932040247").unwrap();
assert!(p.is_checksum_valid());
assert_eq!(p.gender(), Some(Gender::Male));
assert_eq!(p.century(), Some(1900));
assert_eq!(p.birth_date_parts(), Some((1993, 10, 12)));   // (yil, oy, kun)
assert_eq!(p.region_code(), "204");
assert_eq!(p.serial(), "024");

// parse_strict — struktura + checksum + jins/asr belgisi + sana strukturasi
assert!(Pinfl::parse_strict("31210932040247").is_ok());
assert_eq!(Pinfl::parse_strict("31210932040248"), Err(PinflError::Checksum));
assert_eq!(Pinfl::parse_strict("00000000000000"), Err(PinflError::Structure));

// `date` feature bilan: haqiqiy kalendar sanasi sifatida
let birth_date = p.birth_date().unwrap();
assert_eq!(birth_date.to_string(), "1993-10-12");
```

### PhoneNumber

O'zbekiston telefon raqami. Ichkarida **har doim** `998` + 9 raqam (`+` siz, 12 raqam).

```rust
use uz_types::{PhoneNumber, PhoneNumberError};

// Bo'sh joy, `-`, `(`, `)` va boshidagi `+` — hammasi tozalanadi
for input in ["+998 (90) 123-45-67", "998 90 123 45 67", "+998901234567"] {
    assert_eq!(PhoneNumber::parse(input).unwrap().as_str(), "998901234567");
}

let phone = PhoneNumber::parse("+998711234567").unwrap();
assert_eq!(phone.operator_code(), "71");
assert_eq!(phone.subscriber_number(), "1234567");
assert_eq!(phone.to_international(), "+998711234567");
assert!(!phone.is_mobile());            // 71 — Toshkent shahar kodi
assert!(phone.is_known_operator());     // hudud kodlari oralig'ida (60..=79)

assert_eq!(PhoneNumber::parse("997901234567"), Err(PhoneNumberError::Prefix));
assert_eq!(PhoneNumber::parse("99890123456"), Err(PhoneNumberError::Length));
assert_eq!(PhoneNumber::parse("998a01234567"), Err(PhoneNumberError::Format));
```

Ma'lum kodlar `PhoneNumber::MOBILE_CODES` (slice) va `PhoneNumber::REGIONAL_CODES` (`60..=79`) konstantalarida. Ro'yxat eskirsa ham `parse()` ishlayveradi — faqat `is_known_operator()` / `parse_strict()` ta'sirlanadi.

### EmailAddress

`local-part@domain.tld`, lowercase saqlanadi. Faqat ASCII manzillar.

```rust
use uz_types::{EmailAddress, EmailAddressError};

let e = EmailAddress::parse("  User.Name@Example.COM ").unwrap();
assert_eq!(e.as_str(), "user.name@example.com");
assert_eq!(e.local_part(), "user.name");
assert_eq!(e.domain(), "example.com");

assert_eq!(EmailAddress::parse("a@b"), Err(EmailAddressError::Format));      // TLD yo'q
assert_eq!(EmailAddress::parse("a@b..c"), Err(EmailAddressError::Format));   // bo'sh label
assert!(EmailAddress::parse("a@b.co").is_ok());
```

Tekshiriladigan qoidalar: local-part bo'sh emas va 64 belgidan uzun emas, `.` bilan boshlanmaydi/tugamaydi, `..` yo'q, faqat RFC 5322 ruxsat bergan belgilar; domain kamida ikki label, har label `[a-z0-9-]`, `-` bilan boshlanmaydi/tugamaydi, TLD kamida 2 ta harf; umumiy uzunlik ≤ 254.

### BirthDate (feature `date`)

Tug'ilgan sana: `MIN_YEAR` (1900) dan keyin va kelajakda emas (UTC bo'yicha ertangi kungacha yon beriladi — UTC+14 gacha bo'lgan mintaqalar uchun).

```rust
use uz_types::{BirthDate, BirthDateError, DateFormat};
use chrono::NaiveDate;

let d = BirthDate::parse("1990-05-15").unwrap();               // YYYY-MM-DD
let same = BirthDate::parse_with_format("15.05.1990", DateFormat::DmyDot).unwrap();
assert_eq!(d, same);

assert_eq!(d.format_as(DateFormat::DmyHyphen), "15-05-1990");
assert_eq!((d.year(), d.month(), d.day()), (1990, 5, 15));

// Deterministik (testlanadigan) variantlar — "bugun" tashqaridan beriladi
let today = NaiveDate::from_ymd_opt(2026, 9, 3).unwrap();
assert_eq!(d.age_at(today), 36);
assert_eq!(
    BirthDate::parse_with_format_at("2026-09-05", DateFormat::YmdHyphen, today),
    Err(BirthDateError::FutureDate)
);
assert_eq!(BirthDate::parse("1899-12-31"), Err(BirthDateError::TooOld));
```

`age()` va `parse()` tizim soatidan (UTC) foydalanadi; `age_at()`, `parse_with_format_at()`, `from_naive_date_at()` — deterministik. Kelajak tekshiruvi _monoton_: bir marta qabul qilingan sana keyinchalik hech qachon rad etilmaydi, shuning uchun saqlangan ma'lumotni qayta o'qish (replay) xavfsiz.

### Id\<Tag\> va NumId\<Tag\> (feature `id`)

Tipli identifikatorlar. `Tag` — faqat compile-time belgisi: `Id<Job>` ni `Id<Session>` o'rniga berib bo'lmaydi.

```rust
use uz_types::{Id, NumId, IdError, JobId};

// O'z tag'ingizni o'zingiz e'lon qilasiz — crate'ga PR kerak emas
pub enum Order {}
pub type OrderId = Id<Order>;
pub type LegacyOrderId = NumId<Order>;   // eski tizimlardagi raqamli ID (BIGINT)

let a = OrderId::now_v7();               // vaqt bo'yicha tartiblangan — DB primary key uchun
let b = OrderId::new_v4();               // tasodifiy
assert_ne!(a, b);
assert_eq!(OrderId::parse(&a.to_string()).unwrap(), a);
assert_eq!(a.version(), Some(uuid::Version::SortRand));

let legacy = LegacyOrderId::parse("42").unwrap();
assert_eq!(legacy.get(), 42);
assert_eq!(LegacyOrderId::parse("-1"), Err(IdError::Number));
assert_eq!(OrderId::parse("42"), Err(IdError::Uuid));   // raqam UUID emas

// Tayyor alias'lar: JobId, SessionId, RequestId
let job = JobId::now_v7();
assert!(!job.is_nil());
```

- `Id<Tag>` — istalgan RFC 9562 UUID'ni qabul qiladi (hyphenated, simple, braced, urn). Versiya cheklovi kerak bo'lsa `version()` bilan tekshiring.
- `Id<Tag>` JSON'da **har doim** string, `NumId<Tag>` — **har doim** integer. DB'da mos ravishda `UUID` va `BIGINT`.
- Ikkala tip ham `Copy`, `Eq`, `Ord`, `Hash`, `Send + Sync` — `Tag` qanday bo'lishidan qat'i nazar.

### Sir tiplari: AccessToken, RefreshToken, ClientSecret

Sir tiplari **tasodifan** oshkor bo'lmasligi uchun tip darajasida cheklangan:

|                                          | Sir tiplari                                                        | `ClientId` (sir emas) |
| ---------------------------------------- | ------------------------------------------------------------------ | --------------------- |
| `Display` (`{}`, `.to_string()`)         | ❌ compile error                                                   | ✅                    |
| `Debug` (`{:?}`)                         | `AccessToken([REDACTED])`                                          | ✅ ochiq              |
| `as_str()`, `AsRef<str>`, `into_inner()` | ❌                                                                 | ✅                    |
| Qiymatga kirish                          | faqat `expose_secret()`                                            | `as_str()`            |
| `==`                                     | constant-time (`subtle`)                                           | oddiy                 |
| serde                                    | `Deserialize` ✅; `Serialize` faqat `serialize-secrets` feature'da | ikkalasi ✅           |
| sqlx                                     | ❌ (token DB'da saqlanmasligi kerak)                               | ✅                    |
| `zeroize` feature                        | `Drop` da xotira tozalanadi                                        | —                     |

```rust
use uz_types::{AccessToken, ClientId, TokenError};

let token = AccessToken::parse(" eyJhbGciOiJIUzI1NiJ9.xyz ").unwrap();
assert_eq!(format!("{token:?}"), "AccessToken([REDACTED])");     // logga tushmaydi
assert_eq!(token.expose_secret(), "eyJhbGciOiJIUzI1NiJ9.xyz");   // yagona ochiq yo'l

assert_eq!(AccessToken::parse("   "), Err(TokenError::Empty));
assert!(AccessToken::parse(&"a".repeat(uz_types::MAX_TOKEN_LEN + 1)).is_err());

let client_id = ClientId::parse("my-service").unwrap();
assert_eq!(client_id.to_string(), "my-service");                 // ClientId — oddiy tip
```

`MAX_TOKEN_LEN` (8 KiB) — mantiqiy chegara, xotira DoS'idan himoya emas: `String` bu tekshiruvga kelguncha allaqachon ajratilgan bo'ladi. Body-limit HTTP qatlamida turishi kerak.

---

## Xatolar bilan ishlash

Har bir `parse()` **o'zining aniq** error tipini qaytaradi. Umumiy `TypeError` — `?` operatori orqali avtomatik yig'iladigan aggregate:

```rust
use uz_types::{Passport, PassportError, PhoneNumber, TypeError};

// Aniq tip — ikki holat uchun ikki qatorli match
match Passport::parse("AA123") {
    Ok(p) => println!("{p}"),
    Err(PassportError::Length) => println!("uzunlik noto'g'ri"),
    Err(PassportError::Format) => println!("format noto'g'ri"),
    Err(_) => println!("kelajakdagi variant"),     // enum'lar #[non_exhaustive]
}

// Application kodda `?` TypeError ga o'tadi
fn register(passport: &str, phone: &str) -> Result<(), TypeError> {
    let _passport = Passport::parse(passport)?;
    let _phone = PhoneNumber::parse_strict(phone)?;
    Ok(())
}
assert!(matches!(register("AA123", "998901234567"), Err(TypeError::Passport(PassportError::Length))));
```

Barcha error tiplari `std::error::Error`, `Copy`, `Eq` va `#[non_exhaustive]`; `TypeError` variantlari `#[error(transparent)]` — xabar ichki xatonikidir.

---

## serde integratsiyasi

`features = ["serde"]`. Har tip JSON'da **bitta** shaklga ega:

| Tip                                                            | JSON                                                                   |
| -------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress`, `ClientId` | `"AA1234567"` (normalizatsiya qilingan string)                         |
| `BirthDate`                                                    | `"1990-05-15"`                                                         |
| `Id<Tag>`                                                      | `"9b7e597e-893e-4e11-92cf-f4e7d4f923b1"` (bincode/postcard'da 16 bayt) |
| `NumId<Tag>`                                                   | `42` (faqat integer; `"42"` string qabul qilinmaydi)                   |
| `AccessToken` va boshqa sirlar                                 | `Deserialize` ✅; `Serialize` faqat `serialize-secrets`                |

Deserializatsiya **validatsiyadan o'tadi** — noto'g'ri JSON `Err` beradi, `#[derive(Deserialize)]` kabi smart constructor chetlab o'tilmaydi:

```rust
# #[cfg(feature = "serde")] {
use uz_types::prelude::*;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct User {
    passport: Passport,
    phone: PhoneNumber,
    birth_date: BirthDate,
    id: JobId,
}

let json = r#"{"passport":"aa1234567","phone":"+998 90 123 45 67","birth_date":"1990-05-15","id":"01912d68-783e-7c1f-bcf6-9a5b4c3d2e1f"}"#;
let user: User = serde_json::from_str(json).unwrap();
assert_eq!(user.passport.as_str(), "AA1234567");
assert_eq!(user.phone.as_str(), "998901234567");

assert!(serde_json::from_str::<Passport>("\"nope\"").is_err());
assert!(serde_json::from_str::<PhoneNumber>("\"997901234567\"").is_err());
# }
```

`serde` dependency'si `derive` feature'siz ulanadi — proc-macro kompilyatsiyasi kerak emas.

---

## sqlx integratsiyasi

`features = ["sqlx-postgres"]` (yoki faqat `sqlx` + o'zingizning driver'ingiz). Ustun tiplari:

| Tip                                                            | Postgres ustuni    |
| -------------------------------------------------------------- | ------------------ |
| `Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress`, `ClientId` | `TEXT` / `VARCHAR` |
| `BirthDate`                                                    | `DATE`             |
| `Id<Tag>`                                                      | `UUID`             |
| `NumId<Tag>`                                                   | `BIGINT`           |

`Decode` ham `parse()` orqali o'tadi: DB'dagi buzuq yozuv `try_get` da xato beradi, jimgina ichkariga kirmaydi.

```rust,ignore
use uz_types::prelude::*;

// query_as! da maxsus tip uchun ustun override sintaksisi: `ustun AS "ustun: Tip"`
let row = sqlx::query_as!(
    UserRow,
    r#"SELECT id AS "id: JobId", passport AS "passport: Passport",
              phone AS "phone: PhoneNumber", birth_date AS "birth_date: BirthDate"
       FROM users WHERE id = $1"#,
    job_id.as_uuid()
)
.fetch_one(&pool)
.await?;

// bind — Encode
sqlx::query("UPDATE users SET phone = $1 WHERE id = $2")
    .bind(&phone)
    .bind(job_id)
    .execute(&pool)
    .await?;

// Vec<T> — PgHasArrayType (sqlx-postgres)
let passports: Vec<Passport> = vec![/* … */];
sqlx::query("SELECT * FROM users WHERE passport = ANY($1)").bind(&passports);
```

**Migratsiya eslatmasi:** eski tizimdan kelgan DB'da `parse()` strukturasiga mos kelmaydigan yozuvlar (masalan, 13 raqamli PINFL) bo'lsa, ular `SELECT` da xato beradi — migratsiyadan oldin ma'lumotni tozalang. `parse_strict` darajasi (checksum) talab qilinmaydi.

---

## Cheklovlar

| Tip            | Cheklov                                                                                                                                                                                                           |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Passport`     | Faqat format. Seriya amaldagi ro'yxatga kiradimi — tekshirilmaydi.                                                                                                                                                |
| `Pinfl`        | `parse()` faqat 14 raqam. Checksum, jins/asr va sana — `parse_strict()` yoki query metodlar. Haqiqiy shaxsga tegishliligini faqat davlat xizmati (my.gov.uz va h.k.) tasdiqlaydi.                                 |
| `PhoneNumber`  | 9 raqamli mahalliy shakl (`90 123 45 67`) qabul qilinmaydi — `998` bilan yuboring. `MOBILE_CODES` ro'yxati crate ichida; eskirsa `parse()` ta'sirlanmaydi, `is_known_operator()` / `parse_strict()` ta'sirlanadi. |
| `EmailAddress` | Faqat ASCII. IDN (unicode domenlar), quoted local-part (`"a b"@x.com`) va IP-literal (`a@[1.2.3.4]`) qabul qilinmaydi. Local-part ham lowercase qilinadi.                                                         |
| `BirthDate`    | `parse()`/`age()` tizim soatiga (UTC) tayanadi; testlarda `*_at()` variantlarini ishlating.                                                                                                                       |
| `NumId<Tag>`   | DB'da `BIGINT` (`i64`): `i64::MAX` dan katta qiymat `Encode` da xato beradi.                                                                                                                                      |
| Sir tiplari    | `zeroize` — "best effort": `String` realloc/clone nusxalari va HTTP/serde buferlari tozalanmaydi.                                                                                                                 |

---

## MSRV va semver

- **MSRV: Rust 1.94** (edition 2024). MSRV ko'tarilishi _minor_ reliz hisoblanadi.
- Barcha public enum'lar `#[non_exhaustive]` — `match` da `_` tarmog'ini qoldiring.
- Public konstantalar slice/`RangeInclusive` — yangi kod qo'shilishi breaking emas.
- Feature nomlari 1.0 gacha qulflangan: `date`, `id`, `serde`, `sqlx`, `sqlx-postgres`, `zeroize`, `serialize-secrets`.
- Har reliz `cargo semver-checks` bilan tekshiriladi; breaking o'zgarishlar [CHANGELOG](CHANGELOG.md) da migratsiya jadvali bilan beriladi.

---

## Rivojlantirish

Talab: [`just`](https://just.systems), `cargo-hack`, `cargo-semver-checks`.

```bash
just check          # fmt + clippy + test + feature powerset + rustdoc — commit'dan oldin
just test-features  # har feature alohida va feature'siz
just bench          # criterion benchmark
just semver         # oldingi relizga nisbatan API tekshiruvi
just msrv           # rustup toolchain install 1.94.0
```

Testlar: unit + integration (`tests/api.rs`, `tests/serde.rs`, `tests/sqlx_bounds.rs`) + property-based (`tests/props.rs`: hech qanday input panic qilmaydi, `parse` idempotent). Postgres integration testi `DATABASE_URL` bo'lsa ishlaydi (CI'da `postgres:16` service).

---

## Litsenziya

MIT yoki Apache-2.0 — o'zingizga qulayini tanlang.
