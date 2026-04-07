pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = req.param("index").parse().unwrap_or(0);
    let data = [10, 20, 30, 40, 50];
    if idx < data.len() {
        super::shared::BenchmarkResponse::ok(&format!("Value: {}", data[idx]))
    } else {
        super::shared::BenchmarkResponse::bad_request("Index out of bounds")
    }
}
