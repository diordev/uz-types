use serde::{Deserialize, Serialize};

use crate::error::{TypeError, TypeResult};

/// O'zbekiston pasport seriyasi va raqami.
///
/// # Format
///
/// - Aynan **9 ta** belgi: **2 katta lotin harfi** + **7 raqam**
/// - Ichki saqlashda seriya uppercase ga normalizatsiya qilinadi
///
// # Misollar
//
// ```
// use myid::types::Passport;
//
// let passport = Passport::parse("AA1234567").unwrap();
//
// assert_eq!(passport.as_str(), "AA1234567");
// assert_eq!(passport.series(), "AA");
// assert_eq!(passport.number(), "1234567");
//
// // Lowercase avtomatik uppercase bo'ladi.
// let passport = Passport::parse("ab1234567").unwrap();
// assert_eq!(passport.as_str(), "AB1234567");
// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct Passport(String);

impl Passport {
    /// Pasport seriyasi uzunligi.
    pub const SERIES_LEN: usize = 2;

    /// Pasport raqami uzunligi.
    pub const NUMBER_LEN: usize = 7;

    /// Pasportning umumiy uzunligi.
    pub const LEN: usize = Self::SERIES_LEN + Self::NUMBER_LEN;

    /// String qiymatdan `Passport` yaratadi.
    ///
    /// Kiruvchi qiymat:
    /// - trim qilinadi;
    /// - seriya uppercase ga normalizatsiya qilinadi;
    /// - format tekshiriladi.
    pub fn parse(value: impl AsRef<str>) -> TypeResult<Self> {
        let raw = value.as_ref().trim();

        if raw.len() != Self::LEN {
            return Err(TypeError::validation(format!(
                "passport must contain exactly {} characters, got {}: {raw}",
                Self::LEN,
                raw.len(),
            )));
        }

        let (series, number) = raw.split_at(Self::SERIES_LEN);

        if !series.bytes().all(|b| b.is_ascii_alphabetic()) {
            return Err(TypeError::validation(format!(
                "passport series must contain exactly 2 ASCII letters: {raw}",
            )));
        }

        if !number.bytes().all(|b| b.is_ascii_digit())  {
            return Err(TypeError::validation(format!(
                "passport number must contain exactly 7 digits: {raw}",
            )));
        }

        Ok(Self(format!(
            "{}{}",
            series.to_ascii_uppercase(),
            number
        )))
    }

    /// To'liq pasport qiymati.
    #[inline]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Pasport seriyasi.
    #[inline]
    pub fn series(&self) -> &str {
        &self.0[..Self::SERIES_LEN]
    }

    /// Pasport raqami.
    #[inline]
    pub fn number(&self) -> &str {
        &self.0[Self::SERIES_LEN..]
    }

    /// Ichki `String` qiymatni qaytaradi.
    #[inline]
    pub fn into_inner(self) -> String {
        self.0
    }
}

// Passport ichidagi String qiymatni &str ko'rinishida olish imkonini beradi.
// AsRef traiti orqali Passport type'i String yoki &str kabi
// generic funksiyalar bilan ishlay oladi.
// Bu hech qanday nusxa olmaydi (clone qilmaydi), faqat reference qaytaradi.
impl AsRef<str> for Passport {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

// Passport ichidagi String qiymatni `&str` ko'rinishida olish imkonini beradi.
// `Borrow` traiti asosan kolleksiya turlari (`HashMap`, `HashSet`, `BTreeMap`)
// bilan ishlash uchun kerak bo'ladi.
// Bu ham nusxa olmaydi, faqat ichki qiymatga reference qaytaradi.
impl std::borrow::Borrow<str> for Passport {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

// Passport qiymatini oddiy matn ko'rinishida chiqarish uchun.
// println!("{}", passport) kabi formatlarda ishlaydi.
// Ichki String qiymatini qo'shimcha nusxalamasdan qaytaradi.
impl std::fmt::Display for Passport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

// &str qiymatdan Passport yaratish uchun ishlatiladi.
// Qiymat validatsiyadan o'tmasa TypeError qaytaradi.
impl TryFrom<&str> for Passport {
    type Error = TypeError;

    fn try_from(value: &str) -> TypeResult<Self> {
        Self::parse(value)
    }
}

// Egasi bilan berilgan String qiymatdan Passport yaratadi.
// Ichki &str ko'rinishiga o'tkazib, mavjud TryFrom logikasidan foydalanadi.
impl TryFrom<String> for Passport {
    type Error = TypeError;

    fn try_from(value: String) -> TypeResult<Self> {
        Self::try_from(value.as_str())
    }
}

// Passport ichidagi String qiymatni tashqariga chiqarish uchun ishlatiladi.
// Egalik (ownership) Passport'dan String'ga o'tadi.
// Qo'shimcha nusxa olmaydi, ichki qiymatni qaytaradi.
impl From<Passport> for String {
    fn from(value: Passport) -> Self {
        value.into_inner()
    }
}