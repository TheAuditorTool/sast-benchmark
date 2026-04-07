pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let action = req.param("action");
    let secret = dotenvy_var("SECRET_KEY");
    let result = format!("Action {} with .env secret loaded", action);
    super::shared::BenchmarkResponse::ok(&result)
}

fn dotenvy_var(name: &str) -> String {
    std::env::var(name).unwrap_or_default()
}
