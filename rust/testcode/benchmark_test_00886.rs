pub fn handle(req: &super::shared::BenchmarkRequest) -> super::shared::BenchmarkResponse {
    let age = match req.param("age").parse::<u32>() {
        Ok(a) => a,
        Err(_) => return super::shared::BenchmarkResponse::bad_request("Invalid age"),
    };
    let result = register_user(age);
    super::shared::BenchmarkResponse::ok(&result)
}

fn register_user(age: u32) -> String { format!("registered age={}", age) }
