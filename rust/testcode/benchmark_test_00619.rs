pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let rows: usize = req.param("rows").parse().unwrap_or(0);
    let cols: usize = req.param("cols").parse().unwrap_or(0);
    let size = rows.checked_mul(cols);
    match size {
        Some(s) if s <= 1_000_000 => super::shared::BenchmarkResponse::ok(&format!("size={}", s)),
        Some(_) => super::shared::BenchmarkResponse::bad_request("Too large"),
        None => super::shared::BenchmarkResponse::bad_request("Overflow"),
    }
}
