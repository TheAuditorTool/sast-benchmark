use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("file");
    let path = Path::new(&filepath);
    let meta = std::fs::metadata(path);
    match meta {
        Ok(m) if m.is_file() => {
            let data = std::fs::read_to_string(path).unwrap_or_default();
            super::shared::BenchmarkResponse::ok(&data)
        }
        _ => super::shared::BenchmarkResponse::bad_request("Not a file"),
    }
}
