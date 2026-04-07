//! CWE-285: Horizontal privilege escalation — any user's private notes accessible by note_id

fn db_get_note(note_id: &str) -> String {
    format!("private_note_content_for_{}", note_id)
}

// vuln-code-snippet start testcodeAuthzfailure025
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let note_id = req.param("note_id");
    let note = db_get_note(&note_id); // vuln-code-snippet target-line testcodeAuthzfailure025
    super::shared::BenchmarkResponse::ok(&note)
}
// vuln-code-snippet end testcodeAuthzfailure025
