use meco_core::{translate, CodeType};

const MENK_BIRGAS: &str = "\u{E23F}\u{E240}\u{E241}\u{E242}";
const UNICODE_BIRGAS: &str = "\u{11660}\u{11661}\u{11662}\u{11663}";

#[test]
fn menk_shape_birgas_use_unicode_codepoints_in_zvvnmod() {
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, MENK_BIRGAS).unwrap(),
        UNICODE_BIRGAS
    );
}

#[test]
fn zvvnmod_birgas_are_encoded_for_menk_shape() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, UNICODE_BIRGAS).unwrap(),
        MENK_BIRGAS
    );
}

#[test]
fn birgas_roundtrip_through_zvvnmod_in_both_directions() {
    let zvvnmod = translate(CodeType::MenkShape, CodeType::Zvvnmod, MENK_BIRGAS).unwrap();
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, &zvvnmod).unwrap(),
        MENK_BIRGAS
    );

    let menk = translate(CodeType::Zvvnmod, CodeType::MenkShape, UNICODE_BIRGAS).unwrap();
    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, &menk).unwrap(),
        UNICODE_BIRGAS
    );
}

#[test]
fn delehi_and_menk_letter_preserve_nirugu_in_zvvnmod() {
    const MIXED: &str = "A\u{180A}B";
    for source in [CodeType::Delehi, CodeType::MenkLetter] {
        assert_eq!(
            translate(source, CodeType::Zvvnmod, MIXED).unwrap(),
            MIXED,
            "source={source:?}"
        );
    }

    assert_eq!(
        translate(CodeType::Delehi, CodeType::Zvvnmod, "\u{1826}\u{180A}").unwrap(),
        "\u{E000}\u{E008}\u{E006}\u{180A}"
    );

    let contexts = [
        ("\u{180A}\u{1820}", "\u{180A}\u{E00C}"),
        (
            "\u{1820}\u{180A}\u{1820}",
            "\u{E000}\u{E005}\u{180A}\u{E00C}",
        ),
        ("\u{1820}\u{180A}", "\u{E000}\u{E005}\u{180A}"),
    ];
    for source in [CodeType::Delehi, CodeType::MenkLetter] {
        for (input, expected) in contexts {
            assert_eq!(
                translate(source, CodeType::Zvvnmod, input).unwrap(),
                expected,
                "source={source:?}, input={input:?}"
            );
        }
    }
}

#[test]
fn nirugu_uses_unicode_in_zvvnmod_and_unicode_letter_encodings() {
    const NIRUGU: &str = "\u{180A}";
    const MENK_SHAPE_NIRUGU: &str = "\u{E23E}";

    assert_eq!(
        translate(CodeType::MenkShape, CodeType::Zvvnmod, MENK_SHAPE_NIRUGU).unwrap(),
        NIRUGU
    );
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, NIRUGU).unwrap(),
        MENK_SHAPE_NIRUGU
    );

    for encoding in [CodeType::Delehi, CodeType::MenkLetter, CodeType::Z52] {
        assert_eq!(
            translate(encoding, CodeType::Zvvnmod, NIRUGU).unwrap(),
            NIRUGU,
            "source={encoding:?}"
        );
        assert_eq!(
            translate(CodeType::Zvvnmod, encoding, NIRUGU).unwrap(),
            NIRUGU,
            "target={encoding:?}"
        );
        assert_eq!(
            translate(CodeType::MenkShape, encoding, MENK_SHAPE_NIRUGU).unwrap(),
            NIRUGU,
            "MenkShape -> {encoding:?}"
        );
        assert_eq!(
            translate(encoding, CodeType::MenkShape, NIRUGU).unwrap(),
            MENK_SHAPE_NIRUGU,
            "{encoding:?} -> MenkShape"
        );
    }
}

#[test]
fn unicode_letter_encodings_use_zvvnmod_birgas() {
    for encoding in [CodeType::Delehi, CodeType::MenkLetter, CodeType::Z52] {
        assert_eq!(
            translate(CodeType::MenkShape, encoding, MENK_BIRGAS).unwrap(),
            UNICODE_BIRGAS,
            "MenkShape -> {encoding:?}"
        );
        assert_eq!(
            translate(encoding, CodeType::MenkShape, UNICODE_BIRGAS).unwrap(),
            MENK_BIRGAS,
            "{encoding:?} -> MenkShape"
        );
    }
}
