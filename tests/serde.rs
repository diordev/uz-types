#![cfg(feature = "serde")]

use uz_types::prelude::*;

#[test]
fn string_newtypes_roundtrip_and_reject_invalid() {
    let p = Passport::parse("aa1234567").unwrap();
    let json = serde_json::to_string(&p).unwrap();
    assert_eq!(json, "\"AA1234567\"");
    assert_eq!(serde_json::from_str::<Passport>(&json).unwrap(), p);

    // visit_str yo'li (serde_json::from_str) — normalizatsiya ishlaydi
    assert_eq!(
        serde_json::from_str::<PhoneNumber>("\"+998 (90) 123-45-67\"")
            .unwrap()
            .as_str(),
        "998901234567"
    );
    // visit_string yo'li (serde_json::Value → owned String)
    let value = serde_json::Value::String("  aa1234567 ".into());
    assert_eq!(Passport::deserialize_from(value).as_str(), "AA1234567");

    // Invalid → Err (smart constructor chetlab o'tilmaydi)
    assert!(serde_json::from_str::<Passport>("\"nope\"").is_err());
    assert!(serde_json::from_str::<Pinfl>("\"12345\"").is_err());
    assert!(serde_json::from_str::<EmailAddress>("\"not-an-email\"").is_err());
    assert!(serde_json::from_str::<AccessToken>("\"\"").is_err());
    // Xato xabari `expecting` ni o'z ichiga oladi
    let err = serde_json::from_str::<Passport>("42")
        .unwrap_err()
        .to_string();
    assert!(err.contains("passport number"), "{err}");
}

trait DeserializeFromValue: Sized {
    fn deserialize_from(v: serde_json::Value) -> Self;
}
impl<T: serde::de::DeserializeOwned> DeserializeFromValue for T {
    fn deserialize_from(v: serde_json::Value) -> Self {
        serde_json::from_value(v).unwrap()
    }
}

#[cfg(feature = "id")]
#[test]
fn ids_have_one_json_shape() {
    enum Order {}
    let id = Id::<Order>::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1").unwrap();
    assert_eq!(
        serde_json::to_string(&id).unwrap(),
        "\"9b7e597e-893e-4e11-92cf-f4e7d4f923b1\""
    );
    assert_eq!(
        serde_json::from_str::<Id<Order>>(&serde_json::to_string(&id).unwrap()).unwrap(),
        id
    );
    assert!(serde_json::from_str::<Id<Order>>("42").is_err()); // integer UUID emas

    let n = NumId::<Order>::new(42);
    assert_eq!(serde_json::to_string(&n).unwrap(), "42");
    assert_eq!(serde_json::from_str::<NumId<Order>>("42").unwrap(), n);
    assert!(serde_json::from_str::<NumId<Order>>("\"42\"").is_err()); // string emas
    assert!(serde_json::from_str::<NumId<Order>>("-1").is_err()); // u64 repr manfiyni olmaydi

    // i64 repr — manfiy legacy ID'lar uchun
    let s = NumId::<Order, i64>::new(-1);
    assert_eq!(serde_json::to_string(&s).unwrap(), "-1");
    assert_eq!(serde_json::from_str::<NumId<Order, i64>>("-1").unwrap(), s);
    assert!(serde_json::from_str::<NumId<Order, i64>>("\"-1\"").is_err());
    // u64::MAX i64 repr'ga sig'maydi — deserializatsiya darhol rad etadi
    assert!(serde_json::from_str::<NumId<Order, i64>>("18446744073709551615").is_err());
}

#[cfg(feature = "date")]
#[test]
fn birth_date_roundtrip() {
    let d = BirthDate::parse("1990-05-15").unwrap();
    let json = serde_json::to_string(&d).unwrap();
    assert_eq!(json, "\"1990-05-15\"");
    assert_eq!(serde_json::from_str::<BirthDate>(&json).unwrap(), d);
    assert!(serde_json::from_str::<BirthDate>("\"3000-01-01\"").is_err());
}

#[test]
fn secrets_deserialize_but_do_not_serialize_by_default() {
    let t: AccessToken = serde_json::from_str("\"tok\"").unwrap();
    assert_eq!(t.expose_secret(), "tok");

    #[cfg(feature = "serialize-secrets")]
    assert_eq!(serde_json::to_string(&t).unwrap(), "\"tok\"");

    // ClientId sir emas — har doim serializatsiya bo'ladi
    let c = ClientId::parse("c-1").unwrap();
    assert_eq!(serde_json::to_string(&c).unwrap(), "\"c-1\"");
}
