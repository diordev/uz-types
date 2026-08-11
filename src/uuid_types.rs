use crate::error::TypeError;

// UUID va Number formatlari uchun xatoliklar ro'yxati
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum IdError {
    // Format UUID ham, Number ham bo'lmaganda chiqadigan xato
    #[error("invalid ID format (must be UUID or a valid number)")]
    Format,

    // UUID versiyasi 4 (Random) bo'lmaganda chiqadigan xato
    #[error("UUID must be version 4 (Random)")]
    Version,
}

// ==========================================
// UNIVERSAL ID FORMAT ENUM
// ==========================================

// ID ma'lumotlarini o'zida saqlovchi yagona enum
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum IdFormat {
    Uuid(uuid::Uuid),
    Number(u64),
}

impl IdFormat {
    // Matnni qabul qilib, Number yoki UUID'ga xatosiz o'giruvchi yagona funksiya
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        let raw = value.as_ref().trim();

        // 1. Agar matn raqam bo'lsa, uni u64 formatida qaytaramiz
        if let Ok(num) = raw.parse::<u64>() {
            return Ok(Self::Number(num));
        }

        // 2. Aks holda matnni UUID sifatida parse qilishga urinamiz
        let parsed = uuid::Uuid::parse_str(raw).map_err(|_| TypeError::Id(IdError::Format))?;

        // 3. O'girilgan UUID faqatgina v4 (Random) bo'lishini talab qilamiz
        if parsed.get_version() != Some(uuid::Version::Random) {
            return Err(TypeError::Id(IdError::Version));
        }

        Ok(Self::Uuid(parsed))
    }
}

impl Default for IdFormat {
    // Obyekt bo'sh yaratilganda avtomatik yangi UUID v4 beriladi
    fn default() -> Self {
        Self::Uuid(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for IdFormat {
    // Formatni foydalanuvchiga matn ko'rinishida chiqarib berish
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uuid(u) => write!(f, "{}", u.as_hyphenated()),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}

impl serde::Serialize for IdFormat {
    // Serialize qilinganda xotirani tejash uchun to'g'ridan-to'g'ri buferga yoziladi
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Uuid(u) => {
                let mut buffer = uuid::Uuid::encode_buffer();
                serializer.serialize_str(u.as_hyphenated().encode_lower(&mut buffer))
            }
            Self::Number(n) => serializer.serialize_u64(*n),
        }
    }
}

impl<'de> serde::Deserialize<'de> for IdFormat {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct IdFormatVisitor;

        impl<'de> serde::de::Visitor<'de> for IdFormatVisitor {
            type Value = IdFormat;

            // Xato bo'lganda qanday ma'lumot kutilganligini bildirish
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UUID v4 string or an integer")
            }

            // 1. Signed son (i64) - Serde trait'ida birinchi turadi
            #[inline]
            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                u64::try_from(value)
                    .map(IdFormat::Number)
                    .map_err(|_| serde::de::Error::custom("ID number must be a non-negative u64"))
            }

            // 2. Unsigned son (u64) - visit_i64 dan keyin keladi
            #[inline]
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(IdFormat::Number(value))
            }

            // 3. Matn (str)
            #[inline]
            fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                IdFormat::parse(value).map_err(serde::de::Error::custom)
            }
        }

        deserializer.deserialize_any(IdFormatVisitor)
    }
}

// ==========================================
// MACRO DEFINITION (JUDA IXCHAM)
// ==========================================

// Har bir yangi ID tipi uchun avtomatik kod yozib beruvchi makro
macro_rules! define_id_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $Name($crate::uuid_types::IdFormat);

        impl $Name {
            /// Satrdan ID obyektini parse qiladi (ajratadi).
            ///
            /// 1. Avval raqamga o'tkazishga (u64) urinadi.
            /// 2. Agar o'xshamasa, UUID sifatida parse qiladi.
            #[inline]
            pub fn parse(value: impl AsRef<str>) -> Result<Self, $crate::error::TypeError> {
                $crate::uuid_types::IdFormat::parse(value).map(Self)
            }

            /// Yangi random (v4) tasodifiy UUID generatsiya qiladi.
            /// Universal bo'lgani bilan tizim default sifatida UUID ishlab chiqaradi.
            #[inline]
            #[must_use]
            pub fn generate() -> Self {
                Self($crate::uuid_types::IdFormat::default())
            }

            /// Obyektni satrga aylantiradi. (Kanonik UUID yoki oddiy raqam)
            #[inline]
            #[must_use]
            pub fn to_string_val(self) -> String {
                self.0.to_string()
            }

            /// Obyekt ichidagi UUIDni qaytaradi (agar u Number bo'lsa `None` qaytadi)
            #[inline]
            #[must_use]
            pub fn as_uuid(&self) -> Option<&uuid::Uuid> {
                match &self.0 {
                    $crate::uuid_types::IdFormat::Uuid(u) => Some(u),
                    _ => None,
                }
            }

            /// Obyekt ichidagi Numberni qaytaradi (agar u Uuid bo'lsa `None` qaytadi)
            #[inline]
            #[must_use]
            pub fn as_number(&self) -> Option<u64> {
                match &self.0 {
                    $crate::uuid_types::IdFormat::Number(n) => Some(*n),
                    _ => None,
                }
            }
        }

        // ==========================================
        // DEFAULT TRAIT IMPLEMENTATSIYALARI
        // ==========================================

        // Print qilish uchun (Display)
        impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        // &str dan xavfsiz o'girish uchun
        impl TryFrom<&str> for $Name {
            type Error = $crate::error::TypeError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        // String dan xavfsiz o'girish uchun
        impl TryFrom<String> for $Name {
            type Error = $crate::error::TypeError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(&value)
            }
        }

        // Tiptan oddiy String yaratish uchun
        impl From<$Name> for String {
            fn from(value: $Name) -> Self {
                value.to_string_val()
            }
        }

        // ==========================================
        // SERDE DELEGATSIYASI (BINARY HAJMINI TEJAYDI)
        // ==========================================

        // JSON ga aylantirishda asosiy enum (IdFormat) dagi mantiqqa yuboradi
        impl serde::Serialize for $Name {
            #[inline]
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                self.0.serialize(serializer)
            }
        }

        // JSON dan o'qishda ham tayyor enum (IdFormat) mantig'iga yuboradi
        impl<'de> serde::Deserialize<'de> for $Name {
            #[inline]
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                $crate::uuid_types::IdFormat::deserialize(deserializer).map(Self)
            }
        }
    };
}

// Macro orqali yangi tiplarni e'lon qilish:

define_id_type! {
    /// Ish (Job) jarayonini identifikatsiya qilish uchun unikal ID (UUID v4).
    pub struct JobId;
}

define_id_type! {
    /// Foydalanuvchi yoki tizim sessiyasini identifikatsiya qilish uchun unikal ID (UUID v4).
    pub struct SessionId;
}

define_id_type! {
    /// Tarmoq so'rovlarini (Request) kuzatish uchun unikal ID (UUID v4).
    pub struct RequestId;
}

define_id_type! {
    /// Secondary so'rovlarini kuzatish uchun unikal ID (UUID v4).
    pub struct Reuid;
}

// ==========================================
// TEST MACRO
// ==========================================

#[cfg(test)]
macro_rules! id_type_tests {
    ($mod_name:ident, $Type:ident) => {
        // Har bir test izolyatsiya qilingan modul ichida bo'ladi
        mod $mod_name {
            use super::*;

            // UUID uchun konstantalar
            const VALID_UUID: &str = "9b7e597e-893e-4e11-92cf-f4e7d4f923b1";
            const VALID_UPPER: &str = "9B7E597E-893E-4E11-92CF-F4E7D4F923B1";
            const VALID_SIMPLE: &str = "9b7e597e893e4e1192cff4e7d4f923b1";
            const INVALID_V1: &str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";

            // Number uchun konstantalar
            const VALID_NUM_STR: &str = "123456789";
            const VALID_NUM_VAL: u64 = 123456789;

            #[test]
            fn valid_uuid_hyphenated_should_parse() {
                let id = $Type::parse(VALID_UUID).unwrap();
                assert!(id.as_uuid().is_some());
                assert_eq!(id.as_number(), None);
            }

            #[test]
            fn valid_number_should_parse() {
                let id = $Type::parse(VALID_NUM_STR).unwrap();
                assert!(id.as_number().is_some());
                assert_eq!(id.as_number().unwrap(), VALID_NUM_VAL);
                assert_eq!(id.as_uuid(), None);
            }

            #[test]
            fn valid_simple_format_should_parse() {
                let id = $Type::parse(VALID_SIMPLE).unwrap();
                assert_eq!(id.to_string_val(), VALID_UUID);
            }

            #[test]
            fn uppercase_should_be_normalized() {
                let id = $Type::parse(VALID_UPPER).unwrap();
                assert_eq!(id.to_string_val(), VALID_UUID);
            }

            #[test]
            fn whitespace_should_be_trimmed() {
                // UUID uchun
                let id_uuid = $Type::parse(format!("  {VALID_UUID}  ")).unwrap();
                assert_eq!(id_uuid.to_string_val(), VALID_UUID);

                // Number uchun
                let id_num = $Type::parse(format!("  {VALID_NUM_STR}  ")).unwrap();
                assert_eq!(id_num.to_string_val(), VALID_NUM_STR);
            }

            #[test]
            fn invalid_formats_should_be_rejected() {
                // Na UUID, na raqam bo'lgan matn
                assert!(matches!(
                    $Type::parse("not-a-uuid-or-number").unwrap_err(),
                    $crate::error::TypeError::Id($crate::uuid_types::IdError::Format)
                ));

                // v4 bo'lmagan UUID
                assert!(matches!(
                    $Type::parse(INVALID_V1).unwrap_err(),
                    $crate::error::TypeError::Id($crate::uuid_types::IdError::Version)
                ));
            }

            #[test]
            fn generated_ids_should_be_unique_and_uuid_by_default() {
                let id1 = $Type::generate();
                let id2 = $Type::generate();

                assert_ne!(id1, id2);
                assert!(id1.as_uuid().is_some()); // Default holatda UUID v4 generatsiya qilinishi kerak
            }

            #[test]
            fn serde_should_support_roundtrip_for_uuid() {
                let id = $Type::parse(VALID_UUID).unwrap();
                let json = serde_json::to_string(&id).unwrap();
                assert_eq!(json, format!("\"{VALID_UUID}\""));

                let back: $Type = serde_json::from_str(&json).unwrap();
                assert_eq!(id, back);
            }

            #[test]
            fn serde_should_support_roundtrip_for_number() {
                let id = $Type::parse(VALID_NUM_STR).unwrap();

                // 1. Serialization tekshiramiz (Number JSON'da qo'shtirnoqsiz bo'lishi kerak)
                let json_num = serde_json::to_string(&id).unwrap();
                assert_eq!(json_num, VALID_NUM_STR);

                // 2. Deserialization tekshiramiz (Int formati kelganda)
                let back_from_int: $Type = serde_json::from_str(&json_num).unwrap();
                assert_eq!(id, back_from_int);

                // 3. Deserialization tekshiramiz (String formati kelganda - misol: "123456789")
                let json_str = format!("\"{VALID_NUM_STR}\"");
                let back_from_str: $Type = serde_json::from_str(&json_str).unwrap();
                assert_eq!(id, back_from_str);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    id_type_tests!(job_id_tests, JobId);
    id_type_tests!(session_id_tests, SessionId);
}
