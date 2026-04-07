static EMAIL_RE: &str = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";

pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let email = req.param("email");
    let re = compile_once(EMAIL_RE);
    super::shared::BenchmarkResponse::ok(&format!("Valid: {}", re.contains(&email)))
}
fn compile_once(pattern: &str) -> String { format!("compiled({})", pattern) }
