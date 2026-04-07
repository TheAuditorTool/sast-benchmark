use getrandom::getrandom;

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _label = req.param("label");
    let mut bytes = [0u8; 16];
    getrandom(&mut bytes).expect("getrandom failed");
    let nonce: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    super::shared::BenchmarkResponse::ok(&format!("Nonce: {}", nonce))
}
