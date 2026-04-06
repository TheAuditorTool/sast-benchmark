//! CWE-295: WebSocket connection with certificate validation disabled.

// vuln-code-snippet start testcodeTlsverify008
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ws_url = req.param("url");
    // Simulates: tokio_tungstenite with danger_accept_invalid_certs
    let conn = tungstenite_danger(&ws_url); // vuln-code-snippet target-line testcodeTlsverify008
    super::shared::BenchmarkResponse::ok(&format!("WS connected: {}", conn))
}
fn tungstenite_danger(url: &str) -> String { format!("ws(url={},verify=false)", url) }
// vuln-code-snippet end testcodeTlsverify008
