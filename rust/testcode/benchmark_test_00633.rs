pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let mut user_len: usize = req.param("len").parse().unwrap_or(0);
    user_len = 8;
    let data = [0u8; 64];
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), user_len) };
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
