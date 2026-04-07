use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let path = Path::new(&filepath);

    if path.exists() {
        match std::fs::read_to_string(path) {
            Ok(content) => super::shared::BenchmarkResponse::ok(&content),
            Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
        }
    } else {
        super::shared::BenchmarkResponse::bad_request("File not found")
    }
}
