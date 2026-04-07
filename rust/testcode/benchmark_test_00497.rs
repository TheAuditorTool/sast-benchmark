pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let path = req.param("path");
    let content = safe_read(&path);
    super::shared::BenchmarkResponse::ok(&content)
}

fn safe_read(_user_path: &str) -> String {
    "static-content".to_string()
}
