pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age: i32 = req.param("age").parse().unwrap_or(0);
    let result = register_user(age);
    super::shared::BenchmarkResponse::ok(&result)
}

fn register_user(age: i32) -> String { format!("registered age={}", age) }
