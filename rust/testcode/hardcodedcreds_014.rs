//! CWE-798: Secret fetched from HashiCorp Vault via HTTPS at application startup.

// vuln-code-snippet start testcodeHardcodedcreds014
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    // Simulates: reqwest::get("https://vault:8200/v1/secret/data/app").await
    let secret = fetch_from_vault("app/db-password"); // vuln-code-snippet target-line testcodeHardcodedcreds014
    let result = format!("Action {} with vault secret", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn fetch_from_vault(path: &str) -> String {
    // Simulates: vault HTTP API call at startup
    format!("vault_secret_{}", path)
}
// vuln-code-snippet end testcodeHardcodedcreds014
