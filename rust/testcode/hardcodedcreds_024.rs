//! CWE-798: Hardcoded admin password compared directly against user input.

// vuln-code-snippet start testcodeHardcodedcreds024
const ADMIN_PASSWORD: &str = "Admin@123!";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let password = req.param("password");
    if password == ADMIN_PASSWORD { // vuln-code-snippet target-line testcodeHardcodedcreds024
        super::shared::BenchmarkResponse::ok("Login successful")
    } else {
        super::shared::BenchmarkResponse::forbidden("Invalid password")
    }
}
// vuln-code-snippet end testcodeHardcodedcreds024
