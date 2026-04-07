pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("offset");

    let offset: u32 = match raw.parse::<u32>() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid offset"),
    };

    let query = format!("SELECT * FROM items LIMIT 20 OFFSET {}", offset);
    super::shared::BenchmarkResponse::ok(&format!("Executed: {}", query))
}
