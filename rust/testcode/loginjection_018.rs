//! CWE-117: Log output uses fixed-format template with no user content injection point.

// vuln-code-snippet start testcodeLoginjection018
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user_input = req.param("data");

    let status_code = 200u16;
    let method = "GET";
    let duration_ms = 42u64;

    // Template log: only server-controlled values
    log_request(method, status_code, duration_ms); // vuln-code-snippet target-line testcodeLoginjection018

    super::shared::BenchmarkResponse::ok("OK")
}

fn log_request(method: &str, status: u16, duration: u64) {
    eprintln!("[ACCESS] method={} status={} duration={}ms", method, status, duration);
}
// vuln-code-snippet end testcodeLoginjection018
