use std::io::Write;
use std::process::{Command, Stdio};

fn meco() -> Command {
    Command::new(env!("CARGO_BIN_EXE_meco"))
}

#[test]
fn translates_positional_text_without_adding_a_newline() {
    let output = meco()
        .args([
            "translate",
            "--from",
            "z52",
            "--to",
            "menk_shape",
            "plain text",
        ])
        .output()
        .expect("meco command should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"plain text");
    assert!(output.stderr.is_empty());
}

#[test]
fn converts_to_utn57_without_any_external_backend() {
    let output = meco()
        .args(["translate", "--from", "zvvnmod", "--to", "utn57", "\u{E0E5}"])
        .env_clear()
        .output()
        .expect("meco command should run");

    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, "\u{180A}".as_bytes());
    assert!(output.stderr.is_empty());
}

#[test]
fn accepts_utn57_as_a_source() {
    // ᠮᠣᠩᠭᠣᠯ spelled the UTN #57 way, which is not how delehi spells it.
    let utn57 = "\u{182E}\u{1833}\u{180C}\u{182D}\u{180C}\u{182C}\u{180B}\u{1823}\u{182F}";
    let output = meco()
        .args(["translate", "--from", "utn57", "--to", "delehi", utn57])
        .output()
        .expect("meco command should run");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert_eq!(output.stdout, "\u{182E}\u{1823}\u{1829}\u{182D}\u{1823}\u{182F}".as_bytes());
    assert!(output.stderr.is_empty());
}

#[test]
fn still_rejects_oyun_as_a_source() {
    let output = meco()
        .args(["translate", "--from", "oyun", "--to", "z52", "\u{1820}"])
        .output()
        .expect("meco command should run");

    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("conversion not supported for Oyun"));
}

#[test]
fn reads_input_from_stdin_when_text_is_omitted() {
    let mut child = meco()
        .args(["translate", "--from", "z52", "--to", "menk_shape"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("meco command should start");

    child
        .stdin
        .take()
        .expect("stdin should be piped")
        .write_all(b"line one\nline two")
        .expect("stdin should accept input");

    let output = child
        .wait_with_output()
        .expect("meco command should finish");
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(output.stdout, b"line one\nline two");
    assert!(output.stderr.is_empty());
}

#[test]
fn help_lists_the_command_and_supported_encoding_names() {
    let output = meco()
        .arg("--help")
        .output()
        .expect("meco command should run");

    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("help should be UTF-8");
    assert!(stdout.contains("meco translate --from <encoding> --to <encoding> [text]"));
    assert!(stdout.contains("reads UTF-8 text from stdin"));
    for encoding in [
        "zvvnmod",
        "delehi",
        "menk_shape",
        "menk_letter",
        "oyun",
        "utn57",
        "z52",
    ] {
        assert!(stdout.contains(encoding), "help omitted {encoding}");
    }
    assert!(output.stderr.is_empty());
}

#[test]
fn version_matches_the_meco_core_package() {
    let output = meco()
        .arg("--version")
        .output()
        .expect("meco command should run");

    assert!(output.status.success());
    assert_eq!(
        String::from_utf8(output.stdout).expect("version should be UTF-8"),
        format!("meco {}\n", env!("CARGO_PKG_VERSION"))
    );
    assert!(output.stderr.is_empty());
}
