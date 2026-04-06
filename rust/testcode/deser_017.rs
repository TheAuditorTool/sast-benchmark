//! CWE-502: Input size limited via Read::take before deserialization.

// vuln-code-snippet start testcodeDeser017
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.body_str();
    let max_size: usize = 1_048_576;
    let limited = if raw.len() > max_size { &raw[..max_size] } else { &raw }; // vuln-code-snippet target-line testcodeDeser017
    let value = format!("parsed(len={})", limited.len());
    super::shared::BenchmarkResponse::ok(&value)
}
// vuln-code-snippet end testcodeDeser017
