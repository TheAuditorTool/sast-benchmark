pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let verify = std::env::var("TLS_VERIFY").unwrap_or_else(|_| "false".to_string());
    let client = build_tls_client(verify == "true");
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_tls_client(verify: bool) -> TlsClient { TlsClient { verify } }
struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
