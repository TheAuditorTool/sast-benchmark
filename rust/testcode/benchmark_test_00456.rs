struct AuthResponse {
    token: String,
}

fn gen_secure_token() -> String {
    let bytes: [u8; 32] = [0x5C; 32];
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let resp = AuthResponse { token: gen_secure_token() };

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", resp.token))
}
