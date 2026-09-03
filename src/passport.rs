use crate::macros::string_newtype;

string_newtype! {
    /// O'zbekiston pasport / ID-karta seriyasi va raqami (`AA1234567`).
    ///
    /// Normalizatsiya: trim, ichki bo'sh joylar olib tashlanadi (`AA 1234567`),
    /// harflar KATTA qilinadi. Faqat **format** tekshiriladi.
    pub struct Passport;
    error = PassportError;
    expecting = "a passport number: 2 latin letters followed by 7 digits";
}

impl Passport {
    /// Seriya uzunligi (harflar soni).
    pub const SERIES_LEN: usize = 2;
    /// Raqam uzunligi.
    pub const NUMBER_LEN: usize = 7;
    /// Umumiy uzunlik.
    pub const LEN: usize = Self::SERIES_LEN + Self::NUMBER_LEN;

    fn normalize(s: &mut String) {
        // `retain` char-by-char yuradi — faqat kerak bo'lganda (ichki bo'sh joy bor bo'lsa).
        if s.bytes().any(|b| b.is_ascii_whitespace()) {
            s.retain(|c| !c.is_ascii_whitespace());
        }
        s.make_ascii_uppercase();
    }

    fn validate(s: &str) -> Result<(), PassportError> {
        if s.len() != Self::LEN {
            return Err(PassportError::Length);
        }
        // `is_ascii` split_at dan OLDIN: ko'p baytli belgi chegarasida panic bo'lmasligi uchun.
        if !s.is_ascii() {
            return Err(PassportError::Format);
        }
        let (series, number) = s.split_at(Self::SERIES_LEN);
        if !series.bytes().all(|b| b.is_ascii_uppercase())
            || !number.bytes().all(|b| b.is_ascii_digit())
        {
            return Err(PassportError::Format);
        }
        Ok(())
    }

    /// Seriya (`AA`).
    #[inline]
    #[must_use]
    pub fn series(&self) -> &str {
        &self.0[..Self::SERIES_LEN]
    }

    /// Raqam (`1234567`).
    #[inline]
    #[must_use]
    pub fn number(&self) -> &str {
        &self.0[Self::SERIES_LEN..]
    }
}

/// `Passport` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum PassportError {
    /// Uzunlik 9 belgi emas.
    #[error("passport length is invalid, expected {} characters", Passport::LEN)]
    Length,
    /// 2 lotin harfi + 7 raqam emas.
    #[error("passport format is invalid, expected 2 letters followed by 7 digits")]
    Format,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_normalizes() {
        assert_eq!(
            Passport::parse("  aa 1234567 ").unwrap().as_str(),
            "AA1234567"
        );
        assert_eq!(Passport::parse("ab1234567").unwrap().series(), "AB");
    }

    #[test]
    fn try_from_string_reuses_buffer() {
        let s = String::from("  aa1234567 ");
        let cap = s.capacity();
        let p = Passport::try_from(s).unwrap();
        assert_eq!(p.as_str(), "AA1234567");
        assert_eq!(p.into_inner().capacity(), cap); // allocation yo'q
    }

    #[test]
    fn errors_are_precise() {
        assert_eq!(Passport::parse("AA123"), Err(PassportError::Length));
        assert_eq!(Passport::parse("A11234567"), Err(PassportError::Format));
        assert_eq!(
            Passport::parse("a\u{00C4}234567"),
            Err(PassportError::Format)
        );
    }

    #[test]
    fn borrow_str_lookup_works() {
        let mut map = std::collections::HashMap::new();
        map.insert(Passport::parse("AA1234567").unwrap(), 1);
        assert_eq!(map.get("AA1234567"), Some(&1));
    }
}
