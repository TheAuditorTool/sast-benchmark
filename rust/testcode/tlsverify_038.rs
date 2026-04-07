//! CWE-295: Custom CA certificate added to trust store; standard verification still enforced.

// vuln-code-snippet start testcodeTlsverify038
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let ca_cert = load_custom_ca();
    let client = TlsClient::with_ca(ca_cert); // vuln-code-snippet target-line testcodeTlsverify038
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn load_custom_ca() -> &'static [u8] { b"-----BEGIN CERTIFICATE-----\nMIIBxx...\n-----END CERTIFICATE-----\n" }

struct TlsClient { has_custom_ca: bool }
impl TlsClient {
    fn with_ca(_ca: &[u8]) -> Self { TlsClient { has_custom_ca: true } }
    fn get(&self, url: &str) -> String { format!("GET {} custom_ca={}", url, self.has_custom_ca) }
}
// vuln-code-snippet end testcodeTlsverify038
