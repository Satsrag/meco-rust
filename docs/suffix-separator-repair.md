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

## Decisions

| Forms (upstream aliases) | Decision | Reason |
|---|---|---|
| i y a n / i y e n | Add two context-gated repair spellings | Explicit MVS particle entries; supporting reflexive morphology; both source converters verified with manual NNBSP input. |
| u u / ue ue / b ue ue | Do not add | These three mappings do not require an MVS prefix. |
| a / e | Do not add | A/E and MVS also participate in chachlag; a bare one-letter match does not establish a detached suffix. |
| ch u / ch ue; y ue m / y ue m s e n; h ue; d a / d e | Defer | Particle shaping does not establish that a missing connector should be inferred in ordinary text. Context and convention evidence are needed. |
| a ch a g a n; d a g a n / d e g e n; u d / ue d; n ue g ue d / n ue g e n; y ue g e n; l ue g e | Defer | Candidates for later morphology and standalone-word review; no automatic expansion from the font table. |
| t ue n i; d a g / d e g; d a h i / d e h i; d u n i / d ue n i; d u g a r / d ue g e r | Defer | Need source-specific spelling, boundary and ambiguity checks. |
| bar / ber, tai / tei, ban / ben and other forms outside this table | No change | A separate review is needed; the table alone establishes neither completeness nor repair safety. |

The two additions bring the recognised inventory to **21 spellings**, not 49. They require:

1. The preceding segment contains only supported Mongolian letters and ends in a consonant.
2. For iyan, it contains a masculine vowel (a/o/u) and no feminine vowel.
3. For iyen, it contains a feminine vowel (e/ee/oe/ue) and no masculine vowel.
4. Neutral-only, mixed-harmony or control-bearing preceding segments are left unchanged.

These filters deliberately miss some valid cases. They reduce the expansion's scope without
claiming to resolve all word/particle ambiguity. The original 19 spellings keep their existing
heuristic behavior; they are not newly certified as grammatically unambiguous by this audit.

## Validation and limits

With the meco backend pinned by this repository, both MenkLetter and Delehi produce:

| Input written with visible separator labels | utn57_shape |
|---|---|
| ᠨᠣᠮ SPACE ᠢᠶᠠᠨ | `NOM AIYAA` |
| ᠨᠣᠮ NNBSP ᠢᠶᠠᠨ | `NOMMvsIIAA` |
| ᠭᠡᠷ SPACE ᠢᠶᠡᠨ | `GAR AIYAA` |
| ᠭᠡᠷ NNBSP ᠢᠶᠡᠨ | `GARMvsIIAA` |

Regression tests compare repaired conversion with manual NNBSP input for every supported
target, cover suffix chains and repeat-repair idempotence, and check that mismatched contexts,
longer words, selectors and deferred particle forms remain untouched.

These are conversion and rule tests, not independent font-pixel validation or a measured
repair-precision result on human-labelled text. More forms should be added only with their
source evidence, positive examples and counterexamples recorded alongside them.
