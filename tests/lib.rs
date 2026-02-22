mod common;
use common::generate::*;
use common::generate::CharType::*;
use qrfi::*;
use rand::Rng;

#[test]
fn mecardify_escapes_delimiters() {
    let cases = vec![
        (",", "\\,"),
        (":", "\\:"),
        (";", "\\;"),
        ("\\", "\\\\"),
    ];
    for (input, expected) in cases {
        assert_eq!(
            mecardify(input), 
            expected, 
            "Delimiter {:?} should be escaped to {:?}", input, expected
        );
    }
}

#[test]
fn mecardify_preserves_nondelimiters() {
    let cases = vec![
        ("", ""),
        ("\t", "\t"),
        ("\n", "\n"),
        (" ", " "),
        ("!", "!"),
        ("\"", "\""),
        ("#", "#"),
        ("-", "-"),
        ("\'", "\'"),
        ("+", "+"),
        (".", "."),
        ("/", "/"),
        ("0", "0"),
        ("9", "9"),
        ("<", "<"),
        (">", ">"),
        ("@", "@"),
        ("A", "A"),
        ("Z", "Z"),
        ("[", "["),
        ("]", "]"),
        ("a", "a"),
        ("z", "z"),
        ("あ", "あ"),
        ("☕️", "☕️"),
        ("⚡", "⚡"),
    ];
    for (input, expected) in cases {
        assert_eq!(
            mecardify(input), 
            expected, 
            "Non-delimiter {:?} should be preserved as-is", input
        );
    }
}

#[test]
fn ssid_validate_rejects_empty_input() {
    let input = "".to_string();
    let result = Ssid(input).validate();
    assert!(result.is_err(), "Empty SSID should be invalid");
}

#[test]
fn ssid_validate_accepts_valid_length() {
    let cases = vec![
        generate_random_ascii(1),
        generate_random_mbstring(2, &[DoubleByte]),
        generate_random_ascii(32),
        generate_random_mbstring(32, &[DoubleByte, TripleByte, QuadrupleByte]),
    ];
    for input in cases {
        let result = Ssid(input.clone()).validate();
        assert!(
            result.is_ok(), 
            "SSID should be valid for {} bytes: {:?}", input.len(), input
        );
    }
}

#[test]
fn ssid_validate_rejects_excessive_length() {
    let cases = vec![
        generate_random_ascii(33),
        generate_random_mbstring(33, &[TripleByte]),
    ];
    for input in cases {
        let result = Ssid(input.clone()).validate();
        assert!(
            result.is_err(), 
            "SSID should be invalid for {} bytes: {:?}", input.len(), input
        );
    }
}

#[test]
fn ssid_password_validate_accepts_valid_wpa_passphrase() {
    let cases = vec![
        (Some(generate_random_ascii(8)), "8-char ASCII"),
        (Some(generate_random_ascii(63)), "63-char ASCII"),
        (Some(generate_random_hex(64)), "64-char Hex"),
    ];
    for (val, msg) in cases {
        let p = Password { value: val, auth_type: AuthType::Wpa };
        assert!(p.validate().is_ok(), "WPA should accept {}", msg);
    }
}
#[test]
fn ssid_password_validate_rejects_invalid_wpa_passphrase() {
    let cases = vec![
        (Some(generate_random_ascii(7)), "too short"),
        (Some(generate_random_ascii(65)), "too long"),
        (Some(generate_random_mbstring(8, &[TripleByte])), "non-ASCII"),
    ];
    for (val, msg) in cases {
        let p = Password { value: val, auth_type: AuthType::Wpa };
        assert!(p.validate().is_err(), "WPA should reject {}", msg);
    }
}

#[test]
fn ssid_password_validate_accepts_valid_wep_password() {
    let cases = vec![
        (Some(generate_random_ascii(5)), "5-char ASCII"),
        (Some(generate_random_mbstring(5, &[TripleByte])), "5-char MB"),
        (Some(generate_random_hex(10)), "10-char Hex"),
        (Some(generate_random_ascii(13)), "13-char ASCII"),
        (Some(generate_random_hex(26)), "26-char Hex"),
    ];
    for (val, msg) in cases {
        let p = Password { value: val, auth_type: AuthType::Wep };
        assert!(p.validate().is_ok(), "WEP should accept {}", msg);
    }
}
#[test]
fn ssid_password_validate_rejects_invalid_wep_password() {
    let cases = vec![
        (Some(generate_random_ascii(4)), "too short"),
        (Some(generate_random_hex(11)), "invalid hex length"),
    ];
    for (val, msg) in cases {
        let p = Password { value: val, auth_type: AuthType::Wep };
        assert!(p.validate().is_err(), "WEP should reject {}", msg);
    }
}

#[test]
fn ssid_password_validate_accept_empty_if_authtype_is_nopass() {
    let p = Password { value: None, auth_type: AuthType::Nopass };
    assert!(p.validate().is_ok(), "Nopass should accept None");
}
#[test]
fn ssid_password_validate_rejects_anystrings_if_authtype_is_nopass() {
    let p = Password { value: Some(generate_random_ascii(1)), auth_type: AuthType::Nopass };
    assert!(p.validate().is_err(), "Nopass should reject any string input");
}

#[test]
fn wifi_to_mecard_matches_expected_structure_with_random_inputs() {
    // Check whether the logic for generating the MECARD format matches the description in this test function
    for _ in 0..100 {
        let raw_ssid = generate_random_mbstring(32, &[DoubleByte, TripleByte, QuadrupleByte]);
        let raw_pass = generate_random_mbstring(16, &[DoubleByte, TripleByte]);
        let is_hidden = rand::thread_rng().gen_bool(0.5);
        let wifi = Wifi {
            ssid: Ssid(raw_ssid.clone()),
            password: Password {
                value: Some(raw_pass.clone()),
                auth_type: AuthType::Wpa,
            },
            hidden: is_hidden,
        };
        let result = wifi.to_mecard();
        let expected = format!(
            "WIFI:S:{};T:WPA;P:{};H:{};;",
            mecardify(&raw_ssid),
            mecardify(&raw_pass),
            if is_hidden { "true" } else { "false" }
        );
        assert_eq!(
            result, 
            expected, 
            "MECARD structure mismatch for SSID: {:?}, Pass: {:?}", 
            raw_ssid, 
            raw_pass
        );
    }
}
