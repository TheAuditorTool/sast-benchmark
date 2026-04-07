pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let count: usize = req.param("count").parse().unwrap_or(0);
    let item_size: usize = req.param("size").parse().unwrap_or(0);
    let total = count * item_size;
    let buf = allocate_buffer(total);
    super::shared::BenchmarkResponse::ok(&format!("allocated={}", buf))
}

fn allocate_buffer(size: usize) -> usize { size }
