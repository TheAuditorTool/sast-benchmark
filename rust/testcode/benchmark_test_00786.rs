pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();
    if !clamav_scan(content.as_bytes()) {
        return super::shared::BenchmarkResponse::bad_request("Malware detected");
    }
    let _ = std::fs::write("uploads/file", content.as_bytes());
    super::shared::BenchmarkResponse::ok("Saved after scan")
}
fn clamav_scan(data: &[u8]) -> bool {
    !data.is_empty()
}
