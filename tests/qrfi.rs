mod common;
use common::generate::*;
use common::generate::CharType::*;

use assert_cmd::Command;
use predicates::prelude::*;

fn run_cli_test<T: AsRef<[u8]>>(args: Vec<String>, stdin: Option<String>, expected_success: bool, expected_output: T) {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_qrfi"));
    cmd.args(&args);
    if let Some(input) = stdin {
        cmd.write_stdin(input);
    }
    let assert = cmd.assert();
    if expected_success {
        let pattern = expected_output.as_ref().to_vec();
        assert.success()
            .stdout(predicate::function(move |actual: &[u8]| {
                actual.windows(pattern.len()).any(|window| window == pattern)
            }));
    } else {
        let err_msg = std::str::from_utf8(expected_output.as_ref()).unwrap_or("");
        assert.failure()
            .stderr(predicate::str::contains(err_msg));
    }
}

macro_rules! qrfi_test {
    ($($name:ident: $args:expr, $stdin:expr, $success:expr, $output:expr,)*) => {
        $(
            #[test]
            fn $name() {
                run_cli_test($args, $stdin, $success, $output);
            }
        )*
    }
}

qrfi_test! {
    qrfi_accepts_help_arg: vec!["--help".into()], None, true, format!("{}", env!("CARGO_PKG_DESCRIPTION")),
    qrfi_accepts_nopass_auth_type: vec!["-t".into(), "nopass".into(), format!("--password={}", generate_random_ascii(16)), "--".into(), generate_random_ascii(16)], None, true, "█",
    qrfi_accepts_ssid_via_args: vec![format!("--password={}", generate_random_ascii(16)), "--".into(), generate_random_mbstring(32, &[TripleByte])], None, true, "█",
    qrfi_accepts_ssid_via_stdin: vec![format!("--password={}", generate_random_hex(64))], Some(generate_random_ascii(16)), true, "█",
    qrfi_accepts_version_arg: vec!["--version".into()], None, true, format!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
    qrfi_outputs_png_format: vec![format!("--password={}", generate_random_ascii(16)), "-f".into(), "png".into(), "--".into(), generate_random_mbstring(32, &[DoubleByte])], None, true, &b"\x89PNG"[..],
    qrfi_outputs_svg_format: vec![format!("--password={}", generate_random_ascii(16)), "-f".into(), "svg".into(), "--".into(), generate_random_mbstring(32, &[QuadrupleByte])], None, true, "<svg",
    qrfi_rejects_invalid_ssid: vec![format!("--password={}", generate_random_ascii(16)), "--".into(), generate_random_ascii(33)], None, false, "SSID is too long",
    qrfi_rejects_unsupported_jpeg_format: vec![format!("--password={}", generate_random_ascii(16)), "-f".into(), "jpeg".into(), "--".into(), generate_random_ascii(16)], None, false, "invalid value 'jpeg' for '--format <FORMAT>'",
    qrfi_rejects_unsupported_jpg_format: vec![format!("--password={}", generate_random_ascii(16)), "-f".into(), "jpg".into(), "--".into(), generate_random_ascii(16)], None, false, "invalid value 'jpg' for '--format <FORMAT>'",
}
