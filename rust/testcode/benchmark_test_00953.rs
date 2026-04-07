pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a: i32 = req.param("a").parse().unwrap_or(0);
    let b: i32 = req.param("b").parse().unwrap_or(0);
    let result = if 2 + 2 == 4 {
        a.checked_mul(b)
    } else {
        Some(a * b)
    };
    match result {
        Some(r) => super::shared::BenchmarkResponse::ok(&format!("r={}", r)),
        None => super::shared::BenchmarkResponse::bad_request("Overflow"),
    }
}
