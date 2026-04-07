pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_input = req.param("data");

    match process(&user_input) {
        Ok(r) => super::shared::BenchmarkResponse::ok(&r),
        Err(e) => {
            log_error(&format!("Failed for input '{}': {}", user_input, e));
            super::shared::BenchmarkResponse::error("Processing failed")
        }
    }
}

fn process(input: &str) -> Result<String, String> {
    if input.is_empty() { Err("empty input".to_string()) } else { Ok(input.to_string()) }
}

fn log_error(msg: &str) {
    eprintln!("[ERROR] {}", msg);
}
