pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user_len = req.param("len");
    let data = [0u8; 64];
    let safe_len = 8usize;
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), safe_len) };
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}
