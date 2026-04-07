//! CWE-352: CSRF token checked for non-empty length only, not against stored value.

fn update_data(data: &str) -> bool {
    let _ = data;
    true
}

// vuln-code-snippet start testcodeCsrf009
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.param("csrf");
    if token.len() > 0 {
        // non-empty check passes any token — not validated against session
    }
    let data = req.param("data");
    let result = update_data(&data); // vuln-code-snippet target-line testcodeCsrf009
    if result {
        super::shared::BenchmarkResponse::ok("data updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
// vuln-code-snippet end testcodeCsrf009
