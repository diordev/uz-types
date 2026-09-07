//! Feature'dan mustaqil Gregorian kalendar tekshiruvi.
//!
//! [`Pinfl::parse_strict`](crate::Pinfl::parse_strict) `31.02` yoki `29.02.1900` kabi
//! mavjud bo'lmagan sanalarni rad etishi kerak, ammo bu invariant `date` feature'iga
//! bog'lanmasligi shart: aks holda bir xil qiymat feature holatiga qarab turlicha
//! natija berardi. Shu sabab bu yerda `chrono`ga bog'liq bo'lmagan, allocation
//! qilmaydigan va tizim soatini o'qimaydigan `const fn` helper turadi; `date`
//! yoqilganda test uni `chrono::NaiveDate` bilan 1800–2099 oralig'ida solishtiradi.

/// `year`-`month`-`day` haqiqiy Gregorian sanami (kabisa qoidasi bilan).
///
/// Oy `1..=12`, kun `1..=<oydagi kunlar>` bo'lishi shart; boshqa qiymat `false`.
pub(crate) const fn is_valid_gregorian_date(year: i32, month: u32, day: u32) -> bool {
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 if is_leap_year(year) => 29,
        2 => 28,
        _ => return false,
    };

    day >= 1 && day <= days_in_month
}

/// Gregorian kabisa qoidasi: 400ga bo'linsa kabisa, aks holda 4ga bo'linib 100ga bo'linmasa.
const fn is_leap_year(year: i32) -> bool {
    year % 400 == 0 || (year % 4 == 0 && year % 100 != 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gregorian_boundaries_are_checked() {
        assert!(!is_valid_gregorian_date(1900, 2, 29));
        assert!(is_valid_gregorian_date(2000, 2, 29));
        assert!(!is_valid_gregorian_date(2100, 2, 29));
        assert!(is_valid_gregorian_date(2026, 4, 30));
        assert!(!is_valid_gregorian_date(2026, 4, 31));
        assert!(is_valid_gregorian_date(2026, 1, 31));
        assert!(!is_valid_gregorian_date(2026, 1, 32));
        assert!(!is_valid_gregorian_date(2026, 0, 1));
        assert!(!is_valid_gregorian_date(2026, 13, 1));
        assert!(!is_valid_gregorian_date(2026, 1, 0));
    }

    #[cfg(feature = "date")]
    #[test]
    fn matches_chrono_oracle_exhaustively() {
        for year in 1800..=2099 {
            for month in 0..=13 {
                for day in 0..=32 {
                    assert_eq!(
                        is_valid_gregorian_date(year, month, day),
                        chrono::NaiveDate::from_ymd_opt(year, month, day).is_some(),
                        "Gregorian natija mos emas: {year:04}-{month:02}-{day:02}"
                    );
                }
            }
        }
    }
}
