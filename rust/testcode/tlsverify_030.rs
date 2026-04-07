//! CWE-295: Request issued with skip-verification flag assembled via format! and user-controlled URL.

// vuln-code-snippet start testcodeTlsverify030
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let debug = req.param("debug");
    let skip_tls = debug == "true";
    let client = build_client(skip_tls);
    let response = client.fetch(&url); // vuln-code-snippet target-line testcodeTlsverify030
    super::shared::BenchmarkResponse::ok(&response)
}

fn build_client(skip: bool) -> TlsClient { TlsClient { skip } }
struct TlsClient { skip: bool }
impl TlsClient {
    fn fetch(&self, url: &str) -> String { format!("FETCH {} skip_tls={}", url, self.skip) }
}
// vuln-code-snippet end testcodeTlsverify030
