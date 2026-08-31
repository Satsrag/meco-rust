use meco_core::CodeType;
use meco_utn57_command::{translate, MecoUtn57CommandErrorKind};

#[test]
fn utn57_identity_needs_no_backend() {
    let input = "\u{1820}\u{180A}\u{202F}";

    assert_eq!(
        translate(CodeType::Utn57, CodeType::Utn57, input).unwrap(),
        input
    );
}

#[test]
fn non_utn57_conversion_delegates_to_meco_core() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, "\u{11660}").unwrap(),
        "\u{E23F}"
    );
}

#[test]
fn zvvnmod_passthrough_reaches_utn57_without_starting_a_backend() {
    let input = "English \u{180A}\u{180E}\u{202F}\u{200D} 中";

    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, input).unwrap(),
        input
    );
}

#[test]
fn menk_shape_source_routes_through_zvvnmod_before_utn57() {
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Utn57, "\u{E23E}").unwrap(),
        "\u{180A}"
    );
}

#[test]
fn reverse_utn57_conversion_is_explicitly_unsupported() {
    let error = translate(CodeType::Utn57, CodeType::Zvvnmod, "\u{1820}").unwrap_err();

    assert_eq!(
        error.kind(),
        MecoUtn57CommandErrorKind::UnsupportedDirection {
            from: CodeType::Utn57,
            to: CodeType::Zvvnmod,
        }
    );
}

#[test]
#[ignore = "requires the configured mongol-norm backend"]
fn formal_zvvnmod_shape_converts_through_the_real_backend() {
    let input = char::from_u32(zvvnmod_utn57::O_INIT.codepoint())
        .unwrap()
        .to_string();

    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Utn57, &input).unwrap(),
        "\u{1824}\u{180B}\u{200D}"
    );
}
