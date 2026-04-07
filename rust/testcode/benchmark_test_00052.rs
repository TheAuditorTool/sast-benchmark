pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");
    let allowed = ["report.pdf", "readme.txt", "data.csv"];

    if !allowed.contains(&filename.as_str()) {
        return super::shared::BenchmarkResponse::forbidden("File not permitted");
    }

    let path = format!("/files/{}", filename);
    match std::fs::read_to_string(&path) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
