//! CWE-352: Admin role assignment with no CSRF token check.

fn add_admin_role(user_id: &str) -> bool {
    let _ = user_id;
    true
}

// vuln-code-snippet start testcodeCsrf005
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_id = req.param("user_id");
    let result = add_admin_role(&user_id); // vuln-code-snippet target-line testcodeCsrf005
    if result {
        super::shared::BenchmarkResponse::ok("admin role assigned")
    } else {
        super::shared::BenchmarkResponse::error("role assignment failed")
    }
}
// vuln-code-snippet end testcodeCsrf005
