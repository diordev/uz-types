use crate::error::TypeError;

/// UUID va Number formatlari uchun xatoliklar ro'yxati.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum IdError {
    /// Format UUID ham, Number ham bo'lmaganda chiqadigan xato.
    #[error("invalid ID format (must be UUID or a valid number)")]
    Format,

    /// UUID versiyasi qo'llab-quvvatlanmaganda chiqadigan xato.
    #[error("UUID must be version 4 (Random) or version 7 (sortable/time-ordered)")]
    Version,
}

// ==========================================
// UNIVERSAL ID FORMAT ENUM
// ==========================================

/// ID ma'lumotlarini o'zida saqlovchi yagona enum (crate ichida yashiringan).
///
/// # ⚠️ Serde: bitta tip — ikki xil JSON shakli
///
/// Ichkarida qaysi variant saqlanganiga qarab JSON turlicha chiqadi:
///
/// | Ichki qiymat | JSON natijasi                              |
/// |--------------|--------------------------------------------|
/// | `Uuid`       | `"9b7e597e-893e-4e11-92cf-f4e7d4f923b1"`   |
/// | `Number`     | `123456789` (qo'shtirnoqsiz, integer)      |
///
/// Deserializatsiyada ikkalasi ham (va raqamning string ko'rinishi ham)
/// qabul qilinadi.
///
/// Buning amaliy oqibatlari:
/// - **OpenAPI/JSON Schema**: maydon `oneOf: [string, integer]` bo'ladi,
///   qat'iy bitta tip emas;
/// - **Ma'lumotlar bazasi**: bitta ustunga ba'zan matn, ba'zan son tushadi;
/// - **Boshqa tildagi klient**: `typeof id` barqaror bo'lmaydi.
///
/// Agar API'ingizda bitta barqaror shakl kerak bo'lsa, loyihaning
/// chegarasida faqat bitta variantdan foydalaning (masalan, hamma joyda
/// `generate_v7()` — u har doim string beradi).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum IdFormat {
    Uuid(uuid::Uuid),
    Number(u64),
}

impl IdFormat {
    /// Faqat shu ikki versiya qabul qilinadi: v4 (tasodifiy) va v7
    /// (vaqt-tartiblangan, DB indekslash uchun qulay, zamonaviy standart).
    /// v1/v2/v3/v5/v6 rad etiladi — v1/v6 MAC-manzilni oshkor qiladi,
    /// v3/v5 esa nom-asosli (deterministik) hash, tasodifiy ID emas.
    const ALLOWED_VERSIONS: [uuid::Version; 2] = [uuid::Version::Random, uuid::Version::SortRand];

    /// Matnni qabul qilib, Number yoki UUID'ga xatosiz o'giruvchi yagona funksiya.
    pub fn parse(value: impl AsRef<str>) -> Result<Self, TypeError> {
        let raw = value.as_ref().trim();

        if let Ok(num) = raw.parse::<u64>() {
            return Ok(Self::Number(num));
        }

        let parsed = uuid::Uuid::parse_str(raw).map_err(|_| TypeError::Id(IdError::Format))?;

        let version = parsed.get_version();
        if !Self::ALLOWED_VERSIONS.iter().any(|v| Some(*v) == version) {
            return Err(TypeError::Id(IdError::Version));
        }

        Ok(Self::Uuid(parsed))
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

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a UUID (v4 or v7) string or an integer")
            }

            #[inline]
            fn visit_i64<E: serde::de::Error>(self, value: i64) -> Result<Self::Value, E> {
                u64::try_from(value)
                    .map(IdFormat::Number)
                    .map_err(|_| serde::de::Error::custom("ID number must be a non-negative u64"))
            }

            #[inline]
            fn visit_u64<E: serde::de::Error>(self, value: u64) -> Result<Self::Value, E> {
                Ok(IdFormat::Number(value))
            }

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

/// Har bir yangi ID tipi uchun avtomatik kod yozib beruvchi makro.
macro_rules! define_id_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $Name($crate::uuid_types::IdFormat);

        impl $Name {
            /// Satrdan ID obyektini parse qiladi.
            ///
            /// 1. Avval raqamga (u64) o'tkazishga urinadi.
            /// 2. Aks holda UUID (v4 yoki v7) sifatida parse qiladi.
            #[inline]
            pub fn parse(value: impl AsRef<str>) -> Result<Self, $crate::error::TypeError> {
                $crate::uuid_types::IdFormat::parse(value).map(Self)
            }

            /// Yangi tasodifiy (v4) UUID generatsiya qiladi.
            #[inline]
            #[must_use]
            pub fn generate() -> Self {
                Self($crate::uuid_types::IdFormat::Uuid(uuid::Uuid::new_v4()))
            }

            /// Yangi vaqt-tartiblangan (v7) UUID generatsiya qiladi.
            ///
            /// v7 qiymatlar yaratilish vaqti bo'yicha tartiblanadi — bu
            /// ma'lumotlar bazasida B-tree indeks fragmentatsiyasini kamaytiradi
            /// va primary key sifatida v4 dan afzalroq.
            #[inline]
            #[must_use]
            pub fn generate_v7() -> Self {
                Self($crate::uuid_types::IdFormat::Uuid(uuid::Uuid::now_v7()))
            }

            /// Ichki qiymat UUID bo'lsa, uning versiyasini qaytaradi (v4 yoki v7).
            #[inline]
            #[must_use]
            pub fn uuid_version(&self) -> Option<uuid::Version> {
                self.as_uuid().and_then(uuid::Uuid::get_version)
            }

            /// Obyekt ichidagi UUIDni qaytaradi (agar u Number bo'lsa `None`).
            #[inline]
            #[must_use]
            pub fn as_uuid(&self) -> Option<&uuid::Uuid> {
                match &self.0 {
                    $crate::uuid_types::IdFormat::Uuid(u) => Some(u),
                    _ => None,
                }
            }

            /// Obyekt ichidagi Numberni qaytaradi (agar u Uuid bo'lsa `None`).
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

        // std avtomatik `.to_string()`ni bepul beradi.
         impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        // Rust'ning standart idiomatik parse uslubi ("text".parse() uchun).
        impl std::str::FromStr for $Name {
            type Err = TypeError;

            // Rust'ning standart parse mantig'iga o'zimizning parse'ni ulab qo'yamiz
            #[inline]
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::parse(s)
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
                value.to_string()
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
    /// Ish (Job) jarayonini identifikatsiya qilish uchun unikal ID (UUID v4/v7).
    pub struct JobId;
}

define_id_type! {
    /// Foydalanuvchi yoki tizim sessiyasini identifikatsiya qilish uchun unikal ID (UUID v4/v7).
    pub struct SessionId;
}

define_id_type! {
    /// Tarmoq so'rovlarini (Request) kuzatish uchun unikal ID (UUID v4/v7).
    pub struct RequestId;
}

define_id_type! {
    /// Secondary so'rovlarini kuzatish uchun unikal ID (UUID v4/v7).
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
            const VALID_V7: &str = "01912d68-783e-7c1f-bcf6-9a5b4c3d2e1f";
            const INVALID_V1: &str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";
            // v3 (nom-asosli MD5) va v5 (nom-asosli SHA-1) ham rad etilishi kerak
            const INVALID_V3: &str = "3d813cbb-47fb-32ba-91df-831e1593ac29";
            const INVALID_V5: &str = "21f7f8de-8051-5b89-8680-0195ef798b6a";

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
                assert_eq!(id.to_string(), VALID_UUID);
            }

            #[test]
            fn uppercase_should_be_normalized() {
                let id = $Type::parse(VALID_UPPER).unwrap();
                assert_eq!(id.to_string(), VALID_UUID);
            }

            #[test]
            fn whitespace_should_be_trimmed() {
                // UUID uchun
                let id_uuid = $Type::parse(format!("  {VALID_UUID}  ")).unwrap();
                assert_eq!(id_uuid.to_string(), VALID_UUID);

                // Number uchun
                let id_num = $Type::parse(format!("  {VALID_NUM_STR}  ")).unwrap();
                assert_eq!(id_num.to_string(), VALID_NUM_STR);
            }

            #[test]
            fn valid_v7_uuid_should_parse() {
                let id = $Type::parse(VALID_V7).unwrap();
                assert_eq!(id.uuid_version(), Some(uuid::Version::SortRand));
                assert_eq!(id.to_string(), VALID_V7);
            }

            #[test]
            fn invalid_formats_should_be_rejected() {
                // Na UUID, na raqam bo'lgan matn
                assert!(matches!(
                    $Type::parse("not-a-uuid-or-number").unwrap_err(),
                    $crate::error::TypeError::Id($crate::uuid_types::IdError::Format)
                ));

                // v4/v7 bo'lmagan UUID versiyalari rad etilishi kerak
                for invalid in [INVALID_V1, INVALID_V3, INVALID_V5] {
                    assert!(
                        matches!(
                            $Type::parse(invalid).unwrap_err(),
                            $crate::error::TypeError::Id($crate::uuid_types::IdError::Version)
                        ),
                        "{invalid} rad etilishi kerak edi"
                    );
                }
            }

            #[test]
            fn generated_ids_should_be_unique_and_uuid_by_default() {
                let id1 = $Type::generate();
                let id2 = $Type::generate();

                assert_ne!(id1, id2);
                assert!(id1.as_uuid().is_some()); // Default holatda UUID v4 generatsiya qilinishi kerak
                assert_eq!(id1.uuid_version(), Some(uuid::Version::Random));
            }

            #[test]
            fn generate_v7_should_produce_sortable_uuid() {
                let id1 = $Type::generate_v7();
                let id2 = $Type::generate_v7();

                assert_ne!(id1, id2);
                assert_eq!(id1.uuid_version(), Some(uuid::Version::SortRand));

                // v7 vaqt bo'yicha tartiblangan — matn ko'rinishi ham o'sib boradi
                assert!(id1.to_string() <= id2.to_string());

                // O'zi generatsiya qilgan qiymat qayta parse bo'lishi shart
                assert_eq!($Type::parse(id1.to_string()).unwrap(), id1);
            }

            #[test]
            fn number_ids_have_no_uuid_version() {
                let id = $Type::parse(VALID_NUM_STR).unwrap();
                assert_eq!(id.uuid_version(), None);
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
    id_type_tests!(request_id_tests, RequestId);
    id_type_tests!(reuid_tests, Reuid);
}
