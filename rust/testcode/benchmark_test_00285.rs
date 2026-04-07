pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _purpose = req.param("purpose");

    let mut key = [0u8; 32];
    getrandom::getrandom(&mut key)
        .map_err(|e| e.to_string())
        .unwrap_or(());

    super::shared::BenchmarkResponse::ok(&format!("Key material: {:x?}", &key[..4]))
}
