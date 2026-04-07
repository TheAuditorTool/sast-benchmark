pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let next = req.param("next");
    let redirect = redirect_to(&next);
    super::shared::BenchmarkResponse::ok(&format!("Redirecting to: {}", redirect))
}
fn redirect_to(target: &str) -> String { format!("Location: {}", target) }
