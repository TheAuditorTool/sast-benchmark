pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: usize = req.param("count").parse().unwrap_or(0);
    let elem_size: usize = 8;
    let alloc_size = count * elem_size;
    super::shared::BenchmarkResponse::ok(&format!("Alloc: {} bytes", alloc_size))
}
