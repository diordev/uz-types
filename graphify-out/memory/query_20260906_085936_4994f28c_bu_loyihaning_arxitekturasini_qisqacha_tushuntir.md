---
type: "query"
date: "2026-09-06T08:59:36.667484+00:00"
question: "Bu loyihaning arxitekturasini qisqacha tushuntir"
contributor: "graphify"
source_nodes: ["lib.rs", "macros.rs", "secret.rs", "serde_support.rs", "sqlx_support.rs", "TypeError"]
---

# Q: Bu loyihaning arxitekturasini qisqacha tushuntir

## Answer

Expanded from original query via vocab: [architecture, project, lib, crate, newtype, typed, macros, serde, sqlx, prelude, structural, structure]. Then traversed BFS depth=2 from lib.rs, macros.rs, prelude.rs, serde_support.rs, sqlx_support.rs and domain modules (id.rs, email.rs, passport.rs, phone_number.rs, pinfl.rs, birth_date.rs, secret.rs).

Arxitektura markazi src/lib.rs (L1) - barcha domen modullarini (email.rs, passport.rs, phone_number.rs, pinfl.rs, birth_date.rs, id.rs, secret.rs) yig'adi va prelude.rs orqali qulay import beradi.

Ikkita umumiy makro barcha boilerplate'ni chiqaradi: string_newtype! makrosi macros.rs faylida (trim_in_place funksiyasi L11) EmailAddress, Passport, PhoneNumber, Pinfl kabi String-asosli tiplarga parse/TryFrom/Display/serde/sqlx qobiliyatlarini beradi; secret_newtype! makrosi src/secret.rs faylida esa ataylab kambag'al - Debug redaction, constant-time PartialEq (validate_token funksiyasi L27, TokenError L18).

Id<Tag> (id.rs L65) va NumId<Tag, R> (id.rs L351) generic tag-asosli identifikator tizimi; NumIdRepr sealed trait (L234) faqat u64/i64 ga ruxsat beradi, DB-safe konstruksiya .parse_db_safe()/.try_from() orqali xatoni erta ushlaydi.

Integratsiya qatlami alohida: serde_support.rs (deserialize_string_newtype funksiyasi L21) va sqlx_support.rs - ikkalasi ham makrolar orqali avtomatik ulanadi, DB/serialize xatolari smart constructor orqali o'tadi.

Har bir domen tipi o'zining aniq xatosini beradi: EmailAddressError L129, PhoneNumberError L106, PinflError L140, PassportError L64, BirthDateError L229. TypeError (error.rs L7) esa agregat sifatida #[from] orqali yig'iladi.

## Source Nodes

- lib.rs
- macros.rs
- secret.rs
- serde_support.rs
- sqlx_support.rs
- TypeError