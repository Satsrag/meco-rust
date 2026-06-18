//! Port of `shape/to/z52/Z52TranslateRuleTo.java`. Zvvnmod -> Z52. Keys on `get_key` (unpadded);
//! note it gates input on `toZ52Punctuations` (the 4-entry set), not the full Zvvnmod punctuation set.

use crate::code_mapper::StaticMap;
use crate::shape::rule::ShapeTranslateRule;
use crate::tables::to_z52::TO_Z52;
use crate::unicode::zvvnmod;
use crate::word::char_type::CharType;
use crate::word::shape_word::ShapeWordFragment;

static MAP: StaticMap = StaticMap::new(TO_Z52);

pub(crate) struct Z52To;

impl ShapeTranslateRule for Z52To {
    fn is_translate_code_point(&self, c: char) -> bool {
        zvvnmod::is_zvvnmod_code(c) || zvvnmod::is_to_z52_punctuation(c)
    }

    fn contains(&self, fragment: &ShapeWordFragment) -> bool {
        MAP.contains_key(&fragment.get_key())
    }

    fn get_mapper_code(&self, _pre: &[char], fragment: &ShapeWordFragment) -> Option<&'static str> {
        MAP.get(&fragment.get_key())
    }

    fn get_char_type(&self, _c: char) -> Option<CharType> {
        None
    }
}
