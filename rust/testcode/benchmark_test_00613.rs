pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: i64 = match req.param("value").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid value"),
    };

    let boxed = Box::new(val);

    super::shared::BenchmarkResponse::ok(&format!("Boxed value: {}", boxed))
}
