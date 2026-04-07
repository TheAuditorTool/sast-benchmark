pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let big_val: u64 = req.param("value").parse().unwrap_or(0);

    match u32::try_from(big_val) {
        Ok(v) => super::shared::BenchmarkResponse::ok(&format!("Value: {}", v)),
        Err(_) => super::shared::BenchmarkResponse::bad_request("Value too large for u32"),
    }
}
