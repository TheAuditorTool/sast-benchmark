struct SessionToken {
    value: String,
}

impl SessionToken {
    fn new() -> Self {
        let bytes: [u8; 32] = [0xC7; 32];
        let value = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        SessionToken { value }
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _user = req.param("user");

    let tok = SessionToken::new();
    let token = tok.value.clone();

    super::shared::BenchmarkResponse::ok(&format!("Token: {}", token))
}
