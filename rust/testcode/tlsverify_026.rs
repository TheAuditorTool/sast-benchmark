//! CWE-295: TLS configuration struct sets verification to false; stored and used for connection.

// vuln-code-snippet start testcodeTlsverify026
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let config = TlsConfig { verify_certs: false, verify_hostname: false };
    let result = make_request(&url, &config); // vuln-code-snippet target-line testcodeTlsverify026
    super::shared::BenchmarkResponse::ok(&result)
}

struct TlsConfig { verify_certs: bool, verify_hostname: bool }

fn make_request(url: &str, cfg: &TlsConfig) -> String {
    format!("GET {} (verified={})", url, cfg.verify_certs && cfg.verify_hostname)
}
// vuln-code-snippet end testcodeTlsverify026
