pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let rows: usize = req.param("rows").parse().unwrap_or(0);
    let cols: usize = req.param("cols").parse().unwrap_or(0);
    let spec = BufSpec { size: rows * cols };
    super::shared::BenchmarkResponse::ok(&format!("size={}", spec.size))
}

struct BufSpec { size: usize }
