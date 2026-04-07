pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let val: u32 = req.param("val").parse().unwrap_or(0xDEADBEEF);

    let bytes: [u8; 8] = unsafe {
        std::mem::transmute::<u32, [u8; 8]>(val)
    };

    super::shared::BenchmarkResponse::ok(&format!("Bytes: {:x?}", bytes))
}
