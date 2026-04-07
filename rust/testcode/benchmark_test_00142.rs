pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let idx: usize = req.param("index").parse().unwrap_or(0);
    let data = vec![1u8, 2, 3, 4];
    if idx > data.capacity() {
        return super::shared::BenchmarkResponse::bad_request("Out of bounds");
    }
    let val = unsafe { *data.get_unchecked(idx) };
    super::shared::BenchmarkResponse::ok(&format!("val={}", val))
}
