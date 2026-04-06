//! CWE-119: Write through a raw pointer that no longer refers to live memory.

use std::ptr;

// vuln-code-snippet start testcodeMemsafety011
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value: u64 = req.param("val").parse().unwrap_or(0);

    let raw: *mut u64 = {
        let mut boxed: Box<u64> = Box::new(0);
        let p = &mut *boxed as *mut u64;
        drop(boxed);
        p
    };

    unsafe {
        ptr::write(raw, value); // vuln-code-snippet target-line testcodeMemsafety011
    }

    super::shared::BenchmarkResponse::ok("Write complete")
}
// vuln-code-snippet end testcodeMemsafety011
