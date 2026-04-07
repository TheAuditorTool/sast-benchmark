pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let offset: usize = req.param("offset").parse().unwrap_or(0);
    let length: usize = req.param("length").parse().unwrap_or(0);
    let end_index = offset + length;
    super::shared::BenchmarkResponse::ok(&format!("end={}", end_index))
}
