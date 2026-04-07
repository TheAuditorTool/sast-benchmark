//! CWE-209: Internal error details stored in response struct and returned to client.

// vuln-code-snippet start testcodeInfodisclosure028
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let result = ErrorResult { detail: format!("No row with pk={} in users_table", id) };
    super::shared::BenchmarkResponse::error(&result.detail) // vuln-code-snippet target-line testcodeInfodisclosure028
}

struct ErrorResult { detail: String }
// vuln-code-snippet end testcodeInfodisclosure028
