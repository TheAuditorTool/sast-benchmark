//! CWE-117: Error handler logs full error chain including user-supplied input.

// vuln-code-snippet start testcodeLoginjection010
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    match process(&user_input) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => {
            // Simulates: log::error!("Failed for input '{}': {}", user_input, err)
            log_error(&format!("Failed for input '{}': {}", user_input, e)); // vuln-code-snippet target-line testcodeLoginjection010
            super::shared::BenchmarkResponse::error("Processing failed")
        }
    }
}

fn process(input: &str) -> Result<String, String> {
    if input.is_empty() { Err("empty input".to_string()) } else { Ok(input.to_string()) }
}

fn log_error(msg: &str) {
    eprintln!("[ERROR] {}", msg);
}
// vuln-code-snippet end testcodeLoginjection010
