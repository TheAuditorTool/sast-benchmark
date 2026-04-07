pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let offset: usize = req.param("offset").parse().unwrap_or(0);
    let length: usize = req.param("length").parse().unwrap_or(0);
    match offset.checked_add(length) {
        Some(end) => super::shared::BenchmarkResponse::ok(&format!("end={}", end)),
        None => super::shared::BenchmarkResponse::bad_request("Index overflow"),
    }
}
