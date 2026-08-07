// ==========================================
// UUID XATOLIKLARI
// ==========================================

/// UUID asosidagi tiplar uchun validatsiya xatoliklari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum UuidError {
    /// UUID formati noto'g'ri (masalan: belgilar kam, noto'g'ri simvollar).
    #[error("invalid UUID format")]
    Format,

    /// UUID versiyasi mos kelmaydi (faqat v4 qabul qilinadi).
    #[error("UUID must be version 4 (Random)")]
    Version,
}

// ==========================================
// MACRO DEFINITION
// ==========================================

/// UUID v4 asosidagi strongly-typed wrapper (newtype) yaratish uchun macro.
///
/// Bu macro orqali yaratilgan strukturalar:
/// - Xotira samarador (faqat 16 bayt stack'da saqlanadi, `String` allocation yo'q).
/// - Avtomatik `Deref` orqali `uuid::Uuid` metodlariga to'g'ridan-to'g'ri kirish.
/// - Serilizatsiya va deserilizatsiya jarayonida **zero-allocation** (xotira ajratilmaydi).
/// - `TryFrom<&str>`, `TryFrom<String>`, `Display` kabi standart traitlarga ega bo'ladi.
macro_rules! define_uuid_type {
    (
        $(#[$meta:meta])*
        $vis:vis struct $Name:ident;
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $Name(uuid::Uuid);

        impl $Name {
            /// Satrdan UUID obyektini parse qiladi (ajratadi).
            ///
            /// - Boshidagi va oxiridagi bo'sh joylarni avtomatik qirqadi (`trim`).
            /// - Hyphenated, simple, va uppercase formatlarni qabul qiladi.
            /// - Faqat **UUID v4 (Random)** versiyasini qabul qiladi, boshqalari xatolik beradi.
            pub fn parse(value: impl AsRef<str>) -> Result<Self, $crate::error::TypeError> {
                let raw = value.as_ref().trim();

                // Satrdan UUID yig'ish (zero-allocation parse)
                let parsed = uuid::Uuid::parse_str(raw).map_err(|_| $crate::error::TypeError::Uuid($crate::uuid_types::UuidError::Format))?;

                // Faqat v4 (randomly generated) ekanligini tasdiqlash
                if parsed.get_version() != Some(uuid::Version::Random) {
                    return Err($crate::error::TypeError::Uuid($crate::uuid_types::UuidError::Version));
                }

                Ok(Self(parsed))
            }

            /// Yangi random (v4) tasodifiy UUID generatsiya qiladi.
            #[inline]
            pub fn generate() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            /// UUID ni kanonik "hyphenated" (chiziqchali) formatdagi satr qilib qaytaradi.
            /// Misol: `"9b7e597e-893e-4e11-92cf-f4e7d4f923b1"`.
            ///
            /// **Ogohlantirish**: Bu metod heap xotiradan (String) joy oladi!
            /// Obyektlarni solishtirish uchun to'g'ridan-to'g'ri `==` operatoridan foydalaning.
            #[inline]
            pub fn to_hyphenated_string(self) -> String {
                self.0.as_hyphenated().to_string()
            }

            /// Obyekt ichidagi toza `uuid::Uuid` reference'ini qaytaradi.
            #[inline]
            pub fn as_uuid(&self) -> &uuid::Uuid {
                &self.0
            }

            /// Ichki `uuid::Uuid` ni qaytaradi (`Copy` bo'lgani uchun ownership ko'chadi).
            #[inline]
            pub fn into_inner(self) -> uuid::Uuid {
                self.0
            }
        }

        // ==========================================
        // DEFAULT TRAIT IMPLEMENTATSIYALARI
        // ==========================================

        /// Obyektni to'g'ridan-to'g'ri kanonik formatda chiqaradi.
        impl std::fmt::Display for $Name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0.as_hyphenated())
            }
        }

        /// `Deref` tufayli ichki `uuid::Uuid` funksiyalariga (masalan: `is_nil()`) to'g'ridan to'g'ri ulanish imkoniyati.
        impl std::ops::Deref for $Name {
            type Target = uuid::Uuid;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        /// Satr bo'lagidan (`&str`) UUID v4 obyektini xavfsiz ajratib olish (parse) imkonini beradi.
        impl TryFrom<&str> for $Name {
            type Error = $crate::error::TypeError;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::parse(value)
            }
        }

        /// O'zgaruvchan satrdan (`String`) UUID v4 obyektini xavfsiz ajratib olish imkonini beradi.
        impl TryFrom<String> for $Name {
            type Error = $crate::error::TypeError;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::parse(&value) // String klonlanmasdan reference uzatiladi
            }
        }

        /// UUID obyektini avtomatik ravishda chiziqchali (hyphenated) `String` satriga aylantiradi.
        impl From<$Name> for String {
            fn from(value: $Name) -> Self {
                value.to_hyphenated_string()
            }
        }

        // ==========================================
        // SERDE OPTIMIZATSIYASI (ZERO-ALLOCATION)
        // ==========================================

        impl serde::Serialize for $Name {
            fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
                // String allocation qilmasdan, uuid ni lokal stack buffer'ga yozamiz
                let mut buffer = uuid::Uuid::encode_buffer();
                let s = self.0.as_hyphenated().encode_lower(&mut buffer);
                serializer.serialize_str(s)
            }
        }

        #[allow(unknown_lints)]
        impl<'de> serde::Deserialize<'de> for $Name {
            fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
                // String'ga o'zgartirmasdan, imkon boricha zero-copy (&str) qabul qilamiz
                let s = <&str>::deserialize(deserializer)?;
                Self::parse(s).map_err(serde::de::Error::custom)
            }
        }
    };
}

// Macro orqali yangi tiplarni e'lon qilish:

define_uuid_type! {
    /// Ish (Job) jarayonini identifikatsiya qilish uchun unikal ID (UUID v4).
    pub struct JobId;
}

define_uuid_type! {
    /// Foydalanuvchi yoki tizim sessiyasini identifikatsiya qilish uchun unikal ID (UUID v4).
    pub struct SessionId;
}

define_uuid_type! {
    /// Tarmoq so'rovlarini (Request) kuzatish uchun unikal ID (UUID v4).
    pub struct RequestId;
}

define_uuid_type! {
    /// Secondary so'rovlarini kuzatish uchun unikal ID (UUID v4).
    pub struct Reuid;
}

// ==========================================
// TEST MACRO
// ==========================================

#[cfg(test)]
macro_rules! uuid_type_tests {
    // Xato to'g'rilandi: Endi makro modul nomini ($mod_name) ham qabul qiladi
    ($mod_name:ident, $Type:ident) => {
        // Har bir test izolyatsiya qilingan modul ichida bo'ladi
        mod $mod_name {
            use super::*;

            const VALID: &str = "9b7e597e-893e-4e11-92cf-f4e7d4f923b1";
            const VALID_UPPER: &str = "9B7E597E-893E-4E11-92CF-F4E7D4F923B1";
            const VALID_SIMPLE: &str = "9b7e597e893e4e1192cff4e7d4f923b1";
            const INVALID_V1: &str = "6ba7b810-9dad-11d1-80b4-00c04fd430c8";

            #[test]
            fn valid_hyphenated_should_parse() {
                assert!($Type::parse(VALID).is_ok());
            }

            #[test]
            fn valid_simple_format_should_parse() {
                let id = $Type::parse(VALID_SIMPLE).unwrap();
                assert_eq!(id.to_hyphenated_string(), VALID);
            }

            #[test]
            fn uppercase_should_be_normalized() {
                let id = $Type::parse(VALID_UPPER).unwrap();
                assert_eq!(id.to_hyphenated_string(), VALID);
            }

            #[test]
            fn whitespace_should_be_trimmed() {
                let id = $Type::parse(format!("  {VALID}  ")).unwrap();
                assert_eq!(id.to_hyphenated_string(), VALID);
            }

            #[test]
            fn invalid_uuid_should_be_rejected() {
                assert!(matches!(
                    $Type::parse("not-a-uuid").unwrap_err(),
                    $crate::error::TypeError::Uuid($crate::uuid_types::UuidError::Format)
                ));

                assert!(matches!(
                    $Type::parse(INVALID_V1).unwrap_err(),
                    $crate::error::TypeError::Uuid($crate::uuid_types::UuidError::Version)
                ));
            }

            #[test]
            fn generated_uuids_should_be_unique() {
                assert_ne!($Type::generate(), $Type::generate());
            }

            #[test]
            fn serde_should_support_roundtrip() {
                let id = $Type::parse(VALID).unwrap();
                let json = serde_json::to_string(&id).unwrap();
                assert_eq!(json, format!("\"{VALID}\""));

                let back: $Type = serde_json::from_str(&json).unwrap();
                assert_eq!(id, back);
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    // Xato to'g'rilandi: Testlar nomlar to'qnashmasligi uchun turli modullarga o'raldi
    uuid_type_tests!(job_id_tests, JobId);
    uuid_type_tests!(session_id_tests, SessionId);
    uuid_type_tests!(request_id_tests, RequestId);
}
