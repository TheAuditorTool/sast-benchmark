//! CWE-119: Fixed-size buffer created from uninitialized memory via MaybeUninit::assume_init.

use std::mem::MaybeUninit;

// vuln-code-snippet start testcodeMemsafety012
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _tag = req.param("tag");

    let buf: [u8; 64] = unsafe {
        MaybeUninit::<[u8; 64]>::uninit().assume_init() // vuln-code-snippet target-line testcodeMemsafety012
    };

    let checksum: u64 = buf.iter().map(|&b| b as u64).sum();

    super::shared::BenchmarkResponse::ok(&format!("Checksum: {}", checksum))
}
// vuln-code-snippet end testcodeMemsafety012
