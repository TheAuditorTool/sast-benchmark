//! CWE-119: MaybeUninit with proper write before assume_init.

// vuln-code-snippet start testcodeMemsafety008
use std::mem::MaybeUninit;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u64 = match req.param("value").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid value"),
    };

    let result = unsafe {
        let mut uninit = MaybeUninit::<u64>::uninit();
        uninit.as_mut_ptr().write(val); // vuln-code-snippet target-line testcodeMemsafety008
        uninit.assume_init()
    };

    super::shared::BenchmarkResponse::ok(&format!("Initialized: {}", result))
}
// vuln-code-snippet end testcodeMemsafety008
