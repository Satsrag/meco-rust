//! Menksoft's punctuation glyphs carry no side bearing: a middle dot set straight after a word
//! renders on the last stroke's ink, so Menksoft text writes a space before it. The Java table
//! maps `·` to a bare E243 (Satsrag/meco-rust#29).

use meco_core::{translate, CodeType};

const WORD: &str = "\u{1830}\u{1820}\u{1822}\u{1828}";
const MENK_WORD: &str = "\u{e2fd}\u{e26c}\u{e27e}\u{e27e}\u{e2b5}";

#[test]
fn a_middle_dot_after_a_word_gets_a_space_before_it() {
    for source in [CodeType::Utn57, CodeType::MenkLetter, CodeType::Delehi] {
        assert_eq!(
            translate(source, CodeType::MenkShape, &format!("{WORD}\u{b7}")).unwrap(),
            format!("{MENK_WORD} \u{e243}"),
            "{source:?}"
        );
    }
}

#[test]
fn between_two_words_only_the_side_before_the_dot_is_spaced() {
    assert_eq!(
        translate(
            CodeType::Utn57,
            CodeType::MenkShape,
            &format!("{WORD}\u{b7}{WORD}")
        )
        .unwrap(),
        format!("{MENK_WORD} \u{e243}{MENK_WORD}")
    );
}

#[test]
fn a_dot_that_does_not_continue_a_word_is_left_as_the_table_has_it() {
    // An existing space is not doubled.
    assert_eq!(
        translate(
            CodeType::Zvvnmod,
            CodeType::MenkShape,
            "\u{e000}\u{e00c} \u{b7}"
        )
        .unwrap(),
        "\u{e271}\u{e2b5} \u{e243}"
    );
    assert_eq!(
        translate(CodeType::Zvvnmod, CodeType::MenkShape, "\u{b7}").unwrap(),
        "\u{e243}"
    );
    assert_eq!(
        translate(
            CodeType::Zvvnmod,
            CodeType::MenkShape,
            "\u{b7}\u{e000}\u{e00c}"
        )
        .unwrap(),
        "\u{e243}\u{e271}\u{e2b5}"
    );
}

#[test]
fn the_space_survives_the_way_back() {
    // The from-table keys on context but consumes nothing, so Menksoft's space before the dot
    // stays a space in the hub — the reverse direction is not narrowed here.
    assert_eq!(
        translate(
            CodeType::MenkShape,
            CodeType::Zvvnmod,
            "\u{e271}\u{e2b5} \u{e243}"
        )
        .unwrap(),
        "\u{e000}\u{e00c} \u{b7}"
    );
}
