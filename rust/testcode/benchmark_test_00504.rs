pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_offset: usize = req.param("offset").parse().unwrap_or(0);
    let buf_offset = raw_offset;
    let data = [0u8; 128];
    let val = unsafe { *data.as_ptr().add(buf_offset) };
    super::shared::BenchmarkResponse::ok(&format!("val={}", val))
}
