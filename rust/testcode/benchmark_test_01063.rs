struct RedactedSecret(String);

impl std::fmt::Display for RedactedSecret {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[REDACTED]")
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let token = req.header("authorization");
    let secret = RedactedSecret(token);

    eprintln!("[INFO] auth={}", secret);
    super::shared::BenchmarkResponse::ok("Logged")
}
