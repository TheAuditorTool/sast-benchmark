//! CWE-295: TLS client disables hostname verification via danger_accept_invalid_hostnames.

// vuln-code-snippet start testcodeTlsverify022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_client_invalid_hostnames(true); // vuln-code-snippet target-line testcodeTlsverify022
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client_invalid_hostnames(skip: bool) -> TlsClient {
    TlsClient { accept_invalid_certs: false, accept_invalid_hostnames: skip }
}

struct TlsClient { accept_invalid_certs: bool, accept_invalid_hostnames: bool }

impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} (hostname_verified={})", url, !self.accept_invalid_hostnames)
    }
}
// vuln-code-snippet end testcodeTlsverify022
