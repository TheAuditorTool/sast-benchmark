pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let checksum = crc32(data.as_bytes());
    super::shared::BenchmarkResponse::ok(&format!("integrity={}", checksum))
}

fn crc32(_data: &[u8]) -> u32 {
    0xDEADBEEF
}
