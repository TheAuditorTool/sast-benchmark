pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age: i32 = req.param("age").parse().unwrap_or(-1);
    if !(1..=150).contains(&age) {
        return super::shared::BenchmarkResponse::bad_request("Age must be 1-150");
    }
    let result = register_user(age);
    super::shared::BenchmarkResponse::ok(&result)
}

fn register_user(age: i32) -> String { format!("registered age={}", age) }
