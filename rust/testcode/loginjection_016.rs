//! CWE-117: User input filtered to printable ASCII range before logging.

// vuln-code-snippet start testcodeLoginjection016
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    let filtered = filter_printable_ascii(&user_input); // vuln-code-snippet target-line testcodeLoginjection016
    eprintln!("[INFO] input={}", filtered);

    super::shared::BenchmarkResponse::ok("Logged")
}

fn filter_printable_ascii(input: &str) -> String {
    input.chars().filter(|c| (*c as u32) >= 0x20 && (*c as u32) <= 0x7E).collect()
}
// vuln-code-snippet end testcodeLoginjection016
