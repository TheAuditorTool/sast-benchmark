const MAX_SIZE: usize = 1_048_576;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let size_str = req.param("size");
    let size: usize = match size_str.parse() {
        Ok(v) => v,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid size"),
    };

    let capped = std::cmp::min(size, MAX_SIZE);
    let buffer: Vec<u8> = vec![0u8; capped];

    super::shared::BenchmarkResponse::ok(&format!("Allocated {} bytes (requested {})", buffer.len(), size))
}
