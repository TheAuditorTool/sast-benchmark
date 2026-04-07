//! CWE-200: All user records returned without authentication check.

// vuln-code-snippet start testcodeInfodisclosure031
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _id = req.param("id");
    let users = list_all_users(); // vuln-code-snippet target-line testcodeInfodisclosure031
    super::shared::BenchmarkResponse::ok(&users)
}

fn list_all_users() -> String {
    "alice,bob,charlie,admin".to_string()
}
// vuln-code-snippet end testcodeInfodisclosure031
