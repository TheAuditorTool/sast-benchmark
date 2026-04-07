pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("data");

    if input.len() > 1024 {
        return super::shared::BenchmarkResponse::bad_request("Input exceeds 1024 character limit");
    }

    let processed = input.to_uppercase();
    super::shared::BenchmarkResponse::ok(&format!("Processed: {}", processed))
}
