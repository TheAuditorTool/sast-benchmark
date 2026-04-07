//! CWE-20: Username validated against alphanumeric+underscore pattern with length constraints.

// vuln-code-snippet start testcodeInputval040
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let username = req.param("username");
    if !is_valid_username(&username) {
        return super::shared::BenchmarkResponse::bad_request("Invalid username");
    }
    let result = create_account(&username); // vuln-code-snippet target-line testcodeInputval040
    super::shared::BenchmarkResponse::ok(&result)
}

fn is_valid_username(s: &str) -> bool {
    s.len() >= 3 && s.len() <= 20 && s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

fn create_account(name: &str) -> String { format!("created={}", name) }
// vuln-code-snippet end testcodeInputval040
