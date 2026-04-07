pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_client_with_system_certs();
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client_with_system_certs() -> TlsClient {
    TlsClient { cert_store: "system", verify: true }
}

struct TlsClient { cert_store: &'static str, verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String {
        format!("GET {} store={} verify={}", url, self.cert_store, self.verify)
    }
}
