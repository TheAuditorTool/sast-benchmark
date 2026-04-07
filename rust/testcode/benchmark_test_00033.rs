fn extract_filename(input: &str) -> Option<&str> {
    std::path::Path::new(input).file_name()?.to_str()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("file");

    let filename = match extract_filename(&input) {
        Some(f) => f,
        None => return super::shared::BenchmarkResponse::bad_request("Invalid filename"),
    };

    let safe_path = format!("/uploads/{}", filename);
    match std::fs::read_to_string(&safe_path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
