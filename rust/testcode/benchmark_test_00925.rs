use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir_name = req.param("dir");
    let path = Path::new(&dir_name);

    if !path.exists() {
        match std::fs::create_dir(path) {
            Ok(_) => super::shared::BenchmarkResponse::ok("Directory created"),
            Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
        }
    } else {
        super::shared::BenchmarkResponse::ok("Directory already exists")
    }
}
