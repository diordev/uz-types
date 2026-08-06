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
uz-types = "0.4"
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
}
```