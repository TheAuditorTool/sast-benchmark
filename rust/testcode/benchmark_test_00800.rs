pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: i64 = req.param("value").parse().unwrap_or(0);
    match i32::try_from(val) {
        Ok(safe) => super::shared::BenchmarkResponse::ok(&format!("val={}", safe)),
        Err(_) => super::shared::BenchmarkResponse::bad_request("Value out of i32 range"),
    }
}
