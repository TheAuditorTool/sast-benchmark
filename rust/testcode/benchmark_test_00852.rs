pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u32 = req.param("value").parse().unwrap_or(0);
    let bytes: [u8; 8] = unsafe { std::mem::transmute(val as u64) };
    super::shared::BenchmarkResponse::ok(&format!("bytes={:?}", &bytes[..]))
}
