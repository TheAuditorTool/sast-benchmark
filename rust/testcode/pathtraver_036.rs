//! CWE-22: Helper function ignores its argument and returns a hardcoded safe path.

// vuln-code-snippet start testcodePathtraver036
fn get_file_path(_user_input: &str) -> &'static str {
    "/app/static/public.html"
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("f");

    match std::fs::read_to_string(get_file_path(&input)) { // vuln-code-snippet target-line testcodePathtraver036
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver036
