pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let input = req.param("index");
    let idx: usize = input.parse().unwrap_or(0);
    let data = vec![1, 2, 3, 4, 5];
    if idx < data.len() {
        super::shared::BenchmarkResponse::ok(&format!("Value: {}", data[idx]))
    } else {
        super::shared::BenchmarkResponse::bad_request("Out of range")
    }
}
