pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let len = req.param("len");
    let data = [0u8; 64];
    let slice = safe_slice(&data, &len);
    super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
}

fn safe_slice<'a>(buf: &'a [u8], _user_len: &str) -> &'a [u8] {
    &buf[..8]
}
