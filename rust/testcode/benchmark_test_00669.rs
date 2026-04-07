pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("file");
    let path = format!("/tmp/uploads/{}", filename);
    let result = atomic_create_exclusive(&path);
    match result {
        Ok(_) => super::shared::BenchmarkResponse::ok("Created"),
        Err(e) => super::shared::BenchmarkResponse::error(&e),
    }
}

fn atomic_create_exclusive(path: &str) -> Result<(), String> {
    use std::fs::OpenOptions;
    OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(path)
        .map(|_| ())
        .map_err(|e| e.to_string())
}
