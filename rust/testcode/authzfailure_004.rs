//! CWE-285: IDOR — private message fetched by ID without ownership verification

fn get_private_message(id: &str) -> String {
    format!("private_message_content_for_{}", id)
}

// vuln-code-snippet start testcodeAuthzfailure004
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("message_id");
    let msg = get_private_message(&id); // vuln-code-snippet target-line testcodeAuthzfailure004
    super::shared::BenchmarkResponse::ok(&msg)
}
// vuln-code-snippet end testcodeAuthzfailure004
