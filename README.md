# 🇺🇿 uz-types

Rust loyihalari (ayniqsa O'zbekiston domeniga oid backend tizimlar) uchun qat'iy tiplangan (strongly typed), xavfsiz va qayta ishlatiladigan *Value Object* va *Domain Type*'lar to'plami.

## 🚀 Imkoniyatlar (Features)

- **Qat'iy tiplash (Strongly Typed):** Oddiy `String` o'rniga `Passport`, `Pinfl`, `PhoneNumber` kabi kafolatlangan tiplardan foydalanish.
- **Avtomatik validatsiya (Built-in Validation):** Ma'lumotlar faqat to'g'ri formatda bo'lsagina obyektdan muvaffaqiyatli o'tadi.
- **Xotira samaradorligi (Zero-Allocation):** Ma'lumotlarni keraksiz klonlashlarsiz, xotirani qayta ishlatgan holda tekshirish va saqlash.
- **Serde integratsiyasi:** REST API va ma'lumotlar bazasi uchun tayyor serializatsiya (Serialize) va deserializatsiya (Deserialize).
- **Xavfsiz muhit:** `#![deny(unsafe_code)]` orqali xavfsizlik to'liq kafolatlangan.

## 📦 O'rnatish

Kutubxonani loyihangizga qo'shish uchun `Cargo.toml` faylingizga quyidagini kiriting:

```toml
[dependencies]
uz-types = "0.15"
```

## 🛠 Foydalanish (Quick Start)
```rust
use uz_types::prelude::*;
fn main() {
    // Pasportni tekshirish va yaratish
    let passport = Passport::parse("AA1234567").unwrap();
    println!("Pasport: {}", passport);

    // PINFL ni tekshirish
    let pinfl = Pinfl::parse("31234567890123").unwrap();
    println!("PINFL: {}", pinfl);

    // PhoneNumber ni tekshirish
    let phone = PhoneNumber::parse("998901234567").unwrap();
    println!("Phone Number: {}", phone);

    // Tug'ilgan sanani tekshirish
    let birth_date = BirthDate::parse("1995-08-31").unwrap();
    println!("Tug'ilgan sana: {}", birth_date);

    // Email tekshiruvi
    let email = EmailAddress::parse("diordev@iclud.com").unwrap();
    println!("Email address: {}", email);
    println!();
    // JobId, SessionId, RequestId va Reuid  UUID generatisya qilish yoki parse qilish.

    let job_id: JobId = JobId::generate();
    let session_id: SessionId = SessionId::generate();
    let request_id: RequestId = RequestId::generate();
    let re_uid: Reuid = Reuid::generate();

    println!(
        "JobId: {},\nSessionId: {},\nRequestId: {},\nReuid: {}",
        job_id, session_id, request_id, re_uid
    );
    println!();

    // JobId, SessionId, RequestId va Reuid  u64 generatisya qilish yoki parse qilish.

    let job_id: JobId = JobId::parse("11111").unwrap();
    let session_id: SessionId = SessionId::parse("22222").unwrap();
    let request_id: RequestId = RequestId::parse("333333").unwrap();
    let re_uid: Reuid = Reuid::parse("444444").unwrap();

    println!(
        "JobId: {},\nSessionId: {},\nRequestId: {},\nReuid: {}",
        job_id, session_id, request_id, re_uid
    );

    // API uchun AccessToken, RefreshToken
    let access_token: AccessToken =
        AccessToken::parse(" eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz ").unwrap();
    let refresh_token: RefreshToken =
        RefreshToken::parse(" eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz ").unwrap();

    println!(
        "AccessToken: {},\nRefreshToken: {}",
        access_token, refresh_token
    );
}


```