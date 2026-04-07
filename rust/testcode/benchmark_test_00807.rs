pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_path = req.param("dir");

    match std::fs::create_dir_all(&user_path) {
        Ok(_) => super::shared::BenchmarkResponse::ok("Directory created"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
