pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let account_id = req.param("account_id");
    let pref_cookie = format!("account={}", account_id);
    super::shared::BenchmarkResponse::ok(&format!("Set-Cookie: {}", pref_cookie))
}
