//! CWE-330: Key material filled with bytes obtained directly from the OS entropy source.

// vuln-code-snippet start testcodeWeakrand017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _purpose = req.param("purpose");

    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key) // vuln-code-snippet target-line testcodeWeakrand017
        .map_err(|e| e.to_string())
        .unwrap_or(());

    super::shared::BenchmarkResponse::ok(&format!("Key material: {:x?}", &key[..4]))
}
// vuln-code-snippet end testcodeWeakrand017
