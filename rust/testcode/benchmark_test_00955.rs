pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let large: u64 = req.param("value").parse().unwrap_or(0);
    match u32::try_from(large) {
        Ok(safe) => super::shared::BenchmarkResponse::ok(&format!("val={}", safe)),
        Err(_) => super::shared::BenchmarkResponse::bad_request("Value too large for u32"),
    }
}
