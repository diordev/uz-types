use crate::macros::string_newtype;

string_newtype! {
    /// O'zbekiston telefon raqami — ichkarida har doim `998` + 9 raqam (12 raqam, `+` siz).
    ///
    /// `parse()` **strukturani** tekshiradi va formatga toqatli:
    /// `+998 (90) 123-45-67`, `998.90.123.45.67`, `+998901234567` — hammasi `998901234567`.
    /// Tashlab yuboriladigan ajratuvchilar: bo'shliq, `-`, `(`, `)`, `.` va boshidagi `+`.
    /// Operator/hudud kodi ro'yxati — o'zgaruvchan biznes fakti — `parse` ichida emas:
    /// [`PhoneNumber::is_known_operator`], [`PhoneNumber::is_mobile`] yoki [`PhoneNumber::parse_strict`].
    pub struct PhoneNumber;
    error = PhoneNumberError;
    expecting = "an Uzbek phone number: 998 followed by 9 digits";
}

impl PhoneNumber {
    /// `+` siz raqamlar soni.
    pub const DIGIT_LEN: usize = 12;
    /// Davlat kodi.
    pub const COUNTRY_CODE: &str = "998";
    /// Operator/hudud kodi uzunligi.
    pub const OPERATOR_CODE_LEN: usize = 2;

    /// Ma'lum mobil operator kodlari. **Slice** — yangi kod qo'shish breaking emas.
    pub const MOBILE_CODES: &[&str] = &[
        "20", "33", "50", "55", "77", "88", "90", "91", "93", "94", "95", "97", "98", "99",
    ];

    /// Shahar/hudud (statsionar) kodlari oralig'i.
    pub const REGIONAL_CODES: core::ops::RangeInclusive<u8> = 60..=79;

    fn normalize(s: &mut String) {
        if s.starts_with('+') {
            s.remove(0);
        }
        // Ajratuvchilar faqat kerak bo'lganda tozalanadi (tez yo'l: toza 12 raqam).
        if !s.bytes().all(|b| b.is_ascii_digit()) {
            s.retain(|c| !(c.is_ascii_whitespace() || matches!(c, '-' | '(' | ')' | '.')));
        }
    }

    fn validate(s: &str) -> Result<(), PhoneNumberError> {
        if !s.bytes().all(|b| b.is_ascii_digit()) {
            return Err(PhoneNumberError::Format);
        }
        if s.len() != Self::DIGIT_LEN {
            return Err(PhoneNumberError::Length);
        }
        if !s.starts_with(Self::COUNTRY_CODE) {
            return Err(PhoneNumberError::Prefix);
        }
        Ok(())
    }

    /// Struktura + operator/hudud kodi ro'yxatda bo'lishi shart.
    pub fn parse_strict(value: &str) -> Result<Self, PhoneNumberError> {
        let phone = Self::parse(value)?;
        if !phone.is_known_operator() {
            return Err(PhoneNumberError::UnknownOperatorCode);
        }
        Ok(phone)
    }

    /// Kod (`90`, `71`) — xom, ro'yxatga qaramasdan.
    #[inline]
    #[must_use]
    pub fn operator_code(&self) -> &str {
        let start = Self::COUNTRY_CODE.len();
        &self.0[start..start + Self::OPERATOR_CODE_LEN]
    }

    /// Abonent raqami (`1234567`).
    #[inline]
    #[must_use]
    pub fn subscriber_number(&self) -> &str {
        &self.0[Self::COUNTRY_CODE.len() + Self::OPERATOR_CODE_LEN..]
    }

    /// Kod crate'dagi mobil ro'yxatda bormi.
    #[inline]
    #[must_use]
    pub fn is_mobile(&self) -> bool {
        Self::MOBILE_CODES.contains(&self.operator_code())
    }

    /// Kod mobil ro'yxatda yoki hudud oralig'ida bormi (registry-qatlam tekshiruvi).
    #[must_use]
    pub fn is_known_operator(&self) -> bool {
        self.is_mobile()
            || self
                .operator_code()
                .parse::<u8>()
                .is_ok_and(|n| Self::REGIONAL_CODES.contains(&n))
    }

    /// `+998901234567` (yangi `String`).
    #[must_use]
    pub fn to_international(&self) -> String {
        format!("+{}", self.0)
    }
}

/// `PhoneNumber` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum PhoneNumberError {
    /// Raqamlar soni 12 emas.
    #[error("phone number length is invalid, expected 12 digits")]
    Length,
    /// Raqamdan boshqa belgi.
    #[error("phone number format is invalid")]
    Format,
    /// `998` bilan boshlanmaydi.
    #[error("phone number must start with 998")]
    Prefix,
    /// Kod ro'yxatda yo'q (faqat `parse_strict`).
    #[error("phone number has an unknown operator or region code")]
    UnknownOperatorCode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn separators_are_tolerated() {
        for input in [
            "+998 (90) 123-45-67",
            "998 90 123 45 67",
            "+998901234567",
            "998901234567",
            " +998-90-123-45-67 ",
            "998.90.123.45.67",
            "+998 (90) 123.45.67",
        ] {
            assert_eq!(
                PhoneNumber::parse(input).unwrap().as_str(),
                "998901234567",
                "{input}"
            );
        }
    }

    #[test]
    fn structure_vs_registry() {
        let fake = PhoneNumber::parse("998000000000").unwrap(); // struktura o'tadi
        assert!(!fake.is_known_operator());
        assert_eq!(
            PhoneNumber::parse_strict("998000000000"),
            Err(PhoneNumberError::UnknownOperatorCode)
        );
        assert!(
            PhoneNumber::parse("998711234567")
                .unwrap()
                .is_known_operator()
        );
        assert_eq!(
            PhoneNumber::parse("997901234567"),
            Err(PhoneNumberError::Prefix)
        );
        assert_eq!(
            PhoneNumber::parse("99890123456"),
            Err(PhoneNumberError::Length)
        );
        assert_eq!(
            PhoneNumber::parse("998a01234567"),
            Err(PhoneNumberError::Format)
        );
        assert_eq!(
            PhoneNumber::parse("998+901234567"),
            Err(PhoneNumberError::Format)
        );
    }
}
