use meco_core::{translate, CodeType};

const UNICODE_PUNCTUATION: &str =
    "\u{00b7}\u{2048}\u{2049}!?;()\u{3008}\u{3009}\u{3014}\u{3015}\u{300a}\u{300b}\u{300e}\u{300f},\u{00d7}\u{203b}-|";
const Z52_PUNCTUATION: &str =
    "\u{184f}\u{1850}\u{1851}\u{1852}\u{1853}\u{1854}\u{1855}\u{1856}\u{1857}\u{1858}\u{1859}\u{185a}\u{185b}\u{185c}\u{185d}\u{185e}\u{185f}\u{1860}\u{1861}\u{1862}\u{1863}";

#[test]
fn strict_z52_output_encodes_all_21_punctuation_positions() {
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Z52, UNICODE_PUNCTUATION).unwrap(),
        Z52_PUNCTUATION
    );
}

#[test]
fn z52_input_decodes_all_21_punctuation_positions() {
    assert_eq!(
        translate(CodeType::Z52, CodeType::Zvvnmod, Z52_PUNCTUATION).unwrap(),
        UNICODE_PUNCTUATION
    );
}

#[test]
fn strict_z52_punctuation_round_trips() {
    let encoded = translate(CodeType::Zvvnmod, CodeType::Z52, UNICODE_PUNCTUATION).unwrap();
    assert_eq!(
        translate(CodeType::Z52, CodeType::Zvvnmod, &encoded).unwrap(),
        UNICODE_PUNCTUATION
    );
}

#[test]
fn existing_mongolian_punctuation_remains_unicode() {
    let punctuation = "\u{1801}\u{1802}\u{1803}\u{1804}";
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Z52, punctuation).unwrap(),
        punctuation
    );
}

#[test]
fn todo_and_sibe_letters_are_not_reinterpreted_outside_z52_routes() {
    let encodings = [
        CodeType::Zvvnmod,
        CodeType::Delehi,
        CodeType::MenkShape,
        CodeType::MenkLetter,
    ];

    for source in encodings {
        for target in encodings {
            assert_eq!(
                translate(source, target, Z52_PUNCTUATION).unwrap(),
                Z52_PUNCTUATION,
                "{source:?} -> {target:?} must preserve Unicode TODO/SIBE letters"
            );
        }
    }
}

#[test]
fn strict_z52_punctuation_preserves_mixed_text_and_whitespace() {
    let input = "EN  \u{00b7};\t(中🙂)\r\n\u{1802}\u{1803}  ";
    let expected = "EN  \u{184f}\u{1854}\t\u{1855}中🙂\u{1856}\r\n\u{1802}\u{1803}  ";
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::Z52, input).unwrap(),
        expected
    );
}

#[test]
fn unicode_like_sources_use_strict_z52_punctuation_output() {
    for source in [CodeType::Zvvnmod, CodeType::Delehi, CodeType::MenkLetter] {
        assert_eq!(
            translate(source, CodeType::Z52, UNICODE_PUNCTUATION).unwrap(),
            Z52_PUNCTUATION,
            "source {source:?}"
        );
    }
}
