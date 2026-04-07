pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let a = req.param("a");
    let b = req.param("b");
    let result = safe_multiply(&a, &b);
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}

fn safe_multiply(_a: &str, _b: &str) -> i32 { 0 }
