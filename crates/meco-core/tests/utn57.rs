//! UTN #57 output is always available: the pure-Rust `zvvnmod-utn57` backend is linked into the
//! core, so these tests run in the default build on every platform.

use meco_core::{translate, CodeType, MecoError};

#[test]
fn passthrough_text_reaches_utn57_unchanged() {
    let input = "plain\u{180A}\u{180E}\u{202F}\u{200D}";

    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, input).unwrap(),
        input
    );
}

#[test]
fn utn57_identity_short_circuits() {
    let input = "\u{1820}\u{180A}\u{202F}";

    assert_eq!(
        translate(CodeType::Utn57, CodeType::Utn57, input).unwrap(),
        input
    );
}

#[test]
fn formal_zvvnmod_nirugu_converts_in_process() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, "\u{E0E5}").unwrap(),
        "\u{180A}"
    );
}

#[test]
fn legacy_sources_route_through_the_zvvnmod_hub() {
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Utn57, "\u{E23E}").unwrap(),
        "\u{180A}"
    );
}

#[test]
fn z52_word_converts_to_canonical_unicode() {
    let output = translate(
        CodeType::Z52,
        CodeType::Utn57,
        "ᡳᡬᡦ ᢌᡭᡪᢊᡱᡱᡭᢐ ᢋᡭᡬᢎᡭᡧ",
    )
    .unwrap();

    assert!(!output.is_empty());
    assert!(
        output
            .chars()
            .all(|c| !('\u{E000}'..='\u{F8FF}').contains(&c)),
        "UTN #57 output must not contain private-use ZVVNMOD shapes: {output:?}"
    );
    assert!(
        output.chars().any(|c| ('\u{1820}'..='\u{1842}').contains(&c)),
        "UTN #57 output should contain Unicode Mongolian letters: {output:?}"
    );
    assert_eq!(output.matches(' ').count(), 2, "spaces pass through: {output:?}");
}

#[test]
fn legacy_controls_are_discarded() {
    assert_eq!(
        translate(
            CodeType::Zvvnmod,
            CodeType::Utn57,
            "\u{E140}\u{E141}\u{E142}\u{E143}\u{E144}",
        )
        .unwrap(),
        ""
    );
}

#[test]
fn reverse_utn57_conversion_remains_unsupported() {
    assert_eq!(
        translate(CodeType::Utn57, CodeType::MenkShape, "\u{1820}").unwrap_err(),
        MecoError::Unsupported(CodeType::Utn57)
    );
}

#[test]
fn utn57_error_variant_display_is_stable() {
    assert_eq!(
        MecoError::Utn57("backend unavailable".to_owned()).to_string(),
        "UTN #57 conversion failed: backend unavailable"
    );
}
