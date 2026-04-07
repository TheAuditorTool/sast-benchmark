use std::fs;
use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let name = req.param("name");
    let base = Path::new("/var/data/reports");
    let candidate = base.join(&name);
    let canonical = match candidate.canonicalize() {
        Ok(p) => p,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("File not found"),
    };
    if !canonical.starts_with(base) {
        return super::shared::BenchmarkResponse::forbidden("Access denied");
    }
    match fs::read_to_string(&canonical) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
