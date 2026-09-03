use crate::macros::string_newtype;

string_newtype! {
    /// Elektron pochta manzili (`local-part@domain.tld`), lowercase saqlanadi. Faqat ASCII.
    pub struct EmailAddress;
    error = EmailAddressError;
    expecting = "an ASCII email address (local-part@domain.tld)";
}

impl EmailAddress {
    /// RFC 5321: maksimal uzunlik.
    pub const MAX_LEN: usize = 254;
    /// RFC 5321: local-part maksimal uzunligi.
    pub const LOCAL_PART_MAX_LEN: usize = 64;
    /// RFC 1035: domain maksimal uzunligi.
    pub const DOMAIN_MAX_LEN: usize = 253;
    /// RFC 1035: bitta label maksimal uzunligi.
    pub const DOMAIN_LABEL_MAX_LEN: usize = 63;
    /// TLD minimal uzunligi.
    pub const TLD_MIN_LEN: usize = 2;

    fn normalize(s: &mut str) {
        s.make_ascii_lowercase();
    }

    fn validate(s: &str) -> Result<(), EmailAddressError> {
        if s.len() > Self::MAX_LEN {
            return Err(EmailAddressError::Length);
        }
        if !s.is_ascii() || s.contains(char::is_whitespace) {
            return Err(EmailAddressError::Format);
        }
        let Some((local, domain)) = s.split_once('@') else {
            return Err(EmailAddressError::Format);
        };
        if domain.contains('@') {
            return Err(EmailAddressError::Format);
        }
        Self::validate_local_part(local)?;
        Self::validate_domain(domain)
    }

    const fn is_allowed_local_byte(b: u8) -> bool {
        b.is_ascii_alphanumeric()
            || matches!(
                b,
                b'.' | b'!'
                    | b'#'
                    | b'$'
                    | b'%'
                    | b'&'
                    | b'\''
                    | b'*'
                    | b'+'
                    | b'-'
                    | b'/'
                    | b'='
                    | b'?'
                    | b'^'
                    | b'_'
                    | b'`'
                    | b'{'
                    | b'|'
                    | b'}'
                    | b'~'
            )
    }

    fn validate_local_part(local: &str) -> Result<(), EmailAddressError> {
        if local.is_empty()
            || local.len() > Self::LOCAL_PART_MAX_LEN
            || local.starts_with('.')
            || local.ends_with('.')
            || local.contains("..")
            || !local.bytes().all(Self::is_allowed_local_byte)
        {
            return Err(EmailAddressError::Format);
        }
        Ok(())
    }

    fn validate_domain(domain: &str) -> Result<(), EmailAddressError> {
        if domain.is_empty() || domain.len() > Self::DOMAIN_MAX_LEN {
            return Err(EmailAddressError::Format);
        }
        let mut labels = 0usize;
        let mut tld = "";
        for label in domain.split('.') {
            if label.is_empty()
                || label.len() > Self::DOMAIN_LABEL_MAX_LEN
                || label.starts_with('-')
                || label.ends_with('-')
                || !label
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-')
            {
                return Err(EmailAddressError::Format);
            }
            labels += 1;
            tld = label;
        }
        if labels < 2
            || tld.len() < Self::TLD_MIN_LEN
            || !tld.bytes().all(|b| b.is_ascii_alphabetic())
        {
            return Err(EmailAddressError::Format);
        }
        Ok(())
    }

    /// `@` dan oldingi qism. Panic yo'li yo'q (`expect` o'rniga `split_once`).
    #[inline]
    #[must_use]
    pub fn local_part(&self) -> &str {
        self.0.split_once('@').map_or(self.0.as_str(), |(l, _)| l)
    }

    /// `@` dan keyingi qism.
    #[inline]
    #[must_use]
    pub fn domain(&self) -> &str {
        self.0.split_once('@').map_or("", |(_, d)| d)
    }
}

/// `EmailAddress` validatsiya xatolari.
#[derive(Debug, Clone, Copy, PartialEq, Eq, thiserror::Error)]
#[non_exhaustive]
pub enum EmailAddressError {
    /// 254 belgidan uzun.
    #[error("email is too long, maximum is {} characters", EmailAddress::MAX_LEN)]
    Length,
    /// Format noto'g'ri.
    #[error("email format is invalid, expected valid local-part and domain")]
    Format,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_and_accessors() {
        let e = EmailAddress::parse("  User.Name@Example.COM ").unwrap();
        assert_eq!(e.as_str(), "user.name@example.com");
        assert_eq!(e.local_part(), "user.name");
        assert_eq!(e.domain(), "example.com");
        assert!(EmailAddress::parse("a@b.co").is_ok());
        assert_eq!(EmailAddress::parse("a@b.c"), Err(EmailAddressError::Format));
        assert_eq!(EmailAddress::parse("a@b"), Err(EmailAddressError::Format));
    }
}
