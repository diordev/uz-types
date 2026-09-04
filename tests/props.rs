//! Property-based testlar: (1) hech qanday input panic qilmaydi; (2) roundtrip.
use proptest::prelude::*;
use uz_types::prelude::*;

macro_rules! never_panics_and_roundtrips {
    ($name:ident, $T:ty) => {
        proptest! {
            #[test]
            fn $name(input in "\\PC{0,64}") {
                // 1. Panic yo'q
                let parsed = <$T>::parse(&input);
                // 2. Roundtrip: normalizatsiya idempotent
                if let Ok(v) = parsed {
                    let again = <$T>::parse(v.as_str()).unwrap();
                    prop_assert_eq!(&again, &v);
                    prop_assert_eq!(<$T>::try_from(v.as_str().to_owned()).unwrap(), v);
                }
            }
        }
    };
}

never_panics_and_roundtrips!(passport_props, Passport);
never_panics_and_roundtrips!(pinfl_props, Pinfl);
never_panics_and_roundtrips!(phone_props, PhoneNumber);
never_panics_and_roundtrips!(email_props, EmailAddress);

proptest! {
    // Haqiqiy pasportlar uchun generator: har doim Ok va normalizatsiya to'g'ri
    #[test]
    fn valid_passports_always_parse(series in "[a-zA-Z]{2}", number in "[0-9]{7}", pad in "[ \t]{0,3}") {
        let input = format!("{pad}{series} {number}{pad}");
        let p = Passport::parse(&input).unwrap();
        prop_assert_eq!(p.series(), series.to_ascii_uppercase());
        prop_assert_eq!(p.number(), number);
    }

    #[test]
    fn valid_phones_with_any_separators(code in "[0-9]{2}", rest in "[0-9]{7}", plus in proptest::bool::ANY) {
        let input = format!("{}998 ({code}) {}-{}-{}", if plus { "+" } else { "" }, &rest[..3], &rest[3..5], &rest[5..]);
        let p = PhoneNumber::parse(&input).unwrap();
        prop_assert_eq!(p.operator_code(), code);
        prop_assert_eq!(p.subscriber_number(), rest);
    }

    // NumId: har ikkala repr uchun ham panic yo'q va Display→parse roundtrip
    #[cfg(feature = "id")]
    #[test]
    fn num_id_never_panics_and_roundtrips(input in "\\PC{0,32}") {
        enum Order {}
        if let Ok(v) = NumId::<Order>::parse(&input) {
            prop_assert_eq!(NumId::<Order>::parse(&v.to_string()).unwrap(), v);
        }
        if let Ok(v) = NumId::<Order, i64>::parse(&input) {
            prop_assert_eq!(NumId::<Order, i64>::parse(&v.to_string()).unwrap(), v);
        }
    }

    // parse_db_safe qabul qilgan har qanday qiymat Encode'da HECH QACHON fail bo'lmaydi
    #[cfg(feature = "id")]
    #[test]
    fn db_safe_ids_always_convert_to_bigint(n in 0u64..=u64::MAX) {
        enum Order {}
        let id = NumId::<Order>::new(n);
        prop_assert_eq!(id.is_db_safe(), n <= NumId::<Order>::MAX_DB_SAFE);
        prop_assert_eq!(
            NumId::<Order>::try_new_db_safe(n).is_ok(),
            id.to_bigint().is_ok()
        );
        // i64 repr uchun xato yo'li umuman yo'q
        prop_assert!(NumId::<Order, i64>::new(n as i64).to_bigint().is_ok());
    }

    #[test]
    fn pinfl_checksum_generator_agrees(body in "[1-6][0-9]{12}") {
        // Rasmiy algoritm bilan hisoblangan nazorat raqami is_checksum_valid dan o'tishi kerak
        let sum: u32 = body.bytes().enumerate().map(|(i, b)| u32::from(b - b'0') * [7, 3, 1][i % 3]).sum();
        let full = format!("{body}{}", sum % 10);
        prop_assert!(Pinfl::parse(&full).unwrap().is_checksum_valid());
    }
}
