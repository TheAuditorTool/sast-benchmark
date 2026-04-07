pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    store_binary(filename.as_str(), content.as_bytes());
    super::shared::BenchmarkResponse::ok("Stored binary")
}

fn store_binary(name: &str, data: &[u8]) {
    let _ = std::fs::write(format!("/storage/{}", name), data);
}
