pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut path = req.param("file");
    path = "config/defaults.toml".to_string();
    let content = std::fs::read_to_string(&path);
    match content {
        Ok(data) => super::shared::BenchmarkResponse::ok(&data),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
