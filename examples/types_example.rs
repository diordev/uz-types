//! `uz-types` tiplarining amaliy namunasi.
//!
//! Ishga tushirish: `cargo run --example types_example`

use uz_types::prelude::*;

// ===== ID yaratish tartibi =====
//
// Qadam 1 — tag'lar. Butun loyiha uchun BITTA modulda turishi kerak:
// `Id<a::Order>` va `Id<b::Order>` bir-biriga to'g'ri kelmaydigan turli tiplar.
// `enum {}` uninhabited: instansiya yaratib bo'lmaydi, faqat compile-time belgisi.
mod tag {
    pub enum Job {}
    pub enum Session {}
    pub enum LegacyInvoice {}
}

// Qadam 2 — alias. Ko'rinish (UUID yoki BIGINT) SHU YERDA tanlanadi.
type JobId = Id<tag::Job>; // -> Postgres UUID
type SessionId = Id<tag::Session>; // -> Postgres UUID
type LegacyInvoiceId = NumId<tag::LegacyInvoice, i64>; // -> Postgres BIGINT

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

    // Tug'ilgan sana: kelajak va 1800 dan oldingi sanalar rad etiladi
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

    // Qadam 3 — ishlatish.
    // v7: vaqt bo'yicha tartiblangan — DB primary key uchun afzal (index locality).
    // v4: tasodifiy — yaratilish vaqtini oshkor qilmaydi.
    println!("JobId (v7):        {}", JobId::now_v7());
    println!("SessionId (v4):    {}", SessionId::new_v4());

    // Mavjud qiymatni parse qilish
    let uuid_id = JobId::parse("9b7e597e-893e-4e11-92cf-f4e7d4f923b1")?;
    println!(
        "JobId (UUID):      {uuid_id} -> versiya: {:?}",
        uuid_id.version()
    );

    // Eski tizimdagi BIGINT ID — manfiy qiymat ham qabul qilinadi
    let invoice = LegacyInvoiceId::parse("-42")?;
    println!(
        "LegacyInvoiceId:   {invoice} -> BIGINT: {:?}",
        invoice.to_bigint()
    );

    // Tip xavfsizligi: quyidagi qator kompilyatsiya BO'LMAYDI —
    //     let s: SessionId = JobId::now_v7();
    // chunki Id<tag::Job> va Id<tag::Session> turli tiplar. Aynan shu foyda uchun tag bor.

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
        "\n(Authorization header uchun) Bearer {:#?}...",
        access_token
    );

    Ok(())
}
