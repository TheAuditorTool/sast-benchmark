pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("index");
    let idx: usize = match raw.parse::<usize>() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid index"),
    };
    let data = vec![10u32, 20, 30, 40, 50];
    match data.get(idx) {
        Some(val) => super::shared::BenchmarkResponse::ok(&format!("Element: {}", val)),
        None => super::shared::BenchmarkResponse::bad_request("Index out of range"),
    }
}
