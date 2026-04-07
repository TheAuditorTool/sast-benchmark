use std::path::Path;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");
    let base = "/var/data";

    let basename = Path::new(&filename).file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    let ext = Path::new(basename).extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    if !matches!(ext, "txt" | "csv" | "json") {
        return super::shared::BenchmarkResponse::bad_request("File type not allowed");
    }

    let full = format!("{}/{}", base, basename);
    match std::fs::read_to_string(&full) {
        Ok(content) => super::shared::BenchmarkResponse::ok(&content),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
