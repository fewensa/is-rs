use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;

// ---------------------------------------------------------------------------
// Compiled regex patterns (compiled once, reused on every call)
// Patterns are ported from is.js source to preserve identical behaviour.
// Where the original uses lookahead (unsupported by the `regex` crate), the
// validation is implemented in plain Rust code instead.
// ---------------------------------------------------------------------------

static RE_URL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(https?|ftp)://[^\s/$.?#].[^\s]*$").unwrap());

static RE_EMAIL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}$").unwrap());

static RE_CREDIT_CARD: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:4[0-9]{12}(?:[0-9]{3})?|5[1-5][0-9]{14}|6(?:011|5[0-9]{2})[0-9]{12}|3[47][0-9]{13}|3(?:0[0-5]|[68][0-9])[0-9]{11}|(?:2131|1800|35[0-9]{3})[0-9]{11})$",
    )
    .unwrap()
});

static RE_ALPHA_NUMERIC: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());

static RE_TIME_STRING: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(2[0-3]|[01]?[0-9]):([0-5]?[0-9]):([0-5]?[0-9])$").unwrap());

static RE_DATE_STRING: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(1[0-2]|0?[1-9])/(3[01]|[12][0-9]|0?[1-9])/(?:[0-9]{2})?[0-9]{2}$").unwrap()
});

static RE_US_ZIP_CODE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\d{5}(?:[-\s]\d{4})?$").unwrap());

// CA postal code format: A0A 0A0 (no lookahead needed if we validate chars manually)
static RE_CA_POSTAL_CODE_BASE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([A-Z])([0-9])([A-Z])\s?([0-9])([A-Z])([0-9])$").unwrap());

static RE_UK_POST_CODE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[A-Z]{1,2}[0-9RCHNQ][0-9A-Z]?\s?[0-9][ABD-HJLNP-UW-Z]{2}$|^[A-Z]{2}-?[0-9]{4}$")
        .unwrap()
});

static RE_NANP_PHONE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\(?([0-9]{3})\)?[-. ]?([0-9]{3})[-. ]?([0-9]{4})$").unwrap());

static RE_EPP_PHONE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\+[0-9]{1,3}\.[0-9]{4,14}(?:x.+)?$").unwrap());

// SSN base format: DDD-DD-DDDD  (lookahead exclusions handled in code)
static RE_SSN_BASE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^([0-9]{3})-([0-9]{2})-([0-9]{4})$").unwrap());

static RE_AFFIRMATIVE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:1|t(?:rue)?|y(?:es)?|ok(?:ay)?)$").unwrap());

static RE_HEXADECIMAL: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[0-9a-fA-F]+$").unwrap());

static RE_HEX_COLOR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^#?(?:[0-9a-fA-F]{3}|[0-9a-fA-F]{6})$").unwrap());

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/// Returns `true` if `s` is a valid URL (http, https, or ftp scheme).
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_url;
/// assert!(is_url("https://example.com"));
/// assert!(!is_url("not-a-url"));
/// ```
pub fn is_url(s: &str) -> bool {
    RE_URL.is_match(s)
}

/// Returns `true` if `s` is a valid email address.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_email;
/// assert!(is_email("user@example.com"));
/// assert!(!is_email("not-an-email"));
/// ```
pub fn is_email(s: &str) -> bool {
    RE_EMAIL.is_match(s)
}

/// Returns `true` if `s` is a valid credit card number (major card types).
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_credit_card;
/// assert!(is_credit_card("4111111111111111")); // Visa test number
/// assert!(!is_credit_card("1234567890123456"));
/// ```
pub fn is_credit_card(s: &str) -> bool {
    RE_CREDIT_CARD.is_match(s)
}

/// Returns `true` if `s` contains only alphanumeric characters (a-z, A-Z, 0-9).
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_alpha_numeric;
/// assert!(is_alpha_numeric("abc123"));
/// assert!(!is_alpha_numeric("abc 123"));
/// ```
pub fn is_alpha_numeric(s: &str) -> bool {
    RE_ALPHA_NUMERIC.is_match(s)
}

/// Returns `true` if `s` is a valid time string in `HH:MM:SS` format.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_time_string;
/// assert!(is_time_string("13:45:30"));
/// assert!(!is_time_string("25:00:00"));
/// ```
pub fn is_time_string(s: &str) -> bool {
    RE_TIME_STRING.is_match(s)
}

/// Returns `true` if `s` is a valid date string in `MM/DD/YYYY` format.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_date_string;
/// assert!(is_date_string("12/25/2023"));
/// assert!(!is_date_string("2023-12-25"));
/// ```
pub fn is_date_string(s: &str) -> bool {
    RE_DATE_STRING.is_match(s)
}

/// Returns `true` if `s` is a valid US ZIP code (5-digit or ZIP+4).
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_us_zip_code;
/// assert!(is_us_zip_code("12345"));
/// assert!(is_us_zip_code("12345-6789"));
/// assert!(!is_us_zip_code("1234"));
/// ```
pub fn is_us_zip_code(s: &str) -> bool {
    RE_US_ZIP_CODE.is_match(s)
}

/// Returns `true` if `s` is a valid Canadian postal code.
///
/// The invalid letters D, F, I, O, Q, U are excluded from the first character
/// per Canada Post rules.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_ca_postal_code;
/// assert!(is_ca_postal_code("K1A 0A9"));
/// assert!(!is_ca_postal_code("12345"));
/// ```
pub fn is_ca_postal_code(s: &str) -> bool {
    // Canada Post prohibits D, F, I, O, Q, U in the forward sortation area (first letter).
    const INVALID_FSA_CHARS: &[char] = &['D', 'F', 'I', 'O', 'Q', 'U'];
    match RE_CA_POSTAL_CODE_BASE.captures(s) {
        Some(caps) => {
            let first = caps[1].chars().next().unwrap_or('\0');
            !INVALID_FSA_CHARS.contains(&first)
        }
        None => false,
    }
}

/// Returns `true` if `s` is a valid UK postcode.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_uk_post_code;
/// assert!(is_uk_post_code("SW1A 1AA"));
/// assert!(!is_uk_post_code("12345"));
/// ```
pub fn is_uk_post_code(s: &str) -> bool {
    RE_UK_POST_CODE.is_match(s)
}

/// Returns `true` if `s` is a valid NANP (North American Numbering Plan) phone number.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_nanp_phone;
/// assert!(is_nanp_phone("(555) 555-5555"));
/// assert!(is_nanp_phone("555-555-5555"));
/// assert!(!is_nanp_phone("not-a-phone"));
/// ```
pub fn is_nanp_phone(s: &str) -> bool {
    RE_NANP_PHONE.is_match(s)
}

/// Returns `true` if `s` is a valid EPP (Extensible Provisioning Protocol) phone number.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_epp_phone;
/// assert!(is_epp_phone("+1.5555555555"));
/// assert!(!is_epp_phone("555-555-5555"));
/// ```
pub fn is_epp_phone(s: &str) -> bool {
    RE_EPP_PHONE.is_match(s)
}

/// Returns `true` if `s` is a valid US Social Security Number.
///
/// Rejects invalid area numbers: 000, 666, and 900–999.
/// Rejects invalid group numbers: 00.
/// Rejects invalid serial numbers: 0000.
/// Also rejects the specific invalid SSNs 219-09-9999 and 078-05-1120.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_social_security_number;
/// assert!(is_social_security_number("123-45-6789"));
/// assert!(!is_social_security_number("000-00-0000"));
/// ```
pub fn is_social_security_number(s: &str) -> bool {
    let caps = match RE_SSN_BASE.captures(s) {
        Some(c) => c,
        None => return false,
    };

    let area: u32 = caps[1].parse().unwrap_or(0);
    let group: u32 = caps[2].parse().unwrap_or(0);
    let serial: u32 = caps[3].parse().unwrap_or(0);

    // Reject specific invalid SSNs advertised in media
    if s == "219-09-9999" || s == "078-05-1120" {
        return false;
    }

    // area: not 000, 666, or 900-999
    if area == 0 || area == 666 || area >= 900 {
        return false;
    }

    // group: not 00
    if group == 0 {
        return false;
    }

    // serial: not 0000
    if serial == 0 {
        return false;
    }

    true
}

/// Returns `true` if `s` represents an affirmative value (`1`, `true`, `yes`, `ok`, `okay`,
/// case-insensitive).
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_affirmative;
/// assert!(is_affirmative("yes"));
/// assert!(is_affirmative("YES"));
/// assert!(is_affirmative("1"));
/// assert!(!is_affirmative("no"));
/// ```
pub fn is_affirmative(s: &str) -> bool {
    RE_AFFIRMATIVE.is_match(s)
}

/// Returns `true` if `s` is a valid hexadecimal string.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_hexadecimal;
/// assert!(is_hexadecimal("deadbeef"));
/// assert!(is_hexadecimal("DEADBEEF"));
/// assert!(!is_hexadecimal("xyz"));
/// ```
pub fn is_hexadecimal(s: &str) -> bool {
    RE_HEXADECIMAL.is_match(s)
}

/// Returns `true` if `s` is a valid CSS hex color (3 or 6 hex digits, optional `#` prefix).
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_hex_color;
/// assert!(is_hex_color("#fff"));
/// assert!(is_hex_color("#aabbcc"));
/// assert!(is_hex_color("aabbcc"));
/// assert!(!is_hex_color("#gg0000"));
/// ```
pub fn is_hex_color(s: &str) -> bool {
    RE_HEX_COLOR.is_match(s)
}

/// Returns `true` if `s` is a valid IPv4 or IPv6 address.
///
/// Uses standard library parsing for correctness.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_ip;
/// assert!(is_ip("192.168.1.1"));
/// assert!(is_ip("2001:db8::1"));
/// assert!(!is_ip("999.999.999.999"));
/// ```
pub fn is_ip(s: &str) -> bool {
    Ipv4Addr::from_str(s).is_ok() || Ipv6Addr::from_str(s).is_ok()
}

/// Returns `true` if `s` is a valid IPv4 address.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_ipv4;
/// assert!(is_ipv4("192.168.1.1"));
/// assert!(!is_ipv4("2001:db8::1"));
/// ```
pub fn is_ipv4(s: &str) -> bool {
    Ipv4Addr::from_str(s).is_ok()
}

/// Returns `true` if `s` is a valid IPv6 address.
///
/// # Examples
///
/// ```
/// use is_rs::regexp::is_ipv6;
/// assert!(is_ipv6("2001:db8::1"));
/// assert!(!is_ipv6("192.168.1.1"));
/// ```
pub fn is_ipv6(s: &str) -> bool {
    Ipv6Addr::from_str(s).is_ok()
}
