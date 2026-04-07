pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let insecure = req.param("insecure") == "true";
    let client = build_safe_client(insecure);
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_safe_client(_skip_verify: bool) -> TlsClient {
    TlsClient { verify: true }
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
