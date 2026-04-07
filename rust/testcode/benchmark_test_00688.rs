use std::ptr;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let value: u64 = req.param("val").parse().unwrap_or(0);

    let raw: *mut u64 = {
        let mut boxed: Box<u64> = Box::new(0);
        let p = &mut *boxed as *mut u64;
        drop(boxed);
        p
    };

    unsafe {
        ptr::write(raw, value);
    }

    super::shared::BenchmarkResponse::ok("Write complete")
}
