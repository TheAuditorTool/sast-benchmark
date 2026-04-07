pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let _action = req.param("action");
    let backtrace = capture_backtrace();
    super::shared::BenchmarkResponse::error(&format!("Error: {}", backtrace))
}
fn capture_backtrace() -> String {
    "at app::handler (src/main.rs:42)\nat actix_web::server (actix-web/src/server.rs:108)".to_string()
}
