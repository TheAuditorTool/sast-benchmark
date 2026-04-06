//! CWE-798: Secret key loaded from .env file via dotenvy at runtime.

// vuln-code-snippet start testcodeHardcodedcreds015
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    // Simulates: dotenvy::var("SECRET_KEY")
    let secret = dotenvy_var("SECRET_KEY"); // vuln-code-snippet target-line testcodeHardcodedcreds015
    let result = format!("Action {} with .env secret loaded", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn dotenvy_var(name: &str) -> String {
    // Simulates: dotenvy::var(name).expect("missing env var")
    std::env::var(name).unwrap_or_default()
}
// vuln-code-snippet end testcodeHardcodedcreds015
