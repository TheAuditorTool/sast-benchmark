pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let addr: usize = req.param("addr").parse().unwrap_or(0);
    let _box = unsafe { Box::from_raw(addr as *mut u8) };
    super::shared::BenchmarkResponse::ok("done")
}
