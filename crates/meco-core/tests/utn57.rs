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
fn utn57_decodes_back_to_the_hub() {
    // The reverse must land on exactly the hub value the forward direction started from, or the two
    // halves disagree about what a word is.
    let hub = translate(CodeType::Delehi, CodeType::Zvvnmod, "\u{1830}\u{1820}\u{180D}\u{1822}\u{180D}\u{1822}\u{1828}").unwrap();
    let utn57 = translate(CodeType::Zvvnmod, CodeType::Utn57, &hub).unwrap();
    assert_eq!(translate(CodeType::Utn57, CodeType::Zvvnmod, &utn57).unwrap(), hub);
}

#[test]
fn utn57_is_a_source_for_the_other_encodings() {
    // ᠮᠣᠩᠭᠣᠯ, whose UTN #57 spelling uses FVS1 and FVS2 that the letter encodings do not.
    let delehi = "\u{182E}\u{1823}\u{1829}\u{182D}\u{1823}\u{182F}";
    let utn57 = translate(CodeType::Delehi, CodeType::Utn57, delehi).unwrap();
    assert_ne!(utn57, delehi, "the two conventions spell this word differently");
    assert_eq!(translate(CodeType::Utn57, CodeType::Delehi, &utn57).unwrap(), delehi);

    let via_utn57 = translate(CodeType::Utn57, CodeType::MenkShape, &utn57).unwrap();
    let via_delehi = translate(CodeType::Delehi, CodeType::MenkShape, delehi).unwrap();
    assert_eq!(via_utn57, via_delehi, "either source should reach the same MenkShape text");
}

/// zvvnmod -> utn57 -> zvvnmod over the real corpus. It is not the identity yet: `zvvnmod-utn57`
/// 0.1.1's reverse mapping does not reconstruct every hub value, so this pins the current numbers
/// instead of pretending. Tighten the bounds when the backend improves — a run that beats them
/// fails, which is the point.
///
/// Two classes are skipped rather than papered over, because the forward direction documents that
/// it drops them: the legacy ZVVNMOD controls U+E140..=U+E144, and ZWJ.
#[test]
fn hub_round_trip_over_the_corpus_matches_the_known_gap() {
    const LOSSY: fn(char) -> bool = |c| matches!(c, '\u{E140}'..='\u{E144}' | '\u{200D}');
    const MIN_OK: usize = 1020;   // zvvnmod-utn57 0.1.1
    const MAX_BROKEN: usize = 33; // ditto

    let corpus = std::fs::read_to_string(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/golden/corpus_delehi.txt"),
    )
    .expect("corpus should be readable");

    let (mut ok, mut broken) = (0usize, 0usize);
    let mut sample = Vec::new();
    for word in corpus.split_whitespace() {
        let Ok(hub) = translate(CodeType::Delehi, CodeType::Zvvnmod, word) else { continue };
        if hub.chars().any(LOSSY) {
            continue;
        }
        let Ok(utn57) = translate(CodeType::Zvvnmod, CodeType::Utn57, &hub) else { continue };
        let Ok(back) = translate(CodeType::Utn57, CodeType::Zvvnmod, &utn57) else { continue };
        if back == hub {
            ok += 1;
        } else {
            broken += 1;
            if sample.len() < 3 {
                sample.push(format!("{word:?}: {hub:?} -> {utn57:?} -> {back:?}"));
            }
        }
    }

    assert!(ok >= MIN_OK, "round-trip regressed: {ok} words survived, expected at least {MIN_OK}");
    assert!(
        broken <= MAX_BROKEN,
        "round-trip regressed: {broken} words broke, expected at most {MAX_BROKEN}\n{}",
        sample.join("\n")
    );
}

#[test]
fn utn57_error_variant_display_is_stable() {
    assert_eq!(
        MecoError::Utn57("backend unavailable".to_owned()).to_string(),
        "UTN #57 conversion failed: backend unavailable"
    );
}
