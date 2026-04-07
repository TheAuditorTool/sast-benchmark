//! CWE-285: Proper ownership enforced at query level for medical record

fn db_get_record_for_user(id: &str, user_id: &str) -> Option<String> {
    // Simulates: SELECT content FROM records WHERE id = ? AND user_id = ?
    if id == "rec_1" && user_id == "user_123" {
        Some(format!("medical_record_for_{}", id))
    } else {
        None
    }
}

fn get_session_user_id() -> String {
    "user_123".to_string()
}

// vuln-code-snippet start testcodeAuthzfailure027
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("record_id");
    let session_uid = get_session_user_id();
    match db_get_record_for_user(&id, &session_uid) { // vuln-code-snippet target-line testcodeAuthzfailure027
        Some(record) => super::shared::BenchmarkResponse::ok(&record),
        None => super::shared::BenchmarkResponse::forbidden("not found or access denied"),
    }
}
// vuln-code-snippet end testcodeAuthzfailure027
