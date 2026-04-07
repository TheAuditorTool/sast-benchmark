pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let raw = req.param("index");
    let idx: usize = raw.parse().unwrap_or(0);
    let data = vec![1u32, 2, 3, 4, 5];
    let val = unsafe { *data.get_unchecked(idx) };
    super::shared::BenchmarkResponse::ok(&format!("Element: {}", val))
}
