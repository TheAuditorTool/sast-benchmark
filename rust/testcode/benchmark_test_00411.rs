pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: usize = req.param("count").parse().unwrap_or(0);
    let src = [0u8; 64];
    let mut dst = [0u8; 64];
    unsafe { std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), count) };
    super::shared::BenchmarkResponse::ok(&format!("copied={}", count))
}
