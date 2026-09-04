use crate::macros::string_newtype;

string_newtype! {
    /// JShShIR / PINFL — 14 raqamli shaxsiy identifikatsiya raqami.
    ///
    /// `parse()` faqat **strukturani** (14 ta ASCII raqam) tekshiradi.
    /// Nazorat raqami, jins/asr belgisi va tug'ilgan sana — query metodlar
    /// (`is_checksum_valid`, `gender`, `birth_date`) yoki [`Pinfl::parse_strict`].
    pub struct Pinfl;
    error = PinflError;
    expecting = "a PINFL: exactly 14 digits";
}

/// PINFL 1-raqamidan olinadigan jins.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Gender {
    /// Erkak (1-raqam toq: 1, 3, 5).
    Male,
    /// Ayol (1-raqam juft: 2, 4, 6).
    Female,
}

impl Pinfl {
    /// Uzunlik — aynan 14 raqam.
    pub const LEN: usize = 14;

    /// Nazorat raqami vaznlari (VM qarori №177, 12.04.2022: "7 3 1" takrorlanadi, modul 10).
    const WEIGHTS: [u32; 3] = [7, 3, 1];

    fn normalize(_: &mut str) {}

    fn validate(s: &str) -> Result<(), PinflError> {
        if s.len() != Self::LEN {
            return Err(PinflError::Length);
        }
        if !s.bytes().all(|b| b.is_ascii_digit()) {
            return Err(PinflError::Format);
        }
        Ok(())
    }

    /// Struktura + nazorat raqami + jins/asr belgisi + tug'ilgan sana — hammasi tekshiriladi.
    pub fn parse_strict(value: &str) -> Result<Self, PinflError> {
        let pinfl = Self::parse(value)?;
        if !pinfl.is_checksum_valid() {
            return Err(PinflError::Checksum);
        }
        if pinfl.gender().is_none() || pinfl.birth_date_parts().is_none() {
            return Err(PinflError::Structure);
        }
        Ok(pinfl)
    }

    #[inline]
    fn digit(&self, index: usize) -> u32 {
        u32::from(self.0.as_bytes()[index] - b'0')
    }

    /// 14-raqam rasmiy algoritm bo'yicha to'g'rimi (7‑3‑1 vaznlar, mod 10).
    #[must_use]
    pub fn is_checksum_valid(&self) -> bool {
        let sum: u32 = (0..Self::LEN - 1)
            .map(|i| self.digit(i) * Self::WEIGHTS[i % 3])
            .sum();
        sum % 10 == self.digit(Self::LEN - 1)
    }

    /// 1-raqam: jins (1,3,5 — erkak; 2,4,6 — ayol). Boshqa qiymat → `None`.
    #[must_use]
    pub fn gender(&self) -> Option<Gender> {
        match self.digit(0) {
            1 | 3 | 5 => Some(Gender::Male),
            2 | 4 | 6 => Some(Gender::Female),
            _ => None,
        }
    }

    /// 1-raqam: asr (1,2 → 1800; 3,4 → 1900; 5,6 → 2000).
    #[must_use]
    pub fn century(&self) -> Option<i32> {
        match self.digit(0) {
            1 | 2 => Some(1800),
            3 | 4 => Some(1900),
            5 | 6 => Some(2000),
            _ => None,
        }
    }

    /// 2–7 raqamlar `DDMMYY` + asr → `(yil, oy, kun)`. Oddiy diapazon tekshiruvi
    /// (kun 1..=31, oy 1..=12); kalendar to'g'riligini `birth_date()` tekshiradi.
    #[must_use]
    pub fn birth_date_parts(&self) -> Option<(i32, u32, u32)> {
        let century = self.century()?;
        let day = self.digit(1) * 10 + self.digit(2);
        let month = self.digit(3) * 10 + self.digit(4);
        let year = century + i32::try_from(self.digit(5) * 10 + self.digit(6)).ok()?;
        ((1..=31).contains(&day) && (1..=12).contains(&month)).then_some((year, month, day))
    }

    /// 8–10 raqamlar: tug'ilgan hudud kodi.
    #[inline]
    #[must_use]
    pub fn region_code(&self) -> &str {
        &self.0[7..10]
    }

    /// 11–13 raqamlar: tartib raqami.
    #[inline]
    #[must_use]
    pub fn serial(&self) -> &str {
        &self.0[10..13]
    }

    /// PINFL ichidagi tug'ilgan sana (kalendar bo'yicha haqiqiy bo'lsa).
    ///
    /// **Tizim soatiga (UTC) tayanadi** — `BirthDate` kelajak sanasini rad etadi,
    /// shuning uchun 2000-yillar PINFL'i (`5`/`6` + `YY` kelajakda) bugungi kunga
    /// qarab `Some`/`None` bo'ladi. Testlarda [`birth_date_at`](Self::birth_date_at).
    #[cfg(feature = "date")]
    #[must_use]
    pub fn birth_date(&self) -> Option<crate::BirthDate> {
        let (y, m, d) = self.birth_date_parts()?;
        let date = chrono::NaiveDate::from_ymd_opt(y, m, d)?;
        crate::BirthDate::from_naive_date(date).ok()
    }

    /// [`birth_date`](Self::birth_date) ning deterministik varianti — "bugun" tashqaridan.
    #[cfg(feature = "date")]
    #[must_use]
    pub fn birth_date_at(&self, today: chrono::NaiveDate) -> Option<crate::BirthDate> {
        let (y, m, d) = self.birth_date_parts()?;
        let date = chrono::NaiveDate::from_ymd_opt(y, m, d)?;
        crate::BirthDate::from_naive_date_at(date, today).ok()
    }
}

/// `Pinfl` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum PinflError {
    /// 14 ta raqam emas.
    #[error("pinfl length is invalid, expected 14 digits")]
    Length,
    /// Raqamdan boshqa belgi bor.
    #[error("pinfl format is invalid, expected only digits")]
    Format,
    /// Nazorat raqami mos kelmaydi (faqat `parse_strict`).
    #[error("pinfl checksum is invalid")]
    Checksum,
    /// Jins/asr belgisi yoki tug'ilgan sana strukturasi noto'g'ri (faqat `parse_strict`).
    #[error("pinfl structure is invalid (gender/century digit or birth date)")]
    Structure,
}

#[cfg(test)]
mod tests {
    use super::*;

    // Rasmiy hujjatlardagi misollar: VM qarori №200 (1996) va №177 (2022).
    const OFFICIAL_1996: &str = "31210632040244";
    const OFFICIAL_2022: &str = "31210932040247";

    #[test]
    fn official_examples_pass_strict() {
        for s in [OFFICIAL_1996, OFFICIAL_2022] {
            let p = Pinfl::parse_strict(s).unwrap();
            assert!(p.is_checksum_valid());
            assert_eq!(p.gender(), Some(Gender::Male));
            assert_eq!(p.region_code(), "204");
            assert_eq!(p.serial(), "024");
        }
        assert_eq!(
            Pinfl::parse(OFFICIAL_1996).unwrap().birth_date_parts(),
            Some((1963, 10, 12))
        );
        assert_eq!(
            Pinfl::parse(OFFICIAL_2022).unwrap().birth_date_parts(),
            Some((1993, 10, 12))
        );
    }

    #[test]
    fn structural_parse_is_lenient_strict_is_not() {
        let zeros = "00000000000000";
        assert!(Pinfl::parse(zeros).is_ok()); // struktura: 14 raqam
        assert_eq!(Pinfl::parse_strict(zeros), Err(PinflError::Structure)); // checksum 0 == 0, lekin 1-raqam 0
        assert_eq!(
            Pinfl::parse_strict("31210632040245"),
            Err(PinflError::Checksum)
        );
        assert_eq!(Pinfl::parse("1234567890123"), Err(PinflError::Length));
        assert_eq!(Pinfl::parse("1234567890123a"), Err(PinflError::Format));
    }

    #[cfg(feature = "date")]
    #[test]
    fn birth_date_is_extracted() {
        let p = Pinfl::parse(OFFICIAL_2022).unwrap();
        assert_eq!(p.birth_date().unwrap().to_string(), "1993-10-12");
    }

    #[cfg(feature = "date")]
    #[test]
    fn birth_date_at_is_deterministic() {
        let today = chrono::NaiveDate::from_ymd_opt(2026, 9, 3).unwrap();

        // 1-raqam `1`/`2` → asr 1800. BirthDate::MIN_YEAR = 1900 bo'lganda bu
        // har doim None qaytarardi — MIN_YEAR = 1800 aynan shu teshikni yopdi.
        let old = Pinfl::parse("11210632040244").unwrap();
        assert_eq!(old.century(), Some(1800));
        assert_eq!(
            old.birth_date_at(today).unwrap().to_string(),
            "1863-10-12" // 12.10.63 + 1800
        );

        // 2000-yillar: `5`/`6` + kelajakdagi YY → BirthDate rad etadi (soatga bog'liq emas)
        let future = Pinfl::parse("51210992040244").unwrap();
        assert_eq!(future.birth_date_parts(), Some((2099, 10, 12)));
        assert_eq!(future.birth_date_at(today), None);
    }
}
