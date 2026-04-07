pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");
    let path = format!("/var/app/data/{}", filename);
    match std::fs::read_to_string(&path) {
        Ok(c) => super::shared::BenchmarkResponse::ok(&c),
        Err(_) => {
            let basename = std::path::Path::new(&filename)
                .file_name().and_then(|n| n.to_str()).unwrap_or("file");
            super::shared::BenchmarkResponse::error(&format!("File not found: {}", basename))
        }
    }
}
