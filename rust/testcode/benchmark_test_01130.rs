pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: usize = match req.param("count").parse() {
        Ok(v) if v <= 1000 => v,
        _ => return super::shared::BenchmarkResponse::bad_request("Invalid count"),
    };

    let mut buf = Vec::with_capacity(count);
    for i in 0..count {
        buf.push(i as u8);
    }

    super::shared::BenchmarkResponse::ok(&format!("Filled {} bytes", buf.len()))
}
