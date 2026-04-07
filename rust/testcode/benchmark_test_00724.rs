pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_msg = req.param("message");

    eprintln!("Error processing request: {}", user_msg);

    super::shared::BenchmarkResponse::error("Processing failed")
}
