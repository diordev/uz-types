---
name: idiomatic-rust
description: uz-types crate'ida idiomatik Rust yozish qoidalari — type-driven design, newtype va smart constructor, xato dizayni (thiserror), allocation intizomi, trait/generic API, feature gating, panic xavfsizligi va MSRV chegaralari. Bu repoda ISTALGAN `.rs` faylga kod yozishdan, refactor qilishdan, review qilishdan yoki yangi tip/metod/feature qo'shishdan OLDIN shu skillni o'qing — foydalanuvchi "idiomatik" so'zini aytmasa ham. Shuningdek `Cargo.toml`, feature, MSRV, semver, clippy yoki API dizayni haqida savol berilganda ishlating.
---

# Idiomatik Rust — `uz-types`

Bu skill **qaror qabul qilish** haqida: qaysi tipni tanlash, xatoni qanday modellash,
allocation'ni qayerda tejash. Loyihaning arxitekturasi va buyruqlari `CLAUDE.md` da —
u yerda takrorlanmaydi.

Ishlash tartibi: kod yozishdan oldin quyidagi bo'limlarni o'qing → yozing →
oxiridagi **Chiqish nazorati** ro'yxatidan o'tkazing → `just check`.

---

## 1. Type-driven design — asosiy tamoyil

**Noto'g'ri holatni umuman ifodalab bo'lmasin.** Validatsiya bir marta, chegarada
bo'ladi; undan keyin tip o'zi kafolat beradi.

- `String` o'rniga newtype: `Passport(String)` — bir marta tekshirilgan, keyin hech
  qachon qayta tekshirilmaydi. Funksiya `&str` qabul qilsa, chaqiruvchi har safar
  "bu tekshirilganmi?" deb o'ylashi kerak bo'ladi.
- **"Parse, don't validate"** — `fn validate(s: &str) -> bool` emas, balki
  `fn parse(s: &str) -> Result<Self, Error>`. Birinchisi natijani yo'qotadi,
  ikkinchisi tipga muhrlaydi.
- Ichki maydon `pub` bo'lmasin — aks holda smart constructor chetlab o'tiladi va
  invariant buziladi. `serde`/`sqlx` yo'llari ham `parse`/`TryFrom` orqali o'tishi shart.
- Yopiq to'plam → `enum` (`Gender`, `DateFormat`), ochiq to'plam → sealed trait
  (`NumIdRepr`: faqat `u64`/`i64`).
- Compile-time belgisi kerak bo'lsa marker tip: `PhantomData<fn() -> Tag>`
  (`PhantomData<Tag>` emas — u `Send`/`Sync` ni `Tag` dan meros qilib, kovariantlikni buzadi).

## 2. Konstruksiya va normalizatsiya

Bu crate'da yagona yo'l: `trim → normalize → validate` (makro bajaradi).

- Yangi string tip = `string_newtype!` chaqiruvi + `normalize` + `validate`.
  Qo'lda `Display`, `FromStr`, `TryFrom`, `Borrow` yozmang — makroda bor va
  ular orasidagi kelishuv (`Hash`/`Eq`/`Borrow<str>` mosligi) allaqachon to'g'ri.
- `normalize` imzosi ma'no tashiydi: uzunlik o'zgarsa `&mut String`, faqat case
  o'zgarsa `&mut str`. Ikkinchisi "bu funksiya hech qachon qayta ajratmaydi" degan
  kafolatni tipda ifodalaydi.
- `validate` **normalizatsiyadan keyingi** matnni ko'radi. U yerda yana trim qilish —
  ikki marta ish va kelajakdagi bug manbai.
- Normalizatsiya **idempotent** bo'lishi shart: `parse(parse(x).as_str()) == parse(x)`.
  Buni `tests/props.rs` qulflaydi; buzilsa property test tushadi.

## 3. Xato dizayni

- Har modul o'zining aniq xatosini beradi (`PassportError`, `IdError`), `thiserror`
  bilan. Umumiy `TypeError` — faqat `#[from]` yig'uvchi; yangi xato tipi qo'shsangiz
  unga variant qo'shing, aks holda `?` iste'molchida ishlamaydi.
- Xato variantlari **sababni ajratsin**: `Length` va `Format` alohida — chunki
  foydalanuvchiga ko'rsatiladigan xabar boshqacha. "InvalidInput" bitta variant —
  ma'lumot yo'qotish.
- Barcha public enum'lar `#[non_exhaustive]`: keyin variant qo'shish breaking bo'lmaydi.
- Xato tipi arzon bo'lsin (`Copy` bo'lsa yaxshi) — u issiq yo'lda qaytariladi.
  Xatoga `String` solmang, agar aynan qiymat kerak bo'lsa strukturaviy maydon qo'ying
  (`NumberTooLarge { value: u64 }`).
- Kutubxona kodida `unwrap`/`expect`/`panic!` yo'q. Testda va doctestda — mumkin.
- `Option` faqat "sabab qiziq emas" bo'lganda (`gender()`); aks holda `Result`.

## 4. Ownership va allocation

Bu crate "minimal allocation" da'vosini beradi va uni bench bilan qulflaydi — shuning
uchun allocation qo'shish **ataylab qaror**, tasodif emas.

- Ikki yo'lni ajrating: `parse(&str)` bitta allocation qiladi (nusxa kerak);
  `TryFrom<String>` **nol** qo'shimcha allocation — kelgan bufer qayta ishlatiladi.
  Yangi tipda ikkinchi yo'lni buzmang (`to_owned()` chaqirib qo'ymang).
- In-place ishlang: `retain`, `truncate`, `drain`, `make_ascii_uppercase`.
  `s = s.replace(...)` — yangi allocation.
- Tez yo'lni ajrating: avval "toza inputmi?" deb tekshiring, keyingina tozalang
  (`if !s.bytes().all(u8::is_ascii_digit) { s.retain(...) }`). Ko'p holatda input
  allaqachon toza bo'ladi.
- `clone()` refleks emas: avval `&`, keyin `Cow`, keyin `clone`. `clone()` yozayotgan
  bo'lsangiz, izohda nega kerakligini yozing yoki dizaynni o'zgartiring.
- Argument tipi: o'qish uchun `&str`, egalik kerak bo'lsa `String` (yashirin `Into`
  emas — `impl Into<String>` chaqiruvchidan allocation'ni yashiradi).
- Qaytarish: ichki ma'lumotga `&str` (`as_str`), egalikni berish `into_inner`.
  `into_*` egalikni oladi, `as_*` qarz beradi, `to_*` allocation qiladi — bu nomlash
  konvensiyasidan chetga chiqmang.

## 5. API sirti

- `#[must_use]` — natijasi ishlatilmasa metod ma'nosiz bo'lgan har joyda
  (`is_valid`, `as_str`, konstruktorlar). Bu bug tutadi, bezak emas.
- `#[inline]` — faqat trivial, cross-crate chaqiriladigan getter/adapterlarga.
  Hamma joyga qo'yish kompilyatsiyani sekinlashtiradi va foyda bermaydi.
- `const fn` — imkon bo'lsa (`pattern()`, `try_new_db_safe`). Bu iste'molchiga
  compile-time'da ishlatish yo'lini ochadi va keyin olib tashlash breaking bo'ladi —
  ya'ni ataylab qo'shing.
- Public konstantalar: massiv emas, **slice** (`&[&str]`) yoki `RangeInclusive`.
  `[&str; 14]` bo'lsa element qo'shish breaking.
- `derive` noto'g'ri bound qo'shsa, qo'lda impl yozing. `#[derive(Clone)]` generic
  strukturada `Tag: Clone` talab qiladi — `Id<Tag>` da bu noto'g'ri, shuning uchun
  `Clone`, `Copy`, `PartialEq`, `Hash` qo'lda yozilgan.
- `Borrow<str>` implement qilsangiz, `Hash` va `Eq` **aynan** ichki `str` niki bilan
  mos bo'lishi shart — aks holda `HashMap` da qidiruv jimgina ishlamay qoladi.
- Sir tiplarga qulaylik qo'shmang: `Display`, `AsRef<str>`, `Deref`, `into_inner`,
  derive `PartialEq` — bularning yo'qligi bug emas, xavfsizlik qarori.
  Sir taqqoslash `subtle` orqali constant-time bo'ladi.

## 6. Trait va generic

- Generic'ni ehtiyoj bo'lganda kiriting. `sqlx` impl'lari `DB: Database` bo'yicha
  generic — chunki iste'molchi driver'ni o'zi tanlaydi; driverga qotirilgan impl
  yozish foydalanuvchini cheklaydi.
- Sealed trait (`mod sealed { pub trait Sealed {} }`) — to'plamni yopiq saqlash va
  keyin metod qo'shishni breaking qilmaslik uchun.
- Trait ichida `const` va assotsiatsiyalangan tip orqali variantlar orasidagi farqni
  ifodalang (`NumIdRepr::DEBUG_SUFFIX`), `match`-lash uchun runtime flag qo'shmang.
- Blanket impl (`impl<T: Foo> Bar for T`) — coherence'ni qulflaydi va keyin
  ortga qaytarib bo'lmaydi. Ehtiyot bo'ling.
- Makro faqat **haqiqiy** takrorlanish uchun (bu yerda 5+ tip bitta shaklda).
  Ikki joy uchun makro yozmang — o'qilishi qiyinlashadi, xato xabarlari yomonlashadi.

## 7. Panic va xavfsizlik

`#![deny(unsafe_code)]` — `unsafe` yozmang. Panic esa `deny` bilan tutilmaydi, diqqat:

- `split_at(n)`, `&s[..n]`, `s.as_bytes()[i]` — UTF-8 chegarasidan tashqarida **panic**.
  `is_ascii()` tekshiruvini kesishdan **oldin** qo'ying (`passport.rs` da aynan shunday).
- Arifmetika: `a - b` usize'da underflow → debug'da panic. `checked_*`/`saturating_*`
  yoki tartibni o'zgartiring.
- `unwrap` yo'q; `expect` faqat testda.
- Cheklovni chegarada qo'ying: `MAX_TOKEN_LEN` xotira DoS'ini **to'xtatmaydi**
  (`String` allaqachon ajratilgan) — bunday cheklovning haqiqiy o'rnini izohda ayting,
  yolg'on kafolat bermang.
- `tests/props.rs` invarianti: **hech qanday input panic qilmasin**. Yangi public
  `parse` qo'shsangiz uni shu testga qo'shing.

## 8. Feature gating

- `#[cfg(feature = "x")]` bilan birga **doim** `#[cfg_attr(docsrs, doc(cfg(feature = "x")))]` —
  docs.rs da foydalanuvchi qaysi feature kerakligini ko'radi.
- Optional dep'lar orasidagi bog'lanish `Cargo.toml` da: `date = ["dep:chrono", "sqlx?/chrono"]`.
  `?` — "sqlx yoqilgan bo'lsa". Buni unutish "feature powerset" da kompilyatsiya xatosi beradi.
- Har feature qo'shilganda `cargo hack check --feature-powerset --all-targets` (`just features`)
  ishlatilishi kerak: `--all-features` bitta nuqta, teshiklar kombinatsiyalarda chiqadi.
- Integration test faylini butunlay gate qilish: fayl boshida `#![cfg(feature = "serde")]`.

## 9. MSRV 1.85 chegarasi (tekshirilgan)

Edition 2024, lekin `rust-version = "1.85"`. Quyidagilar **ishlamaydi**:

- **let-chains**: `if let Some(x) = a && let Some(y) = b { … }` — 1.88 dan.
  O'rniga ichma-ich `if let` yoki `let ... else`.
- Yangi stabilizatsiyalangan API'lardan foydalanishdan oldin o'ylang; shubha bo'lsa
  `cargo +1.85.0 check --no-default-features` (`just msrv`).

`sqlx` feature'i 1.94+ talab qiladi — `#[cfg(feature = "sqlx")]` ostidagi kodga
1.85 cheklovi tegishli emas.

## 10. Hujjat va test

- `#![warn(missing_docs)]` + CI `-D warnings`: har bir public element, jumladan enum
  varianti va struktura maydoni hujjatlanadi. Hujjat **o'zbekcha**.
- Doc misollari kompilyatsiya bo'ladi va `just check` da ishlaydi. `no_run`/`ignore` —
  faqat haqiqiy sabab bo'lganda (masalan, iste'molchi kodini ko'rsatayotganda).
- Intra-doc link ishlating (rustdoc kvadrat-qavs sintaksisi), tip nomini oddiy matn sifatida
  yozib qo'ymang — buzilgan havolani `just doc-check` tutadi.
- Unit test modul ichida (`#[cfg(test)] mod tests`) — private funksiyalarga tegadi.
  Integration `tests/` da — faqat public API'ni ko'radi, ya'ni API sirtini ham sinaydi.
- Testda **xatoning aniq turini** tekshiring (`assert_eq!(x, Err(E::Length))`),
  `is_err()` emas — aks holda xato tipini almashtirib qo'ysangiz test sezmaydi.
- Trait mavjudligini compile-time'da qulflash uchun bo'sh generic funksiya
  (`fn assert_pg_type<T: Type<Postgres> + …>() {}`) — DB kerak emas.

## 11. Qaytariladigan anti-pattern'lar

| Anti-pattern | Nega yomon | O'rniga |
| --- | --- | --- |
| `pub struct T(pub String)` | invariant chetlab o'tiladi | private maydon + `parse` |
| `parse()` ichida biznes ro'yxati | ro'yxat eskirsa eski yozuvlar o'qilmaydi | `is_*()` / `parse_strict()` |
| `unwrap()` kutubxona kodida | iste'molchi jarayonini o'ldiradi | `Result` |
| `String` argument, faqat o'qiladi | keraksiz allocation | `&str` |
| `clone()` borrow checker'ni tinchitish uchun | yashirin narx | lifetime yoki `Cow` |
| `#[derive(...)]` generic ustida | noto'g'ri `Tag: Clone` bound | qo'lda impl |
| Yangi public enum `#[non_exhaustive]` siz | variant qo'shish breaking | atribut qo'ying |
| `Vec<T>` public konstanta | element qo'shish breaking | `&[T]` slice |
| Sir tipiga `Display` | log'ga sir chiqadi | `expose_secret()` |
| `mod.rs` da hamma narsa | o'qish qiyin | tip = fayl, `lib.rs` faqat re-export |

## Chiqish nazorati

Kod yozib bo'lgach, `just check` dan **oldin** o'zingizni tekshiring:

1. Yangi public element — hujjatlanganmi, `#[must_use]` kerakmi, `#[non_exhaustive]` kerakmi?
2. Yangi xato — `TypeError` ga qo'shildimi?
3. Yangi tip — `prelude.rs`, `tests/props.rs`, `tests/sqlx_bounds.rs` yangilandimi?
4. Yangi `cfg(feature)` — `doc(cfg(...))` juftligi bormi, `Cargo.toml` dagi `?/` bog'lanishi to'g'rimi?
5. Allocation qo'shildimi? Agar ha — ataylabmi, izohda yozildimi?
6. Kesish/indeks bor joyda UTF-8 chegarasi tekshirilganmi?
7. `unwrap`/`expect`/`panic!` qoldimi?
8. MSRV 1.85 da kompilyatsiya bo'ladimi (let-chains ishlatilmadimi)?
9. API o'zgardimi? Unda `just semver-detail` va CHANGELOG migratsiya jadvali.

```bash
just check
```
