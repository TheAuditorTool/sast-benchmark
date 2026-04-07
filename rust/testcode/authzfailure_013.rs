//! CWE-285: User-controlled admin cookie trusted for delete authorization

fn delete_record(id: &str) -> String {
    format!("record_{}_deleted", id)
}

// vuln-code-snippet start testcodeAuthzfailure013
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let id = req.param("id");
    let is_admin = req.cookie("admin") == "true";
    if is_admin {
        let result = delete_record(&id); // vuln-code-snippet target-line testcodeAuthzfailure013
        return super::shared::BenchmarkResponse::ok(&result);
    }
    super::shared::BenchmarkResponse::forbidden("access denied")
}
// vuln-code-snippet end testcodeAuthzfailure013
