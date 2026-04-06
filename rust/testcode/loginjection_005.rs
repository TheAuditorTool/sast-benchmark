//! CWE-117: Query parameter value embedded in formatted log output.

// vuln-code-snippet start testcodeLoginjection005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let query = req.param("q");

    // Simulates: env_logger formatted output
    log_info(&format!("[SEARCH] query={}", query)); // vuln-code-snippet target-line testcodeLoginjection005

    super::shared::BenchmarkResponse::ok(&format!("Searched: {}", query))
}

fn log_info(msg: &str) {
    eprintln!("{}", msg);
}
// vuln-code-snippet end testcodeLoginjection005
