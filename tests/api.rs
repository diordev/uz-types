//! Public API integration testlari.
//!
//! Bu fayl crate'ni **tashqi foydalanuvchi** nuqtai nazaridan tekshiradi:
//! faqat `uz_types::...` orqali chaqiriladigan narsalargina bu yerda ishlaydi.
//!
//! Maqsad — quyidagilarni qulflab qo'yish:
//! 1. Har bir tip crate ildizidan chaqirilishi kerak (`uz_types::Passport`).
//! 2. Har bir xato (error) turi nomlanadigan va `match` qilinadigan bo'lishi kerak.
//! 3. `prelude` ham xuddi shu to'plamni berishi kerak.

use uz_types::{
    AccessToken, BirthDate, BirthDateError, ClientId, ClientSecret, DateFormat, EmailAddress,
    EmailAddressError, IdError, JobId, Passport, PassportError, PhoneNumber, PhoneNumberError,
    Pinfl, PinflError, RefreshToken, RequestId, Reuid, SessionId, TokenError, TypeError,
};

// ==========================================
// 1. Har bir tip crate ildizidan yaratilishi
// ==========================================

#[test]
fn every_type_is_constructible_from_crate_root() {
    assert_eq!(Passport::parse("AA1234567").unwrap().as_str(), "AA1234567");
    assert_eq!(
        Pinfl::parse("12345678901234").unwrap().as_str(),
        "12345678901234"
    );
    assert_eq!(
        PhoneNumber::parse("+998901234567").unwrap().as_str(),
        "998901234567"
    );
    assert_eq!(
        EmailAddress::parse("Test@Example.COM").unwrap().as_str(),
        "test@example.com"
    );
    assert_eq!(
        BirthDate::parse("1990-05-15").unwrap().to_string(),
        "1990-05-15"
    );

    assert!(JobId::generate().as_uuid().is_some());
    assert_eq!(SessionId::parse("42").unwrap().as_number(), Some(42));
    assert!(RequestId::generate().as_uuid().is_some());
    assert!(Reuid::generate().as_uuid().is_some());

    assert_eq!(AccessToken::parse(" abc ").unwrap().as_str(), "abc");
    assert_eq!(RefreshToken::parse("r-1").unwrap().as_str(), "r-1");
    assert_eq!(ClientId::parse("client-1").unwrap().as_str(), "client-1");
    assert_eq!(ClientSecret::parse("s3cr3t").unwrap().as_str(), "s3cr3t");
}

// ==========================================
// 2. Xato turlarini tashqaridan `match` qilish
// ==========================================

#[test]
fn passport_errors_are_matchable_from_outside() {
    assert!(matches!(
        Passport::parse("AA123").unwrap_err(),
        TypeError::Passport(PassportError::Length)
    ));
    assert!(matches!(
        Passport::parse("A11234567").unwrap_err(),
        TypeError::Passport(PassportError::Format)
    ));
}

#[test]
fn pinfl_errors_are_matchable_from_outside() {
    assert!(matches!(
        Pinfl::parse("123").unwrap_err(),
        TypeError::PINFL(PinflError::Length)
    ));
    assert!(matches!(
        Pinfl::parse("1234567890123a").unwrap_err(),
        TypeError::PINFL(PinflError::Format)
    ));
}

#[test]
fn phone_number_errors_are_matchable_from_outside() {
    assert!(matches!(
        PhoneNumber::parse("99890").unwrap_err(),
        TypeError::PhoneNumber(PhoneNumberError::Length)
    ));
    assert!(matches!(
        PhoneNumber::parse("998a01234567").unwrap_err(),
        TypeError::PhoneNumber(PhoneNumberError::Format)
    ));
    assert!(matches!(
        PhoneNumber::parse("997901234567").unwrap_err(),
        TypeError::PhoneNumber(PhoneNumberError::Prefix)
    ));
    assert!(matches!(
        PhoneNumber::parse("998000000000").unwrap_err(),
        TypeError::PhoneNumber(PhoneNumberError::OperatorCode)
    ));
}

#[test]
fn email_errors_are_matchable_from_outside() {
    assert!(matches!(
        EmailAddress::parse("a@b").unwrap_err(),
        TypeError::EmailAddress(EmailAddressError::Length)
    ));
    assert!(matches!(
        EmailAddress::parse("test.example.com").unwrap_err(),
        TypeError::EmailAddress(EmailAddressError::Format)
    ));
}

#[test]
fn birth_date_errors_are_matchable_from_outside() {
    assert!(matches!(
        BirthDate::parse("not-a-date").unwrap_err(),
        TypeError::BirthDate(BirthDateError::Date)
    ));
    assert!(matches!(
        BirthDate::parse("3000-01-01").unwrap_err(),
        TypeError::BirthDate(BirthDateError::FutureDate)
    ));
    assert!(matches!(
        BirthDate::parse("0001-01-01").unwrap_err(),
        TypeError::BirthDate(BirthDateError::TooOld)
    ));
}

#[test]
fn id_errors_are_matchable_from_outside() {
    assert!(matches!(
        JobId::parse("not-a-uuid-or-number").unwrap_err(),
        TypeError::Id(IdError::Format)
    ));
    // v1 UUID — MAC-manzilni oshkor qilgani uchun rad etiladi
    assert!(matches!(
        JobId::parse("6ba7b810-9dad-11d1-80b4-00c04fd430c8").unwrap_err(),
        TypeError::Id(IdError::Version)
    ));
}

#[test]
fn v7_ids_are_generatable_and_reparseable_from_outside() {
    let id = RequestId::generate_v7();
    assert!(id.as_uuid().is_some());
    assert_eq!(RequestId::parse(id.to_string()).unwrap(), id);

    // Raqamli ID'da UUID versiyasi bo'lmaydi
    assert_eq!(JobId::parse("42").unwrap().uuid_version(), None);
}

#[test]
fn id_serde_shape_is_uuid_string_or_bare_integer() {
    // UUID — JSON string
    let uuid_id = SessionId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap();
    assert_eq!(
        serde_json::to_string(&uuid_id).unwrap(),
        "\"9b7e597e-893e-4e11-92cf-f4e7d4f923b1\""
    );

    // Number — qo'shtirnoqsiz integer
    let num_id = SessionId::parse("123").unwrap();
    assert_eq!(serde_json::to_string(&num_id).unwrap(), "123");

    // Deserializatsiyada ikkala shakl ham qabul qilinadi
    assert_eq!(serde_json::from_str::<SessionId>("123").unwrap(), num_id);
    assert_eq!(
        serde_json::from_str::<SessionId>("\"123\"").unwrap(),
        num_id
    );
}

#[test]
fn token_errors_are_matchable_from_outside() {
    assert!(matches!(
        AccessToken::parse("   ").unwrap_err(),
        TypeError::Token(TokenError::Empty)
    ));
    assert!(matches!(
        ClientSecret::parse("").unwrap_err(),
        TypeError::Token(TokenError::Empty)
    ));
    assert!(matches!(
        AccessToken::parse("a".repeat(uz_types::MAX_TOKEN_LEN + 1)).unwrap_err(),
        TypeError::Token(TokenError::TooLong)
    ));
}

// ==========================================
// 3. `TypeError` std::error::Error sifatida ishlashi
// ==========================================

#[test]
fn type_error_works_as_boxed_std_error() {
    fn fallible() -> Result<Passport, Box<dyn std::error::Error>> {
        Ok(Passport::parse("AA123")?)
    }

    let err = fallible().unwrap_err();
    assert!(err.to_string().contains("passport length"));
}

// ==========================================
// 4. Yordamchi metodlar va trait'lar tashqaridan
// ==========================================

#[test]
fn public_helper_methods_are_reachable() {
    let passport = Passport::parse("aa1234567").unwrap();
    assert_eq!(passport.series(), "AA");
    assert_eq!(passport.number(), "1234567");

    let phone = PhoneNumber::parse("998901234567").unwrap();
    assert_eq!(phone.to_international(), "+998901234567");
    assert_eq!(phone.operator_code(), "90");
    assert_eq!(phone.subscriber_number(), "1234567");
    assert!(phone.is_mobile());

    let email = EmailAddress::parse("user.name@domain.com").unwrap();
    assert_eq!(email.local_part(), "user.name");
    assert_eq!(email.domain(), "domain.com");

    let date = BirthDate::parse_with_format("15.05.1990", DateFormat::DmyDot).unwrap();
    assert_eq!(date.format_as(DateFormat::YmdHyphen), "1990-05-15");
    assert_eq!(date.format_reversed(DateFormat::DmyDot), "1990.05.15");
    assert_eq!((date.year(), date.month(), date.day()), (1990, 5, 15));

    // Yosh hisoblash tashqaridan ham chaqirilishi kerak.
    // `NaiveDate` ni tipini nomlamasdan, BirthDate orqali olamiz.
    let reference = BirthDate::parse("2026-05-16").unwrap().into_inner();
    assert_eq!(date.age_at(reference), 36);
    assert!(date.age() >= 35);
}

#[test]
fn from_str_is_implemented_for_every_type() {
    // Har bir public tip `str::parse()` orqali ham yaratilishi kerak
    let passport: Passport = "aa1234567".parse().unwrap();
    assert_eq!(passport.as_str(), "AA1234567");

    let pinfl: Pinfl = "12345678901234".parse().unwrap();
    assert_eq!(pinfl.as_str(), "12345678901234");

    let phone: PhoneNumber = "+998901234567".parse().unwrap();
    assert_eq!(phone.as_str(), "998901234567");

    let email: EmailAddress = "A@Example.com".parse().unwrap();
    assert_eq!(email.as_str(), "a@example.com");

    let birth_date: BirthDate = "1990-05-15".parse().unwrap();
    assert_eq!(birth_date.to_string(), "1990-05-15");

    let id: JobId = "42".parse().unwrap();
    assert_eq!(id.as_number(), Some(42));

    let token: AccessToken = "tok".parse().unwrap();
    assert_eq!(token.as_str(), "tok");

    let client_id: ClientId = "c-1".parse().unwrap();
    assert_eq!(client_id.as_str(), "c-1");

    // Xatolar ham `TypeError` bo'lib qaytadi
    assert!(matches!(
        "AA123".parse::<Passport>().unwrap_err(),
        TypeError::Passport(PassportError::Length)
    ));
}

#[test]
fn try_from_conversions_are_reachable() {
    let passport = Passport::try_from(String::from("AA1234567")).unwrap();
    let raw: String = passport.into();
    assert_eq!(raw, "AA1234567");

    let email = EmailAddress::try_from("user@test.com").unwrap();
    assert_eq!(email.domain(), "test.com");
}

// ==========================================
// 5. Serde tashqaridan
// ==========================================

#[test]
fn serde_roundtrip_through_public_api() {
    #[derive(serde::Serialize, serde::Deserialize, PartialEq, Debug)]
    struct User {
        passport: Passport,
        pinfl: Pinfl,
        phone: PhoneNumber,
        email: EmailAddress,
        birth_date: BirthDate,
        session: SessionId,
        token: AccessToken,
    }

    let user = User {
        passport: Passport::parse("AA1234567").unwrap(),
        pinfl: Pinfl::parse("12345678901234").unwrap(),
        phone: PhoneNumber::parse("998901234567").unwrap(),
        email: EmailAddress::parse("user@example.com").unwrap(),
        birth_date: BirthDate::parse("1990-05-15").unwrap(),
        session: SessionId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap(),
        token: AccessToken::parse("tok123").unwrap(),
    };

    let json = serde_json::to_string(&user).unwrap();
    let restored: User = serde_json::from_str(&json).unwrap();
    assert_eq!(user, restored);
}

#[test]
fn serde_rejects_invalid_values() {
    assert!(serde_json::from_str::<Passport>("\"nope\"").is_err());
    assert!(serde_json::from_str::<PhoneNumber>("\"997881234567\"").is_err());
    assert!(serde_json::from_str::<EmailAddress>("\"not-an-email\"").is_err());
    assert!(serde_json::from_str::<AccessToken>("\"\"").is_err());
}

// ==========================================
// 6. `prelude` ildiz bilan bir xil to'plamni berishi
// ==========================================

#[test]
fn prelude_exposes_the_same_types() {
    use uz_types::prelude::*;

    let _: Passport = Passport::parse("AA1234567").unwrap();
    let _: Pinfl = Pinfl::parse("12345678901234").unwrap();
    let _: PhoneNumber = PhoneNumber::parse("998901234567").unwrap();
    let _: EmailAddress = EmailAddress::parse("a@example.com").unwrap();
    let _: BirthDate = BirthDate::parse("1990-05-15").unwrap();
    let _: JobId = JobId::generate();
    let _: SessionId = SessionId::generate();
    let _: RequestId = RequestId::generate();
    let _: Reuid = Reuid::generate();
    let _: AccessToken = AccessToken::parse("a").unwrap();
    let _: RefreshToken = RefreshToken::parse("r").unwrap();
    let _: ClientId = ClientId::parse("c").unwrap();
    let _: ClientSecret = ClientSecret::parse("s").unwrap();

    // Error tiplari ham prelude'da bo'lishi kerak
    let _: fn(PassportError) -> TypeError = TypeError::Passport;
    let _: fn(PinflError) -> TypeError = TypeError::PINFL;
    let _: fn(PhoneNumberError) -> TypeError = TypeError::PhoneNumber;
    let _: fn(EmailAddressError) -> TypeError = TypeError::EmailAddress;
    let _: fn(BirthDateError) -> TypeError = TypeError::BirthDate;
    let _: fn(IdError) -> TypeError = TypeError::Id;
    let _: fn(TokenError) -> TypeError = TypeError::Token;
}

// ==========================================
// 7. Token sirlari Debug'da yashiringan bo'lishi
// ==========================================

#[test]
fn token_debug_output_is_redacted() {
    let token = AccessToken::parse("super-secret-value").unwrap();
    let debug = format!("{:?}", token);

    assert!(!debug.contains("super-secret-value"));
    assert!(debug.contains("REDACTED"));

    let secret = ClientSecret::parse("super-secret-value").unwrap();
    assert!(!format!("{:?}", secret).contains("super-secret-value"));
}
