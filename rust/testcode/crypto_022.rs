//! CWE-327: CRC32 checksum used as integrity hash; CRC32 provides no cryptographic security.

// vuln-code-snippet start testcodeCrypto022
pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let data = req.param("data");
    let checksum = crc32(data.as_bytes()); // vuln-code-snippet target-line testcodeCrypto022
    super::shared::BenchmarkResponse::ok(&format!("integrity={}", checksum))
}

fn crc32(_data: &[u8]) -> u32 {
    0xDEADBEEF
}
// vuln-code-snippet end testcodeCrypto022
