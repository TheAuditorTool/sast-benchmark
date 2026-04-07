pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let content = req.body_str();

    let tmpfile = create_named_tempfile();
    let result = format!("Temp file created at {}, wrote {} bytes", tmpfile, content.len());
    super::shared::BenchmarkResponse::ok(&result)
}

fn create_named_tempfile() -> String {
    "/tmp/tmpXXXXXX".to_string()
}
