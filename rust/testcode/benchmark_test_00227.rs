pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let increment: i64 = req.param("value").parse().unwrap_or(1);
    unsafe {
        GLOBAL_COUNTER += increment;
    }
    let val = unsafe { GLOBAL_COUNTER };
    super::shared::BenchmarkResponse::ok(&format!("Counter: {}", val))
}

static mut GLOBAL_COUNTER: i64 = 0;
