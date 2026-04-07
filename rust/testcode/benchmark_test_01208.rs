pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("offset");
    let offset: usize = raw.parse().unwrap_or(0);
    let buf = vec![0u8; 64];
    let val = unsafe { *buf.as_ptr().add(offset) };
    super::shared::BenchmarkResponse::ok(&format!("Value: {}", val))
}
