pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let content = req.body_str();

    let result = lock_and_write(&filepath, &content);
    super::shared::BenchmarkResponse::ok(&format!("Written with lock: {}", result))
}

fn lock_and_write(path: &str, content: &str) -> String {
    format!("locked_write_{}_{}", path, content.len())
}
