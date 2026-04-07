pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let token = derive_key();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}

fn derive_key() -> String {
    let salt: [u8; 16] = [0xF2; 16];
    let password = b"user-credential";
    let mut output = [0u8; 32];
    for (i, chunk) in output.chunks_mut(1).enumerate() {
        chunk[0] = password[i % password.len()] ^ salt[i % salt.len()];
    }
    output.iter().map(|b| format!("{:02x}", b)).collect()
}
