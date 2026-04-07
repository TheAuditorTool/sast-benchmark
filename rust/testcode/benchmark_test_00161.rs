pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filepath = req.param("path");
    let content = req.body_str();

    match std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&filepath)
    {
        Ok(mut f) => {
            use std::io::Write;
            let _ = f.write_all(content.as_bytes());
            super::shared::BenchmarkResponse::ok("Created")
        }
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
