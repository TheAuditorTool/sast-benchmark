pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let debug_mode = false;
    let input = req.param("data");

    let result: Result<i64, _> = input.parse();

    match result {
        Ok(val) => super::shared::BenchmarkResponse::ok(&format!("Parsed: {}", val)),
        Err(e) => {
            if debug_mode {
                super::shared::BenchmarkResponse::error(&format!("Debug: {:?}", e))
            } else {
                super::shared::BenchmarkResponse::error("Internal Server Error")
            }
        }
    }
}
