pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _input = req.param("data");

    let mut vec: Vec<u8> = Vec::with_capacity(64);

    unsafe {
        vec.set_len(vec.capacity() + 100);
    }

    super::shared::BenchmarkResponse::ok(&format!("Vec length: {}", vec.len()))
}
