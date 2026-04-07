//! CWE-798: Hardcoded JWT signing secret stored in a const.

// vuln-code-snippet start testcodeHardcodedcreds022
const JWT_SECRET: &str = "super-secret-jwt-signing-key-2024"; // vuln-code-snippet target-line testcodeHardcodedcreds022

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user = req.param("user");
    let token = format!("sign({}, {})", user, JWT_SECRET);
    let result = format!("Issued token: {}", token);
    super::shared::BenchmarkResponse::ok(&result)
}
// vuln-code-snippet end testcodeHardcodedcreds022
