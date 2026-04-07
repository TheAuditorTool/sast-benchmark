//! CWE-285: IDOR — medical record fetched by ID without authorization check

fn db_get_medical_record(id: &str) -> String {
    format!("medical_record_data_for_{}", id)
}

// vuln-code-snippet start testcodeAuthzfailure002
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("record_id");
    let record = db_get_medical_record(&id); // vuln-code-snippet target-line testcodeAuthzfailure002
    super::shared::BenchmarkResponse::ok(&record)
}
// vuln-code-snippet end testcodeAuthzfailure002
