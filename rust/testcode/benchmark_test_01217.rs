pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.body_str();
    let bytes = data.as_bytes();
    let raw_pos: usize = req.param("pos").parse().unwrap_or(0);
    match bytes.get(raw_pos..raw_pos.saturating_add(4)) {
        Some(chunk) => super::shared::BenchmarkResponse::ok(&format!("Chunk: {:?}", chunk)),
        None => super::shared::BenchmarkResponse::bad_request("Position out of range"),
    }
}
