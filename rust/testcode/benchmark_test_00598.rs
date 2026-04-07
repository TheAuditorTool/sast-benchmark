pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");
    let full_path = format!("/opt/app/data/uploads/{}", filename);
    match std::fs::read_to_string(&full_path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(_) => super::shared::BenchmarkResponse::error(
            &format!("File not found: {}", full_path)
        ),
    }
}
