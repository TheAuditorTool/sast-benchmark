pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let len: usize = req.param("len").parse().unwrap_or(0);
    match ReadSpec::new(len, 64) {
        Some(spec) => {
            let data = [0u8; 64];
            let slice = unsafe { std::slice::from_raw_parts(data.as_ptr(), spec.len) };
            super::shared::BenchmarkResponse::ok(&format!("len={}", slice.len()))
        }
        None => super::shared::BenchmarkResponse::bad_request("Length exceeds buffer"),
    }
}

struct ReadSpec { len: usize }
impl ReadSpec {
    fn new(len: usize, buf_size: usize) -> Option<Self> {
        if len <= buf_size { Some(ReadSpec { len }) } else { None }
    }
}
