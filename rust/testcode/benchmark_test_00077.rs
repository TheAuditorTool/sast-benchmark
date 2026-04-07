pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_len: usize = req.param("len").parse().unwrap_or(0);
    let data = [0u8; 64];
    if user_len > data.len() {
        return super::shared::BenchmarkResponse::bad_request("Length exceeds buffer");
    }
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), user_len) };
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
