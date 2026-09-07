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

    /// Ma'lum mobil kodlar. **Slice** — yangi kod qo'shish breaking emas.
    ///
    /// Boshlang'ich reja: ITU'ning 2023-02-10 dagi
    /// [E.164 yangilanishi](https://www.itu.int/dms_pub/itu-t/opb/sp/T-SP-OB.1263-2023-OAS-PDF-E.pdf).
    /// Keyingi manbalar: [20](https://beeline.uz/uz/phone-codes) (2026-09-07 da
    /// tekshirildi), [70](https://uztelecom.uz/uz/yangiliklar/yangiliklar/uztelecom-yangi-operator-kodi-plus998-70-ni-taqdim-etadi/)
    /// (2025-03-19), [80](https://perfectum.uz/uz/cdma) (2026-09-07 da tekshirildi),
    /// [87](https://company.mobi.uz/uz/press/2026/101857/) (2026-04-16) va
    /// [92](https://beeline.uz/uz/events/news/novyy-kod-beeline-uzbekistan_92)
    /// (7-noyabr; 2026-09-07 da tekshirildi).
    ///
    /// `70` kodi mobil va geografik xizmatlarda ishlatiladi, shuning uchun tasniflar
    /// o'zaro istisno emas. Registry vaqt o'tishi bilan eskirishi mumkin;
    /// [`parse`](Self::parse) undan foydalanmaydi.
    pub const MOBILE_CODES: &[&str] = &[
        "20", "33", "50", "70", "77", "80", "87", "88", "90", "91", "92", "93", "94", "95", "97",
        "98", "99",
    ];

    /// Geografik PSTN kodlari.
    ///
    /// Manba: ITU'ning 2023-02-10 dagi
    /// [O'zbekiston E.164 rejasi](https://www.itu.int/dms_pub/itu-t/opb/sp/T-SP-OB.1263-2023-OAS-PDF-E.pdf).
    /// `70` keyinchalik mobil xizmatga ham ajratilgan; shu sabab u ikki to'plamda bor.
    pub const GEOGRAPHIC_CODES: &[&str] = &[
        "61", "62", "65", "66", "67", "69", "70", "71", "72", "73", "74", "75", "76", "79",
    ];

    /// SIP xizmat kodi.
    ///
    /// Manba: ITU'ning 2023-02-10 dagi
    /// [O'zbekiston E.164 rejasi](https://www.itu.int/dms_pub/itu-t/opb/sp/T-SP-OB.1263-2023-OAS-PDF-E.pdf).
    pub const SIP_CODES: &[&str] = &["55"];

    /// Geografik bo'lmagan statsionar tarmoq xizmat kodi.
    ///
    /// Manba: ITU'ning 2023-02-10 dagi
    /// [O'zbekiston E.164 rejasi](https://www.itu.int/dms_pub/itu-t/opb/sp/T-SP-OB.1263-2023-OAS-PDF-E.pdf).
    pub const NON_GEOGRAPHIC_FIXED_CODES: &[&str] = &["78"];

    /// Eski shahar/hudud oralig'i; aniq tasnif uchun ishlatilmaydi.
    #[deprecated(note = "aniq hudud kodlari uchun PhoneNumber::GEOGRAPHIC_CODES dan foydalaning")]
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

    /// Struktura + mobil/geografik/SIP/statsionar kod exact ro'yxatda bo'lishi shart.
    ///
    /// Ro'yxat crate bilan birga keladi va eskirishi mumkin. DB/Kafka/Serde replay
    /// uchun [`parse`](Self::parse) ni, o'z registringiz bo'lsa
    /// [`operator_code`](Self::operator_code) ni ishlating.
    pub fn parse_strict(value: &str) -> Result<Self, PhoneNumberError> {
        let phone = Self::parse(value)?;
        if !phone.is_known_operator() {
            return Err(PhoneNumberError::UnknownOperatorCode);
        }
        Ok(phone)
    }

    /// Kod (`90`, `71`) — xom, ro'yxatga qaramasdan.
    ///
    /// Bu crate registridan chiqish yo'li: kod ajratmalari vaqt bilan o'zgaradi va
    /// crate snapshot'i eskirishi mumkin. Joriy ro'yxat sizga config yoki DB'dan
    /// kelsa, `is_*()` metodlarini emas, shu accessor'ni ishlating — `parse()`
    /// strukturasi barqaror qoladi, siyosat esa sizniki bo'ladi:
    ///
    /// ```
    /// use uz_types::PhoneNumber;
    ///
    /// // Ro'yxat sizniki: config, DB yoki remote'dan keladi.
    /// let allowed = ["90", "91", "99"];
    ///
    /// let phone = PhoneNumber::parse("998911234567").unwrap();
    /// assert!(allowed.contains(&phone.operator_code()));
    ///
    /// // Crate registri hali bilmaydigan yangi kod ham shu yo'l bilan o'tadi.
    /// let fresh = PhoneNumber::parse("998001234567").unwrap();
    /// assert!(!fresh.is_known_operator());
    /// assert_eq!(fresh.operator_code(), "00");
    /// ```
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

    /// Kod crate'dagi aniq geografik PSTN ro'yxatida bormi.
    #[inline]
    #[must_use]
    pub fn is_geographic(&self) -> bool {
        Self::GEOGRAPHIC_CODES.contains(&self.operator_code())
    }

    /// Kod crate'dagi SIP ro'yxatida bormi.
    #[inline]
    #[must_use]
    pub fn is_sip(&self) -> bool {
        Self::SIP_CODES.contains(&self.operator_code())
    }

    /// Kod crate'dagi geografik bo'lmagan statsionar ro'yxatda bormi.
    #[inline]
    #[must_use]
    pub fn is_non_geographic_fixed(&self) -> bool {
        Self::NON_GEOGRAPHIC_FIXED_CODES.contains(&self.operator_code())
    }

    /// Kod crate'dagi aniq mobil, geografik, SIP yoki statsionar ro'yxatda bormi.
    ///
    /// Bu **crate snapshot'iga** nisbatan javob, mutlaq haqiqat emas. Yangi kod
    /// ajratilsa, crate yangilanmaguncha bu metod `false` qaytaradi va
    /// [`parse_strict`](Self::parse_strict) rad etadi — [`parse`](Self::parse) esa
    /// ta'sirlanmaydi. Registrni o'zingiz boshqarmoqchi bo'lsangiz
    /// [`operator_code`](Self::operator_code) misoliga qarang.
    #[must_use]
    pub fn is_known_operator(&self) -> bool {
        self.is_mobile() || self.is_geographic() || self.is_sip() || self.is_non_geographic_fixed()
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

    fn with_code(code: &str) -> String {
        format!("998{code}1234567")
    }

    #[test]
    fn exact_code_sets_are_classified() {
        assert_eq!(
            PhoneNumber::MOBILE_CODES,
            &[
                "20", "33", "50", "70", "77", "80", "87", "88", "90", "91", "92", "93", "94", "95",
                "97", "98", "99"
            ]
        );
        assert_eq!(
            PhoneNumber::GEOGRAPHIC_CODES,
            &[
                "61", "62", "65", "66", "67", "69", "70", "71", "72", "73", "74", "75", "76", "79"
            ]
        );
        assert_eq!(PhoneNumber::SIP_CODES, &["55"]);
        assert_eq!(PhoneNumber::NON_GEOGRAPHIC_FIXED_CODES, &["78"]);

        let overlap = PhoneNumber::parse(&with_code("70")).unwrap();
        assert!(overlap.is_mobile());
        assert!(overlap.is_geographic());

        for code in ["80", "87", "92"] {
            let value = with_code(code);
            let phone = PhoneNumber::parse_strict(&value).unwrap();
            assert!(phone.is_mobile());
        }

        let sip = PhoneNumber::parse(&with_code("55")).unwrap();
        assert!(sip.is_known_operator());
        assert!(sip.is_sip());
        assert!(!sip.is_mobile());

        let fixed = PhoneNumber::parse(&with_code("78")).unwrap();
        assert!(fixed.is_known_operator());
        assert!(fixed.is_non_geographic_fixed());

        let geographic = PhoneNumber::parse(&with_code("71")).unwrap();
        assert!(geographic.is_geographic());
        assert!(!geographic.is_mobile());
    }

    #[test]
    fn unassigned_codes_are_only_structurally_valid() {
        for code in ["00", "60", "63", "64", "68"] {
            let value = with_code(code);
            let phone = PhoneNumber::parse(&value).unwrap();
            assert!(!phone.is_known_operator(), "{code}");
            assert_eq!(
                PhoneNumber::parse_strict(&value),
                Err(PhoneNumberError::UnknownOperatorCode),
                "{code}"
            );
        }
    }
}
