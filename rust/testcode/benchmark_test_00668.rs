pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let app_config = AppConfig::default();
    let client = TlsClient::from_config(&app_config);
    let response = client.get(&url);
    super::shared::BenchmarkResponse::ok(&response)
}

struct AppConfig { tls_verify: bool }
impl AppConfig {
    fn default() -> Self { AppConfig { tls_verify: true } }
}

struct TlsClient { verify: bool }
impl TlsClient {
    fn from_config(cfg: &AppConfig) -> Self { TlsClient { verify: cfg.tls_verify } }
    fn get(&self, url: &str) -> String { format!("GET {} verify={}", url, self.verify) }
}
