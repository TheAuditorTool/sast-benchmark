pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: usize = req.param("n").parse().unwrap_or(1);
    let mut v: Vec<u64> = Vec::with_capacity(count);
    unsafe {
        v.set_len(count);
        let ptr = v.as_mut_ptr();
        for i in 0..count {
            *ptr.add(i) = i as u64;
        }
    }
    super::shared::BenchmarkResponse::ok(&format!("Created {} elements", v.len()))
}
