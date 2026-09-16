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
fn audited_reflexives_match_manual_repairs_across_targets() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (raw, fixed, shape, edits) in [
            ("ᠨᠣᠮ ᠢᠶᠠᠨ", "ᠨᠣᠮ\u{202F}ᠢᠶᠠᠨ", "NOMMvsIIAA", 1),
            ("ᠭᠡᠷ\u{A0}ᠢᠶᠡᠨ", "ᠭᠡᠷ\u{202F}ᠢᠶᠡᠨ", "GARMvsIIAA", 1),
        ] {
            for to in [
                CodeType::MenkLetter,
                CodeType::Delehi,
                CodeType::MenkShape,
                CodeType::Zvvnmod,
                CodeType::Z52,
                CodeType::Utn57,
                CodeType::Utn57Shape,
            ] {
                let result = translate_with_options(from, to, raw, &REPAIR).unwrap();
                assert_eq!(result.text, translate(from, to, fixed).unwrap());
                assert_eq!(result.warnings.len(), edits);
                if to == CodeType::Utn57Shape {
                    assert_eq!(result.text, shape);
                }
            }
            let again = translate_with_options(from, from, fixed, &REPAIR).unwrap();
            assert_eq!(again.text, fixed);
            assert!(again.warnings.is_empty());
        }
        let chain = translate_with_options(from, from, "ᠤᠯᠤᠰ ᠤᠨ ᠢᠶᠠᠨ", &REPAIR).unwrap();
        assert_eq!(chain.text, "ᠤᠯᠤᠰ\u{202F}ᠤᠨ\u{202F}ᠢᠶᠠᠨ");
        assert_eq!(chain.warnings.len(), 2);
        let existing = translate_with_options(from, from, "ᠭᠡᠷ\u{202F}ᠦᠨ ᠢᠶᠡᠨ", &REPAIR).unwrap();
        assert_eq!(existing.text, "ᠭᠡᠷ\u{202F}ᠦᠨ\u{202F}ᠢᠶᠡᠨ");
        assert_eq!(existing.warnings.len(), 1);
    }
}

#[test]
fn reflexive_repair_declines_uncertain_contexts_and_longer_words() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for raw in [
            "ᠭᠡᠷ ᠢᠶᠠᠨ",
            "ᠨᠣᠮ ᠢᠶᠡᠨ", // mismatched harmony
            "ᠬᠣᠲᠠ ᠢᠶᠠᠨ",
            "ᠡᠭᠡᠴᠢ ᠢᠶᠡᠨ", // vowel-final
            "ᠪᠢᠴᠢᠭ ᠢᠶᠠᠨ",
            "ᠪᠢᠴᠢᠭ ᠢᠶᠡᠨ", // neutral-only: deliberately deferred
            "ᠨᠣᠡᠮ ᠢᠶᠠᠨ",
            "ᠨᠣᠡᠮ ᠢᠶᠡᠨ", // mixed-harmony input
            "ᠨᠣᠮ\u{180B} ᠢᠶᠠᠨ",
            "ᠭᠡᠷ\u{200D} ᠢᠶᠡᠨ", // controls
            "ᠨᠣᠮ ᠢᠶᠠᠨ\u{180B}",
            "ᠭᠡᠷ ᠢᠶᠡᠨᠡ",
            "ᠨᠣᠮ ᠢᠶᠠᠨx",
            "ᠨᠣᠮᠢᠶᠠᠨ",
            "ᠭᠡᠷ\nᠢᠶᠡᠨ",
            "ᠢᠶᠠᠨ",
            "ᠢᠶᠡᠨ",
        ] {
            let result = translate_with_options(from, from, raw, &REPAIR).unwrap();
            assert_eq!(result.text, raw);
            assert!(result.warnings.is_empty(), "{raw:?}");
        }
    }
}

#[test]
fn comitative_and_plural_repairs_preserve_spelling_across_targets() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        // The ger examples also occur with NNBSP in the existing Delehi golden corpus.
        // Plural recognition must not require a vowel-final stem: ger ends in r.
        for raw in [
            "ᠭᠡᠷ ᠯᠦᠭᠡ",
            "ᠭᠡᠷ ᠨᠦᠭᠦᠳ",
            "ᠨᠣᠮ ᠯᠤᠭ\u{180E}ᠠ",
            "ᠣᠶᠤᠲᠠᠨ ᠨᠤᠭᠤᠳ",
            "ᠬᠣᠲᠠ ᠨᠤᠭᠤᠳ",
            "ᠡᠭᠡᠴᠢ ᠨᠦᠭᠦᠳ",
        ] {
            let fixed = raw.replace(' ', "\u{202F}");
            for separator in [' ', '\u{A0}'] {
                let input = raw.replace(' ', &separator.to_string());
                for to in [
                    CodeType::MenkLetter,
                    CodeType::Delehi,
                    CodeType::MenkShape,
                    CodeType::Zvvnmod,
                    CodeType::Z52,
                    CodeType::Utn57,
                    CodeType::Utn57Shape,
                ] {
                    let result = translate_with_options(from, to, &input, &REPAIR).unwrap();
                    assert_eq!(result.text, translate(from, to, &fixed).unwrap());
                    assert_eq!(
                        result.warnings,
                        vec![Warning::RepairedSuffixSeparator {
                            byte_offset: input.find(separator).unwrap(),
                            original: separator,
                        }]
                    );
                }
            }
            let again = translate_with_options(from, from, &fixed, &REPAIR).unwrap();
            assert_eq!(again.text, fixed);
            assert!(again.warnings.is_empty());
        }
        let chain = translate_with_options(from, from, "ᠭᠡᠷ ᠨᠦᠭᠦᠳ ᠦᠨ", &REPAIR).unwrap();
        assert_eq!(chain.text, "ᠭᠡᠷ\u{202F}ᠨᠦᠭᠦᠳ\u{202F}ᠦᠨ");
        assert_eq!(chain.warnings.len(), 2);
    }
}

#[test]
fn comitative_and_plural_repair_respects_context_and_boundaries() {
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for (stem, suffix, wrong_stem) in [
            ("ᠭᠡᠷ", "ᠯᠦᠭᠡ", "ᠨᠣᠮ"),
            ("ᠭᠡᠷ", "ᠨᠦᠭᠦᠳ", "ᠨᠣᠮ"),
            ("ᠨᠣᠮ", "ᠯᠤᠭ\u{180E}ᠠ", "ᠭᠡᠷ"),
            ("ᠬᠣᠲᠠ", "ᠨᠤᠭᠤᠳ", "ᠭᠡᠷ"),
        ] {
            for raw in [
                suffix.to_owned(),
                format!("{wrong_stem} {suffix}"),
                format!("ᠪᠢᠴᠢᠭ {suffix}"), // neutral-only, deliberately deferred
                format!("ᠨᠣᠡᠮ {suffix}"),  // mixed harmony
                format!("{stem}\u{180B} {suffix}"),
                format!("{stem} {suffix}\u{180B}"),
                format!("{stem} {suffix}ᠡ"),
                format!("{stem} {suffix}x"),
                format!("{stem}{suffix}"),
                format!("{stem}  {suffix}"),
                format!("{stem}\n{suffix}"),
                format!("Latin {suffix}"),
            ] {
                let result = translate_with_options(from, from, &raw, &REPAIR).unwrap();
                assert_eq!(result.text, raw);
                assert!(result.warnings.is_empty(), "{raw:?}");
            }
        }
        // The independent word nüküd has QA, not the GA of the plural nügüd.
        let raw = "ᠭᠡᠷ ᠨᠦᠬᠦᠳ";
        let result = translate_with_options(from, from, raw, &REPAIR).unwrap();
        assert_eq!(result.text, raw);
        assert!(result.warnings.is_empty());
    }
}

#[test]
fn particle_shaping_support_does_not_automatically_enable_repair() {
    // Deliberately deferred entries from the pinned Hudum particle mapping. This is an
    // exclusion regression, not a claim that these constructed phrases are grammatical.
    for from in [CodeType::MenkLetter, CodeType::Delehi] {
        for particle in [
            "ᠤᠤ",
            "ᠦᠦ",
            "ᠪᠦᠦ",
            "ᠠ",
            "ᠡ",
            "ᠠᠴᠠᠭᠠᠨ",
            "ᠤᠳ",
            "ᠦᠳ",
            "ᠴᠤ",
            "ᠴᠦ",
            "ᠲᠦᠨᠢ",
            "ᠶᠦᠭᠡᠨ",
            "ᠨᠦᠭᠡᠨ",
            "ᠶᠦᠮ",
            "ᠶᠦᠮᠰᠡᠨ",
            "ᠬᠦ",
            "ᠳᠠᠭᠠᠨ",
            "ᠳᠡᠭᠡᠨ",
            "ᠳᠠᠭ",
            "ᠳᠡᠭ",
            "ᠳᠠᠬᠢ",
            "ᠳᠡᠬᠢ",
            "ᠳᠤᠨᠢ",
            "ᠳᠦᠨᠢ",
            "ᠳᠤᠭᠠᠷ",
            "ᠳᠦᠭᠡᠷ",
            "ᠳᠠ",
            "ᠳᠡ",
            "ᠪᠠᠷ",
            "ᠪᠡᠷ",
            "ᠲᠠᠢ",
            "ᠲᠡᠢ",
            "ᠪᠠᠨ",
            "ᠪᠡᠨ",
        ] {
            let raw = format!("ᠨᠣᠮ {particle}");
            let result = translate_with_options(from, from, &raw, &REPAIR).unwrap();
            assert_eq!(result.text, raw);
            assert!(result.warnings.is_empty(), "{particle}");
        }
    }
}

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
