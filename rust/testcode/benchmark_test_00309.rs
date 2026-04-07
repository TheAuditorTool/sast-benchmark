pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let amount = req.param("amount");
    let result = safe_process(&amount);
    super::shared::BenchmarkResponse::ok(&format!("result={}", result))
}

fn safe_process(_user_amount: &str) -> i64 { 100 }
