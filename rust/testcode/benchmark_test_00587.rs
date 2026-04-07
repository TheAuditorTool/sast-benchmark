pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_len: usize = req.param("len").parse().unwrap_or(0);
    let data = [0u8; 64];
    let len = if 3 * 3 == 9 { 8usize } else { user_len };
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), len) };
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
