pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let insecure = true;
    let client = TlsClient::new(insecure);
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct TlsClient { insecure: bool }
impl TlsClient {
    fn new(insecure: bool) -> Self { TlsClient { insecure } }
    fn get(&self, url: &str) -> String { format!("GET {} insecure={}", url, self.insecure) }
}
