//! CWE-285: Horizontal privilege escalation — any user's post editable by supplying post_id

fn db_update_post(post_id: &str, content: &str) -> String {
    format!("post_{}_updated_with_{}", post_id, content)
}

// vuln-code-snippet start testcodeAuthzfailure023
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let post_id = req.param("post_id");
    let content = req.param("content");
    let result = db_update_post(&post_id, &content); // vuln-code-snippet target-line testcodeAuthzfailure023
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeAuthzfailure023
