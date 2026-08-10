// ==========================================
// UUID XATOLIKLARI
// ==========================================

/// UUID asosidagi tiplar uchun validatsiya xatoliklari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum IdError {
    /// ID formati noto'g'ri (na UUID, na Number).
    #[error("invalid ID format (must be UUID or a valid number)")]
    Format,

    /// UUID versiyasi mos kelmaydi (faqat v4 qabul qilinadi).
    #[error("UUID must be version 4 (Random)")]
    Version,
}

// ==========================================
// UNIVERSAL ID FORMAT ENUM
// ==========================================

/// ID ning ichki ma'lumot formati.
/// Universal identifikatorlar: yozuv (UUID) yoki raqamli (u64) bo'lishi mumkin.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IdFormat {
    Uuid(uuid::Uuid),
    Number(u64),
}

impl Default for IdFormat {
    /// Default sifatida har doim yangi UUID v4 yaratiladi.
    fn default() -> Self {
        Self::Uuid(uuid::Uuid::new_v4())
    }
}

impl std::fmt::Display for IdFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Uuid(u) => write!(f, "{}", u.as_hyphenated()),
            Self::Number(n) => write!(f, "{}", n),
        }
    }
}
// ==========================================
// MACRO DEFINITION
// ==========================================

/// Universal (UUID yoki Number) asosidagi strongly-typed wrapper yaratish uchun macro.
macro_rules! define_id_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $Name(pub $crate::uuid_types::IdFormat);

        impl $Name {
            /// Satrdan ID obyektini parse qiladi (ajratadi).
            ///
            /// 1. Avval raqamga o'tkazishga (u64) urinadi.
            /// 2. Agar o'xshamasa, UUID sifatida parse qiladi.
            pub fn parse(value: impl AsRef<str>) -> Result<Self, $crate::error::TypeError> {
                let raw = value.as_ref().trim();

                // 1-qadam: Raqam ekanligini tekshiramiz (tezroq ishlaydi)
                if let Ok(num) = raw.parse::<u64>() {
                    return Ok(Self($crate::uuid_types::IdFormat::Number(num)));
                }

                // 2-qadam: UUID ekanligini tekshiramiz
                let parsed = uuid::Uuid::parse_str(raw)
                    .map_err(|_| $crate::error::TypeError::IdError($crate::uuid_types::IdError::Format))?;

                if parsed.get_version() != Some(uuid::Version::Random) {
                    return Err($crate::error::TypeError::IdError($crate::uuid_types::IdError::Version));
                }

                Ok(Self($crate::uuid_types::IdFormat::Uuid(parsed)))
            }

            /// Yangi random (v4) tasodifiy UUID generatsiya qiladi.
            /// Universal bo'lgani bilan tizim default sifatida UUID ishlab chiqaradi.
            #[inline]
            pub fn generate() -> Self {
                Self($crate::uuid_types::IdFormat::default())
            }

            /// Obyektni satrga aylantiradi. (Kanonik UUID yoki oddiy raqam)
            #[inline]
            pub fn to_string_val(self) -> String {
                self.0.to_string()
            }

            /// Obyekt ichidagi UUIDni qaytaradi (agar u Number bo'lsa `None` qaytadi)
            #[inline]
            pub fn as_uuid(&self) -> Option<&uuid::Uuid> {
                match &self.0 {
                    $crate::uuid_types::IdFormat::Uuid(u) => Some(u),
                    _ => None,
                }
            }

            /// Obyekt ichidagi Numberni qaytaradi (agar u Uuid bo'lsa `None` qaytadi)
            #[inline]
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

        impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }

        impl TryFrom<&str> for $Name {
            type Error = $crate::error::TypeError;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        impl TryFrom<String> for $Name {
            type Error = $crate::error::TypeError;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(&value)
            }
        }

        impl From<$Name> for String {
            fn from(value: $Name) -> Self {
                value.to_string_val()
            }
        }

        // ==========================================
        // SERDE OPTIMIZATSIYASI (ZERO-ALLOCATION)
        // ==========================================

        impl serde::Serialize for $Name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                match &self.0 {
                    $crate::uuid_types::IdFormat::Uuid(u) => {
                        let mut buffer = uuid::Uuid::encode_buffer();
                        let s = u.as_hyphenated().encode_lower(&mut buffer);
                        serializer.serialize_str(s)
                    }
                    $crate::uuid_types::IdFormat::Number(n) => {
                        // Raqamlarni JSON'ga int formatda saqlaymiz
                        serializer.serialize_u64(*n)
                    }
                }
            }
        }

        #[allow(unknown_lints)]
        impl<'de> serde::Deserialize<'de> for $Name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                // Turli xil JSON tiplarni (String yoki Int) allocation'siz ushlash uchun maxsus Visitor
                struct IdVisitor;

                impl<'de> serde::de::Visitor<'de> for IdVisitor {
                    type Value = $crate::uuid_types::IdFormat;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("a UUID string or a number")
                    }

                    fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                        Ok($crate::uuid_types::IdFormat::Number(value))
                    }

                    fn visit_str<E: serde::de::Error>(self, value: &str) -> Result<Self::Value, E> {
                        if let Ok(num) = value.parse::<u64>() {
                            Ok($crate::uuid_types::IdFormat::Number(num))
                        } else if let Ok(u) = uuid::Uuid::parse_str(value) {
                            if u.get_version() == Some(uuid::Version::Random) {
                                Ok($crate::uuid_types::IdFormat::Uuid(u))
                            } else {
                                Err(serde::de::Error::custom("UUID must be version 4"))
                            }
                        } else {
                            Err(serde::de::Error::custom("invalid ID format"))
                        }
                    }
                }

                deserializer.deserialize_any(IdVisitor).map(Self)
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
                    $crate::error::TypeError::IdError($crate::uuid_types::IdError::Format)
                ));

                // v4 bo'lmagan UUID
                assert!(matches!(
                    $Type::parse(INVALID_V1).unwrap_err(),
                    $crate::error::TypeError::IdError($crate::uuid_types::IdError::Version)
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
