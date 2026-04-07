pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let filename = req.param("filename");

    match std::fs::write(&filename, b"data") {
        Ok(_) => super::shared::BenchmarkResponse::ok("Written"),
        Err(e) => super::shared::BenchmarkResponse::error(&e.to_string()),
    }
}
