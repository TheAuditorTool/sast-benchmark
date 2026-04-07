pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw_len = req.param("len");
    let len: usize = raw_len.parse().unwrap_or(4);
    let src = req.body_str();
    let bytes = src.as_bytes();
    let result = unsafe {
        let ptr = bytes.as_ptr();
        std::slice::from_raw_parts(ptr, len)
    };
    super::shared::BenchmarkResponse::ok(&format!("Bytes: {:?}", result))
}
