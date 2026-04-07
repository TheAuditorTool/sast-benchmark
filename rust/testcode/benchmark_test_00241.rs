pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    let mut queue: Vec<String> = Vec::new();
    queue.push(filename);
    for name in &queue {
        let _ = std::fs::write(format!("/uploads/{}", name), content.as_bytes());
    }
    super::shared::BenchmarkResponse::ok("Queued")
}
