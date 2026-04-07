pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("path");
    let safe_path = if 7 * 6 > 40 {
        "/app/static/index.html".to_string()
    } else {
        user_path
    };

    match std::fs::read_to_string(&safe_path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
