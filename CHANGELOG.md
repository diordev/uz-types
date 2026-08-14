# O'zgarishlar tarixi

Format [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) asosida,
versiyalash [SemVer](https://semver.org/lang/uz/) bo'yicha.

## [0.17.0] — 2026-08-14.

Bu relizning asosiy maqsadi — **public API'ni tashqi foydalanuvchi uchun
ishlaydigan holga keltirish** va validatsiyadagi buglarni yopish.

### Qo'shildi

- **Crate ildizidan to'liq eksport.** Endi `use uz_types::Passport;` ishlaydi.
  Ilgari barcha modullar `private` bo'lgani uchun tiplarga faqat
  `uz_types::prelude::*` orqali kirish mumkin edi.
- **Barcha error tiplari eksport qilindi**: `PassportError`, `PinflError`,
  `PhoneNumberError`, `EmailAddressError`, `BirthDateError`, `IdError`,
  `TokenError`. Ilgari ular hech qayerdan chaqirib bo'lmas, ya'ni `TypeError`
  ichidagi aniq xatoni `match` qilish imkonsiz edi.
- **`ClientId` va `ClientSecret` eksport qilindi.** Ular avval yaratilgan,
  lekin `prelude` ga qo'shilmagan — foydalanuvchiga umuman ko'rinmas edi.
- `tests/api.rs` — public API'ni tashqaridan tekshiruvchi integration testlar.
- `PhoneNumber`: `operator_code()`, `subscriber_number()`, `is_mobile()`,
  `is_known_operator_code()`, `MOBILE_CODES`, `REGIONAL_CODE_RANGE`,
  `OPERATOR_CODE_LEN`.
- `BirthDate`: `age()`, `age_at()`, `from_naive_date_with_today()`, `MIN_YEAR`.
- ID tiplari: `generate_v7()` (vaqt bo'yicha tartiblangan UUID) va
  `uuid_version()`. `uuid` dependency'siga `v7` feature qo'shildi.
- Token tiplari: `MAX_TOKEN_LEN` (8 KiB) va `<Type>::MAX_LEN`.
- `EmailAddress`: `LOCAL_PART_MAX_LEN`, `DOMAIN_MAX_LEN`,
  `DOMAIN_LABEL_MAX_LEN`, `TLD_MIN_LEN` konstantalari.
- **`FromStr`** endi `Passport`, `Pinfl`, `PhoneNumber`, `EmailAddress` va
  `BirthDate` uchun ham implementatsiya qilingan — `"AA1234567".parse::<Passport>()`
  ishlaydi. Ilgari u faqat token va ID tiplarida bor edi.
- `.github/workflows/ci.yml` — fmt, clippy, test, rustdoc, MSRV va
  `cargo package` tekshiruvlari.
- Ushbu `CHANGELOG.md`.

### O'zgardi (validatisya jarayoni murakkablashtirildi)

> Quyidagi input ma'lumotlar ilgari **noto'g'ri qabul qilinardi**, endi rad etiladi.
> Agar ma'lumotlar bazangizda shunday qiymatlar bo'lsa, migratsiyadan oldin
> tekshirib chiqing.

- **`EmailAddress`** — quyidagilar endi rad etiladi:
  - bo'sh domain label: `a@b..c`, `a@.b.com`, `a@b.com.`;
  - `-` bilan boshlanuvchi/tugovchi label: `a@-b.com`, `a@b-.com`;
  - local-part'da ketma-ket nuqta: `a..b@c.com`;
  - 1 harfli yoki raqamli TLD: `a@b.c`, `a@b.c1`;
  - 64 belgidan uzun local-part;
  - local-part'dagi ruxsatsiz belgilar (`(`, `,`, `:`, `\` va h.k.);
  - bir nechta `@`: `a@b@c.com`.
- **`PhoneNumber`** — `998` dan keyingi 2 raqam ma'lum operator yoki hudud
  kodi bo'lishi shart. `998000000000` kabi soxta raqamlar endi
  `PhoneNumberError::OperatorCode` xatosini beradi.
- **`BirthDate`** — `MIN_YEAR` (1900) dan oldingi sanalar
  `BirthDateError::TooOld` bilan rad etiladi (`0001-01-01` avval valid edi).
- **`BirthDate`** — kelajak chegarasi endi UTC bo'yicha *ertangi* kun.
  Ilgari UTC bilan solishtirilgani uchun UTC+5 (Toshkent) da bugun tug'ilgan
  chaqaloq xato `FutureDate` olishi mumkin edi.
- **Token tiplari** — 8 KiB dan uzun qiymat `TokenError::TooLong` beradi
  (deserializatsiya orqali cheksiz xotira sarfining oldini oladi).

### Optimallashtirildi

- `EmailAddress::validate` endi `Vec<&str>` yig'maydi (`split_once` ishlatiladi)
  — validatsiya butunlay allocationsiz.
- `EmailAddress::parse` avval validatsiya qiladi, keyin lowercase qiladi —
  noto'g'ri kirish uchun umuman xotira ajratilmaydi.

### Eskirdi (deprecated)

- `TypeError::validation()` — crate ichida ishlatilmaydi, 1.0 da
  `TypeError::Validation` varianti bilan birga olib tashlanadi.

### Hujjatlashtirildi

- `Pinfl` **faqat formatni** tekshirishi ochiq yozildi — checksum, jins/asr
  belgisi va tug'ilgan sana strukturasi tekshirilmaydi.
- `Passport` ham xuddi shunday **faqat formatni** tekshirishi yozildi —
  seriya amaldagi ro'yxatga kirishi tekshirilmaydi (`ZZ0000000` ham valid).
- Har bir tipning `parse()` metodiga `# Xatolar` bo'limi qo'shildi.
- Token tiplarida `Display` (`{}`, `.to_string()`) tokenni to'liq ochishi
  haqida ogohlantirish qo'shildi (`Debug` esa yashiradi).
- ID tiplari serde'da ikki xil JSON shakl berishi (UUID → string,
  raqam → integer) va uning OpenAPI/DB uchun oqibatlari yozildi.
- README'dagi "Zero-Allocation" da'vosi "Minimal allocation" ga
  aniqlashtirildi; cheklovlar jadvali qo'shildi.

### Tuzatildi

- README va example'dagi `iclud.com` typo'si.
- `pinfl.rs` dagi "Pinfl uzunligi — aynan 14 Pinfl" jumlasi.
- `phone_number.rs` dagi "bo'lsahma" typo'si.
- `Cargo.toml` `include` ro'yxatiga `tests/**/*.rs` qo'shildi.
- Beshta fayldagi `Deserialize` impl ustidan o'lik `#[allow(unknown_lints)]`
  atributi olib tashlandi — u hech qanday lint'ni bostirmasdi.
- Fayllar bo'ylab izchillik: `// XATOLIKLAR ENUMI` banneri va
  "Xotira ajratmaydi" iborasi hamma joyda bir xil qilindi.

### Test qamrovi

- `id_type_tests!` endi `RequestId` va `Reuid` uchun ham ishlaydi
  (avval faqat `JobId` va `SessionId`).
- UUID v3/v5/v7 uchun testlar qo'shildi.
- Testlar soni: 67 → 122 unit + 17 integration + 3 doctest.

---

## Rejalashtirilgan breaking o'zgarishlar (1.0)

- Token tiplarida `Display` ham `***REDACTED***` qaytaradi; haqiqiy qiymat
  faqat `expose_secret()` orqali olinadi (`secrecy` uslubi).
- `TypeError::PINFL` varianti Rust konvensiyasiga moslab `Pinfl` deb
  qayta nomlanadi.
- `Reuid` aniqroq nom bilan almashtiriladi.
- `serde`, `chrono` va `uuid` optional feature'larga chiqariladi — faqat
  kerakli tiplarni yuklash imkoni bo'ladi.
- `Deref` implementatsiyalari olib tashlanadi (Rust API Guidelines newtype
  uchun deref polymorphism'ni tavsiya etmaydi); `as_str()`/`AsRef` qoladi.
- `TypeError::Validation` va `TypeError::validation()` olib tashlanadi.

## [0.16.0] va undan oldingi versiyalar

Bu versiyalar uchun o'zgarishlar hujjatlashtirilmagan — git tarixiga qarang.
