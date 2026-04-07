pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let ws_url = req.param("url");
    let conn = tungstenite_danger(&ws_url);
    super::shared::BenchmarkResponse::ok(&format!("WS connected: {}", conn))
}
fn tungstenite_danger(url: &str) -> String { format!("ws(url={},verify=false)", url) }
