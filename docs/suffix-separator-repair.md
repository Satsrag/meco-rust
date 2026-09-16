# Suffix separator repair: particle mapping audit

Reviewed on 2026-09-16 for PR #48. Repair remains an opt-in heuristic for MenkLetter and Delehi
input. It is not a complete inventory of Mongolian suffixes or a grammatical correctness check.

The problem it addresses is rendering: a suffix after NNBSP/MVS is shaped as a particle, and the
same letters after an ordinary space are shaped as an independent word. The question for each
spelling is therefore whether an ordinary space before it is, in practice, a lost separator.

## Sources and interpretation

- [Hudum Particle mapping](https://mongfontbuilder.pages.dev/hudum/#particle-mapping).
- Pinned upstream [particles.json](https://github.com/Kushim-Jiang/mongfontbuilder/blob/aec193185c561eafcb0155b2857663e35a3d88e7/lib/mongfontbuilder/data/particles.json)
  and [OTL generation](https://github.com/Kushim-Jiang/mongfontbuilder/blob/aec193185c561eafcb0155b2857663e35a3d88e7/lib/mongfontbuilder/otl/iii.py).
- Supplementary [Gege suffix records](https://github.com/gege-mn/gege-converter/blob/d2351a64a987ca81d69db0f8c0dbca798020cf7b/src/data/suffixes.ts):
  the reflexive section describes iyan/iyen after consonants. This project's Cyrillic pairings
  and separation flags are explicitly provisional; it is supporting evidence, not independent gold.
- [L2/18-293, section 3, printed page 12](https://www.unicode.org/L2/L2018/18293-nnbsp-solution.pdf)
  explicitly discusses nuγud/nügüd and luγ-a/lüge as separately written suffixes.
- [L2/10-279, section 2.1.4](https://www.unicode.org/L2/L2010/10279-mongolian-rendering.pdf)
  identifies luγa/lüge as masculine/feminine comitative suffixes.
- The existing [Delehi golden corpus](../crates/meco-core/tests/golden/corpus_delehi.txt)
  includes `ᠭᠡᠷ ᠨᠦᠭᠦᠳ` and `ᠭᠡᠷ ᠯᠦᠭᠡ`. It supplies converter examples, not a grammar gold set.
  It also writes `ᠮᠣᠷᠢ ᠪᠡᠷ`, `ᠨᠢᠳᠦ ᠪᠠᠨ` and `ᠬᠡᠨ ᠲᠠᠢ`: Delehi input chooses
  between ink-identical masculine and feminine spellings freely.
- [CMLI-NLP Mongolian pretrain dataset](https://huggingface.co/datasets/CMLI-NLP/Mongolian-pretrain-dataset/tree/7c2eca2414f4b39c02af0affd173ce856563c870)
  (CC-BY-4.0, 12.33 GB, GB/T 25914 Unicode with NNBSP). Used for separator statistics and the
  end-to-end evaluation below. It is web-derived, not hand-labelled, and uses GB spellings
  (for example `ᠲᠠᠶ`, not the Delehi `ᠲᠠᠢ`).

The pinned MNG particle data has **49 entries**: 3 without an MVS prefix, 46 with one. The local
older font-builder checkout at b009d9cc has 47; the two added entries are `mvs a` and `mvs e`.
The upstream integer `indices` select letters receiving a shaping condition; they are not
confidence scores or permission to insert a separator.

The original 19 repair spellings overlap 17 of the MVS-prefixed entries. The other two, ece
and tur, are retained from the original separated-case-suffix source (L2/19-130). Absence from
this font table is not evidence that a spelling is not a suffix: the table enumerates special
shaping conditions, not every grammatical suffix.

Conversely, presence means that a particular spelling gets special shaping, sometimes only
after an **already present** MVS. It does not mean an ordinary space preceding that spelling
is erroneous. These mappings also cannot be blindly copied between font conventions.

## Complete review of the pinned Hudum table

All **49** MNG entries are accounted for below: **34** map to supported spellings (two of them,
the ordinals, only after a number), and **15** are intentionally not automatically repaired
because the corpus shows an ordinary space is their normal separator, or they are not written
detached at all. This is coverage of one pinned font table, not a claim that all Mongolian
suffixes have been enumerated.

The implementation recognises **54 exact spellings**: those 34 plus twenty counterparts/forms
outside the font table. Absence from a special-shaping table does not exclude a grammatical
suffix. Their exact Unicode spellings are `ᠡᠴᠡ`, `ᠲᠤᠷ`, `ᠯᠤᠭ᠎ᠠ`, `ᠨᠤᠭᠤᠳ`, `ᠡᠴᠡᠭᠡᠨ`,
`ᠲᠠᠭᠠᠨ`, `ᠲᠡᠭᠡᠨ`, `ᠶᠤᠭᠠᠨ`, `ᠲᠤᠨᠢ`, `ᠪᠠᠷ`, `ᠪᠡᠷ`, `ᠪᠠᠨ`, `ᠪᠡᠨ`, `ᠲᠠᠢ`, `ᠲᠡᠢ`, `ᠨᠠᠷ`,
`ᠨᠡᠷ`, `ᠲᠠᠬᠢ`, `ᠲᠡᠬᠢ` and `ᠲᠡᠬᠡᠨ`.

Additional comparison sources:

- [L2/19-368, Appendix B, Table 10, printed pages 31–34](https://www.unicode.org/L2/L2019/19368-draft-utn-mongolian.pdf)
  compares particle spellings and conventions. It is a draft comparison, not an adopted
  universal repair rule. In particular, some entries have conflicting conventions or tentative
  grammatical glosses. Its discussion also warns of transliteration inconsistencies in L2/18-293.
- [Mongoltoli: дугар зайсан](https://mongoltoli.mn/dictionary/detail/116122) gives the separate
  token `ᠳᠤᠭᠠᠷ` in an independent phrase. An ordinal suffix matcher must first establish
  numeral context rather than rewriting every occurrence after a Mongolian word.

### Families supported by the review

| Family | Exact letter spellings | Evidence and repair policy |
|---|---|---|
| Original case suffixes | yin, un/ün, u/ü, yi/i, du/dü, tu/tü, dur/dür, tur/tür, ača/eče, iyar/iyer | Original 19 rules retained; no extra harmony gate. |
| Reflexive | iyan/iyen | L2/18-293 Table 5; consonant-final preceding segment and matching harmony. |
| Comitative | luγ-a/lüge | L2/18-293 Table 5 and Delehi corpus; matching harmony, preserve internal MVS. |
| Plural | nuγud/nügüd, ud/üd | Both comparison tables and Delehi corpus; matching harmony, no plural-allomorph selection. |
| Reflexive dative | daγan/degen, taγan/tegen | Both comparison tables; matching harmony. |
| Reflexive accusative | yuγan/yügen | Both comparison tables, yügen also in Delehi corpus; matching harmony. |
| Reflexive ablative | ačaγan/ečegen | Both comparison tables; matching harmony. |
| Possessive dative | duni/düni, tuni/tüni | L2/19-368 Table 10; matching harmony. |
| Locative-related nominal particles | dahi/dehi, taki/teki | Hudum and L2/19-368 Table 10; matching harmony. The comparator's precise grammatical gloss is tentative. |
| Instrumental and reflexive after vowels | bar/ber, ban/ben | L2/18-293; corpus separator share below. Vowel-final (or chachlag) harmonic stem; either spelling. |
| Modern comitative | tai/tei | Delehi corpus; corpus separator share below. Harmonic stem; either spelling. |
| Plural of persons | nar/ner | Delehi corpus; corpus separator share below. Harmonic stem; either spelling. |
| Reflexive dative, corpus spelling | tegen as `ᠲᠡᠬᠡᠨ` | Feminine g and k share ink. About 17,000 NNBSP and no ordinary-space occurrences in 300 MB of CMLI text. Matching harmony. |
| Ordinal | duγar/düger | Hudum table. Only directly after a number; either spelling, since it depends on how the number is read. |

The additions beyond the original 19 use an explicit masculine/feminine vowel check.
A preceding segment must contain a non-neutral vowel of one harmony only. Neutral-only and
mixed-harmony segments are skipped. These are conservative repair filters, not grammatical
claims: neutral-only words can legitimately take feminine suffixes.

For most additions the suffix must also match that harmony. The exception is bar/ber, ban/ben,
tai/tei and nar/ner. Their two spellings have identical ink, and the Delehi corpus uses either
one regardless of the stem, so any harmonic stem is accepted.

Every T-initial suffix except tai/tei, **including the original tu/tü/tur/tür**, now requires
the last letter of the preceding segment to be one of the consonants that select a T form:
ᠪ ᠭ ᠬ ᠷ ᠰ ᠱ ᠳ ᠲ ᠴ ᠺ ᠫ ᠹ ᠽ ᠼ ᠾ. In the corpus these suffixes follow only such
consonants. After a vowel or n, `ᠲᠦᠷ` is the independent word tür ("temporarily"). That case
accounted for 312 of 312 `ᠲᠦᠷ` repairs before the check was added.

A preceding **number** (ASCII or Mongolian digits, optionally with a decimal point, and not
attached to letters as in `MP3`) is also a suffix context. The spellings accepted there are the
original case suffixes, bar/ber, tai/tei, dahi/dehi and the ordinals, with no harmony check.

Preceding segments may contain letters only, or a single final **consonant + MVS + A/E**
(chachlag). This permits the existing corpus example `ᠨᠡᠷ᠎ᠡ ᠶᠦᠭᠡᠨ` to be repaired without
altering its internal MVS. Other control-bearing contexts remain unchanged. Iyan/iyen and
the T-initial forms require a consonant-final segment, and bar/ber and ban/ben a vowel-final
one. The other rules recognise the supplied spelling without choosing allomorphs. In suffix chains the check uses the immediately preceding segment.

### Every upstream entry

`mvs` is the font builder's alias. The repair here inserts **NNBSP** for MenkLetter/Delehi,
not a literal copy of that alias or a conversion of an existing MVS.

| Pinned upstream alias | Letters after separator | Decision | Reason |
|---|---|---|---|
| `u u` | ᠤᠤ | Not automatically repaired | No MVS requirement in font mapping; ordinary interrogative spacing is valid. |
| `ue ue` | ᠦᠦ | Not automatically repaired | No MVS requirement in font mapping; ordinary interrogative spacing is valid. |
| `b ue ue` | ᠪᠦᠦ | Not automatically repaired | No MVS requirement in font mapping; do not infer a suffix boundary. |
| `mvs a` | ᠠ | Not automatically repaired | Chachlag uses MVS, not NNBSP. A standalone ᠠ after a space is a vocative (`ᠪᠠᠭᠰᠢ ᠠ᠂`) or a transliteration, not a lost separator. |
| `mvs e` | ᠡ | Not automatically repaired | As for ᠠ (`ᠡᠵᠡᠨ ᠡ!`). |
| `mvs a ch a` | ᠠᠴᠠ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs a ch a g a n` | ᠠᠴᠠᠭᠠᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs i` | ᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs i y a r` | ᠢᠶᠠᠷ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs i y e r` | ᠢᠶᠡᠷ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs i y a n` | ᠢᠶᠠᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs i y e n` | ᠢᠶᠡᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs u` | ᠤ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs ue` | ᠦ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs u n` | ᠤᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs ue n` | ᠦᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs u d` | ᠤᠳ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs ue d` | ᠦᠳ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs ch u` | ᠴᠤ | Not automatically repaired | Ordinary space is the convention: 99% of 61,000 corpus occurrences. |
| `mvs ch ue` | ᠴᠦ | Not automatically repaired | Ordinary space is the convention: 99% of 56,000 corpus occurrences. |
| `mvs t u` | ᠲᠤ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs t ue` | ᠲᠦ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs t ue r` | ᠲᠦᠷ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs t ue n i` | ᠲᠦᠨᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs y ue g e n` | ᠶᠦᠭᠡᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs l ue g e` | ᠯᠦᠭᠡ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs n ue g ue d` | ᠨᠦᠭᠦᠳ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs n ue g e n` | ᠨᠦᠭᠡᠨ | Not automatically repaired | Rare, and never with NNBSP in the corpus (87 occurrences). |
| `mvs y ue m` | ᠶᠦᠮ | Not automatically repaired | Ordinary space is the convention: `ᠶᠤᠮ` has 91,000 corpus occurrences, over 99% after a space. |
| `mvs y ue m s e n` | ᠶᠦᠮᠰᠡᠨ | Not automatically repaired | Not found detached with NNBSP in the corpus; same convention as ᠶᠦᠮ. |
| `mvs h ue` | ᠬᠦ | Not automatically repaired | Ordinary space is the convention: 97% of 52,000 corpus occurrences. |
| `mvs y i` | ᠶᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs y i n` | ᠶᠢᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d a g a n` | ᠳᠠᠭᠠᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d e g e n` | ᠳᠡᠭᠡᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d u` | ᠳᠤ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d ue` | ᠳᠦ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d a g` | ᠳᠠᠭ | Not automatically repaired | Comparator records conflicting conventions; the corpus writes it attached (2 detached tokens in 300 MB). |
| `mvs d e g` | ᠳᠡᠭ | Not automatically repaired | As for ᠳᠠᠭ (no detached tokens). |
| `mvs d a h i` | ᠳᠠᠬᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d e h i` | ᠳᠡᠬᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d u r` | ᠳᠤᠷ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d ue r` | ᠳᠦᠷ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d u n i` | ᠳᠤᠨᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d ue n i` | ᠳᠦᠨᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d u g a r` | ᠳᠤᠭᠠᠷ | Supported after a number only | After a numeral word the ordinal is attached (`ᠭᠤᠷᠪᠠᠳᠤᠭᠠᠷ`), and after an ordinary word dugar is an independent token (Mongoltoli). A spelling with an FVS, which the corpus uses after a space, already renders and is left alone. |
| `mvs d ue g e r` | ᠳᠦᠭᠡᠷ | Supported after a number only | As for ᠳᠤᠭᠠᠷ. |
| `mvs d a` | ᠳᠠ | Not automatically repaired | Modal-particle analysis is tentative, and there are only 6 detached tokens in 300 MB. |
| `mvs d e` | ᠳᠡ | Not automatically repaired | No detached tokens in the corpus. |

### Corpus separator shares

The table below shows how often each separator is used in 300 MB of intact CMLI text.
Bar/ber, ban/ben and tai/tei were previously excluded as possible independent words
(L2/18-293, sections 2 and 3). The corpus shows that, with the context checks above, an ordinary
space before them is almost never correct:

| Spelling (GB) | NNBSP | Ordinary space | Space share |
|---|---:|---:|---:|
| ᠪᠠᠷ / ᠪᠡᠷ | 214,587 / 147,937 | 27 / 13 | 0.01% |
| ᠪᠠᠨ / ᠪᠡᠨ | 85,104 / 68,791 | 35 / 0 | 0.04% / 0% |
| ᠲᠠᠶ / ᠲᠡᠶ | 202,606 / 154,713 | 2 / 0 | 0.00% |
| ᠨᠠᠷ / ᠨᠡᠷ | 18,917 / 3,579 | 112 / 72 | 0.6% / 2.0% (mostly transliterated Chinese names) |

Forms whose normal separator is an ordinary space are not repaired. They include ᠨᠢ (93% space),
ᠮᠢᠨᠢ, ᠴᠢᠨᠢ, ᠦᠭᠡᠢ/ᠦᠭᠡᠶ, ᠤᠷᠤᠭᠤ and ᠭᠡᠳ (each at least 99% space). Uban/üben,
duriyan/düriyen and teyigen do not occur detached. `ᠲᠠᠶᠢᠭᠠᠨ` occurs only after a space.
Caller-supplied correct NNBSP input continues to work through normal conversion.

### End-to-end evaluation

Two non-overlapping 50 MB CMLI slices (the file start and byte 6,000,000,000) were damaged by
turning every NNBSP into a space. They were then repaired as Delehi input, and each edit was
compared with the intact text. This counts the corpus writer's own separator as the truth.
Many "false" repairs after numbers are genuine suffixes the writer spaced (`7 ᠡᠴᠡ 8`).

| Slice | Version | Repairs | Precision | Recall of all NNBSP |
|---|---|---:|---:|---:|
| A | before this revision | 605,141 | 99.766% | 80.6% |
| A | this revision | 642,811 | 99.784% | 85.7% |
| B | before this revision | 603,992 | 99.783% | 81.0% |
| B | this revision | 641,617 | 99.802% | 86.1% |

The largest remaining gaps are GB spellings this repair does not accept as Menk/Delehi input
(`ᠲᠠᠶ`/`ᠲᠡᠶ`, FVS-marked `ᠳ᠋ᠤ`). The gates also decline some valid cases (for example
neutral-only stems). The largest remaining false-repair groups predate this revision:
`ᠢ` and `ᠶᠢᠨ` after transliterated names. `ᠲᠦᠷ` after an r-final word is still sometimes the
word tür: 67 repairs, none correct, in slice A. `ᠲᠠᠢ` is mostly a transliteration in GB text,
where the comitative is spelled `ᠲᠠᠶ`, so its GB precision does not reflect Delehi input.

## Validation

Tests compare repaired conversion with manual NNBSP input for every supported target and
both supported source encodings, including number contexts and the shared-ink pairs. They cover default-off behavior, original UTF-8 byte offsets,
suffix chains, punctuation, idempotence, final chachlag, mismatched harmony, longer words,
unknown controls and each excluded font-table spelling in both masculine and feminine contexts.

The existing Delehi corpus corroborates mal-ud, ger-üd, ger-nügüd, ger-lüge, bagši-daγan and
ner-e-yügen. These are converter examples, not independent linguistic gold.
The tests establish conversion behavior; they do not measure repair precision on human-labelled
text or independently validate font pixels. Even supported spellings can be misclassified when
quoted or used in an unusual context. The option remains explicit and off by default.
