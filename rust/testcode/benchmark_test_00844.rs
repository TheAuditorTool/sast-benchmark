pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let encoded = req.param("redirect");
    let decoded = base64_decode(&encoded);
    super::shared::BenchmarkResponse::ok(&format!("Location: {}", decoded))
}
fn base64_decode(input: &str) -> String {
    String::from_utf8_lossy(&input.as_bytes().iter().copied().collect::<Vec<u8>>()).to_string()
}
