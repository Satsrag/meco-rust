use meco_core::{CodeType, MecoError};
use meco_utn57_command::{translate_to_utn57, MecoUtn57Error};

#[test]
fn unsupported_source_fails_before_starting_the_external_backend() {
    let error = translate_to_utn57(CodeType::Oyun, "not blank").unwrap_err();

    assert!(matches!(
        error,
        MecoUtn57Error::Meco(MecoError::Unsupported(CodeType::Oyun))
    ));
}

#[test]
#[ignore = "requires the configured mongol-norm command backend"]
fn routes_delehi_through_zvvnmod_to_utn57() {
    let actual = translate_to_utn57(CodeType::Delehi, "\u{1824}\u{180b}\u{1824}").unwrap();

    assert_eq!(actual, "\u{1824}\u{180b}\u{1823}\u{180c}");
}
