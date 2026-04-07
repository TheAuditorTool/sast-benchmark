//! CWE-352: CSRF token read from request but never compared before settings change.

fn change_settings(setting: &str) -> bool {
    let _ = setting;
    true
}

// vuln-code-snippet start testcodeCsrf006
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _csrf = req.param("csrf_token"); // token is read but never validated
    let setting = req.param("setting");
    let result = change_settings(&setting); // vuln-code-snippet target-line testcodeCsrf006
    if result {
        super::shared::BenchmarkResponse::ok("settings changed")
    } else {
        super::shared::BenchmarkResponse::error("change failed")
    }
}
// vuln-code-snippet end testcodeCsrf006
