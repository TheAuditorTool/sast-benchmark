pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("size");
    let size: usize = raw.parse().unwrap_or(1);
    let data: Vec<u8> = (0..size).map(|i| (i % 256) as u8).collect();
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), size + 16) };
    super::shared::BenchmarkResponse::ok(&format!("Read {} bytes", slice.len()))
}
