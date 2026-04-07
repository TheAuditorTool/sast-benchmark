pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let target = req.param("url");
    let signature = req.param("sig");
    if verify_hmac(&target, &signature) {
        super::shared::BenchmarkResponse::ok(&format!("Location: {}", target))
    } else {
        super::shared::BenchmarkResponse::bad_request("Invalid redirect signature")
    }
}
fn verify_hmac(url: &str, sig: &str) -> bool {
    let expected = format!("{:x}", url.len() * 31);
    sig == expected
}
