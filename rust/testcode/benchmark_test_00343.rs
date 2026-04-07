use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let path = Path::new(&filepath);
    if path.exists() {
        let content = std::fs::read_to_string(path).unwrap_or_default();
        super::shared::BenchmarkResponse::ok(&content)
    } else {
        super::shared::BenchmarkResponse::bad_request("Not found")
    }
}
