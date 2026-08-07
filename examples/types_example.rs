use uz_types::prelude::*;
fn main() {
    // Pasportni tekshirish va yaratish
    let passport = Passport::parse("AA1234567").unwrap();
    println!("Pasport: {}", passport);

    // PINFL ni tekshirish
    let pinfl = Pinfl::parse("31234567890123").unwrap();
    println!("PINFL: {}", pinfl);

    // PhoneNumber ni tekshirish
    let phone = PhoneNumber::parse("998901234567").unwrap();
    println!("Phone Number: {}", phone);

    // Tug'ilgan sanani tekshirish
    let birth_date = BirthDate::parse("1995-08-31").unwrap();
    println!("Tug'ilgan sana: {}", birth_date);

    // Email tekshiruvi
    let email = EmailAddress::parse("diordev@iclud.com").unwrap();
    println!("Email address: {}", email);

    // JobId, SessionId, RequestId  generatisya qilish yoki parse qilish.

    let job_id: JobId = JobId::generate();
    let session_id: SessionId = SessionId::generate();
    let request_id: RequestId = RequestId::generate();

    println!(
        "JobId: {},\nSessionId: {},\nRequestId: {}",
        job_id, session_id, request_id
    )
}
