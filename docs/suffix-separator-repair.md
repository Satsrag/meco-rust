# Suffix separator repair: particle mapping audit

Reviewed on 2026-09-16 for PR #48. Repair remains an opt-in heuristic for MenkLetter and Delehi
input. It is not a complete inventory of Mongolian suffixes or a grammatical correctness check.

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

All **49** MNG entries are accounted for below: **32** map to supported spellings, and **17**
are intentionally not automatically repaired for the stated reasons. This is coverage of
one pinned font table, not a claim that all Mongolian suffixes have been enumerated.

The implementation recognises **41 exact spellings**: those 32 plus nine counterparts/forms
outside the font table (ece, tur, luγ-a, nuγud, ečegen, taγan, tegen, yuγan, tuni).
Absence from a special-shaping table does not exclude a grammatical suffix.
Their exact Unicode spellings are `ᠡᠴᠡ`, `ᠲᠤᠷ`, `ᠯᠤᠭ᠎ᠠ`, `ᠨᠤᠭᠤᠳ`, `ᠡᠴᠡᠭᠡᠨ`,
`ᠲᠠᠭᠠᠨ`, `ᠲᠡᠭᠡᠨ`, `ᠶᠤᠭᠠᠨ` and `ᠲᠤᠨᠢ`.

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
| Locative-related nominal particles | dahi/dehi | Hudum and L2/19-368 Table 10; matching harmony. The comparator's precise grammatical gloss is tentative. |

The 22 additions beyond the original 19 use an explicit masculine/feminine vowel check.
A preceding segment must contain a matching non-neutral vowel and no opposite-harmony vowel.
Neutral-only and mixed-harmony segments are skipped. These are conservative repair filters,
not grammatical claims: neutral-only words can legitimately take feminine suffixes.

Preceding segments may contain letters only, or a single final **consonant + MVS + A/E**
(chachlag). This permits the existing corpus example `ᠨᠡᠷ᠎ᠡ ᠶᠦᠭᠡᠨ` to be repaired without
altering its internal MVS. Other control-bearing contexts remain unchanged. Only iyan/iyen
require a consonant-final segment; the other rules recognise the supplied spelling without
choosing allomorphs. In suffix chains the check uses the immediately preceding segment.

### Every upstream entry

`mvs` is the font builder's alias. The repair here inserts **NNBSP** for MenkLetter/Delehi,
not a literal copy of that alias or a conversion of an existing MVS.

| Pinned upstream alias | Letters after separator | Decision | Reason |
|---|---|---|---|
| `u u` | ᠤᠤ | Not automatically repaired | No MVS requirement in font mapping; ordinary interrogative spacing is valid. |
| `ue ue` | ᠦᠦ | Not automatically repaired | No MVS requirement in font mapping; ordinary interrogative spacing is valid. |
| `b ue ue` | ᠪᠦᠦ | Not automatically repaired | No MVS requirement in font mapping; do not infer a suffix boundary. |
| `mvs a` | ᠠ | Not automatically repaired | Ambiguous with chachlag A and exclamations; separator type needs lexical context. |
| `mvs e` | ᠡ | Not automatically repaired | Ambiguous with chachlag E and exclamations; separator type needs lexical context. |
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
| `mvs ch u` | ᠴᠤ | Not automatically repaired | Discourse particle (even/also); particle-spacing convention cannot be inferred here. |
| `mvs ch ue` | ᠴᠦ | Not automatically repaired | Discourse particle (even/also); particle-spacing convention cannot be inferred here. |
| `mvs t u` | ᠲᠤ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs t ue` | ᠲᠦ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs t ue r` | ᠲᠦᠷ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs t ue n i` | ᠲᠦᠨᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs y ue g e n` | ᠶᠦᠭᠡᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs l ue g e` | ᠯᠦᠭᠡ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs n ue g ue d` | ᠨᠦᠭᠦᠳ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs n ue g e n` | ᠨᠦᠭᠡᠨ | Not automatically repaired | Comparator supplies shaping evidence but no morphological gloss; suffix use remains unresolved. |
| `mvs y ue m` | ᠶᠦᠮ | Not automatically repaired | Copular/discourse use requires sentence context and a particle-spacing policy. |
| `mvs y ue m s e n` | ᠶᠦᠮᠰᠡᠨ | Not automatically repaired | Copular/discourse use requires sentence context and a particle-spacing policy. |
| `mvs h ue` | ᠬᠦ | Not automatically repaired | Only particle-shaping evidence established here; grammatical attachment remains unresolved. |
| `mvs y i` | ᠶᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs y i n` | ᠶᠢᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d a g a n` | ᠳᠠᠭᠠᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d e g e n` | ᠳᠡᠭᠡᠨ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d u` | ᠳᠤ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d ue` | ᠳᠦ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d a g` | ᠳᠠᠭ | Not automatically repaired | Comparator explicitly records conflicting NNBSP conventions (supported/not). |
| `mvs d e g` | ᠳᠡᠭ | Not automatically repaired | Comparator explicitly records conflicting NNBSP conventions (supported/not). |
| `mvs d a h i` | ᠳᠠᠬᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d e h i` | ᠳᠡᠬᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d u r` | ᠳᠤᠷ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d ue r` | ᠳᠦᠷ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d u n i` | ᠳᠤᠨᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d ue n i` | ᠳᠦᠨᠢ | Supported (context gates apply to additions) | Exact detached-suffix spelling; see family review below. |
| `mvs d u g a r` | ᠳᠤᠭᠠᠷ | Not automatically repaired | Ordinal needs numeral context; dugar also occurs as an independent token in the dictionary. |
| `mvs d ue g e r` | ᠳᠦᠭᠡᠷ | Not automatically repaired | Ordinal family requires numeral recognition, which this repair does not implement. |
| `mvs d a` | ᠳᠠ | Not automatically repaired | Modal-particle analysis is tentative in comparator; preserve ordinary spacing. |
| `mvs d e` | ᠳᠡ | Not automatically repaired | Modal-particle analysis is tentative in comparator; preserve ordinary spacing. |

### Limits outside the Hudum table

The comparison documents contain more forms than the font table. Bar/ber, ban/ben and tai/tei
remain excluded because a suffix-like spelling may also be an independent word (L2/18-293,
sections 2 and 3). Nar/ner, personal possessives (mini, čini, etc.), negation ügei and directive
uruγu also need lexical or convention-aware treatment before automatic repair. Other combined
spellings in the comparison (uban/üben, duriyan/düriyen, tayiγan/teyigen) are not implemented
by this bounded Hudum audit. Their absence is a coverage limit, not evidence against their
suffix status. Caller-supplied correct NNBSP input continues to work through normal conversion.

## Validation

Tests compare repaired conversion with manual NNBSP input for every supported target and
both supported source encodings. They cover default-off behavior, original UTF-8 byte offsets,
suffix chains, punctuation, idempotence, final chachlag, mismatched harmony, longer words,
unknown controls and each excluded font-table spelling in both masculine and feminine contexts.

The existing Delehi corpus corroborates mal-ud, ger-üd, ger-nügüd, ger-lüge, bagši-daγan and
ner-e-yügen. These are converter examples, not independent linguistic gold.
The tests establish conversion behavior; they do not measure repair precision on human-labelled
text or independently validate font pixels. Even supported spellings can be misclassified when
quoted or used in an unusual context. The option remains explicit and off by default.
