# O'zgarishlar tarixi

Format [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) asosida,
versiyalash [SemVer](https://semver.org/lang/uz/) bo'yicha.

Bo'limlar: **Qo'shildi** · **O'zgardi** · **Olib tashlandi** · **Tuzatildi** · **Xavfsizlik** · **Hujjatlashtirildi**.
Breaking o'zgarishlar ⚠️ bilan belgilanadi va reliz oxirida migratsiya jadvali beriladi.

## [Unreleased]

_Hozircha bo'sh._

---

## [0.21.0] — 2026-09-05

Bitta mavzu: **0.20.0 boshlagan ish oxiriga yetkazildi**. `ClientId` — crate'da
qolgan oxirgi domen nomi edi, u ham o'chirildi.

0.20.0 da `JobId`, `SessionId`, `RequestId` aynan shu sabab olib tashlangan, lekin
`ClientId` e'tibordan chetda qolgan. U ham xuddi o'sha muammolarni takrorlardi:

- **Prelude nom to'qnashuvi** — `use uz_types::prelude::*` har bir servisga `ClientId`
  nomini tiqardi. OAuth bilan ishlaydigan servis aynan shu nomni o'zi e'lon qilishni
  xohlaydi.
- **Ko'rinish domen nomiga yopishtirilgan edi** — `ClientId` majburan string. Client
  identifikatorini UUID yoki `BIGINT` qilib saqlaydigan tizim uchun nom band edi.
- **Tip hech narsa qo'shmasdi** — validatsiyasi bo'sh (`normalize` hech nima qilmaydi,
  `validate` faqat "bo'sh emas + 8 KiB dan qisqa" deb tekshiradi). Bu qoida OAuth
  spetsifikatsiyasidan emas, token tipidan meros bo'lib qolgan edi.

Sirlar (`AccessToken`, `RefreshToken`, `ClientSecret`) o'z joyida — ular **mexanizm**
beradi (yashirilgan `Debug`, constant-time `==`, `zeroize`), domen nomi emas.

### Qo'shildi

- `NumId<Tag, R>` uchun `TryFrom<&str>`, `TryFrom<String>` va `From<NumId<Tag, R>> for String`.
  Endi konversiya sirti `Id<Tag>` va string tiplari bilan bir xil: `T: TryFrom<String>`
  bound'i ostidagi generic kod uchala oilada ham ishlaydi. Ilgari `NumId` da faqat
  `FromStr` bor edi — crate'dagi yagona istisno.

### ⚠️ Breaking

- `uz_types::ClientId` — **o'chirildi**.
- `prelude` dan ham chiqarildi.
- `secret` modulida endi `sqlx` impl'lari yo'q: `ClientId` u yerdagi yagona
  `string_newtype!` edi, sir tiplarida esa sqlx ataylab yo'q.

### Hujjatlashtirildi

- README § Sir tiplari: taqqoslash jadvalidan `ClientId` ustuni olib tashlandi —
  jadval endi faqat sir tiplarining cheklovlarini ko'rsatadi.
- README § `Id\<Tag\> va NumId\<Tag\>`: `NumId` ning to'liq konversiya sirti yozildi.
- `examples/types_example.rs` va `Cargo.toml` dagi eskirgan `JobId` izohi tuzatildi.

### Migratsiya 0.20 → 0.21

| 0.20 | 0.21 |
| --- | --- |
| `use uz_types::ClientId;` | o'z tipingiz: `pub struct ClientId(String);` yoki `Id<ClientTag>` |
| `use uz_types::prelude::*` → `ClientId` | endi berilmaydi; o'zingiz e'lon qilasiz |
| `ClientId` majburan string edi | ko'rinish sizniki: string, `Id<Tag>` yoki `NumId<Tag, i64>` |
| `ClientId` sqlx orqali `TEXT` edi | o'z tipingizga `#[derive(sqlx::Type)]` yoki `string_newtype!` shabloni |

`ClientId` validatsiyasi bo'sh edi, shuning uchun oddiy `String` ga qaytish ham yo'qotish
emas. Qat'iy tip xohlasangiz — crate'dagi `Passport` shablonini nusxa oling.

---

## [0.20.0] — 2026-09-04

Bitta mavzu: **crate endi domen nomlarini bermaydi**. `JobId`, `SessionId`,
`RequestId` alias'lari va `tag::{Job, Session, Request}` moduli o'chirildi.

Ular 0.17 → 0.18 migratsiya shim'i sifatida qo'shilgan edi (kod izohining o'zida
shunday yozilgan) va endi zarar keltirardi:

- **Prelude nom to'qnashuvi** — `use uz_types::prelude::*` har bir servisga
  `SessionId`, `RequestId`, `JobId` nomlarini tiqardi. Bular aynan servis o'zi
  e'lon qilishni xohlaydigan nomlar.
- **Ko'rinish domen nomiga yopishtirilgan edi** — `SessionId` majburan UUID.
  Raqamli session ID xohlagan foydalanuvchi uchun nom band edi.
- **Crate o'z falsafasiga zid edi** — hujjatlar "o'z tag'ingizni o'zingiz e'lon
  qilasiz" deb turib, uchta tayyor tag berardi.

Endi crate faqat **mexanizm** beradi: `Id<Tag>` (UUID) va `NumId<Tag, R>` (BIGINT).
Nomni ham, ko'rinishni ham siz tanlaysiz.

### ⚠️ Breaking

- `uz_types::{JobId, SessionId, RequestId}` — **o'chirildi**.
- `uz_types::tag` moduli (`Job`, `Session`, `Request`) — **o'chirildi**.
- `prelude` dan ham chiqarildi.

### Hujjatlashtirildi

- README: `Id`/`NumId` bo'limi qayta yozildi — **yaratish tartibi 3 qadamda**
  (tag → alias → ishlatish), `compile_fail` doctest bilan tip xavfsizligi demosi,
  va nima uchun crate tayyor nom bermasligi tushuntirildi.
- `examples/types_example.rs` shu 3 qadamning ishga tushadigan ko'rinishiga aylandi.
- `uuid` re-export qilinmagani README'da aniq yozildi.

### Migratsiya 0.19 → 0.20

| 0.19 | 0.20 |
| --- | --- |
| `use uz_types::JobId;` | `pub enum Job {}` + `pub type JobId = Id<Job>;` |
| `uz_types::tag::Session` | `pub enum Session {}` — o'z loyihangizda |
| `use uz_types::prelude::*` → `SessionId` | endi berilmaydi; o'zingiz e'lon qilasiz |
| `SessionId` majburan UUID edi | endi tanlov sizniki: `Id<Session>` yoki `NumId<Session, i64>` |

Tag'larni butun loyiha uchun **bitta** modulda saqlang — `Id<a::Order>` va
`Id<b::Order>` bir-biriga to'g'ri kelmaydigan turli tiplar.

---

## [0.19.0] — 2026-09-04

Ikkita mavzu: **`BirthDate` ning quyi chegarasi PINFL bilan moslashtirildi** va
**`NumId` ↔ `BIGINT` nomuvofiqligi yopildi**. Deyarli hammasi additive —
`#[non_exhaustive]` va default type parameter tufayli mavjud kod o'zgarmaydi;
bitta istisno pastda ⚠️ bilan berilgan.

### O'zgardi

- **`BirthDate::MIN_YEAR`: 1900 → 1800.** PINFL 1-raqami `1`/`2` bo'lganda asr 1800
  (`Pinfl::century()`), shuning uchun eski chegarada `Pinfl::birth_date()` bunday
  PINFL uchun **har doim `None`** qaytarardi. `MIN_YEAR` — sanity floor, biznes
  qoidasi emas: yosh chegarasini `age_at()` bilan ilova qatlamida qo'ying.
- **MSRV 1.94 → 1.85.** 1.94 ni faqat `sqlx 0.9` talab qiladi, u esa *optional*.
  `Cargo.toml` dagi `rust-version` endi eng past umumiy qiymat; `sqlx`/`sqlx-postgres`
  feature'lari uchun 1.94+ kerakligi hujjatlangan va CI ikkala polni alohida tekshiradi.
  MSRV iste'molchi uchun o'lchanadi — dev-dependency'lar (`criterion` → 1.86) kirmaydi.
- `PhoneNumber::parse()` `.` ajratuvchisini ham qabul qiladi (`998.90.123.45.67`).
- `IdError::Number` xabari `u64`/`i64` ikkala ko'rinish uchun umumiy qilindi.
- `NumId` ning sqlx xatolari `String` o'rniga strukturali `IdError`
  (`BoxDynError` ichida `downcast_ref::<IdError>()` bilan ushlanadi).

### Qo'shildi

- **`NumId<Tag, R>`** — ichki raqam ko'rinishi endi parametr, `R: NumIdRepr`
  (sealed: `u64` | `i64`), default `u64`. `NumId<Order>` avvalgidek ishlaydi.

  | | `NumId<Tag>` (`u64`) | `NumId<Tag, i64>` |
  | --- | --- | --- |
  | `Encode` → `BIGINT` | `> i64::MAX` → `NumberTooLarge` | **xato yo'li yo'q** |
  | `Decode` ← `BIGINT` | manfiy → `NumberNegative` | **xato yo'li yo'q** |
  | Manfiy legacy ID | ❌ | ✅ |

- `NumId::<Tag, u64>::MAX_DB_SAFE`, `try_new_db_safe()`, `parse_db_safe()` —
  `BIGINT` chegarasini **query paytidan konstruksiya paytiga** ko'chiradi.
- `NumId::to_bigint()`, `NumId::is_db_safe()` — query yuborishdan oldin tekshirish.
- `TryFrom` ikkala yo'nalishda: `NumId<Tag, u64>` ↔ `NumId<Tag, i64>` (tekshiriladi).
- `IdError::NumberTooLarge { value: u64 }` va `IdError::NumberNegative { value: i64 }`
  (`match` uchun breaking emas — `IdError` `#[non_exhaustive]`; lekin ⚠️ pastga qarang).
- `Pinfl::birth_date_at(today)` — `birth_date()` ning deterministik varianti
  (u tizim soatiga tayanadi, chunki `BirthDate` kelajak sanasini rad etadi).
- `prelude` ga `tag` moduli va `NumIdRepr` qo'shildi.
- `[[example]] required-features = ["date", "id"]` — busiz
  `cargo check --all-targets --no-default-features` kompilyatsiya bo'lmasdi.

### Tuzatildi

- `MIN_YEAR` o'zgarishidan keyin qolib ketgan test va README doctest
  (`BirthDate::parse("1899-12-31")` endi `Ok`) — ikkalasi ham `main` da qizil edi.
- README'dagi mavjud bo'lmagan narsalar: `just test-features`/`bench`/`semver`
  recipe'lari, `tests/api.rs`, `postgres:16` CI service va `cargo semver-checks`
  da'vosi — hech biri yo'q edi.
- `justfile` `msrv` recipe'idagi 1.85/1.94 nomuvofiqligi.

### ⚠️ Breaking

- **`IdError` endi "field-less" enum emas** — unga struct-variant qo'shildi
  (`NumberTooLarge { value }`), shuning uchun `IdError::Uuid as isize` kabi
  raqamli cast'lar endi kompilyatsiya bo'lmaydi. `match` ga ta'sir yo'q.
  `cargo semver-checks` (`enum_discriminants_undefined_non_unit_variant`) topgan;
  0.x da 0.18 → 0.19 major bump bo'lgani uchun semver buzilmagan.

### Migratsiya 0.18 → 0.19

| 0.18 | 0.19 |
| --- | --- |
| `NumId<Order>` | o'zgarmaydi (`R` default `u64`) |
| Manfiy `BIGINT` ustun → `Decode` xatosi | `NumId<Order, i64>` |
| `> i64::MAX` ID → query paytida xato | `parse_db_safe()` / `try_new_db_safe()`, yoki `NumId<Order, i64>` |
| sqlx xatosini matn bo'yicha tekshirish | `err.downcast_ref::<IdError>()` |
| `IdError::Uuid as isize` | ⚠️ endi kompilyatsiya bo'lmaydi — `match` ishlating |
| `BirthDate::parse("1850-01-01")` → `TooOld` | `Ok`; yosh chegarasi `age_at()` bilan ilovada |

---

## [0.18.0] — 2026-09-03

Bu relizning maqsadi — crate'ni **1.0 ga tayyorlash**: barcha breaking o'zgarishlar
bitta relizda chiqariladi, keyingi relizlar faqat additive bo'ladi.

Uchta katta o'zgarish:

1. **Feature flag'lar.** `serde`, `chrono`, `uuid` endi majburiy emas — tiplar
   default'da bor, integratsiyalar (`serde`, `sqlx`, `zeroize`) siz tanlaysiz.
2. **Ikki qatlamli validatsiya.** `parse()` faqat o'zgarmas strukturani tekshiradi;
   o'zgaruvchan qoidalar (operator kodlari ro'yxati, PINFL nazorat raqami) —
   `is_*()` metodlar va yangi `parse_strict()`.
3. **Sirlar type-driven.** Token tiplaridan `Display` olib tashlandi — sirni
   faqat `expose_secret()` ochadi.

### ⚠️ Breaking o'zgarishlar (qisqa ro'yxat)

- MSRV 1.85 → **1.94**.
- `serde` endi feature (`features = ["serde"]`); `chrono` → `date`, `uuid` → `id` feature (ikkalasi default'da yoqiq).
- `parse()` har tipda **o'zining** error tipini qaytaradi (`PassportError`, `PinflError`, …), `TypeError` emas.
- `Deref` barcha tiplardan olib tashlandi.
- Token tiplarida `Display`, `as_str()`, `into_inner()`, `AsRef<str>` yo'q — `expose_secret()`.
- `PhoneNumber::parse()` operator kodini tekshirmaydi (→ `parse_strict()`), ajratuvchilarni qabul qiladi.
- ID tiplari: `IdFormat` (UUID yoki raqam) o'rniga `Id<Tag>` (faqat UUID) va `NumId<Tag>` (faqat u64). `Reuid` o'chirildi.
- `TypeError::PINFL` → `TypeError::Pinfl`; `TypeError::Validation` o'chirildi.
- `EmailAddress::MIN_LEN` o'chirildi; `DateFormat` `#[non_exhaustive]`; `DateFormat::as_str()` → `pattern()`.

To'liq ro'yxat va o'rniga nima ishlatish — quyidagi [migratsiya jadvali](#migratsiya-017--018).

### Qo'shildi

**Feature'lar** (`Cargo.toml`):


| Feature             | Default | Nima beradi                                                         |
| ------------------- | ------- | ------------------------------------------------------------------- |
| `date`              | ✅      | `BirthDate`, `DateFormat`, `Pinfl::birth_date()` (`chrono`)         |
| `id`                | ✅      | `Id<Tag>`, `NumId<Tag>`, `JobId`, `SessionId`, `RequestId` (`uuid`) |
| `serde`             |         | `Serialize`/`Deserialize` (sirlar — faqat `Deserialize`)           |
| `sqlx`              |         | `Type`/`Encode`/`Decode` — driver'ga bog'liq emas                  |
| `sqlx-postgres`     |         | `sqlx` + `PgHasArrayType` (`Vec<T>`, `= ANY($1)`)                   |
| `zeroize`           |         | Sir tiplari`Drop` da xotirani tozalaydi                             |
| `serialize-secrets` |         | Sir tiplari uchun`Serialize`                                        |

**sqlx integratsiyasi** (`sqlx` / `sqlx-postgres` feature): `Passport`, `Pinfl`,
`PhoneNumber`, `EmailAddress`, `ClientId` → `TEXT`; `BirthDate` → `DATE`;
`Id<Tag>` → `UUID`; `NumId<Tag>` → `BIGINT`. `Decode` `parse()` orqali o'tadi —
DB'dagi buzuq yozuv `try_get` da xato beradi, jimgina ichkariga kirmaydi.
Sir tiplari uchun sqlx impl'lar ataylab yo'q.

**`Pinfl`** — rasmiy struktura (Vazirlar Mahkamasining 12.04.2022 dagi 177-son qarori):

- `parse_strict()` — struktura + nazorat raqami (vaznlar 7‑3‑1, mod 10) + jins/asr belgisi + sana strukturasi;
- `is_checksum_valid()`, `gender() -> Option<Gender>`, `century()`, `birth_date_parts() -> Option<(yil, oy, kun)>`,
  `region_code()`, `serial()`; `date` feature bilan `birth_date() -> Option<BirthDate>`;
- yangi `Gender` enum'i; `PinflError::Checksum`, `PinflError::Structure` variantlari.

**`PhoneNumber`**:

- `parse()` endi ajratuvchilarni qabul qiladi: `+998 (90) 123-45-67`, `998 90 123 45 67` → `998901234567`;
- `parse_strict()` — struktura + operator/hudud kodi ro'yxatda bo'lishi shart;
- `is_known_operator()` metodi (avvalgi `is_known_operator_code(code)` o'rniga);
- `REGIONAL_CODES: RangeInclusive<u8>` (`60..=79`) konstantasi.

**`Passport`**: ichki bo'sh joy qabul qilinadi — `AA 1234567` → `AA1234567`.

**`BirthDate`**: deterministik konstruktorlar `parse_with_format_at(value, format, today)`
va `from_naive_date_at(date, today)`; `as_naive_date()`; `PartialOrd`/`Ord`.

**ID tiplari**:

- `Id<Tag>` — tipli UUID (`PhantomData<fn() -> Tag>`): `new_v4()`, `now_v7()`, `from_uuid()`,
  `parse()`, `as_uuid()`, `into_uuid()`, `version()`, `is_nil()`. Foydalanuvchi o'z tag'ini o'zi
  e'lon qiladi (`pub enum Order {}; type OrderId = Id<Order>;`);
- `NumId<Tag>` — tipli `u64` (legacy tizimlar, `BIGINT`): `new()`, `parse()`, `get()`;
- `tag::{Job, Session, Request}` va `JobId`/`SessionId`/`RequestId` alias'lari;
- ikkala tip `Copy`, `Eq`, `Ord`, `Hash`, `Send + Sync` — `Tag` qanday bo'lishidan qat'i nazar.

**Sir tiplari**: `expose_secret()`; constant-time `==` (`subtle`); `serialize-secrets` va
`zeroize` feature'lari. `ClientId` sir emas — oddiy string tipi (`Display`, `as_str()`).

**Barcha `String` tiplar**: `Borrow<str>` (`HashMap<Passport, _>::get("AA…")` ishlaydi),
`PartialOrd`/`Ord`.

**Testlar va sifat**: property-based testlar (`proptest`: hech qanday input panic qilmaydi,
`parse` idempotent, PINFL checksum generatori), `criterion` benchmark, `cargo hack`
feature powerset, `cargo semver-checks`, `cargo audit`/`cargo deny`, Postgres integration
job — CI'da.

### O'zgardi

- **`parse()` imzosi**: `parse(value: impl AsRef<str>) -> Result<Self, TypeError>`
  → `parse(value: &str) -> Result<Self, <Tip>Error>`. `FromStr::Err`, `TryFrom::Error` ham aniq tip.
  `?` operatori `TypeError` ga avvalgidek o'tadi (`#[from]`).
- **Yagona konstruktor yo'li.** `TryFrom<String>` — kanonik yo'l (trim → normalizatsiya →
  validatsiya, hammasi in-place, qo'shimcha allocation yo'q). `parse`, `FromStr`, `TryFrom<&str>`,
  serde `Deserialize`, sqlx `Decode` shu yo'ldan o'tadi. Avvalgi "agar toza bo'lsa xotirani qayta
  ishlataman" degan shart yo'q.
- **`PhoneNumber::parse()`** operator/hudud kodini tekshirmaydi — `998000000000` endi `Ok`.
  Qat'iylik uchun `parse_strict()`. Sabab: ro'yxat o'zgaruvchan biznes fakti; yangi kod
  ajratilganda foydalanuvchilar rad etilmasligi va DB/Kafka'dagi eski yozuvlar o'qilishi kerak.
- **`Id::parse()`** har qanday RFC 9562 UUID'ni qabul qiladi (v1/v3/v5 ham). Versiya —
  `version()` bilan tekshiriladi. `generate()`/`generate_v7()` → `new_v4()`/`now_v7()`
  (`uuid` crate uslubi; jim xatti-harakat o'zgarishi bo'lmasligi uchun eski nom saqlanmadi).
- **serde**: `Id<Tag>` JSON'da har doim string, `NumId<Tag>` — har doim integer
  (`"42"` string qabul qilinmaydi). `deserialize_any` ishlatilmaydi — bincode/postcard ishlaydi.
  `Id` binary formatlarda 16 bayt.
- **`EmailAddress::parse("a@b")`** → `Format` (avval `Length`).
- **`PhoneNumber::MOBILE_CODES`**: `[&str; 14]` → `&[&str]` (yangi kod qo'shish breaking emas).
- **`PhoneNumberError::OperatorCode`** → `UnknownOperatorCode` (faqat `parse_strict` qaytaradi).
- **`BirthDate::from_naive_date_with_today`** → `from_naive_date_at`; `into_inner()` → `as_naive_date()`.
- **`DateFormat`**: `#[non_exhaustive]`; `as_str()` → `pattern()`.
- **`TypeError::PINFL`** → `TypeError::Pinfl`. `BirthDate` va `Id` variantlari mos feature bilan.
- **`#![deny(missing_docs)]`** → `#![warn(missing_docs)]` (CI'da `-D warnings` bor; yangi
  rustc lint'i foydalanuvchi build'ini sindirmaydi).
- `serde` dependency'si `derive` feature'siz — barcha impl'lar qo'lda; `chrono` va `uuid`
  `default-features = false` bilan.
- Modul nomlari: `token_types.rs` → `secret.rs`, `uuid_types.rs` → `id.rs` (public path'lar
  o'zgarmagan — hammasi crate ildizidan).

### Olib tashlandi

- `Deref<Target = str>` (`Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress`) va
  `Deref<Target = NaiveDate>` (`BirthDate`) — Rust API Guidelines C-DEREF. `as_str()`/`AsRef`/`Borrow<str>` qoladi.
- Token tiplarida: `Display`, `as_str()`, `into_inner()`, `AsRef<str>`, derive `PartialEq`/`Hash`, default `Serialize`.
- `IdFormat` (UUID/raqam aralash enum), `Reuid`, `as_number()`, `uuid_version()`, `IdError::Version`.
- `TypeError::Validation` va `TypeError::validation()` (0.17.0 da deprecated edi).
- `EmailAddress::MIN_LEN`.
- `PhoneNumber::is_known_operator_code(code)` (assotsiativ fn), `REGIONAL_CODE_RANGE`.
- `BirthDate::format_reversed()` — `format_as(format.reversed())` ishlating; `From<BirthDate> for String` — `to_string()`.
- Barcha tiplarda `Cow<str>` orqali deserializatsiya (o'lik kod edi — pastga qarang).

### Xavfsizlik

- **Token tiplarida `Display` yo'q.** `format!("{token}")`, `.to_string()`, `tracing::info!("{token}")`
  endi compile error — sir logga tushib qolish yo'li yopildi. Yagona ochiq yo'l: `expose_secret()`.
- **Constant-time taqqoslash.** Sir tiplarining `==` operatori `subtle::ConstantTimeEq` bilan —
  timing side-channel yo'q (faqat uzunlik farqi oshkor bo'ladi, bu standart).
- **`Serialize` default'da yo'q.** Token ichida bo'lgan struct'ni `serde_json::to_string` qilish
  sirni oshkor qilmaydi — kerak bo'lsa `serialize-secrets` feature.
- **`zeroize` feature.** `Drop` da xotira nolga to'ldiriladi; rad etilgan qiymat ham. Cheklov:
  `String` realloc/clone nusxalari va HTTP/serde buferlari tozalanmaydi ("best effort").

### Tuzatildi

- **`Cow::Borrowed` — o'lik kod.** `serde` ning `Cow<str>` uchun `Deserialize` impl'i har doim
  `Cow::Owned` qaytaradi (`serde_core` `impls.rs`), shuning uchun 9 ta tipdagi `Borrowed` tarmog'i
  hech qachon ishlamas edi. O'rniga bitta umumiy `Visitor`: `visit_str` → `parse`,
  `visit_string` → `TryFrom<String>` (deserializer bergan xotira qayta ishlatiladi).
- `EmailAddress::local_part()`/`domain()` dagi `.expect()` — panic yo'li umuman olib tashlandi
  (`split_once` bilan).
- `TryFrom<String>` mantig'i 5 tipda 5 xil yozilgan edi (Pinfl'da trim yo'li baribir allocate
  qilardi) — bitta makroda birlashtirildi.
- `BirthDate`: tizim soatiga murojaat bitta private funksiyaga jamlandi; barcha konstruktorlarning
  deterministik `*_at()` varianti bor.

### Hujjatlashtirildi

- README to'liq qayta yozildi: feature jadvali, ikki qatlamli validatsiya, har tip uchun misollar
  (barchasi doctest sifatida tekshiriladi), serde/sqlx bo'limlari, cheklovlar, MSRV/semver siyosati.
- `MAX_TOKEN_LEN` haqidagi "xotirani to'ldirishning oldini oladi" da'vosi tuzatildi — `String`
  tekshiruvga kelguncha allaqachon ajratilgan bo'ladi; himoya HTTP body-limit darajasida.
- "Minimal allocation" da'vosi o'lchov bilan: `Passport::parse(&str)` ≈ 28 ns (bitta allocation),
  `Passport::try_from(String)` ≈ 27 ns (allocation yo'q), `EmailAddress::parse` ≈ 110 ns.
- docs.rs'da feature badge'lari (`doc_cfg`).

### Migratsiya 0.17 → 0.18

`Cargo.toml`:

```toml
# 0.17
uz-types = "0.17"

# 0.18 — serde kerak bo'lsa aniq yozing
uz-types = { version = "0.18", features = ["serde"] }
# DB bilan ishlasangiz:
uz-types = { version = "0.18", features = ["serde", "sqlx-postgres"] }
```

Kod:


| 0.17                                                         | 0.18                                                                                        |
| ------------------------------------------------------------ | ------------------------------------------------------------------------------------------- |
| `Passport::parse(s) -> Result<_, TypeError>`                 | `-> Result<_, PassportError>`; `?` avvalgidek `TypeError` ga o'tadi                         |
| `Passport::parse(some_string)` (`impl AsRef<str>`)           | `Passport::parse(&some_string)`                                                             |
| `passport.starts_with("AA")` (Deref)                         | `passport.as_str().starts_with("AA")`                                                       |
| `TypeError::PINFL(_)`                                        | `TypeError::Pinfl(_)`                                                                       |
| `TypeError::Validation` / `TypeError::validation()`          | o'chirildi — o'z domeningiz uchun alohida error tipi yarating                              |
| `EmailAddress::parse("a@b")` → `Err(Length)`                | →`Err(Format)`                                                                             |
| `PhoneNumber::parse("998000000000")` → `Err(OperatorCode)`  | →`Ok`; `parse_strict()` → `Err(UnknownOperatorCode)`                                      |
| `PhoneNumber::is_known_operator_code(code)`                  | `phone.is_known_operator()`                                                                 |
| `PhoneNumber::REGIONAL_CODE_RANGE.0 / .1`                    | `PhoneNumber::REGIONAL_CODES.start() / .end()`                                              |
| `format!("{token}")`, `token.as_str()`, `token.into_inner()` | `token.expose_secret()`                                                                     |
| `serde_json::to_string(&access_token)`                       | `serialize-secrets` feature, yoki struct'da `access_token: &str` maydon (`expose_secret()`) |
| `ClientId` sir sifatida (`{:?}` yashirilgan)                 | oddiy tip:`Display`, `as_str()`; `Debug` ochiq                                              |
| `JobId::generate()` / `generate_v7()`                        | `JobId::new_v4()` / `JobId::now_v7()`                                                       |
| `JobId::parse("42")` → `Ok` (raqam)                         | →`Err(IdError::Uuid)`; raqamli ID uchun `NumId<Tag>`                                       |
| `id.as_uuid() -> Option<&Uuid>`                              | `id.as_uuid() -> &Uuid`                                                                     |
| `id.as_number() -> Option<u64>`                              | `NumId<Tag>::get() -> u64`                                                                  |
| `id.uuid_version()`                                          | `id.version()`                                                                              |
| v1/v3/v5 UUID →`Err(IdError::Version)`                      | `Ok`; kerak bo'lsa `id.version()` bilan tekshiring                                          |
| `Reuid`                                                      | `pub enum MyTag {}` + `type MyId = Id<MyTag>;`                                              |
| ID JSON:`"uuid"` yoki `123`                                  | `Id` — faqat `"uuid"`; `NumId` — faqat `123` (`"123"` string emas)                        |
| `BirthDate::from_naive_date_with_today(d, today)`            | `BirthDate::from_naive_date_at(d, today)`                                                   |
| `birth_date.into_inner()`                                    | `birth_date.as_naive_date()`                                                                |
| `birth_date.leap_year()` (Deref)                             | `birth_date.as_naive_date().leap_year()`                                                    |
| `birth_date.format_reversed(fmt)`                            | `birth_date.format_as(fmt.reversed())`                                                      |
| `DateFormat::as_str()`                                       | `DateFormat::pattern()`                                                                     |
| `match date_format { … }` (to'liq)                          | `_` tarmog'i qo'shing (`#[non_exhaustive]`)                                                 |
| MSRV 1.85                                                    | 1.94 (`rustup update`)                                                                      |

Compiler yordam beradi: yuqoridagi o'zgarishlarning deyarli hammasi compile error sifatida
ko'rinadi — jim xatti-harakat o'zgarishi faqat ikkita: `PhoneNumber::parse` operator kodini
tekshirmaydi va `Id::parse` UUID versiyasini tekshirmaydi. Qat'iylik kerak bo'lgan joyda
`parse_strict()` / `version()` ishlating.

---

## [0.17.0] — 2026-08-14

Bu relizning maqsadi — public API'ni tashqi foydalanuvchi uchun ishlaydigan holga
keltirish va validatsiyadagi teshiklarni yopish.

### Qo'shildi

- **Crate ildizidan eksport.** `use uz_types::Passport;` ishlaydi. Ilgari tiplarga faqat
  `prelude` orqali kirish mumkin edi.
- **Barcha error tiplari eksport qilindi**: `PassportError`, `PinflError`, `PhoneNumberError`,
  `EmailAddressError`, `BirthDateError`, `IdError`, `TokenError`. Ilgari `TypeError` ichidagi
  aniq xatoni `match` qilib bo'lmas edi.
- `ClientId` va `ClientSecret` eksport qilindi (avval yaratilgan, lekin ko'rinmas edi).
- `tests/api.rs` — public API'ni tashqaridan tekshiruvchi integration testlar.
- `PhoneNumber`: `operator_code()`, `subscriber_number()`, `is_mobile()`,
  `is_known_operator_code()`, `MOBILE_CODES`, `REGIONAL_CODE_RANGE`, `OPERATOR_CODE_LEN`.
- `BirthDate`: `age()`, `age_at()`, `from_naive_date_with_today()`, `MIN_YEAR`.
- ID tiplari: `generate_v7()` (vaqt bo'yicha tartiblangan UUID), `uuid_version()`.
- Token tiplari: `MAX_TOKEN_LEN` (8 KiB) va `<Tip>::MAX_LEN`.
- `EmailAddress`: `LOCAL_PART_MAX_LEN`, `DOMAIN_MAX_LEN`, `DOMAIN_LABEL_MAX_LEN`, `TLD_MIN_LEN`.
- `FromStr` barcha tiplar uchun — `"AA1234567".parse::<Passport>()` ishlaydi.
- CI (`.github/workflows/ci.yml`): fmt, clippy, test, rustdoc, MSRV, `cargo package`.
- Ushbu `CHANGELOG.md`.

### O'zgardi — validatsiya qat'iylashdi

> Quyidagi qiymatlar ilgari **noto'g'ri qabul qilinardi**, endi rad etiladi.
> DB'ingizda shunday qiymatlar bo'lsa, migratsiyadan oldin tekshirib chiqing.

- `EmailAddress`: bo'sh domain label (`a@b..c`, `a@.b.com`, `a@b.com.`); `-` bilan
  boshlanuvchi/tugovchi label; local-part'da `..`; 1 harfli yoki raqamli TLD (`a@b.c`, `a@b.c1`);
  64 belgidan uzun local-part; ruxsatsiz belgilar (`(`, `,`, `:`, `\`); bir nechta `@`.
- `PhoneNumber`: `998` dan keyingi 2 raqam ma'lum operator/hudud kodi bo'lishi shart —
  `998000000000` → `PhoneNumberError::OperatorCode`.
- `BirthDate`: `MIN_YEAR` (1900) dan oldingi sanalar → `BirthDateError::TooOld`.
- `BirthDate`: kelajak chegarasi endi UTC bo'yicha _ertangi_ kun — UTC+5 (Toshkent) da bugun
  tug'ilgan chaqaloq xato `FutureDate` olmaydi.
- Token tiplari: 8 KiB dan uzun qiymat → `TokenError::TooLong`.

### Optimallashtirildi

- `EmailAddress::validate` `Vec<&str>` yig'maydi (`split_once`) — validatsiya allocation'siz.
- `EmailAddress::parse` avval validatsiya, keyin lowercase — noto'g'ri kirish uchun allocation yo'q.

### Eskirdi (deprecated)

- `TypeError::validation()` — 0.18.0 da `TypeError::Validation` bilan birga olib tashlandi.

### Hujjatlashtirildi

- `Pinfl` va `Passport` faqat formatni tekshirishi ochiq yozildi.
- Har tipning `parse()` metodiga `# Xatolar` bo'limi.
- Token tiplarida `Display` sirni ochishi haqida ogohlantirish (0.18.0 da `Display` olib tashlandi).
- ID tiplari serde'da ikki xil JSON shakl berishi (0.18.0 da bitta shaklga keltirildi).
- README'dagi "Zero-Allocation" → "Minimal allocation"; cheklovlar jadvali.

### Tuzatildi

- README va example'dagi `iclud.com` typo'si; `pinfl.rs` va `phone_number.rs` dagi typo'lar.
- `Cargo.toml` `include` ro'yxatiga `tests/**/*.rs` qo'shildi.
- `Deserialize` impl ustidagi o'lik `#[allow(unknown_lints)]` atributi olib tashlandi.

### Test qamrovi

- `id_type_tests!` `RequestId` va `Reuid` uchun ham; UUID v3/v5/v7 testlari.
- 67 → 122 unit + 17 integration + 3 doctest.

---

## [0.16.0] va undan oldingi versiyalar

Bu versiyalar uchun o'zgarishlar hujjatlashtirilmagan — git tarixiga qarang.

---

## Rejalashtirilgan

**1.0 gacha**: Postgres integration testlari CI'da (`#[sqlx::test]`) — hozircha
sqlx impl'lari faqat compile-time tekshiriladi, jonli DB'da sinalmagan;
`trybuild` compile-fail testlar (sir tiplari `Display`/`Serialize` bermasligini qulflash);
`deny.toml` (litsenziya/manba siyosati).

**1.0.0**: `cargo semver-checks` kamida bitta minor reliz davomida yashil bo'lgandan va 0.18/0.19
real servisda ishlatilgandan keyin. Feature nomlari va public API qulflanadi.

**1.0 dan keyin** (yangi tiplar, crate'ga kirmaydi): `Inn`/`Stir`, `BankCard` (Luhn), `Mfo`,
`AccountNumber`; `PhoneNumber::parse_local()` (9 raqamli mahalliy shakl).

[Unreleased]: https://github.com/diordev/uz-types/compare/v0.21.0...HEAD
[0.21.0]: https://github.com/diordev/uz-types/compare/v0.20.0...v0.21.0
[0.20.0]: https://github.com/diordev/uz-types/compare/v0.19.0...v0.20.0
[0.19.0]: https://github.com/diordev/uz-types/compare/v0.18.0...v0.19.0
[0.18.0]: https://github.com/diordev/uz-types/compare/v0.17.0...v0.18.0
[0.17.0]: https://github.com/diordev/uz-types/releases/tag/v0.17.0
