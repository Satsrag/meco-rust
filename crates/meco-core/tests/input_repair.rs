use meco_core::{
    translate, translate_with_options, translate_with_warnings, CodeType, MecoError,
    TranslationOptions, Warning,
};

const REPAIR: TranslationOptions = TranslationOptions {
    repair_suffix_separators: true,
};
const RAW: &str = "ᠤᠯᠤᠰ ᠤᠨ";
const FIXED: &str = "ᠤᠯᠤᠰ\u{202F}ᠤᠨ";

#[test]
fn repair_precedes_decoding_for_every_target() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for to in [
            CodeType::MenkLetter,
            CodeType::Delehi,
            CodeType::MenkShape,
            CodeType::Zvvnmod,
            CodeType::Z52,
            CodeType::Utn57,
            CodeType::Utn57Shape,
        ] {
            let result = translate_with_options(from, to, RAW, &REPAIR).unwrap();
            assert_eq!(
                result.text,
                translate(from, to, FIXED).unwrap(),
                "{from:?} -> {to:?}"
            );
            assert_eq!(
                result.warnings[0],
                Warning::RepairedSuffixSeparator {
                    byte_offset: "ᠤᠯᠤᠰ".len(),
                    original: ' ',
                }
            );
            assert_eq!(
                translate_with_options(from, to, RAW, &TranslationOptions::default()).unwrap(),
                translate_with_warnings(from, to, RAW).unwrap(),
            );
        }
    }
    assert_ne!(
        translate(CodeType::MenkLetter, CodeType::Zvvnmod, RAW).unwrap(),
        translate(CodeType::MenkLetter, CodeType::Zvvnmod, FIXED).unwrap(),
    );
}

#[test]
fn repairs_suffix_chains_and_reports_original_byte_offsets() {
    let raw = "中文: ᠤᠯᠤᠰ\u{A0}ᠤᠨ ᠢᠶᠠᠷ᠃\nᠭᠡᠷ ᠦᠨ";
    let result =
        translate_with_options(CodeType::MenkLetter, CodeType::MenkLetter, raw, &REPAIR).unwrap();
    assert_eq!(
        result.text,
        "中文: ᠤᠯᠤᠰ\u{202F}ᠤᠨ\u{202F}ᠢᠶᠠᠷ᠃\nᠭᠡᠷ\u{202F}ᠦᠨ"
    );
    let offsets = [
        raw.find('\u{A0}').unwrap(),
        raw.find(" ᠢᠶᠠᠷ").unwrap(),
        raw.find(" ᠦᠨ").unwrap(),
    ];
    for (warning, offset) in result.warnings.iter().zip(offsets) {
        assert!(
            matches!(warning, Warning::RepairedSuffixSeparator { byte_offset, .. } if *byte_offset == offset)
        );
    }
    assert_eq!(result.warnings.len(), 3);
    let again = translate_with_options(
        CodeType::MenkLetter,
        CodeType::MenkLetter,
        &result.text,
        &REPAIR,
    )
    .unwrap();
    assert_eq!(again.text, result.text);
    assert!(again.warnings.is_empty());
}

#[test]
fn preserves_layout_existing_controls_and_unrecognised_words() {
    for raw in [
        "",
        "  \n",
        "ᠤᠨ",
        " ᠤᠨ",
        "Latin ᠤᠨ",
        "ᠤᠯᠤᠰ\nᠤᠨ",
        "ᠤᠯᠤᠰ\tᠤᠨ",
        "ᠤᠯᠤᠰ  ᠤᠨ",
        "ᠤᠯᠤᠰ\u{A0}\u{A0}ᠤᠨ",
        "ᠤᠯᠤᠰᠤᠨ",
        "ᠤᠯᠤᠰ ᠤᠨᠠᠭ᠎ᠠ",
        "ᠤᠯᠤᠰ ᠤᠨ\u{180B}",
        "ᠤᠯᠤᠰ ᠤᠨ\u{200D}",
        "ᠤᠯᠤᠰ\u{202F}ᠤᠨ",
        "ᠤᠯᠤᠰ\u{180E}ᠤᠨ",
        "ᠤᠯᠤᠰ\u{202F}\u{202F}ᠤᠨ",
        "ᠴᠠᠭᠠᠨ ᠪᠠᠷ",
        "ᠤᠯᠤᠰ ᠲᠠᠢ",
        "ᠤᠯᠤᠰ ᠤᠨLatin",
        "ᠤᠯᠤᠰ ᠤᠨ_abc",
        "ᠤᠯᠤᠰ, ᠤᠨ",
    ] {
        let result =
            translate_with_options(CodeType::Delehi, CodeType::Delehi, raw, &REPAIR).unwrap();
        assert_eq!(result.text, raw);
        assert!(result.warnings.is_empty(), "{raw:?}");
    }
}

#[test]
fn rejects_repair_for_encodings_whose_suffix_spellings_are_not_supported() {
    for from in [
        CodeType::Utn57,
        CodeType::Utn57Shape,
        CodeType::MenkShape,
        CodeType::Z52,
        CodeType::Zvvnmod,
        CodeType::Oyun,
    ] {
        assert_eq!(
            translate_with_options(from, from, RAW, &REPAIR),
            Err(MecoError::UnsupportedInputRepair(from))
        );
    }
}
