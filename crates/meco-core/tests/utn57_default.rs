#![cfg(not(feature = "utn57-command"))]

use meco_core::{translate, CodeType, MecoError};

#[test]
fn default_build_keeps_utn57_target_unsupported() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, "\u{E0E5}").unwrap_err(),
        MecoError::Unsupported(CodeType::Utn57)
    );
}

#[test]
fn utn57_backend_error_variant_is_stable_without_the_feature() {
    assert_eq!(
        MecoError::Utn57("backend unavailable".to_owned()).to_string(),
        "UTN #57 conversion failed: backend unavailable"
    );
}
