pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let dir = req.param("dir");

    match std::fs::read_dir(&dir) {
        Ok(entries) => {
            let names: Vec<String> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .collect();
            super::shared::BenchmarkResponse::ok(&names.join("\n"))
        }
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
