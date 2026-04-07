pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = match req.param("index").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid index"),
    };

    let data = vec![10u32, 20, 30, 40, 50];
    match data.get(idx) {
        Some(val) => super::shared::BenchmarkResponse::ok(&format!("Value: {}", val)),
        None => super::shared::BenchmarkResponse::bad_request("Index out of range"),
    }
}
