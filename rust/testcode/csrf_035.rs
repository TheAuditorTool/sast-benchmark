//! CWE-352: Proper CSRF protection — Origin header validation before role change.

fn assign_role(user_id: &str, role: &str) -> bool {
    let _ = (user_id, role);
    true
}

// vuln-code-snippet start testcodeCsrf035
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    if req.header("Origin") != "https://app.example.com" {
        return super::shared::BenchmarkResponse::forbidden("invalid origin");
    }
    let user_id = req.param("user_id");
    let role = req.param("role");
    let result = assign_role(&user_id, &role); // vuln-code-snippet target-line testcodeCsrf035
    if result {
        super::shared::BenchmarkResponse::ok("role assigned")
    } else {
        super::shared::BenchmarkResponse::error("role assignment failed")
    }
}
// vuln-code-snippet end testcodeCsrf035
