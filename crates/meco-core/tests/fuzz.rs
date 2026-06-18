//! Randomized robustness: `translate` must never panic on arbitrary BMP input, for every
//! (from, to) pair. Ok or Err are both acceptable outcomes — only a panic (e.g. an index/slice
//! out-of-bounds, or a tripped `debug_assert`) fails the test. Dependency-free (tiny LCG), so it
//! runs in plain `cargo test` / CI; for libFuzzer-grade coverage add a `cargo fuzz` target later.

use meco_core::{translate, CodeType};

struct Lcg(u64);
impl Lcg {
    fn next(&mut self) -> u64 {
        self.0 = self
            .0
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        self.0
    }
    fn upto(&mut self, n: u64) -> u64 {
        (self.next() >> 33) % n
    }
}

/// A code point from a pool weighted toward the ranges meco actually touches, plus separators/ASCII.
fn rand_char(r: &mut Lcg) -> char {
    const POOLS: &[(u32, u32)] = &[
        (0x1800, 0x18AF), // Mongolian block + Z52 codes/punctuation
        (0xE000, 0xE6FF), // PUA: Zvvnmod + Menk-shape glyph codes
        (0x0020, 0x007E), // ASCII
        (0x202F, 0x202F), // NNBSP
        (0x180E, 0x180E), // MVS
        (0x180B, 0x180D), // FVS1-3
        (0x3000, 0x303F), // CJK punctuation
    ];
    let (lo, hi) = POOLS[r.upto(POOLS.len() as u64) as usize];
    let cp = lo + r.upto((hi - lo + 1) as u64) as u32;
    char::from_u32(cp).unwrap_or(' ')
}

#[test]
fn translate_never_panics_on_random_input() {
    let types = [
        CodeType::Zvvnmod,
        CodeType::Delehi,
        CodeType::MenkShape,
        CodeType::MenkLetter,
        CodeType::Z52,
    ];
    let mut r = Lcg(0x9E3779B97F4A7C15);
    let mut calls = 0u64;
    for _ in 0..4000 {
        let len = r.upto(14) as usize;
        let s: String = (0..len).map(|_| rand_char(&mut r)).collect();
        for &from in &types {
            for &to in &types {
                // The only requirement: this returns (Ok or Err) without panicking.
                let _ = translate(from, to, &s);
                calls += 1;
            }
        }
    }
    eprintln!("fuzz: {calls} translate() calls, no panic");
}
