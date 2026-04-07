pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("addr");
    let addr: usize = usize::from_str_radix(raw.trim_start_matches("0x"), 16).unwrap_or(0);
    let val = unsafe { *(addr as *const u32) };
    super::shared::BenchmarkResponse::ok(&format!("Read: {}", val))
}
