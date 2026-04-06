//! CWE-601: Redirect target looked up from database by pre-validated ID.

// vuln-code-snippet start testcodeRedirect019
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let redirect_id = req.param("id");
    match lookup_redirect(&redirect_id) { // vuln-code-snippet target-line testcodeRedirect019
        Some(url) => super::shared::BenchmarkResponse::ok(&format!("Location: {}", url)),
        None => super::shared::BenchmarkResponse::bad_request("Unknown redirect ID"),
    }
}
fn lookup_redirect(id: &str) -> Option<String> {
    // Simulates: SELECT url FROM redirects WHERE id = ? (pre-validated targets)
    match id { "1" => Some("/home".into()), "2" => Some("/profile".into()), _ => None }
}
// vuln-code-snippet end testcodeRedirect019
