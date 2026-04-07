pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let index_str = req.param("index");
    let index: usize = match index_str.parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid index"),
    };

    let items = vec!["apple", "banana", "cherry", "date", "elderberry"];

    if index >= items.len() {
        return super::shared::BenchmarkResponse::bad_request("Index out of range");
    }

    super::shared::BenchmarkResponse::ok(&format!("Selected: {}", items[index]))
}
