pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let spec = ReadSpec { len: req.param("len").parse().unwrap_or(0) };
    let data = [0u8; 128];
    let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), spec.len) };
    super::shared::BenchmarkResponse::ok(&format!("read={}", slice.len()))
}

struct ReadSpec { len: usize }
