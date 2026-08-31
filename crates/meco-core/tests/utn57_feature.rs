#![cfg(feature = "utn57-command")]

use meco_core::{translate, CodeType, MecoError};

#[test]
fn enabled_feature_routes_utn57_target_through_the_existing_api() {
    let input = "plain\u{180A}\u{180E}\u{202F}\u{200D}";

    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, input).unwrap(),
        input
    );
}

#[test]
fn utn57_identity_needs_no_backend() {
    let input = "\u{1820}\u{180A}\u{202F}";

    assert_eq!(
        translate(CodeType::Utn57, CodeType::Utn57, input).unwrap(),
        input
    );
}

#[test]
fn source_routes_through_existing_zvvnmod_conversion() {
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Utn57, "\u{E23E}").unwrap(),
        "\u{180A}"
    );
}

#[test]
fn legacy_controls_are_discarded_without_starting_a_backend() {
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
#[ignore = "requires ZVVNMOD_MONGOL_NORM_PATH to name a missing absolute path"]
fn unavailable_backend_uses_the_existing_meco_error_channel() {
    let error = translate(CodeType::Zvvnmod, CodeType::Utn57, "\u{E0E5}").unwrap_err();

    match error {
        MecoError::Utn57(reason) => assert!(!reason.is_empty()),
        other => panic!("expected UTN #57 backend error, got {other:?}"),
    }
}

#[test]
#[ignore = "requires the configured mongol-norm backend"]
fn formal_zvvnmod_nirugu_converts_through_the_real_backend() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, "\u{E0E5}").unwrap(),
        "\u{180A}"
    );
}
