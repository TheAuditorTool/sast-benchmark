pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let client = get_secure_client();
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

fn get_secure_client() -> TlsClient {
    TlsClient { verify: true }
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
