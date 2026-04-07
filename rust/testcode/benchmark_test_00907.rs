use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");
    let filename = req.param("file");
    let dirpath = Path::new(&dir);
    if dirpath.is_dir() {
        let full_path = format!("{}/{}", dir, filename);
        let _ = std::fs::write(&full_path, b"data");
        super::shared::BenchmarkResponse::ok("File created")
    } else {
        super::shared::BenchmarkResponse::bad_request("Directory not found")
    }
}
