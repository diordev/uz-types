//! `uz-types` tiplarining amaliy namunasi.
//!
//! Ishga tushirish: `cargo run --example types_example`

use uz_types::prelude::*;

fn main() -> Result<(), TypeError> {
    // ---------- Shaxsiy ma'lumotlar ----------

    // Pasport: trim va uppercase avtomatik bajariladi
    let passport = Passport::parse("  aa1234567  ")?;
    println!(
        "Pasport: {passport} | seriya: {} | raqam: {}",
        passport.series(),
        passport.number()
    );

    // PINFL: faqat format tekshiriladi (checksum emas)
    let pinfl = Pinfl::parse("31234567890123")?;
    println!("PINFL: {pinfl}");

    // Telefon: `+` ixtiyoriy, operator/hudud kodi tekshiriladi
    let mobile = PhoneNumber::parse("+998901234567")?;
    let landline = PhoneNumber::parse("998711234567")?;
    println!(
        "Mobil:      {} | kod: {} | abonent: {} | mobilmi: {}",
        mobile.to_international(),
        mobile.operator_code(),
        mobile.subscriber_number(),
        mobile.is_mobile()
    );
    println!(
        "Statsionar: {} | kod: {} | mobilmi: {}",
        landline.to_international(),
        landline.operator_code(),
        landline.is_mobile()
    );

    // Tug'ilgan sana: kelajak va 1900 dan oldingi sanalar rad etiladi
    let birth_date = BirthDate::parse("1995-08-31")?;
    println!(
        "Tug'ilgan sana: {birth_date} | yosh: {} | DD.MM.YYYY: {}",
        birth_date.age(),
        birth_date.format_as(DateFormat::DmyDot)
    );

    // Email: lowercase qilib saqlanadi
    let email = EmailAddress::parse("Diordev@iCloud.com")?;
    println!(
        "Email: {email} | local-part: {} | domain: {}",
        email.local_part(),
        email.domain()
    );

    // ---------- ID tiplari ----------
    println!();

    // v4 — tasodifiy; v7 — vaqt bo'yicha tartiblangan (DB indeksi uchun afzal)
    println!("JobId (v4):      {}", JobId::generate());
    println!("SessionId (v7):  {}", SessionId::generate_v7());
    println!("RequestId (v7):  {}", RequestId::generate_v7());

    // Mavjud qiymatni parse qilish — UUID yoki u64
    let numeric_id = Reuid::parse("444444")?;
    let uuid_id = JobId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1")?;
    println!(
        "Reuid (raqam):   {numeric_id} -> as_number(): {:?}",
        numeric_id.as_number()
    );
    println!(
        "JobId (UUID):    {uuid_id} -> versiya: {:?}",
        uuid_id.uuid_version()
    );

    // ---------- Tokenlar ----------
    println!();

    let access_token = AccessToken::parse(" eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.xyz ")?;
    let refresh_token = RefreshToken::parse("rt_9f8e7d6c5b4a")?;
    let client_id = ClientId::parse("my-service")?;
    let client_secret = ClientSecret::parse("s3cr3t-value")?;

    // Loglashda `{:?}` — qiymat yashiringan holda chiqadi
    println!("AccessToken:   {access_token:?}");
    println!("RefreshToken:  {refresh_token:?}");
    println!("ClientId:      {client_id:?}");
    println!("ClientSecret:  {client_secret:?}");

    // Haqiqiy qiymat faqat tashqi xizmatga uzatishda olinadi
    println!(
        "\n(Authorization header uchun) Bearer {}...",
        &access_token.as_str()[..10]
    );

    // ---------- Xatolarni ajratish ----------
    println!();

    for input in ["AA1234567", "AA123", "A11234567"] {
        match Passport::parse(input) {
            Ok(p) => println!("{input:<12} -> OK: {p}"),
            Err(TypeError::Passport(PassportError::Length)) => {
                println!("{input:<12} -> uzunlik noto'g'ri")
            }
            Err(TypeError::Passport(PassportError::Format)) => {
                println!("{input:<12} -> format noto'g'ri")
            }
            Err(e) => println!("{input:<12} -> boshqa xato: {e}"),
        }
    }

    Ok(())
}
