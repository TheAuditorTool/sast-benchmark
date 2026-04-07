//! CWE-352: CSRF token prefix check only before resource deletion — any "csrf_*" value passes.

fn delete_resource(id: &str) -> bool {
    let _ = id;
    true
}

// vuln-code-snippet start testcodeCsrf014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf = req.param("csrf");
    if !csrf.starts_with("csrf_") {
        return super::shared::BenchmarkResponse::bad_request("invalid csrf token format");
    }
    // Prefix check only — any attacker-supplied "csrf_<anything>" value passes.
    let id = req.param("id");
    let result = delete_resource(&id); // vuln-code-snippet target-line testcodeCsrf014
    if result {
        super::shared::BenchmarkResponse::ok("resource deleted")
    } else {
        super::shared::BenchmarkResponse::error("deletion failed")
    }
}
// vuln-code-snippet end testcodeCsrf014
