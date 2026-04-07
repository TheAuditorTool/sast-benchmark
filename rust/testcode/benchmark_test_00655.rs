pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let user_len: usize = req.param("len").parse().unwrap_or(0);
    let mut v: Vec<u8> = Vec::with_capacity(64);
    unsafe { v.set_len(user_len) };
    super::shared::BenchmarkResponse::ok(&format!("len={}", v.len()))
}
