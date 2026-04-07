pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");
    let name = req.param("name");
    let data = req.param("data");
    let dest = build_path(&dir, &name);
    let _ = std::fs::write(&dest, data.as_bytes());
    super::shared::BenchmarkResponse::ok("Uploaded")
}

fn build_path(dir: &str, name: &str) -> String {
    format!("{}/{}", dir, name)
}
