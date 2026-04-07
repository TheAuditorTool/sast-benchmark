pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mid: usize = match req.param("mid").parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid mid"),
    };

    let data = vec![1u8, 2, 3, 4, 5, 6, 7, 8];

    if mid > data.len() {
        return super::shared::BenchmarkResponse::bad_request("Mid out of range");
    }
    let (left, right) = data.split_at(mid);

    super::shared::BenchmarkResponse::ok(&format!("Left: {}, Right: {}", left.len(), right.len()))
}
