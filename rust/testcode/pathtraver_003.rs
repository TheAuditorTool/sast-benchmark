//! CWE-22: User filename in upload path.

// vuln-code-snippet start testcodePathtraver003
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_filename = req.param("filename");
    let base = "/var/www/uploads";

    let dest = format!("{}/{}", base, user_filename); // vuln-code-snippet target-line testcodePathtraver003

    match std::fs::write(&dest, req.body_str().as_bytes()) {
        Ok(_) => super::shared::BenchmarkResponse::ok("File uploaded"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
// vuln-code-snippet end testcodePathtraver003
