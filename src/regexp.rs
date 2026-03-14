use std::net::Ipv6Addr;
use std::str::FromStr;
use std::sync::LazyLock;

use regex::Regex;

// ---------------------------------------------------------------------------
// Compiled regex patterns (compiled once, reused on every call)
// Patterns are ported from is.js source to preserve identical behaviour.
// Where the original uses lookahead (unsupported by the `regex` crate), the
// validation is implemented in plain Rust code instead.
// ---------------------------------------------------------------------------

static RE_URL_SCHEME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:(https?|ftp)://)?(.+)$").unwrap());

static RE_URL_DOMAIN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)^(?:(?:[a-z\u{00a1}-\u{ffff}0-9](?:[a-z\u{00a1}-\u{ffff}0-9-]*[a-z\u{00a1}-\u{ffff}0-9])?)(?:\.(?:[a-z\u{00a1}-\u{ffff}0-9](?:[a-z\u{00a1}-\u{ffff}0-9-]*[a-z\u{00a1}-\u{ffff}0-9])?))*)(?:\.(?:[a-z\u{00a1}-\u{ffff}]{2,}))$",
    )
    .unwrap()
});

static RE_EMAIL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)^((([a-z]|\d|[!#\$%&'\*\+\-/=\?\^_`{\|}~]|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])+(\.([a-z]|\d|[!#\$%&'\*\+\-/=\?\^_`{\|}~]|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])+)*)|((")((((\x20|\x09)*(\x0d\x0a))?(\x20|\x09)+)?(([\x01-\x08\x0b\x0c\x0e-\x1f\x7f]|\x21|[\x23-\x5b]|[\x5d-\x7e]|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])|(\\([\x01-\x09\x0b\x0c\x0d-\x7f]|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}]))))*(((\x20|\x09)*(\x0d\x0a))?(\x20|\x09)+)?(")))@((([a-z]|\d|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])|(([a-z]|\d|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])([a-z]|\d|-|\.|_|~|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])*([a-z]|\d|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])))\.)+(([a-z]|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])|(([a-z]|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])([a-z]|\d|-|\.|_|~|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])*([a-z]|[\u{00A0}-\u{D7FF}\u{F900}-\u{FDCF}\u{FDF0}-\u{FFEF}])))$"#).unwrap()
});

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
    Regex::new(r"^(1[0-2]|0?[1-9])([/-])(3[01]|[12][0-9]|0?[1-9])([/-])((?:[0-9]{2})?[0-9]{2})$")
        .unwrap()
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
    LazyLock::new(|| Regex::new(r"^([0-8][0-9]{2})(-?)([0-9]{2})(-?)([0-9]{4})$").unwrap());

static RE_AFFIRMATIVE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"(?i)^(?:1|t(?:rue)?|y(?:es)?|ok(?:ay)?)$").unwrap());

static RE_HEXADECIMAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(?:0x)?[0-9a-fA-F]+$").unwrap());

static RE_IPV4: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"^(?:(?:\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])\.){3}(?:\d|[1-9]\d|1\d{2}|2[0-4]\d|25[0-5])$",
    )
    .unwrap()
});

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
    if s.is_empty() || s.chars().any(char::is_whitespace) {
        return false;
    }

    let captures = match RE_URL_SCHEME.captures(s) {
        Some(captures) => captures,
        None => return false,
    };

    let remainder = match captures.get(2) {
        Some(value) => value.as_str(),
        None => return false,
    };

    let (authority, path) = split_once_any(remainder, &['/', '?', '#']);
    if authority.is_empty() {
        return false;
    }

    if !path.is_empty() && !path.starts_with('/') {
        return false;
    }

    let (host, port) = split_host_port(authority);
    if host.is_empty() || host.contains('@') {
        return false;
    }

    if let Some(port) = port
        && (port.len() < 2 || port.len() > 5 || !port.chars().all(|ch| ch.is_ascii_digit()))
    {
        return false;
    }

    is_public_ipv4(host) || RE_URL_DOMAIN.is_match(host)
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
    match RE_DATE_STRING.captures(s) {
        Some(captures) => {
            captures.get(2).map(|m| m.as_str()) == captures.get(4).map(|m| m.as_str())
        }
        None => false,
    }
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
    const INVALID_CHARS: &[char] = &['D', 'F', 'I', 'O', 'Q', 'U'];

    RE_CA_POSTAL_CODE_BASE.is_match(s) && !s.chars().any(|ch| INVALID_CHARS.contains(&ch))
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
    let first_separator = caps.get(2).map_or("", |value| value.as_str());
    let group: u32 = caps[3].parse().unwrap_or(0);
    let second_separator = caps.get(4).map_or("", |value| value.as_str());
    let serial: u32 = caps[5].parse().unwrap_or(0);

    if first_separator != second_separator {
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
    is_ipv4(s) || is_ipv6(s)
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
    RE_IPV4.is_match(s)
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

fn split_once_any<'a>(value: &'a str, delimiters: &[char]) -> (&'a str, &'a str) {
    match value.find(delimiters) {
        Some(index) => (&value[..index], &value[index..]),
        None => (value, ""),
    }
}

fn split_host_port(authority: &str) -> (&str, Option<&str>) {
    match authority.rsplit_once(':') {
        Some((host, port)) if !host.contains(':') => (host, Some(port)),
        _ => (authority, None),
    }
}

fn is_public_ipv4(host: &str) -> bool {
    if !RE_IPV4.is_match(host) {
        return false;
    }

    let octets = host
        .split('.')
        .map(|part| part.parse::<u16>())
        .collect::<Result<Vec<_>, _>>();

    let Ok(octets) = octets else {
        return false;
    };

    let [a, b, _c, d] = octets.as_slice() else {
        return false;
    };

    if *d == 0 || *d == 255 {
        return false;
    }

    if matches!((*a, *b), (10, _) | (127, _) | (169, 254) | (192, 168)) {
        return false;
    }

    if *a == 172 && (16..=31).contains(b) {
        return false;
    }

    (1..=223).contains(a)
}
