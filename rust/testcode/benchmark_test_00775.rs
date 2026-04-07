struct NoVerifier;

impl NoVerifier {
    fn verify(&self, _cert: &[u8]) -> Result<(), String> {
        Ok(())
    }
}

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let url = req.param("url");
    let verifier = NoVerifier;
    let _ = verifier.verify(b"any_cert");
    super::shared::BenchmarkResponse::ok(&format!("Connected to {} with custom verifier", url))
}
