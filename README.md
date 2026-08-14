# 🇺🇿 uz-types

Rust loyihalari (ayniqsa O'zbekiston domeniga oid backend tizimlar) uchun qat'iy tiplangan (strongly typed), xavfsiz va qayta ishlatiladigan *Value Object* va *Domain Type*'lar to'plami.

## 🚀 Imkoniyatlar (Features)

- **Qat'iy tiplash (Strongly Typed):** Oddiy `String` o'rniga `Passport`, `Pinfl`, `PhoneNumber` kabi kafolatlangan tiplardan foydalanish.
- **Avtomatik validatsiya (Built-in Validation):** Ma'lumotlar faqat to'g'ri formatda bo'lsagina obyektdan muvaffaqiyatli o'tadi.
- **Minimal allocation:** Validatsiya xotira ajratmasdan bajariladi; noto'g'ri qiymat uchun umuman allocation bo'lmaydi. `String` dan yaratishda mavjud xotira imkon qadar qayta ishlatiladi.
- **Aniq xatolar:** Har bir tipning o'z error enum'i bor va `TypeError` orqali `match` qilinadi.
- **Serde integratsiyasi:** REST API va ma'lumotlar bazasi uchun tayyor serializatsiya va deserializatsiya.
- **Xavfsiz muhit:** `#![deny(unsafe_code)]` orqali xavfsizlik to'liq kafolatlangan.

## 📦 O'rnatish

```toml
[dependencies]
uz-types = "0.17"
```

## 🛠 Foydalanish (Quick Start)

```rust
use uz_types::prelude::*;

fn main() -> Result<(), TypeError> {
    // Pasport — trim va uppercase avtomatik
    let passport = Passport::parse("  aa1234567  ")?;
    println!("Pasport: {passport} (seriya: {})", passport.series());

    // PINFL — faqat format tekshiriladi (checksum emas, pastdagi izohga qarang)
    let pinfl = Pinfl::parse("31234567890123")?;
    println!("PINFL: {pinfl}");

    // Telefon raqami — `+` ixtiyoriy, operator/hudud kodi tekshiriladi
    let phone = PhoneNumber::parse("+998901234567")?;
    println!(
        "Telefon: {} | kod: {} | mobil: {}",
        phone.to_international(),
        phone.operator_code(),
        phone.is_mobile()
    );

    // Tug'ilgan sana — kelajak va 1900 dan oldingi sanalar rad etiladi
    let birth_date = BirthDate::parse("1995-08-31")?;
    println!("Tug'ilgan sana: {birth_date}, yosh: {}", birth_date.age());

    // Email — lowercase qilib saqlanadi
    let email = EmailAddress::parse("Diordev@iCloud.com")?;
    println!("Email: {email} (domain: {})", email.domain());

    Ok(())
}
```

### Xatolarni ajratish

```rust
use uz_types::{Passport, PassportError, TypeError};

match Passport::parse("AA123") {
    Ok(p) => println!("OK: {p}"),
    Err(TypeError::Passport(PassportError::Length)) => println!("uzunlik noto'g'ri"),
    Err(TypeError::Passport(PassportError::Format)) => println!("format noto'g'ri"),
    Err(e) => println!("boshqa xato: {e}"),
}
```

> `TypeError` va uning ichidagi error enum'lari `#[non_exhaustive]` — `match` da har doim `_` yoki `Err(e)` tarmog'ini qoldiring.

### ID tiplari (`JobId`, `SessionId`, `RequestId`, `Reuid`)

```rust
use uz_types::prelude::*;

// UUID generatsiya
let job_id = JobId::generate();        // v4 — tasodifiy
let request_id = RequestId::generate_v7(); // v7 — vaqt bo'yicha tartiblangan (DB uchun afzal)

// Yoki mavjud qiymatni parse qilish (UUID v4/v7 yoki u64)
let session_id = SessionId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1")?;
let numeric_id = JobId::parse("11111")?;
# Ok::<(), uz_types::TypeError>(())
```

### Token tiplari (`AccessToken`, `RefreshToken`, `ClientId`, `ClientSecret`)

```rust
use uz_types::prelude::*;

let access_token = AccessToken::parse(" eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz ")?;
let client_secret = ClientSecret::parse("s3cr3t")?;

// ✅ Loglash uchun — qiymat yashiringan
println!("{access_token:?}");   // AccessToken("***REDACTED***")

// ❌ Sirni logga tushiradi
// println!("{access_token}");
# Ok::<(), uz_types::TypeError>(())
```

## ⚠️ Bilib qo'yish kerak bo'lgan cheklovlar

| Tip | Cheklov |
|---|---|
| `Pinfl` | **Faqat format** (14 ta raqam) tekshiriladi. Checksum, jins/asr belgisi va tug'ilgan sana strukturasi tekshirilmaydi — `00000000000000` ham qabul qilinadi. |
| `AccessToken` va boshqa token tiplari | `Debug` (`{:?}`) qiymatni yashiradi, lekin `Display` (`{}`, `.to_string()`) sirni **to'liq ochadi**. Loglashda `{:?}` ishlating. |
| `JobId` va boshqa ID tiplari | Serde'da bitta tip ikki xil JSON shakl beradi: UUID → `"..."` (string), raqam → `123` (integer). |
| `PhoneNumber` | Operator/hudud kodlari ro'yxati crate ichida qat'iy belgilangan (`MOBILE_CODES` va `60`–`79` oralig'i). Yangi kod ajratilsa, crate yangilanishi kerak. |
| `EmailAddress` | Faqat ASCII. IDN (unicode domenlar), quoted local-part va IP-literal qabul qilinmaydi. |

## 📄 Litsenziya

MIT yoki Apache-2.0 — o'zingizga qulayini tanlang.
