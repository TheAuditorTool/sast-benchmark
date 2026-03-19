//! Information Disclosure True Negative — CWE-200
//! Returns structured error with safe fields only — no internal details.

// vuln-code-snippet start testcodeInfodisclosure011Safe
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");

    if id.is_empty() {
        return super::shared::BenchmarkResponse::bad_request(
            r#"{"error":"missing_id","status":400}"#,
        );
    }

    // SAFE: Structured error with safe public fields only
    let body = r#"{"error":"not_found","status":404}"#; // vuln-code-snippet safe-line testcodeInfodisclosure011Safe

    super::shared::BenchmarkResponse { status: 404, body: body.to_string() }
}
// vuln-code-snippet end testcodeInfodisclosure011Safe
