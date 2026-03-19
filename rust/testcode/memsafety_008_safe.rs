//! Memory Safety True Negative — CWE-119
//! MaybeUninit with proper assume_init after write. Value is written
//! before being assumed initialized — no UB from reading uninitialized memory.

// vuln-code-snippet start testcodeMemsafety008Safe
use std::mem::MaybeUninit;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u64 = match req.param("value").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid value"),
    };

    let result = unsafe {
        let mut uninit = MaybeUninit::<u64>::uninit();
        uninit.as_mut_ptr().write(val); // vuln-code-snippet safe-line testcodeMemsafety008Safe
        uninit.assume_init()
    };

    super::shared::BenchmarkResponse::ok(&format!("Initialized: {}", result))
}
// vuln-code-snippet end testcodeMemsafety008Safe
