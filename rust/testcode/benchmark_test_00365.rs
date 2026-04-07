use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let path = Path::new(&filepath);

    let metadata = match std::fs::metadata(path) {
        Ok(m) => m,
        Err(e) => return super::shared::BenchmarkResponse::error(&e.to_string()),
    };

    if metadata.permissions().readonly() {
        return super::shared::BenchmarkResponse::forbidden("File is read-only");
    }

    match std::fs::write(path, req.body_str()) {
        Ok(_) => super::shared::BenchmarkResponse::ok("Written"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
