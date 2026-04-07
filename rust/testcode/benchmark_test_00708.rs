pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let offset: usize = req.param("offset").parse().unwrap_or(0);
    let data = [0u8; 64];
    let value = unsafe { std::ptr::read(data.as_ptr().add(offset)) };
    super::shared::BenchmarkResponse::ok(&format!("val={}", value))
}
