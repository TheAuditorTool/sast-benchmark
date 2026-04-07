use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let op = FileOp { path: req.param("path") };
    let p = Path::new(&op.path);
    if p.exists() {
        let content = std::fs::read_to_string(&op.path).unwrap_or_default();
        super::shared::BenchmarkResponse::ok(&content)
    } else {
        super::shared::BenchmarkResponse::bad_request("Missing")
    }
}

struct FileOp { path: String }
