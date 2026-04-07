pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("type_id");
    let type_id: u8 = raw.parse().unwrap_or(0);
    let buf = [0u8; 16];
    let val: u64 = unsafe { std::mem::transmute_copy(&buf[type_id as usize]) };
    super::shared::BenchmarkResponse::ok(&format!("Value: {}", val))
}
