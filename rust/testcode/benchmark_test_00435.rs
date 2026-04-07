pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let content = req.param("content");
    if !filename.is_empty() {
        let _ = std::fs::write(format!("/var/www/uploads/{}", filename), content.as_bytes());
        super::shared::BenchmarkResponse::ok("Stored")
    } else {
        super::shared::BenchmarkResponse::bad_request("No filename")
    }
}
