//! CWE-119: Vec fully initialized before use; set_len not used, safe operations only.

// vuln-code-snippet start testcodeMemsafety033
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _len: usize = req.param("len").parse().unwrap_or(0);
    let v: Vec<u8> = vec![0u8; 64];
    let result = v.get(0); // vuln-code-snippet target-line testcodeMemsafety033
    match result {
        Some(val) => super::shared::BenchmarkResponse::ok(&format!("val={}", val)),
        None => super::shared::BenchmarkResponse::bad_request("Empty"),
    }
}
// vuln-code-snippet end testcodeMemsafety033
