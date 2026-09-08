# 🇺🇿 uz-types

[![crates.io](https://img.shields.io/crates/v/uz-types.svg)](https://crates.io/crates/uz-types)
[![docs.rs](https://docs.rs/uz-types/badge.svg)](https://docs.rs/uz-types)
[![CI](https://github.com/diordev/uz-types/actions/workflows/ci.yml/badge.svg)](https://github.com/diordev/uz-types/actions/workflows/ci.yml)
[![MSRV](https://img.shields.io/badge/MSRV-1.85-blue.svg)](#msrv-va-semver)
[![license](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-green.svg)](#litsenziya)

Rust loyihalari (ayniqsa O'zbekiston domeniga oid backend tizimlar) uchun qat'iy tiplangan (strongly typed), xavfsiz va qayta ishlatiladigan **value object** kutubxonasi.

Oddiy `String` o'rniga `Passport`, `Pinfl`, `PhoneNumber` kabi tiplardan foydalanasiz — qiymat tipga **faqat validatsiyadan o'tib** kiradi, keyin esa u bilan ishlash xavfsiz.

Bu README — kutubxonani **qanday ishlatish** bo'yicha foydalanuvchi qo'llanmasi.
Ichkarida u **qanday va nega ishlashi**, dizayn qarorlari hamda maintainer
retseptlari uchun [`docs/architecture.md`](docs/architecture.md) ga qarang.

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
- [Production joriy etish](#production-joriy-etish)
- [Cheklovlar](#cheklovlar)
- [MSRV va semver](#msrv-va-semver)
- [Rivojlantirish](#rivojlantirish)
- [Litsenziya](#litsenziya)

---

## Imkoniyatlar

|                               | Nima beradi                                                                                                                                                            |
| ----------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Qat'iy tiplash**            | `Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress`, `BirthDate`, `Id<Tag>`, `NumId<Tag>`, `AccessToken`, `RefreshToken`, `ClientSecret`                    |
| **Yagona konstruktor yo'li**  | `parse()`, `FromStr`, `TryFrom`, serde `Deserialize`, sqlx `Decode` — hammasi **bitta** validatsiya yo'lidan o'tadi. Validatsiyani chetlab o'tib tip yaratib bo'lmaydi |
| **Ikki qatlamli validatsiya** | `parse()` — barqaror struktura; `parse_strict()` — telefon registri yoki PINFL checksum + jins/asr + to'liq Gregorian sana                                  |
| **Aniq xatolar**              | Har bir tipning o'z error enum'i (`PassportError`, `PinflError`, …); umumiy `TypeError` `?` orqali avtomatik yig'iladi                                                 |
| **Xavfsiz sirlar**            | Token tiplarida `Display` yo'q, `Debug` yashirilgan, taqqoslash constant-time; sirni faqat `expose_secret()` ochadi                                                    |
| **Minimal allocation**        | `String` dan yaratishda qo'shimcha allocation yo'q (normalizatsiya in-place); `&str` dan — bitta                                                                       |
| **Opt-in integratsiyalar**    | `serde`, `sqlx` (Postgres/MySQL/SQLite), `zeroize` — faqat feature yoqilganda kompilyatsiya bo'ladi                                                                    |
| **Xavfsiz kod**               | `#![deny(unsafe_code)]`, panic yo'llari yo'q (property-based testlar bilan qulflangan)                                                                                 |

---

## O'rnatish va feature'lar

```toml
[dependencies]
uz-types = "0.26"
```

Yoki kerakli feature'lar bilan:

```toml
[dependencies]
uz-types = { version = "0.26", features = ["serde", "sqlx-0_9-postgres"] }
```

| Feature             | Default | Nima yoqadi                                                                    | Qo'shimcha dependency |
| ------------------- | ------- | ------------------------------------------------------------------------------ | --------------------- |
| `date`              | ✅      | `BirthDate`, `DateFormat`, `Pinfl::birth_date()`                               | `chrono`              |
| `id`                | ✅      | `Id<Tag>` (UUID), `NumId<Tag, R>` (BIGINT) — nomlarni o'zingiz berasiz        | `uuid`                |
| `serde`             |         | `Serialize` / `Deserialize` barcha tiplar uchun (sirlar — faqat `Deserialize`) | `serde`               |
| `sqlx-0_9`          |         | `Type` / `Encode` / `Decode` — driver'ga bog'liq emas (SQLx 0.9, **rustc 1.94+**) | `sqlx 0.9`          |
| `sqlx-0_9-postgres` |         | `sqlx-0_9` + `PgHasArrayType` (`Vec<T>`, `= ANY($1)`)                          | `sqlx 0.9/postgres`   |
| `sqlx-0_8`          |         | Xuddi shu sirt SQLx 0.8 ustida (1.94 poli yo'q — quyida MSRV bo'limiga qarang)  | `sqlx 0.8`            |
| `sqlx-0_8-postgres` |         | `sqlx-0_8` + `PgHasArrayType`                                                  | `sqlx 0.8/postgres`   |
| `zeroize`           |         | Sir tiplari `Drop` da xotirani nolga to'ldiradi                                | `zeroize`             |
| `serialize-secrets` |         | Sir tiplari uchun `Serialize` (masalan, auth-servis token javobi)              | `serde`               |

**Qoida:** tiplar default'da bor, integratsiyalar — siz tanlaysiz. Faqat `Passport` kerak bo'lgan servis `chrono`/`uuid` ni ham xohlamasa:

```toml
uz-types = { version = "0.26", default-features = false }
```

`Pinfl::parse_strict()`ning Gregorian tekshiruvi `date` feature'iga bog'liq emas;
`date` faqat sana qismlarini `BirthDate`ga aylantiradigan metodlarni yoqadi.

---

## Tezkor boshlash

```rust
use uz_types::prelude::*;

// ID nomlari sizniki: tag e'lon qiling, so'ng alias bering (pastda batafsil)
pub mod tag {
    pub enum Job {}
}
pub type JobId = Id<tag::Job>;

fn main() -> Result<(), TypeError> {
    // Pasport — trim, ichki bo'sh joy va katta harf avtomatik
    let passport = Passport::parse("aa 1234567")?;
    println!("{passport} | seriya: {} | raqam: {}", passport.series(), passport.number());

    // PINFL — parse: 14 ASCII raqam; strict: + checksum, jins/asr va Gregorian sana
    let pinfl = Pinfl::parse_strict("31210932040247")?;
    println!("{pinfl} | jins: {:?} | tug'ilgan: {:?}", pinfl.gender(), pinfl.birth_date_parts());

    // Telefon — istalgan ajratuvchi bilan, ichkarida har doim 998XXXXXXXXX
    let phone = PhoneNumber::parse("+998 90 123 45 67")?;
    println!("{} | kod: {} | mobil: {}", phone.to_international(), phone.operator_code(), phone.is_mobile());

    // Email — lowercase qilib saqlanadi
    let email = EmailAddress::parse("Diordev@iCloud.com")?;
    println!("{email} | domain: {}", email.domain());

    // Tug'ilgan sana — kelajak va 1800 dan oldingi sanalar rad etiladi
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

| Qatlam                  | Nima tekshiradi                                                                  | Qayerda                               |
| ----------------------- | -------------------------------------------------------------------------------- | ------------------------------------- |
| **Barqaror struktura**  | uzunlik, ASCII belgilar va prefiks; `Pinfl` uchun aynan 14 ta ASCII raqam         | `parse()`                             |
| **Qat'iy joriy kirish** | telefon kodi registri; PINFL checksum'i, jins/asr indeksi va to'liq Gregorian sana | `is_*()` metodlar va `parse_strict()` |

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

`Pinfl::parse()` ham checksum yoki sanani emas, faqat 14 ta ASCII raqamni tekshiradi;
`Pinfl::parse_strict()` esa checksum, jins/asr indeksi va `31.02`/`29.02.1900` kabi
noto'g'ri Gregorian sanalarni ham rad etadi.

Qisqa qoida: **DB/Kafka/Serde replay → `parse()`; foydalanuvchi kiritgan ma'lumot → `parse_strict()`.**

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

14 raqamli shaxsiy identifikatsiya raqami. Tuzilishi, `7-3-1` checksum formulasi va
quyidagi misollar [Vazirlar Mahkamasining 2022-yil 12-apreldagi 177-son qarori](https://lex.uz/uz/docs/-5955665)ga asoslangan:

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

// parse_strict — struktura + checksum + jins/asr belgisi + to'liq Gregorian sana
assert!(Pinfl::parse_strict("31210932040247").is_ok());
let female = Pinfl::parse_strict("40201902050010").unwrap();
assert_eq!(female.gender(), Some(Gender::Female));
assert_eq!(female.birth_date_parts(), Some((1990, 1, 2)));
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
assert!(phone.is_geographic());
assert!(phone.is_known_operator());

let overlap = PhoneNumber::parse_strict("998701234567").unwrap();
assert!(overlap.is_mobile());
assert!(overlap.is_geographic());       // 70 ikkala xizmat turida ishlatiladi

assert_eq!(PhoneNumber::parse("997901234567"), Err(PhoneNumberError::Prefix));
assert_eq!(PhoneNumber::parse("99890123456"), Err(PhoneNumberError::Length));
assert_eq!(PhoneNumber::parse("998a01234567"), Err(PhoneNumberError::Format));
```

Aniq tasniflar `MOBILE_CODES`, `GEOGRAPHIC_CODES`, `SIP_CODES` (`55`) va
`NON_GEOGRAPHIC_FIXED_CODES` (`78`) slice'larida. Eski `REGIONAL_CODES = 60..=79`
deprecate qilingan: oraliq ichidagi `60`, `63`, `64`, `68` aniq registrda yo'q, `70`
esa ham mobil, ham geografik. Boshlang'ich manba — [ITU E.164 rejasi](https://www.itu.int/dms_pub/itu-t/opb/sp/T-SP-OB.1263-2023-OAS-PDF-E.pdf);
yangi mobil ajratmalar operatorlarning [20](https://beeline.uz/uz/phone-codes),
[70](https://uztelecom.uz/uz/yangiliklar/yangiliklar/uztelecom-yangi-operator-kodi-plus998-70-ni-taqdim-etadi/),
[80](https://perfectum.uz/uz/cdma), [87](https://company.mobi.uz/uz/press/2026/101857/)
va [92](https://beeline.uz/uz/events/news/novyy-kod-beeline-uzbekistan_92) sahifalari
bilan to'ldirilgan. Registry vaqt o'tishi bilan eskirishi mumkin; bu `parse()`ga ta'sir
qilmaydi, faqat `is_*()` va `parse_strict()` natijasini o'zgartiradi.

| Tasnif | Exact kodlar |
| --- | --- |
| Mobil | `20, 33, 50, 70, 77, 80, 87, 88, 90, 91, 92, 93, 94, 95, 97, 98, 99` |
| Geografik PSTN | `61, 62, 65, 66, 67, 69, 70, 71, 72, 73, 74, 75, 76, 79` |
| SIP | `55` |
| Geografik bo'lmagan statsionar | `78` |

#### O'z registringizni ishlatish

Kod ajratmalari o'zgaruvchan fakt — crate snapshot'i eskirsa, `is_*()` va
`parse_strict()` yangi kodlarni rad etadi. Ro'yxatni o'zingiz boshqarsangiz
(config, DB yoki operator API'sidan), crate relizini kutish shart emas: struktura
`parse()` da barqaror, siyosat esa `operator_code()` orqali sizniki bo'ladi.

```rust
use uz_types::PhoneNumber;

// Ro'yxat sizniki — deploysiz yangilanadi.
let allowed = ["90", "91", "99"];

let phone = PhoneNumber::parse("998911234567").unwrap();   // struktura — crate tekshiradi
assert!(allowed.contains(&phone.operator_code()));         // registry — siz tekshirasiz

// Crate hali bilmaydigan yangi kod shu yo'l bilan bloklanmaydi.
let fresh = PhoneNumber::parse("998001234567").unwrap();
assert!(!fresh.is_known_operator());
assert_eq!(fresh.operator_code(), "00");
```

Ya'ni tanlov "registry bor / yo'q" emas — **registry default**: crate ro'yxati tayyor
javob beradi, kerak bo'lsa uni chetlab o'tasiz.

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

Tekshiriladigan qoidalar: local-part bo'sh emas va 64 baytdan uzun emas, `.` bilan boshlanmaydi/tugamaydi, `..` yo'q, faqat RFC 5322 ruxsat bergan belgilar; domain kamida ikki label, har label `[a-z0-9-]`, `-` bilan boshlanmaydi/tugamaydi, TLD kamida 2 ta harf; umumiy uzunlik ≤ 254 bayt.

### BirthDate (feature `date`)

Tug'ilgan sana: `MIN_YEAR` (1800) dan keyin va kelajakda emas (UTC bo'yicha ertangi kungacha yon beriladi — UTC+14 gacha bo'lgan mintaqalar uchun).

`MIN_YEAR` — **sanity floor**, biznes qoidasi emas. 1800 tanlangani: PINFL 1-raqami `1`/`2` bo'lganda asr 1800 bo'ladi (`Pinfl::century()`), shuning uchun `MIN_YEAR = 1900` da `Pinfl::birth_date()` bunday PINFL uchun har doim `None` qaytarardi. Yosh chegarasi (`>= 18`, `<= 120` va h.k.) — ilova qatlamining ishi, `age_at()` bilan tekshiring.

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
assert!(BirthDate::parse("1800-01-01").is_ok());
assert_eq!(BirthDate::parse("1799-12-31"), Err(BirthDateError::TooOld));
```

`age()` va `parse()` tizim soatidan (UTC) foydalanadi; `age_at()`, `parse_with_format_at()`, `from_naive_date_at()` — deterministik. Kelajak tekshiruvi _monoton_: bir marta qabul qilingan sana keyinchalik hech qachon rad etilmaydi, shuning uchun saqlangan ma'lumotni qayta o'qish (replay) xavfsiz.

### Id\<Tag\> va NumId\<Tag\> (feature `id`)

Crate tayyor ID **nomlarini bermaydi**. `OrderId`, `SessionId`, `UserId` — bular sizning domeningiz, crate'niki emas: nomni ham, ko'rinishni ham siz tanlaysiz. Crate faqat ikkita mexanizm beradi:

| Mexanizm        | Postgres ustuni | Qachon                                          |
| --------------- | --------------- | ----------------------------------------------- |
| `Id<Tag>`       | `UUID`          | yangi jadvallar — sxemani o'zingiz boshqarasiz  |
| `NumId<Tag, R>` | `BIGINT`        | mavjud `BIGINT` ustunlar, eski tizim ID'lari    |

#### Yaratish tartibi — 3 qadam

```rust
use uz_types::{Id, IdError, NumId};

// Qadam 1 — TAG'LAR. Butun loyiha uchun bitta modulda.
pub mod tag {
    pub enum Order {}
    pub enum Session {}
    pub enum LegacyInvoice {}
}

// Qadam 2 — ALIAS. Ko'rinish (UUID yoki BIGINT) aynan shu yerda tanlanadi.
pub type OrderId = Id<tag::Order>;                          // UUID
pub type SessionId = Id<tag::Session>;                      // UUID
pub type LegacyInvoiceId = NumId<tag::LegacyInvoice, i64>;  // BIGINT

// Qadam 3 — ISHLATISH.
let a = OrderId::now_v7();      // v7 — vaqt bo'yicha tartiblangan, DB primary key uchun
let b = OrderId::new_v4();      // v4 — tasodifiy, yaratilish vaqtini oshkor qilmaydi
assert_ne!(a, b);
assert_eq!(OrderId::parse(&a.to_string()).unwrap(), a);
assert_eq!(a.version(), Some(uuid::Version::SortRand));

// BIGINT ID — manfiy legacy qiymatlar ham qabul qilinadi
let inv = LegacyInvoiceId::parse("-42").unwrap();
assert_eq!(inv.get(), -42);

assert_eq!(OrderId::parse("42"), Err(IdError::Uuid));   // raqam UUID emas
```

**1-qadam haqida.** `enum {}` — uninhabited: instansiya yaratib bo'lmaydi, faqat compile-time belgisi (runtime'da hajmi nol). Tag'ni **ikki joyda e'lon qilmang** — `Id<a::Order>` va `Id<b::Order>` bir-biriga to'g'ri kelmaydigan turli tiplar.

**2-qadam haqida.** Session'ni raqamli qilmoqchimisiz? Faqat shu qatorni o'zgartiring — `NumId<tag::Session, i64>`. Crate bu qarorni siz uchun qilmaydi.

**Nima uchun shunday.** `SessionId`, `OrderId` — domen nomlari. Agar crate ularni o'zi e'lon qilsa, (a) `use uz_types::prelude::*` sizning nomlaringiz bilan to'qnashadi, (b) ko'rinish (UUID) sizga majburlanadi. Shuning uchun 0.20.0 dan boshlab crate faqat mexanizmni beradi.

Tip xavfsizligi — asosiy foyda:

```rust,compile_fail
use uz_types::Id;

pub mod tag {
    pub enum Order {}
    pub enum Session {}
}
type OrderId = Id<tag::Order>;
type SessionId = Id<tag::Session>;

fn cancel(id: OrderId) {}

let s = SessionId::now_v7();
cancel(s);   // ❌ compile error: expected Id<tag::Order>, found Id<tag::Session>
```

#### Xususiyatlari

- `Id<Tag>` — istalgan RFC 9562 UUID'ni qabul qiladi (hyphenated, simple, braced, urn). Versiya cheklovi kerak bo'lsa `version()` bilan tekshiring; nil UUID (`0000…`) ham o'tadi — muhim bo'lsa `is_nil()`.
- `Id<Tag>` JSON'da **har doim** string, `NumId<Tag, R>` — **har doim** integer. DB'da mos ravishda `UUID` va `BIGINT`.
- Ikkala tip ham `Copy`, `Eq`, `Ord`, `Hash`, `Send + Sync` — `Tag` qanday bo'lishidan qat'i nazar.
- `uuid` **re-export qilinmagan**: `Uuid` bilan bevosita ishlasangiz (`from_uuid`, `version`) uni o'z `Cargo.toml`ingizga qo'shing.
- Konversiya sirti crate'dagi boshqa tiplar bilan bir xil: `parse()`, `FromStr`, `TryFrom<&str>`, `TryFrom<String>` va `From<T> for String`. Ya'ni `fn ingest<T: TryFrom<String>>(…)` kabi generic kod `Id<Tag>`, `NumId<Tag, R>` va string tiplarining hammasi bilan ishlaydi.

#### `NumId` ning ichki ko'rinishi: `u64` yoki `i64`

DB'dagi `BIGINT` — bu `i64`. `u64` esa kengroq, ya'ni **type system va DB bir xil narsani va'da qilmaydi**. Farq faqat runtime'da chiqadi, shuning uchun ko'rinish `R` parametri bilan tanlanadi (default `u64` — mavjud kod o'zgarmaydi):

|                          | `NumId<Tag>` (`u64`)                     | `NumId<Tag, i64>`       |
| ------------------------ | ---------------------------------------- | ----------------------- |
| Diapazon                 | `0..=u64::MAX`                           | `i64::MIN..=i64::MAX`   |
| `Encode` → `BIGINT`      | `> i64::MAX` → `IdError::NumberTooLarge` | **xato yo'li yo'q**     |
| `Decode` ← `BIGINT`      | manfiy → `IdError::NumberNegative`       | **xato yo'li yo'q**     |
| Manfiy legacy ID         | ❌                                        | ✅                       |

```rust
use uz_types::{IdError, NumId};
pub enum Order {}
type LegacyOrderId = NumId<Order>;

// u64 repr: `new`/`parse` kengroq — xato faqat query paytida chiqardi.
let too_big = LegacyOrderId::new(LegacyOrderId::MAX_DB_SAFE + 1);
assert!(!too_big.is_db_safe());

// ...shuning uchun chegarani INPUT tomonida qo'ying, query paytida emas:
assert!(LegacyOrderId::parse_db_safe("9223372036854775808").is_err());
assert!(LegacyOrderId::try_new_db_safe(42).is_ok());

// Ko'rinishlar orasida konversiya har doim tekshiriladi
assert_eq!(
    NumId::<Order, i64>::try_from(LegacyOrderId::new(u64::MAX)),
    Err(IdError::NumberTooLarge { value: u64::MAX })
);
```

DB bilan ishlaganda `i64` ko'rinishi runtime xatolarining butun sinfini yo'q qiladi — legacy `BIGINT` ustunlar uchun **shuni tanlang**. `u64` faqat qiymat haqiqatan ham manfiy bo'lmasligi domen fakti bo'lganda mantiqli.

### Sir tiplari: AccessToken, RefreshToken, ClientSecret

Sir tiplari **tasodifan** oshkor bo'lmasligi uchun tip darajasida cheklangan:

|                                          | Sir tiplari                                                        |
| ---------------------------------------- | ------------------------------------------------------------------ |
| `Display` (`{}`, `.to_string()`)         | ❌ compile error                                                   |
| `Debug` (`{:?}`)                         | `AccessToken([REDACTED])`                                          |
| `as_str()`, `AsRef<str>`, `into_inner()` | ❌                                                                 |
| Qiymatga kirish                          | faqat `expose_secret()`                                            |
| `==`                                     | constant-time (`subtle`)                                           |
| serde                                    | `Deserialize` ✅; `Serialize` faqat `serialize-secrets` feature'da |
| sqlx                                     | ❌ (token DB'da saqlanmasligi kerak)                               |
| `zeroize` feature                        | `Drop` da xotira tozalanadi                                        |

```rust
use uz_types::{AccessToken, TokenError};

let token = AccessToken::parse(" eyJhbGciOiJIUzI1NiJ9.xyz ").unwrap();
assert_eq!(format!("{token:?}"), "AccessToken([REDACTED])");     // logga tushmaydi
assert_eq!(token.expose_secret(), "eyJhbGciOiJIUzI1NiJ9.xyz");   // yagona ochiq yo'l

assert_eq!(AccessToken::parse("   "), Err(TokenError::Empty));
assert!(AccessToken::parse(&"a".repeat(uz_types::MAX_TOKEN_LEN + 1)).is_err());
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

Modulga xos leaf error tiplari (`PassportError`, `PinflError` va boshqalar)
`std::error::Error`, `Copy`, `Eq` va `#[non_exhaustive]`. Ularni yig'uvchi
`TypeError` ham `std::error::Error`, `Clone`, `Eq` va `#[non_exhaustive]`, ammo
`Copy` emas; uning variantlari `#[error(transparent)]`, ya'ni xabar ichki
xatonikidir.

---

## serde integratsiyasi

`features = ["serde"]`. Har tip JSON'da **bitta** shaklga ega:

| Tip                                                            | JSON                                                                   |
| -------------------------------------------------------------- | ---------------------------------------------------------------------- |
| `Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress`             | `"AA1234567"` (normalizatsiya qilingan string)                         |
| `BirthDate`                                                    | `"1990-05-15"`                                                         |
| `Id<Tag>`                                                      | `"9b7e597e-893e-4e11-92cf-f4e7d4f923b1"` (bincode/postcard'da 16 bayt) |
| `NumId<Tag, R>`                                                | `42` / `-1` (faqat integer; `"42"` string qabul qilinmaydi)            |
| `AccessToken` va boshqa sirlar                                 | `Deserialize` ✅; `Serialize` faqat `serialize-secrets`                |

Deserializatsiya **validatsiyadan o'tadi** — noto'g'ri JSON `Err` beradi, `#[derive(Deserialize)]` kabi smart constructor chetlab o'tilmaydi:

```rust
# #[cfg(feature = "serde")] {
use uz_types::prelude::*;

pub mod tag {
    pub enum User {}
}
pub type UserId = Id<tag::User>;

#[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
struct User {
    passport: Passport,
    phone: PhoneNumber,
    birth_date: BirthDate,
    id: UserId,
}

let json = r#"{"passport":"aa1234567","phone":"+998 90 123 45 67","birth_date":"1990-05-15","id":"01912d68-783e-7c1f-bcf6-9a5b4c3d2e1f"}"#;
let user: User = serde_json::from_str(json).unwrap();
assert_eq!(user.passport.as_str(), "AA1234567");
assert_eq!(user.phone.as_str(), "998901234567");

assert!(serde_json::from_str::<Passport>("\"nope\"").is_err());
assert!(serde_json::from_str::<PhoneNumber>("\"997901234567\"").is_err());

// Replay strukturaviy parserdan o'tadi; joriy registry/checksum siyosati keyin tekshiriladi.
let replay_phone: PhoneNumber = serde_json::from_str("\"998000000000\"").unwrap();
assert!(!replay_phone.is_known_operator());
assert_eq!(
    PhoneNumber::parse_strict(replay_phone.as_str()),
    Err(PhoneNumberError::UnknownOperatorCode)
);
let replay_pinfl: Pinfl = serde_json::from_str("\"31210932040248\"").unwrap();
assert_eq!(
    Pinfl::parse_strict(replay_pinfl.as_str()),
    Err(PinflError::Checksum)
);
# }
```

`serde` dependency'si `derive` feature'siz ulanadi — proc-macro kompilyatsiyasi kerak emas.

---

## sqlx integratsiyasi

SQLx 0.9 uchun `features = ["sqlx-0_9-postgres"]` (yoki faqat `sqlx-0_9` +
o'zingizning driver'ingiz). SQLx 0.8 uchun mos ravishda `sqlx-0_8-postgres` yoki
`sqlx-0_8` tanlanadi. Ustun tiplari:

| Tip                                                            | Postgres ustuni    |
| -------------------------------------------------------------- | ------------------ |
| `Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress`             | `TEXT` / `VARCHAR` |
| `BirthDate`                                                    | `DATE`             |
| `Id<Tag>`                                                      | `UUID`             |
| `NumId<Tag, R>`                                                | `BIGINT`           |

`Decode` ham `parse()` orqali o'tadi: DB'dagi buzuq yozuv `try_get` da xato beradi, jimgina ichkariga kirmaydi.
`NumId<Tag, u64>` overflow'ida SQLx 0.9 `Query::try_bind()` aniq `IdError`ni
qaytaradi; oddiy `bind()` uni ichkarida matnga aylantirib, executionda tashqi
`sqlx::Error::Encode`ni beradi. Leaf kerak bo'lsa `try_bind()` yoki input chegarasida
`try_new_db_safe()` ishlating.

```rust,ignore
use uz_types::prelude::*;

// query_as! da maxsus tip uchun ustun override sintaksisi: `ustun AS "ustun: Tip"`
let row = sqlx::query_as!(
    UserRow,
    r#"SELECT id AS "id: UserId", passport AS "passport: Passport",
              phone AS "phone: PhoneNumber", birth_date AS "birth_date: BirthDate"
       FROM users WHERE id = $1"#,
    user_id.as_uuid()
)
.fetch_one(&pool)
.await?;

// bind — Encode
sqlx::query("UPDATE users SET phone = $1 WHERE id = $2")
    .bind(&phone)
    .bind(job_id)
    .execute(&pool)
    .await?;

// Vec<T> — PgHasArrayType (sqlx-0_9-postgres)
let passports: Vec<Passport> = vec![/* … */];
sqlx::query("SELECT * FROM users WHERE passport = ANY($1)").bind(&passports);
```

### Qaysi sqlx liniyasini tanlash

Crate ikkala amaldagi SQLx minor liniyasini qo'llab-quvvatlaydi. Yagona farq —
`Database::ArgumentBuffer` (0.8 da lifetime'li GAT, 0.9 da lifetime'siz); qolgan sirt
(`Type`, `Encode`, `Decode`, `PgHasArrayType::array_compatible`, `Query::try_bind`)
bir xil va ikkalasi ham jonli PostgreSQL 16 da tekshiriladi.

| Servisingiz | Feature | rustc |
| --- | --- | --- |
| SQLx 0.9 da | `sqlx-0_9-postgres` | 1.94+ (qat'iy) |
| SQLx 0.8 da | `sqlx-0_8-postgres` | 1.85+ (lockfile'ga bog'liq — quyiga qarang) |

```toml
# SQLx 0.8 da qolgan servis — qo'lda `String`/`Uuid`/`i64` map qilish shart emas.
uz-types = { version = "0.26", default-features = false, features = ["date", "id", "sqlx-0_8-postgres"] }
```

SQLx 0.8 o'z `rust-version`ini e'lon qilmaydi, lekin uning tranzitiv `url` → `idna` →
`icu_*` zanjirining **eng yangi** versiyalari 1.86–1.88 talab qiladi. Cargo 1.85+ ning
MSRV-aware resolver'i yangi lockfile yaratganda mos (eskiroq) versiyalarni o'zi
tanlaydi, shuning uchun 1.85 da ishlaydi — buni CI `cargo generate-lockfile` bilan
takrorlab tekshiradi. Mavjud lockfile'ingiz yangiroq toolchain bilan yaratilgan bo'lsa,
`cargo generate-lockfile` ni 1.85 bilan qayta ishga tushiring yoki amaliy polni ~1.88
deb hisoblang.

`sqlx_0_8::Type` va `sqlx_0_9::Type` — turli crate'lardagi turli trait'lar, shuning
uchun ikkala feature birga yoqilsa ham kompilyatsiya buziladi emas (dependency grafida
feature unification bo'lsa ham). Ammo bu ikkita sqlx daraxtini tortadi — amalda
bittasini tanlang.

`sqlx-*-postgres` string newtype'larning ichki `String` qoidalarini massivlarga ham
delegatsiya qiladi: `TEXT[]` va `VARCHAR[]` `Vec<Passport>` kabi decode qilinadi,
`VARCHAR` ustunidagi `array_agg(...)` natijasi olinadi va `Vec<T>`ni
`= ANY($1)`ga bind qilish ishlaydi. Bular PostgreSQL 16 service'ida jonli tekshiriladi:

```bash
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres just postgres-test
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres just postgres-test-08
```

Oddiy `cargo test --all-features` jonli testlarni kompilyatsiya qiladi, ammo ular
`#[ignore]`; `just ci` ham DB service talab qilmaydi. GitHub CI esa `live-postgres`
jobini ikkala sqlx liniyasi uchun matritsa bilan bajaradi.

**Migratsiya eslatmasi:** eski tizimdan kelgan DB'da `parse()` strukturasiga mos
kelmaydigan yozuvlar (masalan, 13 raqamli PINFL) bo'lsa, ular `SELECT`da xato beradi —
migratsiyadan oldin ma'lumotni tozalang. `parse_strict` darajasi (checksum va joriy
registry) replay paytida talab qilinmaydi. Repozitoriydagi sintetik testlar va ixtiyoriy
maxfiy PINFL sample auditi iste'molchining haqiqiy legacy ma'lumotlari toza ekanini
isbotlamaydi.

---

## Production joriy etish

Yangilashdan oldin iste'molchi servisda quyidagilarni tekshiring:

- Asosiy crate uchun `rustc >= 1.85`. SQLx uchun servisingiz qaysi liniyada bo'lsa,
  shu feature'ni tanlang: `sqlx-0_9-postgres` (rustc 1.94+ qat'iy) yoki
  `sqlx-0_8-postgres` (1.94 poli yo'q; amaliy pol lockfile'ingizga bog'liq — MSRV
  bo'limiga qarang). Ikkalasi ham bir xil trait sirtini beradi va bir xil jonli
  PostgreSQL suite'idan o'tadi.
- `cargo tree -d` bilan bitta sqlx liniyasi tortilganini tasdiqlang. Ikkala feature
  yoqilgan bo'lsa build buzilmaydi, lekin ikkita sqlx daraxti kiradi — odatda bu
  kutilmagan holat.
- **`sqlx-0_8` bilan `cargo audit` RUSTSEC-2023-0071 (`rsa`, Marvin Attack) ni
  ko'rsatadi.** `rsa` sqlx 0.8 ning ixtiyoriy MySQL drayveridan `Cargo.lock` ga
  tushadi; `cargo audit` lockfile'ni o'qiydi va feature'larni ko'rmaydi. Biz faqat
  `postgres` drayverini yoqamiz, shuning uchun `rsa` hech qanday feature yoki
  target'da kompilyatsiya qilinmaydi — buni `cargo tree --target all -i rsa
  --all-features` bilan o'zingiz tasdiqlang (natija bo'sh bo'lishi kerak) va
  shundan keyingina audit siyosatingizda e'tiborsiz qoldiring. sqlx 0.9 da bu
  yozuv yo'q.
- Yangilangan `Cargo.lock`ni commit qiling; deploydan oldin legacy jadvallar va eventlarda
  strukturaviy invalid qiymat, `NULL`, `TEXT[]`/`VARCHAR[]` hamda ID chegaralarini audit qiling.
- Email normalizatsiyasi local-partni ham lowercase qiladi: unique indeks yoki merge oldidan
  lowercase collision'larni tekshiring.
- Yosh biznes qoidasini `BirthDate::age_at(local_today)` bilan, servisning mahalliy
  sanasini aniq uzatib hisoblang.
- PostgreSQL `BIGINT` uchun odatda `NumId<Tag, i64>`ni tanlang. `u64` ishlatilsa
  `i64::MAX` chegarasini inputda tekshiring; JavaScript DTO'da `2^53`dan katta ID'ni
  string sifatida yuboring.
- Nil UUID taqiqlanishi kerak bo'lsa, uni crate parseridan tashqarida consumer biznes
  qoidasi sifatida rad eting.
- Sir tiplarining `MAX_TOKEN_LEN` chegarasi xotiraga allaqachon olingan body'ni
  himoya qilmaydi; HTTP/body limitini framework darajasida oldindan qo'ying.
- DB/Kafka/Serde replay uchun `parse()`, joriy foydalanuvchi inputi uchun
  `parse_strict()` ishlating va disposable PostgreSQLda `just postgres-test`ni o'tkazing.

---

## Cheklovlar

| Tip            | Cheklov                                                                                                                                                                                                           |
| -------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `Passport`     | Faqat format. Seriya amaldagi ro'yxatga kiradimi — tekshirilmaydi.                                                                                                                                                |
| `Passport` / `Pinfl` | `Debug` / `Display` loglarda to'liq qiymatni chiqaradi — maskalash iste'molchi zimmasida. |
| `Pinfl`        | `parse()` faqat 14 ta ASCII raqam. Checksum, jins/asr va to'liq Gregorian sana — `parse_strict()` yoki query metodlar. Haqiqiy shaxsga tegishliligini faqat davlat xizmati (my.gov.uz va h.k.) tasdiqlaydi.                     |
| `PhoneNumber`  | 9 raqamli mahalliy shakl (`90 123 45 67`) qabul qilinmaydi — `998` bilan yuboring. Exact kod slice'lari crate ichida; eskirsa `parse()` ta'sirlanmaydi, `is_*()` / `parse_strict()` ta'sirlanadi.                               |
| `EmailAddress` | Faqat ASCII. IDN (unicode domenlar), quoted local-part (`"a b"@x.com`) va IP-literal (`a@[1.2.3.4]`) qabul qilinmaydi. Local-part ham lowercase qilinadi.                                                         |
| `BirthDate`    | `parse()`/`age()` tizim soatiga (UTC) tayanadi; testlarda `*_at()` variantlarini ishlating.                                                                                                                       |
| `NumId<Tag>`   | `u64` repr: `i64::MAX` dan katta qiymat `Encode` da, DB'dagi manfiy qiymat `Decode` da xato beradi — ya'ni **query paytida**. Chegarani `parse_db_safe()`/`try_new_db_safe()` bilan input tomoniga qo'ying yoki `NumId<Tag, i64>` ishlating (u yerda xato yo'li umuman yo'q). `2^53` dan katta ID JSON orqali JS client'ga borsa aniqligini yo'qotadi. |
| Sir tiplari    | `zeroize` — "best effort": `String` realloc/clone nusxalari va HTTP/serde buferlari tozalanmaydi.                                                                                                                 |

---

## MSRV va semver

- **MSRV: Rust 1.85** (edition 2024). MSRV ko'tarilishi _minor_ reliz hisoblanadi.
- **`sqlx-0_9` / `sqlx-0_9-postgres` Rust 1.94+ talab qiladi** — bu `sqlx 0.9` ning o'z MSRV'i, undan qutulib bo'lmaydi.
- **`sqlx-0_8` / `sqlx-0_8-postgres` da 1.94 poli yo'q.** SQLx 0.8 `rust-version` e'lon qilmaydi; amaliy pol tranzitiv `icu_*`/`idna_adapter` dan keladi. MSRV-aware resolve bilan 1.85, committed eng yangi versiyalar bilan ~1.88.
- Cargo per-feature MSRV'ni qo'llab-quvvatlamaydi, shuning uchun `Cargo.toml` dagi `rust-version` eng past umumiy qiymat — 1.85. CI uchta polni alohida tekshiradi: 1.85 sqlx'siz, 1.85 + sqlx 0.8 (qayta resolve bilan), 1.94 + barcha feature.
- MSRV kutubxona iste'molchisi uchun `cargo check` bilan o'lchanadi: dev-dependency'lar
  (`criterion` → 1.86, sqlx 0.9 dev-dep → 1.94) downstreamga kirmaydi. Shu sabab
  repo ichidagi `cargo test` (jumladan `postgres-test-08`) 1.94 talab qiladi —
  bu kutubxona iste'molchisiga taalluqli emas. `cargo bench` uchun 1.86+ kerak.
- Barcha public enum'lar `#[non_exhaustive]` — `match` da `_` tarmog'ini qoldiring.
- Public konstantalar slice/`RangeInclusive` — yangi kod qo'shilishi breaking emas.
- Feature nomlari 1.0 gacha qulflangan: `date`, `id`, `serde`, `sqlx-0_8`, `sqlx-0_8-postgres`, `sqlx-0_9`, `sqlx-0_9-postgres`, `zeroize`, `serialize-secrets`.
- Breaking o'zgarishlar [CHANGELOG](CHANGELOG.md) da migratsiya jadvali bilan beriladi.

---

## Rivojlantirish

Talab: [`just`](https://just.systems), `cargo-hack`, `cargo-audit`, `cargo-machete`, `cargo-semver-checks`.

```bash
just check          # TEZ (~3s): fmt + clippy + test + rustdoc — commit'dan oldin
just ci             # DB-SIZ (~80s): check + example + features + msrv + package + audit + semver
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres just postgres-test
DATABASE_URL=postgres://postgres:postgres@localhost:5432/postgres just postgres-test-08
```

`just ci` — PostgreSQL service talab qilmaydigan CI suite'i; `publish-check` unga
tayanadi. Jonli DB qatlami CI'da va lokal ishda alohida `postgres-test*`. Alohida qismlar:

| Recipe | Nima qiladi |
| ---------------- | ----------------------------------------------------------------------------- |
| `just features`  | `cargo hack` — powerset + har feature alohida test                            |
| `just msrv`      | 1.85 (sqlx'siz) va 1.94 (`--all-features`) pollari                             |
| `just semver`    | tanlangan versiya bump'i o'zgarishlarni qoplaydimi                             |
| `just semver-detail` | aynan **nima** breaking ekanini ko'rsatadi — CHANGELOG yozishdan oldin     |
| `just audit`     | `cargo audit` (CVE) + `cargo machete` (ishlatilmagan dep) — tarmoq kerak       |
| `just bench`     | criterion benchmark (`benches/parse.rs`) — Rust 1.86+ kerak                    |
| `just postgres-test` | PostgreSQL 16 da scalar, NULL, massiv va xato roundtrip'lari (sqlx 0.9) — Rust 1.94 |
| `just postgres-test-08` | Xuddi shu suite sqlx 0.8 kod yo'li ustida (toolchain 1.94 — dev-dep'lar) |
| `just msrv-sqlx-08` | sqlx 0.8 ning Rust 1.85 da resolve bo'lishi — lockfile qayta yaratiladi va tiklanadi |

`just check` warm cache bilan tez; birinchi dependency yuklanishi tarmoq talab qilishi mumkin.
`audit` va `semver` `ci` da turadi.
Justfile `RUSTFLAGS=-D warnings` ni CI bilan bir xil qilib eksport qiladi — shuning
uchun `just test` va oddiy `cargo test` orasida almashganda qayta build bo'ladi.

Testlar: unit (modul ichida) + integration (`tests/serde.rs`, `tests/sqlx_bounds.rs`,
`tests/sqlx_postgres.rs` va `tests/sqlx_postgres_0_8.rs` — tanasi
`tests/common/postgres_suite.rs` da, `tests/sqlx_version_parity.rs`,
`tests/pinfl_dataset.rs`, `tests/compile_fail.rs`) + property-based.
`tests/props.rs` to'rtta `string_newtype!`
tipini `\\PC{0,64}` generatorida panic qilmaslik va muvaffaqiyatli `parse`ning
idempotentligi bo'yicha tekshiradi. `tests/sqlx_bounds.rs` SQLx trait'larini
DB-siz qulflaydi va buni har ikkala sqlx liniyasi uchun alohida modulda qiladi;
`tests/sqlx_version_parity.rs` ikkala liniya bir xil tip nomi va compatibility
natijasini berishini isbotlaydi; ignored jonli suite esa `just postgres-test` /
`just postgres-test-08` orqali PostgreSQL 16da haqiqiy encode/decode/query
yo'llarini tekshiradi.

README `src/lib.rs` orqali crate hujjatiga `date` va `id` feature'lari yoqilganda
qo'shiladi. `cargo test --all-features --doc` oddiy `rust` bloklarini bajaradi va
`rust,compile_fail` blokining kompilyatsiya bo'lmasligini tekshiradi. SQLx bo'limidagi
`rust,ignore` blok esa pool va iste'molchi tiplari kontekstini ko'rsatadigan namuna:
u doctest sifatida kompilyatsiya qilinmaydi.

---

## Litsenziya

MIT yoki Apache-2.0 — o'zingizga qulayini tanlang.
