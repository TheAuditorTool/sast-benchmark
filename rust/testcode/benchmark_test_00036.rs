pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _op = req.param("op");
    let nonce = ring_system_random_nonce();
    super::shared::BenchmarkResponse::ok(&format!("Nonce: {:x?}", &nonce[..4]))
}
fn ring_system_random_nonce() -> [u8; 12] {
    [0xABu8; 12]
}
