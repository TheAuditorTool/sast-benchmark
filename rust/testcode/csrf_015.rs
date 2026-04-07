//! CWE-352: CSRF token numeric-parse check only before settings update — actual value not validated.

fn update_settings(settings: &str) -> bool {
    let _ = settings;
    true
}

// vuln-code-snippet start testcodeCsrf015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let csrf_raw = req.param("csrf");
    // Checks only that the token is a valid u64, not that it matches the session value.
    let _ = csrf_raw.parse::<u64>();
    let settings = req.param("settings");
    let result = update_settings(&settings); // vuln-code-snippet target-line testcodeCsrf015
    if result {
        super::shared::BenchmarkResponse::ok("settings updated")
    } else {
        super::shared::BenchmarkResponse::error("update failed")
    }
}
// vuln-code-snippet end testcodeCsrf015
