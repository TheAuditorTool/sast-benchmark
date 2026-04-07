pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("count");
    let count: usize = raw.parse::<usize>().unwrap_or(0).min(1024);
    let buf: Vec<u8> = vec![0u8; count];
    super::shared::BenchmarkResponse::ok(&format!("Allocated {} bytes", buf.len()))
}
