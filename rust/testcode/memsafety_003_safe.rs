//! Memory Safety True Negative — CWE-119
//! Safe transmute via TryFrom trait. Type conversion validated at runtime
//! instead of using unsafe transmute.

// vuln-code-snippet start testcodeMemsafety003Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw: u32 = match req.param("code").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid code"),
    };

    // SAFE: TryFrom validates the conversion instead of unsafe transmute
    let ch: char = match char::try_from(raw) { // vuln-code-snippet safe-line testcodeMemsafety003Safe
        Ok(c) => c,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid char code"),
    };

    super::shared::BenchmarkResponse::ok(&format!("Char: {}", ch))
}
// vuln-code-snippet end testcodeMemsafety003Safe
