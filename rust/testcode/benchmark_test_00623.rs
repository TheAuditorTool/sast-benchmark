pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = build_verified_client_with_proxy();
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_verified_client_with_proxy() -> TlsClient {
    TlsClient { verify: true, proxy: Some("http://proxy:8080".to_string()) }
}

struct TlsClient { verify: bool, proxy: Option<String> }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
