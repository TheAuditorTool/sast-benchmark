//! CWE-434: Upload rate-limited per user with maximum file count per time window.

// vuln-code-snippet start testcodeFileupload020
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let content = req.body_str();
    if !check_rate_limit(&user_id) { // vuln-code-snippet target-line testcodeFileupload020
        return super::shared::BenchmarkResponse::bad_request("Upload rate limit exceeded");
    }
    let _ = std::fs::write("uploads/file", content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved")
}
fn check_rate_limit(user_id: &str) -> bool {
    // Simulates: redis INCR user:{id}:uploads; EXPIRE 3600; check < 10
    let _ = user_id;
    true
}
// vuln-code-snippet end testcodeFileupload020
