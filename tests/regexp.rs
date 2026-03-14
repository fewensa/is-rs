use is_rs::regexp::*;

// --- is_url ---
#[test]
fn url_accepts_http() {
    assert!(is_url("http://example.com"));
}

#[test]
fn url_accepts_https() {
    assert!(is_url("https://example.com/path?q=1"));
}

#[test]
fn url_accepts_without_scheme() {
    assert!(is_url("example.com"));
    assert!(is_url("example.com:8080/path"));
}

#[test]
fn url_accepts_ftp() {
    assert!(is_url("ftp://files.example.com/file.txt"));
}

#[test]
fn url_rejects_plain_string() {
    assert!(!is_url("not-a-url"));
}

#[test]
fn url_rejects_empty() {
    assert!(!is_url(""));
}

#[test]
fn url_rejects_private_ipv4() {
    assert!(!is_url("http://192.168.1.10"));
    assert!(!is_url("10.0.0.1"));
}

// --- is_email ---
#[test]
fn email_accepts_valid() {
    assert!(is_email("user@example.com"));
    assert!(is_email("user+tag@sub.example.co.uk"));
    assert!(is_email("customer/department=shipping@example.com"));
}

#[test]
fn email_rejects_missing_at() {
    assert!(!is_email("userexample.com"));
}

#[test]
fn email_rejects_empty() {
    assert!(!is_email(""));
}

// --- is_credit_card ---
#[test]
fn credit_card_accepts_visa() {
    assert!(is_credit_card("4111111111111111"));
}

#[test]
fn credit_card_rejects_invalid() {
    assert!(!is_credit_card("1234567890123456"));
}

// --- is_alpha_numeric ---
#[test]
fn alpha_numeric_accepts() {
    assert!(is_alpha_numeric("abc123"));
}

#[test]
fn alpha_numeric_rejects_space() {
    assert!(!is_alpha_numeric("abc 123"));
}

#[test]
fn alpha_numeric_rejects_empty() {
    assert!(!is_alpha_numeric(""));
}

// --- is_time_string ---
#[test]
fn time_string_accepts_valid() {
    assert!(is_time_string("13:45:30"));
    assert!(is_time_string("00:00:00"));
}

#[test]
fn time_string_rejects_invalid_hour() {
    assert!(!is_time_string("25:00:00"));
}

// --- is_date_string ---
#[test]
fn date_string_accepts_valid() {
    assert!(is_date_string("12/25/2023"));
    assert!(is_date_string("1/1/2023"));
    assert!(is_date_string("12-25-2023"));
}

#[test]
fn date_string_rejects_iso_format() {
    assert!(!is_date_string("2023-12-25"));
}

#[test]
fn date_string_rejects_mixed_separators() {
    assert!(!is_date_string("12/25-2023"));
}

// --- is_us_zip_code ---
#[test]
fn us_zip_accepts_five_digit() {
    assert!(is_us_zip_code("12345"));
}

#[test]
fn us_zip_accepts_zip_plus_four() {
    assert!(is_us_zip_code("12345-6789"));
}

#[test]
fn us_zip_rejects_four_digit() {
    assert!(!is_us_zip_code("1234"));
}

// --- is_ca_postal_code ---
#[test]
fn ca_postal_accepts_valid() {
    assert!(is_ca_postal_code("K1A 0A9"));
}

#[test]
fn ca_postal_rejects_us_zip() {
    assert!(!is_ca_postal_code("12345"));
}

// --- is_uk_post_code ---
#[test]
fn uk_post_code_accepts_valid() {
    assert!(is_uk_post_code("SW1A 1AA"));
    assert!(is_uk_post_code("EC1A 1BB"));
}

#[test]
fn uk_post_code_rejects_invalid() {
    assert!(!is_uk_post_code("12345"));
}

// --- is_nanp_phone ---
#[test]
fn nanp_phone_accepts_formatted() {
    assert!(is_nanp_phone("(555) 555-5555"));
    assert!(is_nanp_phone("555-555-5555"));
    assert!(is_nanp_phone("5555555555"));
}

#[test]
fn nanp_phone_rejects_invalid() {
    assert!(!is_nanp_phone("not-a-phone"));
}

// --- is_epp_phone ---
#[test]
fn epp_phone_accepts_valid() {
    assert!(is_epp_phone("+1.5555555555"));
}

#[test]
fn epp_phone_rejects_nanp() {
    assert!(!is_epp_phone("555-555-5555"));
}

// --- is_social_security_number ---
#[test]
fn ssn_accepts_valid() {
    assert!(is_social_security_number("123-45-6789"));
    assert!(is_social_security_number("123456789"));
}

#[test]
fn ssn_rejects_all_zeros() {
    assert!(!is_social_security_number("000-00-0000"));
}

#[test]
fn ssn_rejects_invalid_area() {
    assert!(!is_social_security_number("666-45-6789"));
    assert!(!is_social_security_number("900-45-6789"));
}

// --- is_affirmative ---
#[test]
fn affirmative_accepts_yes_variants() {
    assert!(is_affirmative("yes"));
    assert!(is_affirmative("YES"));
    assert!(is_affirmative("y"));
    assert!(is_affirmative("true"));
    assert!(is_affirmative("1"));
    assert!(is_affirmative("ok"));
    assert!(is_affirmative("okay"));
}

#[test]
fn affirmative_rejects_no() {
    assert!(!is_affirmative("no"));
    assert!(!is_affirmative("false"));
    assert!(!is_affirmative("0"));
}

// --- is_hexadecimal ---
#[test]
fn hexadecimal_accepts_valid() {
    assert!(is_hexadecimal("deadbeef"));
    assert!(is_hexadecimal("DEADBEEF"));
    assert!(is_hexadecimal("0123456789abcdef"));
    assert!(is_hexadecimal("0xdeadbeef"));
}

#[test]
fn hexadecimal_rejects_invalid() {
    assert!(!is_hexadecimal("xyz"));
    assert!(!is_hexadecimal(""));
}

// --- is_hex_color ---
#[test]
fn hex_color_accepts_three_digit() {
    assert!(is_hex_color("#fff"));
    assert!(is_hex_color("fff"));
}

#[test]
fn hex_color_accepts_six_digit() {
    assert!(is_hex_color("#aabbcc"));
    assert!(is_hex_color("aabbcc"));
}

#[test]
fn hex_color_rejects_invalid() {
    assert!(!is_hex_color("#gg0000"));
    assert!(!is_hex_color(""));
}

// --- is_ip ---
#[test]
fn ip_accepts_ipv4() {
    assert!(is_ip("192.168.1.1"));
    assert!(is_ip("255.255.255.255"));
}

#[test]
fn ip_accepts_ipv6() {
    assert!(is_ip("2001:db8::1"));
    assert!(is_ip("::1"));
}

#[test]
fn ip_rejects_invalid() {
    assert!(!is_ip("999.999.999.999"));
    assert!(!is_ip("not-an-ip"));
}

// --- is_ipv4 ---
#[test]
fn ipv4_accepts_valid() {
    assert!(is_ipv4("192.168.1.1"));
    assert!(is_ipv4("0.0.0.0"));
}

#[test]
fn ipv4_rejects_ipv6() {
    assert!(!is_ipv4("2001:db8::1"));
}

// --- is_ipv6 ---
#[test]
fn ipv6_accepts_valid() {
    assert!(is_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334"));
    assert!(is_ipv6("::1"));
}

#[test]
fn ipv6_rejects_ipv4() {
    assert!(!is_ipv6("192.168.1.1"));
}
