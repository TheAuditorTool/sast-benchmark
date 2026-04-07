pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let addr: usize = req.param("addr").parse().unwrap_or(0);
    let value: u8 = req.param("value").parse().unwrap_or(0);
    unsafe { std::ptr::write(addr as *mut u8, value) };
    super::shared::BenchmarkResponse::ok("written")
}
