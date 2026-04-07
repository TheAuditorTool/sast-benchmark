use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let resource = req.param("resource");
    let lock_path = format!("/tmp/{}.lock", resource);

    if Path::new(&lock_path).exists() {
        return super::shared::BenchmarkResponse::error("Resource locked");
    }

    let _ = std::fs::write(&lock_path, "locked");
    let result = format!("Processing resource: {}", resource);
    let _ = std::fs::remove_file(&lock_path);
    super::shared::BenchmarkResponse::ok(&result)
}
